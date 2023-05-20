pub fn run() {
    let mut message = String::from("Hello");
    let message_2 = &mut message;

    // message.push_str(" World");

    println!("Message {}", message_2);
}