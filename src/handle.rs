use std::fs::File;
use std::io::Write;

const PLUGIN_CONTENT: &str = include_str!("../templates/plugin.json");
const CONFIG_CONTENT: &str = include_str!("../templates/cfg.jsonc");

pub fn init(toggle: bool) {
    if toggle {
        let mut output = File::create("./plugin.json").unwrap();
        write!(output, "{PLUGIN_CONTENT}").unwrap();
    } else {
        let mut output = File::create("./cfg.jsonc").unwrap();
        write!(output, "{CONFIG_CONTENT}").unwrap();
    }
}
