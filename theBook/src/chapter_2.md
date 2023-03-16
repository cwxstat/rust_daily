# Chapter 2

This is an example of chapter 2.

```rust,editable
enum Message {
    Quit,
    ChangeColor(u8, u8, u8),
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) {
    // Using match expression
    match msg {
        Message::Quit => println!("Quit"),
        Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
        Message::Move { x, y } => println!("Move to coordinates ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
    }
}

fn main() {
    let msg1 = Message::ChangeColor(255, 0, 0);
    let msg2 = Message::Write(String::from("Hello, World!"));
    let _msg3 = Message::Quit;
    let _msg4 = Message::Move { x: 10, y: 20 };

    // Using if let statement
    if let Message::ChangeColor(r, g, b) = msg1 {
        println!("Change color to ({}, {}, {})", r, g, b);
    } else {
        println!("Not a ChangeColor message");
    }

    process_message(msg1);
    process_message(msg2);
}

```