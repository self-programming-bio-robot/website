use std::error::Error;
use langchain_rust::llm::openai::{OpenAI, OpenAIConfig};
use langchain_rust::{message_formatter, prompt_args, template_jinja2};
use langchain_rust::chain::{Chain, LLMChainBuilder};
use langchain_rust::chain::options::ChainCallOptions;
use langchain_rust::prompt::{FormatPrompter, HumanMessagePromptTemplate, MessageOrTemplate, PromptFromatter};

use zhdanov_website_core::dto::message::Message;
use crate::llm::actions::action::{Action, Actions};
use crate::llm::actions::output_parser::ActionOutputParser;
use crate::llm::utils::message::make_history;

pub struct QuestionAssistant {
    open_ai: OpenAI<OpenAIConfig>,
    actions: Actions,
    parser: ActionOutputParser,
}

impl QuestionAssistant {
    pub fn new(open_ai_key: String) -> Self {
        let config = OpenAIConfig::new()
            .with_api_key(open_ai_key);
        let open_ai = OpenAI::new(config.clone());
        Self { open_ai, actions: Actions::new(), parser: ActionOutputParser::new() }
    }

    pub fn new_with_open_ai(open_ai: OpenAI<OpenAIConfig>) -> Self {
        Self { open_ai, actions: Actions::new(), parser: ActionOutputParser::new()  }
    }

    pub async fn invoke(&self, query: String, messages: Vec<Message>) -> Result<Option<Action>, Box<dyn Error>> {
        let action_names = self.actions.get_actions().iter()
            .map(|action| action.name.clone())
            .collect::<Vec<String>>();
        let action_instructions = self.actions.to_instructions();
        let messages = make_history(&messages);

        let base_prompt = include_str!("prompts/actions/base.txt");
        let actions_prompt = include_str!("prompts/actions/actions.txt");
        let instructions_prompt = include_str!("prompts/actions/instructions.txt");

        let prompt = template_jinja2!(base_prompt, "actions_list", "format_instructions", "actions", "action_names");
        let input_variables_fstring = prompt_args! {
            "actions_list" => actions_prompt,
            "format_instructions" => instructions_prompt,
            "action_names"=>action_names,
            "actions" => action_instructions,
        };
        let prompt = prompt.format(input_variables_fstring)?;
        let formatter = message_formatter![
            MessageOrTemplate::Message(langchain_rust::schemas::Message::new_system_message(prompt)),
            MessageOrTemplate::MessagesPlaceholder("chat_history".to_string()),
            MessageOrTemplate::Template(
                HumanMessagePromptTemplate::new(template_jinja2!(
                    r#""USER'S RESPONSE"
                    "Here is the user's input:"
                    "{{input}}""#,
                    "input"
                ))
                .into()
            ),
        ];

        let t = formatter.format_prompt(prompt_args! {
            "chat_history" => messages,
            "input" => query,
        }).unwrap();

        let chain = LLMChainBuilder::new()
            .options(ChainCallOptions::new().with_temperature(0.0))
            .prompt(formatter)
            .llm(self.open_ai.clone())
            .build()?;

        let response = chain.invoke(prompt_args! {
            "chat_history" => messages,
            "input" => query,
        }).await?;
        dbg!(&response);
        let response = self.parser.parse(response.as_str()).ok();
        Ok(response)
    }
}