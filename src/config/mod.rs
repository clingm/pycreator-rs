use serde_json::json;
use std::collections::HashMap;
use std::io::{self, Write};

static DEFAULT_LSP_SERVER: &str = "pyright";
static DEFAULT_FILE_NAME: &str = "pyrightconfig.json";

pub struct LSPConfig {
    lsp_server: String,
    config_file_name: String,
    config_items: HashMap<String, String>,
}

impl LSPConfig {
    pub fn new() -> Self {
        LSPConfig {
            lsp_server: DEFAULT_LSP_SERVER.to_string(),
            config_file_name: DEFAULT_FILE_NAME.to_string(),
            config_items: HashMap::new(),
        }
    }

    pub fn add_config_item(&mut self, key: &str, value: &str) {
        self.config_items.insert(key.to_string(), value.to_string());
    }

    pub fn lsp_server(&self) -> String {
        self.lsp_server.clone()
    }

    pub fn config_file_name(&self) -> String {
        self.config_file_name.clone()
    }

    pub fn config_items(&self) -> &HashMap<String, String> {
        &self.config_items
    }
}

pub fn write_config<W: Write>(writer: &mut W, config: &LSPConfig) -> io::Result<()> {
    let config_json = json!(config.config_items());
    let pretty_json = serde_json::to_string_pretty(&config_json).unwrap();
    writeln!(writer, "{}", pretty_json)
}
