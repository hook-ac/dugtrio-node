use hudhook::inject::Process;

fn main() {
    let mut cur_exe = std::env::current_exe().unwrap();
    cur_exe.push("..");
    cur_exe.push("discordoverlay.dll");

    let cur_dll = cur_exe.canonicalize().unwrap();

    Process::by_name("osu!.exe")
        .unwrap()
        .inject(cur_dll)
        .unwrap();
}
