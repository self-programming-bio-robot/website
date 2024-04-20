use std::error::Error;
use std::sync::{Arc, Mutex};
use langchain_rust::tools::Tool;
use serde_json::{json, Value};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::agent::agent::Agent;
use crate::map::map::Map;

pub struct Move {
    agent: Arc<Mutex<dyn Agent>>,
    map: Arc<Mutex<Map>>,
}

#[derive(Deserialize, Serialize, Debug)]
struct MoveArguments {
    direction: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct MoveArgumentsWrapper {
    arguments: MoveArguments,
}

impl Move {
    pub fn new(agent: Arc<Mutex<dyn Agent>>, map: Arc<Mutex<Map>>) -> Self {
        Self {
            agent,
            map,
        }
    }
}

#[async_trait]
impl Tool for Move {
    fn name(&self) -> String {
        String::from("Move_Action")
    }
    fn description(&self) -> String {
        String::from(
            r#""This tool let you run move your agent on the map."
            "The input should be one of the following: up, down, left, right"
            "Where up, down, left, right are the directions to move the agent"
            "example of input: {{"direction": "up"}}"
            "#,
        )
    }

    fn parameters(&self) -> Value {
        let prompt = String::from(
            r#""This tool let you run move your agent on the map."
            "The input should be one of the following: up, down, left, right"
            "#,
        );
        json!(

        {
          "description": prompt,
          "type": "object",
          "properties": {
            "arguments": {
              "description": "A move arguments to move the agent in the map",
              "type": "object",
              "properties": {
                "direction": {
                  "description": "A directions to move the agent",
                  "type": "string",
                  "enum": [
                    "up",
                    "down",
                    "left",
                    "right"
                  ]
                }
              },
              "required": ["direction"],
              "additionalProperties": false
            }
          },
          "required": ["arguments"],
          "additionalProperties": false
        }
                )
    }

    async fn run(&self, input: Value) -> Result<String, Box<dyn Error>> {
        let argument: MoveArguments = serde_json::from_value(input)?;
        let mut result = String::new();
        result.push_str("Could not move the agent.");
        
        if let Ok(mut agent) = self.agent.lock() {
            if let Ok(map) = self.map.lock() {
                result.clear();
                match argument.direction.as_str() {
                    "up" => {
                        if agent.move_up(&map) {
                            result.push_str("Agent moved up");
                        } else {
                            result.push_str("Agent can't move up");
                        }
                    }
                    "down" => {
                        if agent.move_down(&map) {
                            result.push_str("Agent moved down");
                        } else {
                            result.push_str("Agent can't move down");
                        }
                    }
                    "left" => {
                        if agent.move_left(&map) {
                            result.push_str("Agent moved left");
                        } else {
                            result.push_str("Agent can't move left");
                        }
                    }
                    "right" => {
                        if agent.move_right(&map) {
                            result.push_str("Agent moved right");
                        } else {
                            result.push_str("Agent can't move right");
                        }
                    }
                    _ => {
                        result.push_str("Invalid direction");
                    }
                }
            }
        }
        
        Ok(result)
    }

    async fn parse_input(&self, input: &str) -> Value {
        let wrapper_result = serde_json::from_str::<MoveArgumentsWrapper>(input);

        if let Ok(wrapper) = wrapper_result {
            serde_json::to_value(wrapper.arguments).unwrap_or_else(|err| {
                Value::Null
            })
        } else {
            let commands_result = serde_json::from_str::<Vec<MoveArguments>>(input);

            commands_result.map_or_else(
                |err| {
                    Value::Null
                },
                |commands| serde_json::to_value(commands).unwrap_or(Value::Null),
            )
        }
    }
}