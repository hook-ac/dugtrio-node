use std::env;

use hudhook::inject::Process;

fn main() {
    // Retrieve the directory of the current executable
    let mut cur_exe = env::current_exe().unwrap();
    cur_exe.pop(); // Navigate up to the directory containing the executable

    // Determine the DLL name based on the feature flag
    let dll_name = if cfg!(feature = "dx11") {
        "discordoverlay_dx11.dll"
    } else if cfg!(feature = "opengl") {
        "discordoverlay_opengl.dll"
    } else {
        "discordoverlay.dll"
    };

    // Construct the path to the DLL
    cur_exe.push(dll_name);

    // Check if the specific DLL exists and use the default if it does not
    let cur_dll = if cur_exe.exists() {
        cur_exe
    } else {
        let mut default_dll = cur_exe.clone();
        default_dll.pop(); // Remove the specific DLL name
        default_dll.push("discordoverlay.dll"); // Use the default
        default_dll
    }
    .canonicalize()
    .unwrap();

    // Output the path being used for debugging
    println!("Using DLL path: {:?}", cur_dll);

    // Dummy implementation of process injection (replace with actual code as needed)
    if let Ok(process) = Process::by_name("osu!.exe") {
        process.inject(cur_dll).unwrap();
    } else {
        println!("Failed to find or inject into process 'osu!.exe'");
    }
}
