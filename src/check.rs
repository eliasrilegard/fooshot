pub fn verify_dependencies() {
  let dependencies = [
    "grim",
    "slurp",
    "Hyprland",
    "hyprpicker",
    "wl-copy",
    "notify-send",
  ];

  println!("Checking dependencies. Make sure any missing ones are installed and available in PATH.");

  for dependency in dependencies {
    if which::which(dependency).is_ok() {
      println!("{}: Ok", dependency);
    } else {
      eprintln!("{}: Not found", dependency);
    }
  }
}
