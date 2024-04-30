use hudhook::ImguiRenderLoop;
use hudhook::{hooks::dx11::ImguiDx11Hooks, hooks::opengl3::ImguiOpenGl3Hooks, *};
use image::io::Reader as ImageReader;
use image::RgbaImage;
use imgui::{Context, FontId, FontSource, Image, TextureId};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Cursor, Write};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub struct DugtrioRenderLoop {
    block_messages: bool,
    lpress: bool,
    image: RgbaImage,
    image_bytes: Vec<u8>,
    image_id: Option<TextureId>,
    textures: HashMap<String, TextureId>,
    text_value: Arc<Mutex<String>>,
    ret_value_clone: Arc<Mutex<String>>,
}

static mut FONTS: Vec<(usize, FontId)> = Vec::new();
static mut FONT_SIZE: usize = 16;
static mut ALIGNMENT: usize = 0;

impl DugtrioRenderLoop {
    pub fn new() -> Self {
        let image = load_image();
        let image_bytes = image.as_raw().to_vec();

        let text_value = Arc::new(Mutex::new(String::new()));
        let text_value_clone = Arc::clone(&text_value);

        let ret_value = Arc::new(Mutex::new(String::new()));
        let ret_value_clone = Arc::clone(&ret_value);

        thread::spawn(move || {
            read_pipe_messages(text_value_clone);
        });

        thread::spawn(move || {
            write_pipe_messages(ret_value);
        });

        DugtrioRenderLoop {
            lpress: false,
            block_messages: true,
            image_bytes,
            image,
            image_id: None,
            text_value,
            ret_value_clone,
            textures: HashMap::new(),
        }
    }
}

impl ImguiRenderLoop for DugtrioRenderLoop {
    fn initialize<'a>(&'a mut self, context: &mut Context, loader: &'a mut dyn RenderContext) {
        unsafe {
            for i in 1..=100 {
                let font_id = context.fonts().add_font(&[FontSource::TtfData {
                    data: include_bytes!("../../poppins.ttf"),
                    size_pixels: i as f32,
                    config: None,
                }]);
                FONTS.push((i, font_id));
            }
        };
        self.image_id = load_texture(loader, &self.image_bytes, &self.image);
    }
    fn set_message_filter(&self, _io: &imgui::Io) -> MessageFilter {
        if self.block_messages {
            MessageFilter::InputAll
        } else {
            MessageFilter::empty()
        }
    }

    fn render(&mut self, ui: &mut imgui::Ui) {
        let text = self.text_value.lock().unwrap().clone();
        if !text.is_empty() {
            {
                draw_commands(&text, ui, self.textures.clone());
            }
        }

        if self.block_messages {
            draw_cursor(ui, self.image_id, &self.image);
        }

        let mut pload = self.ret_value_clone.lock().unwrap();
        let response = json!({
            "mousePosition": ui.io().mouse_pos,
            "mouseDown": ui.io().mouse_down,
            "windowSize": ui.io().display_size,
            "menuActive": self.block_messages
        });

        *pload = response.to_string();

        {
            toggle_block_messages(ui, &mut self.block_messages, &mut self.lpress);
        }
    }

    fn before_render<'a>(&'a mut self, _ctx: &mut Context, loader: &'a mut dyn RenderContext) {
        let text = self.text_value.lock().unwrap().clone();
        if (!text.is_empty() && text.contains("$textureLoad$")) {
            {
                let v: Value = serde_json::from_str(&text).unwrap();
                let commands: Vec<Value> = v["commands"].as_array().unwrap().to_vec();

                for command in commands {
                    let r#type = command["type"].as_str();

                    match r#type {
                        Some("loadTexture") => {
                            let texture_name = command["textureName"].as_str().unwrap().to_string();
                            let data = command["data"].as_str().unwrap().to_string();

                            if !self.textures.contains_key(&texture_name) {
                                let texid = load_texture_from_base64(loader, &data).unwrap();
                                self.textures.insert(texture_name, texid);
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    fn on_wnd_proc(
        &self,
        _hwnd: windows::Win32::Foundation::HWND,
        _umsg: u32,
        _wparam: windows::Win32::Foundation::WPARAM,
        _lparam: windows::Win32::Foundation::LPARAM,
    ) {
    }
}

fn load_image() -> RgbaImage {
    ImageReader::new(Cursor::new(include_bytes!("../Arrow.png")))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .into_rgba8()
}

fn load_texture<'a>(
    loader: &'a mut dyn RenderContext,
    image_bytes: &'a [u8],
    image: &RgbaImage,
) -> Option<TextureId> {
    loader
        .load_texture(image_bytes, image.width() as _, image.height() as _)
        .ok()
}

fn load_texture_from_base64<'a>(
    loader: &'a mut dyn RenderContext,
    base64_string: &str,
) -> Option<TextureId> {
    let image_bytes = base64::decode(base64_string).ok()?;
    let image = image::load_from_memory(&image_bytes).ok()?.to_rgba8();
    loader
        .load_texture(&image, image.width() as _, image.height() as _)
        .ok()
}

fn read_pipe_messages(text_value: Arc<Mutex<String>>) {
    let pipe_path = r"\\.\pipe\discord_ipc";

    loop {
        match File::open(&pipe_path) {
            Ok(pipe) => {
                let mut reader = BufReader::new(pipe);
                loop {
                    let mut line = String::new();
                    match reader.read_line(&mut line) {
                        Ok(0) => break,
                        Ok(_) => {
                            let mut text = text_value.lock().unwrap();
                            *text = line.trim_end().to_string();
                        }
                        Err(_) => break,
                    }
                }
            }
            Err(_) => thread::sleep(Duration::from_secs(1)),
        }
    }
}
fn write_pipe_messages(text_value: Arc<Mutex<String>>) {
    let pipe_path = r"\\.\pipe\discord_ipc_out";

    loop {
        // Attempt to open the pipe
        let pipe = match OpenOptions::new().write(true).open(&pipe_path) {
            Ok(pipe) => pipe,
            Err(e) => {
                eprintln!("Failed to open pipe: {}", e);
                thread::sleep(Duration::from_secs(1)); // Retry after a delay
                continue; // Try to open the pipe again
            }
        };

        let mut writer = BufWriter::new(pipe);

        loop {
            let text = match text_value.lock() {
                Ok(lock) => lock.clone(),
                Err(e) => {
                    eprintln!("Failed to lock mutex: {}", e);
                    break; // Exit the inner loop if we can't acquire the lock
                }
            };

            if let Err(e) = writeln!(writer, "{}", text) {
                eprintln!("Failed to write to pipe: {}", e);
                if e.kind() == std::io::ErrorKind::BrokenPipe {
                    break; // Exit the inner loop if the pipe is broken
                }
            } else {
                if let Err(e) = writer.flush() {
                    eprintln!("Failed to flush to pipe: {}", e);
                    if e.kind() == std::io::ErrorKind::BrokenPipe {
                        break; // Exit the inner loop if the pipe is broken
                    }
                }
            }
            thread::sleep(Duration::from_millis(4)); // Adjust timing as necessary
        }
    }
}
fn draw_commands(text: &str, ui: &mut imgui::Ui, textures: HashMap<String, TextureId>) {
    let v: Value = serde_json::from_str(text).unwrap();
    let mut thickness = 1.0;
    let mut rounding = 0.0;
    let mut color = [255.0, 0.0, 0.0, 1.0];
    let mut drawlist = ui.get_background_draw_list();
    for command in v["commands"].as_array().unwrap() {
        let r#type = command["type"].as_str();

        match r#type {
            Some("thickness") => thickness = command["value"].as_f64().unwrap() as f32,
            Some("rounding") => rounding = command["value"].as_f64().unwrap() as f32,
            Some("fontSize") => unsafe { FONT_SIZE = command["value"].as_f64().unwrap() as usize },
            Some("fontAlign") => {
                unsafe { ALIGNMENT = command["value"].as_f64().unwrap() as usize };
            }
            Some("color") => {
                color = [
                    command["red"].as_f64().unwrap() as f32,
                    command["green"].as_f64().unwrap() as f32,
                    command["blue"].as_f64().unwrap() as f32,
                    command["alpha"].as_f64().unwrap() as f32,
                ];
            }
            Some("rect") => {
                draw_rect(command, &mut drawlist, thickness, rounding, color);
            }
            Some("circle") => {
                draw_circle(command, &mut drawlist, thickness, color);
            }
            Some("texture") => {
                let position = command["position"].as_object().unwrap();
                let size = command["size"].as_object().unwrap();
                let _ = drawlist
                    .add_image(
                        *textures
                            .get(&command["textureId"].as_str().unwrap().to_string())
                            .unwrap(),
                        [
                            position["x"].as_f64().unwrap() as f32,
                            position["y"].as_f64().unwrap() as f32,
                        ],
                        [
                            position["x"].as_f64().unwrap() as f32
                                + size["x"].as_f64().unwrap() as f32,
                            position["y"].as_f64().unwrap() as f32
                                + size["y"].as_f64().unwrap() as f32,
                        ],
                    )
                    .build();
            }
            Some("text") => {
                let pop_font: imgui::FontStackToken<'_> =
                    ui.push_font(unsafe { FONTS.get(FONT_SIZE).unwrap().1 });

                let size =
                    ui.calc_text_size_with_opts(command["text"].as_str().unwrap(), true, 0.0);

                draw_text(command, &mut drawlist, color, size);
                pop_font.pop()
            }
            Some("line") => {
                draw_line(command, &mut drawlist, thickness, color);
            }
            _ => (),
        }
    }
}

fn draw_rect(
    command: &Value,
    drawlist: &mut imgui::DrawListMut,
    thickness: f32,
    rounding: f32,
    color: [f32; 4],
) {
    let position = command["position"].as_object().unwrap();
    let size = command["size"].as_object().unwrap();

    drawlist
        .add_rect(
            [
                position["x"].as_f64().unwrap() as f32,
                position["y"].as_f64().unwrap() as f32,
            ],
            [
                position["x"].as_f64().unwrap() as f32 + size["x"].as_f64().unwrap() as f32,
                position["y"].as_f64().unwrap() as f32 + size["y"].as_f64().unwrap() as f32,
            ],
            imgui::ImColor32::from(color),
        )
        .thickness(thickness)
        .rounding(rounding)
        .filled(command["fill"].as_bool().unwrap())
        .build();
}

fn draw_circle(
    command: &Value,
    drawlist: &mut imgui::DrawListMut,
    thickness: f32,
    color: [f32; 4],
) {
    let position = command["position"].as_object().unwrap();

    drawlist
        .add_circle(
            [
                position["x"].as_f64().unwrap() as f32,
                position["y"].as_f64().unwrap() as f32,
            ],
            command["radius"].as_f64().unwrap() as f32,
            imgui::ImColor32::from(color),
        )
        .thickness(thickness)
        .filled(command["fill"].as_bool().unwrap())
        .build();
}

fn draw_line(command: &Value, drawlist: &mut imgui::DrawListMut, thickness: f32, color: [f32; 4]) {
    let start = command["start"].as_object().unwrap();
    let end = command["end"].as_object().unwrap();

    drawlist
        .add_line(
            [
                start["x"].as_f64().unwrap() as f32,
                start["y"].as_f64().unwrap() as f32,
            ],
            [
                end["x"].as_f64().unwrap() as f32,
                end["y"].as_f64().unwrap() as f32,
            ],
            imgui::ImColor32::from(color),
        )
        .thickness(thickness)
        .build();
}

fn draw_text(command: &Value, drawlist: &mut imgui::DrawListMut, color: [f32; 4], size: [f32; 2]) {
    let mut offset = [0.0, 0.0];
    if (unsafe { ALIGNMENT } == 1) {
        offset[0] = -size[0];
    }
    if (unsafe { ALIGNMENT } == 2) {
        offset[0] = -size[0] / 2.0;
    }
    let position = command["position"].as_object().unwrap();

    drawlist.add_text(
        [
            position["x"].as_f64().unwrap() as f32 + offset[0],
            position["y"].as_f64().unwrap() as f32 + offset[1],
        ],
        imgui::ImColor32::from(color),
        command["text"].as_str().unwrap(),
    )
}

fn draw_cursor(ui: &mut imgui::Ui, image_id: Option<TextureId>, image: &RgbaImage) {
    let style = ui.push_style_var(imgui::StyleVar::WindowPadding([0.0, 0.0]));
    let border = ui.push_style_var(imgui::StyleVar::WindowBorderSize(0.0));
    let rounding = ui.push_style_var(imgui::StyleVar::WindowRounding(0.0));

    if let Some(image_id) = image_id {
        let cursor_pos = ui.io().mouse_pos;
        ui.set_mouse_cursor(Some(imgui::MouseCursor::Arrow));
        ui.window("Cursor")
            .position([cursor_pos[0], cursor_pos[1]], imgui::Condition::Always)
            .size([16.0, 24.0], imgui::Condition::Always)
            .no_decoration()
            .bg_alpha(0.0)
            .no_inputs()
            .build(|| {
                Image::new(image_id, [image.width() as f32, image.height() as f32]).build(ui);
            });
    }
    style.pop();
    border.pop();
    rounding.pop();
}

fn toggle_block_messages(ui: &mut imgui::Ui, block_messages: &mut bool, lpress: &mut bool) {
    let cpressed = ui.io().mouse_down[2];

    if cpressed && !*lpress {
        *block_messages = !*block_messages;
        *lpress = true;
    } else if !cpressed {
        *lpress = false;
    }
}

#[cfg(feature = "dx11")]
hudhook!(ImguiDx11Hooks, DugtrioRenderLoop::new());

#[cfg(feature = "opengl")]
hudhook!(ImguiOpenGl3Hooks, DugtrioRenderLoop::new());
