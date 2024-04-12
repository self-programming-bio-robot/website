use zhdanov_website_core::dto::message::Message;

pub fn convert_message_to_langchain_message(message: Message) -> langchain_rust::schemas::Message {
    match message.is_assistant {
        true => langchain_rust::schemas::Message::new_ai_message(message.content),
        false => langchain_rust::schemas::Message::new_human_message(message.content),
    }
}

pub fn make_history(messages: &Vec<Message>) -> Vec<langchain_rust::schemas::Message> {
    messages.iter()
        .map(|message| convert_message_to_langchain_message(message.clone()))
        .collect()
}