import { $ } from "bun";

// Bun required

// x64
await $`cd native && cargo build -r --features opengl`;
let file1 = Bun.file("./native/target/release/discordoverlay.dll");
await Bun.write("./prebuilt/x64/discordoverlay_opengl.dll", file1);

let file2 = Bun.file("./native/target/release/overlazer.exe");
await Bun.write("./prebuilt/x64/inject_opengl.exe", file2);

await $`cd native &&  cargo build -r  --features dx11`;
let file3 = Bun.file("./native/target/release/discordoverlay.dll");
await Bun.write("./prebuilt/x64/discordoverlay_dx11.dll", file3);

let file4 = Bun.file("./native/target/release/overlazer.exe");
await Bun.write("./prebuilt/x64/inject_dx11.exe", file4);

// x32
await $`cd native && cargo build -r --features opengl --target i686-pc-windows-msvc`;
file1 = Bun.file(
  "./native/target/i686-pc-windows-msvc/release/discordoverlay.dll"
);
await Bun.write("./prebuilt/x32/discordoverlay_opengl.dll", file1);

file2 = Bun.file("./native/target/i686-pc-windows-msvc/release/overlazer.exe");
await Bun.write("./prebuilt/x32/inject_opengl.exe", file2);

$`cd native &&  cargo build -r  --features dx11 --target i686-pc-windows-msvc`;
file3 = Bun.file(
  "./native/target/i686-pc-windows-msvc/release/discordoverlay.dll"
);
await Bun.write("./prebuilt/x32/discordoverlay_dx11.dll", file3);

file4 = Bun.file("./native/target/i686-pc-windows-msvc/release/overlazer.exe");
await Bun.write("./prebuilt/x32/inject_dx11.exe", file4);
