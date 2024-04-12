use serde::{Deserialize, Serialize};
use crate::llm::actions::redirect::RedirectAction;

pub struct Actions {
    actions: Vec<ActionDefinition>,
}

pub struct ActionDefinition {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Redirect(RedirectAction),
}

impl Actions {
    pub fn new() -> Self {
        Self {
            actions: vec![
                ActionDefinition {
                    name: "Redirect".to_string(),
                    description: r#""This action let you redirect user to another page"
                                 "There are the folllowing paths available:"
                                 "'pages/resume' - page represent resume(CV) of Nikolai Zhdanov"
                                 ""
                                 "The input should be an object with a path key"
                                 "Example: { "path": "pages/resume" }""#.to_string(),
                }
            ]
        }
    }

    pub fn get_actions(&self) -> &Vec<ActionDefinition> {
        &self.actions
    }
    
    pub fn to_instructions(&self) -> String {
        let mut instructions = String::new();
        for action in &self.actions {
            instructions.push_str(&format!("ACTION: {}\n{}\n-----------\n", action.name, action.description));
        }
        instructions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actions() {
        let json = r#"{
            "Redirect": {
                "path": "pages/resume"
            }
        }"#;
        
        let action: Action = Action::Redirect(RedirectAction {
                path: "pages/resume".to_string()
            });
        
        assert_eq!(action, serde_json::from_str(json).unwrap());
    }
}