//! Crate for crafting an interactive chatbot notifier.

use std::collections::HashMap;

/// Configuration for the chatbot notifier.
 pub struct NotifierConfig {
    pub chatbot_token: String,
    pub notification_channel: String,
    pub notification_threshold: u64,
}

impl NotifierConfig {
    pub fn new(chatbot_token: String, notification_channel: String, notification_threshold: u64) -> NotifierConfig {
        NotifierConfig {
            chatbot_token,
            notification_channel,
            notification_threshold,
        }
    }
}

/// Chatbot notifier struct.
pub struct ChatbotNotifier {
    config: NotifierConfig,
    message_cache: HashMap<String, u64>,
}

impl ChatbotNotifier {
    pub fn new(config: NotifierConfig) -> ChatbotNotifier {
        ChatbotNotifier {
            config,
            message_cache: HashMap::new(),
        }
    }

    pub fn process_message(&mut self, message: &str) {
        let message_key = message.to_string();
        let message_count = self.message_cache.entry(message_key).or_insert(0);
        *message_count += 1;

        if *message_count >= self.config.notification_threshold {
            // Send notification to the configured channel
            println!("Sending notification to channel {}...", self.config.notification_channel);
        }
    }
}

fn main() {
    let config = NotifierConfig::new(
        "MY_CHATBOT_TOKEN".to_string(),
        "my-notification-channel".to_string(),
        5, // notification threshold
    );

    let mut notifier = ChatbotNotifier::new(config);

    // Simulate incoming messages
    notifier.process_message("Hello!");
    notifier.process_message("Hello!");
    notifier.process_message("Hello!");
    notifier.process_message("Hello!");
    notifier.process_message("Hello!"); // should trigger notification
}