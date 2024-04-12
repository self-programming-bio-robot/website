use std::collections::VecDeque;
use std::error::Error;
use serde_json::Value;
use crate::llm::actions::action::Action;

pub struct ActionOutputParser {}
impl ActionOutputParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl ActionOutputParser {
    pub fn parse(&self, text: &str) -> Result<Action, Box<dyn Error>> {
        match parse_partial_json(text, false) {
            Some(value) => {
                // Deserialize the Value into AgentOutput
                let agent_output: Action = serde_json::from_value(value)?;

                Ok(agent_output)
            }
            None => {
                Err("Failed to parse Agent Action".into())
            }
        }
    }
}

fn parse_partial_json(s: &str, strict: bool) -> Option<Value> {
    // First, attempt to parse the string as-is.
    match serde_json::from_str::<Value>(s) {
        Ok(val) => return Some(val),
        Err(_) if !strict => (),
        Err(_) => return None,
    }

    let mut new_s = String::new();
    let mut stack: VecDeque<char> = VecDeque::new();
    let mut is_inside_string = false;
    let mut escaped = false;

    for char in s.chars() {
        match char {
            '"' if !escaped => is_inside_string = !is_inside_string,
            '{' if !is_inside_string => stack.push_back('}'),
            '[' if !is_inside_string => stack.push_back(']'),
            '}' | ']' if !is_inside_string => {
                if let Some(c) = stack.pop_back() {
                    if c != char {
                        return None; // Mismatched closing character
                    }
                } else {
                    return None; // Unbalanced closing character
                }
            }
            '\\' if is_inside_string => escaped = !escaped,
            _ => escaped = false,
        }
        new_s.push(char);
    }

    // Close any open structures.
    while let Some(c) = stack.pop_back() {
        new_s.push(c);
    }

    // Attempt to parse again.
    serde_json::from_str(&new_s).ok()
}