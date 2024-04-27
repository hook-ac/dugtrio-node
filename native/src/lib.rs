use hudhook::{hooks::dx11::ImguiDx11Hooks, *};
use image::io::Reader as ImageReader;
use image::RgbaImage;
use imgui::{Context, Image, TextureId};
use serde_json::{json, Value};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Cursor, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct DugtrioRenderLoop {
    block_messages: bool,
    image: RgbaImage,
    image_bytes: Vec<u8>,
    image_id: Option<TextureId>,
    text_value: Arc<Mutex<String>>,
    ret_value_clone: Arc<Mutex<String>>,
}

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
            block_messages: false,
            image_bytes,
            image,
            image_id: None,
            text_value,
            ret_value_clone,
        }
    }
}

impl ImguiRenderLoop for DugtrioRenderLoop {
    fn initialize<'a>(&'a mut self, _ctx: &mut Context, mut loader: TextureLoader<'a>) {
        hudhook::alloc_console();
        self.image_id = load_texture(&mut loader, &self.image_bytes, &self.image);
    }

    fn should_block_messages(&self, _io: &imgui::Io) -> bool {
        self.block_messages
    }
    fn render(&mut self, ui: &mut imgui::Ui) {
        if self.block_messages {
            let text = self.text_value.lock().unwrap().clone();
            if !text.is_empty() {
                {
                    let mut drawlist = ui.get_background_draw_list();
                    draw_commands(&text, &mut drawlist);
                }
            }
        }

        if self.block_messages {
            draw_cursor(ui, self.image_id, &self.image);
        }

        let mut pload = self.ret_value_clone.lock().unwrap();
        let response = json!({
            "mousePosition": ui.io().mouse_pos,
            "mouseDown": ui.io().mouse_down,
            "windowSize":  ui.io().display_size
        });

        *pload = response.to_string();

        {
            toggle_block_messages(ui, &mut self.block_messages);
        }
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
    loader: &mut TextureLoader<'a>,
    image_bytes: &'a [u8],
    image: &RgbaImage,
) -> Option<TextureId> {
    loader(image_bytes, image.width() as _, image.height() as _).ok()
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
    let pipe_path = r"\\.\pipe\discord_ipc";

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
            thread::sleep(Duration::from_millis(16)); // Adjust timing as necessary
        }
    }
}
fn draw_commands(text: &str, drawlist: &mut imgui::DrawListMut) {
    let v: Value = serde_json::from_str(text).unwrap();
    let mut thickness = 1.0;
    let mut rounding = 0.0;
    let mut color = [255.0, 0.0, 0.0, 1.0];

    for command in v["commands"].as_array().unwrap() {
        let r#type = command["type"].as_str();

        match r#type {
            Some("thickness") => thickness = command["value"].as_f64().unwrap() as f32,
            Some("rounding") => rounding = command["value"].as_f64().unwrap() as f32,
            Some("color") => {
                color = [
                    command["red"].as_f64().unwrap() as f32,
                    command["green"].as_f64().unwrap() as f32,
                    command["blue"].as_f64().unwrap() as f32,
                    command["alpha"].as_f64().unwrap() as f32,
                ];
            }
            Some("rect") => {
                draw_rect(command, drawlist, thickness, rounding, color);
            }
            Some("circle") => {
                draw_circle(command, drawlist, thickness, color);
            }
            Some("line") => {
                draw_line(command, drawlist, thickness, color);
            }
            _ => panic!("Unknown command type"),
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

fn toggle_block_messages(ui: &mut imgui::Ui, block_messages: &mut bool) {
    if ui.is_key_released(imgui::Key::Home) {
        *block_messages = !*block_messages;
    }
}

hudhook!(ImguiDx11Hooks, DugtrioRenderLoop::new());
