#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("{:?}", self.content);
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

pub fn run() {
    let chat_message_slice = ChatMessage {
        content: "String slice",
        time: "100".to_string(),
    };

    let chat_message_string = ChatMessage {
        content: "String".to_string(),
        time: "100".to_string(),
    };

    let dg_message = ChatMessage {
        content: DigitalContent::VideoFile,
        time: "100".to_string(),
    };

    dg_message.consume_entertainment();

    println!("{:?}", chat_message_slice.retrieve_time());
    println!("{:?}", chat_message_string.retrieve_time());
    println!("{:?}", dg_message.retrieve_time());
}
