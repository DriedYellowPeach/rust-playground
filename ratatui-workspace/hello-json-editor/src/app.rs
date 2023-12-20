use anyhow::Result;
use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl Default for App {
    fn default() -> Self {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn save_kv(&mut self) {
        let mut tmp_key = String::new();
        let mut tmp_val = String::new();
        std::mem::swap(&mut tmp_key, &mut self.key_input);
        std::mem::swap(&mut tmp_val, &mut self.value_input);
        self.pairs.insert(tmp_key, tmp_val);
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        match self.currently_editing {
            Some(CurrentlyEditing::Key) => self.currently_editing = Some(CurrentlyEditing::Value),
            Some(CurrentlyEditing::Value) => self.currently_editing = Some(CurrentlyEditing::Key),
            None => self.currently_editing = Some(CurrentlyEditing::Key),
        }
    }

    pub fn print_json(&self) -> Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}
