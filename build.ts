import { $ } from "bun";

// Bun required
await $`cd native && cargo build -r --features opengl`;
const file1 = Bun.file("./native/target/release/discordoverlay.dll");
await Bun.write("./prebuilt/discordoverlay_opengl.dll", file1);

const file2 = Bun.file("./native/target/release/overlazer.exe");
await Bun.write("./prebuilt/inject_opengl.exe", file2);

await $`cd native &&  cargo build -r  --features dx11`;
const file3 = Bun.file("./native/target/release/discordoverlay.dll");
await Bun.write("./prebuilt/discordoverlay_dx11.dll", file3);

const file4 = Bun.file("./native/target/release/overlazer.exe");
await Bun.write("./prebuilt/inject_dx11.exe", file4);
