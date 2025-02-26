use std::{thread::sleep, time};
use discord_rich_presence::{activity::{self, Activity}, DiscordIpc, DiscordIpcClient};

fn main() {
    let kernel_version = std::fs::read_to_string("/proc/sys/kernel/osrelease").unwrap();
    let plasma_version = std::process::Command::new("plasmashell")
        .arg("--version")
        .output()
        .expect("Failed to execute command");
    let plasma_version = String::from_utf8_lossy(&plasma_version.stdout);
    let plasma_version = plasma_version.replace("plasmashell ", "");

    let kwin_version = std::process::Command::new("kwin_x11")
    .arg("--version")
    .output()
    .expect("Failed to execute command");
    let kwin_version = String::from_utf8_lossy(&kwin_version.stdout);
    let kwin_version = kwin_version.replace("kwin ", "");

    let distro = std::fs::read_to_string("/etc/os-release").unwrap();
    let distro = distro.split("\n").collect::<Vec<&str>>();
    let distro = distro.iter().find(|&&x| x.contains("PRETTY_NAME")).unwrap();
    let distro = distro.replace("PRETTY_NAME=", "").replace("\"", "");



    let display_server = std::env::var("XDG_SESSION_TYPE").unwrap();
    // Create the client
    let mut client = DiscordIpcClient::new("1322629749612351538").unwrap();
    client.connect().unwrap();

    // Set the activity
    client.set_activity(Activity::new()
        .state(&format!("{} | {}",distro, kernel_version))
        .details(&format!("{} | KWin {} ({})", plasma_version, kwin_version, display_server))
        .buttons(vec![
            activity::Button::new("Visit KDE", "https://kde.org")
        ])
    ).unwrap();
    loop {
        sleep(time::Duration::from_secs(1));
    }
}
