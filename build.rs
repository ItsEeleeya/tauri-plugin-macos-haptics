const COMMANDS: &[&str] = &["is_supported", "perform"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
