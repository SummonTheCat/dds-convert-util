// D:/Work/Projects/Modding/Skyrim/dds-edit/src/logging/mod.rs

use colored::{Color, Colorize};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

/// Enum to represent the type of log action.
#[derive(Debug, Clone, Copy)]
pub enum LogType {
    Print,
    Println,
}

/// Struct to hold individual log items.
#[derive(Debug, Clone)]
pub struct LogItem {
    pub log_type: LogType,
    pub content: String,
    pub color: Color,
}

/// Struct to manage a queue of log items.
#[derive(Debug, Clone)]
pub struct LogQueue {
    queue: Arc<Mutex<Vec<LogItem>>>,
}

impl LogQueue {
    /// Creates a new, empty `LogQueue`.
    ///
    /// # Returns
    ///
    /// * `Self` - An instance of `LogQueue`.
    pub fn new() -> Self {
        LogQueue {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Enqueues a new log item into the queue.
    ///
    /// # Arguments
    ///
    /// * `log_type` - The type of log action (`Print` or `Println`).
    /// * `content` - The message to log.
    /// * `color` - The color to apply to the message.
    pub fn enqueue(&self, log_type: LogType, content: String, color: Color) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(LogItem {
            log_type,
            content,
            color,
        });
    }

    /// Processes and prints all log items in the queue.
    ///
    /// After processing, the queue is cleared.
    pub fn process_queue(&self) {
        let mut queue = self.queue.lock().unwrap();
        for item in queue.iter() {
            match item.log_type {
                LogType::Print => {
                    let colored_text = item.content.color(item.color);
                    print!("{}", colored_text);
                }
                LogType::Println => {
                    let colored_text = item.content.color(item.color);
                    println!("{}", colored_text);
                }
            }
        }
        // Clear the queue after processing.
        queue.clear();
    }
}

/// Prints a line of colored text to the console.
///
/// This function immediately prints the text with the specified color, followed by a newline.
///
/// # Arguments
///
/// * `text` - The message to print.
/// * `color` - The `Color` to apply to the text.
///
/// # Example
///
/// ```
/// smnprintln("Processing started", Color::Green);
/// ```
pub fn smnprintln<T: AsRef<str>>(text: T, color: Color) {
    let colored_text = text.as_ref().color(color);
    println!("{}", colored_text);
}

/// Prints colored text to the console without adding a newline.
///
/// This function immediately prints the text with the specified color without a newline.
/// To add a newline, use `smnprintln`.
///
/// # Arguments
///
/// * `text` - The message to print.
/// * `color` - The `Color` to apply to the text.
///
/// # Example
///
/// ```
/// smnprint("Processing... ", Color::Blue);
/// ```
pub fn smnprint<T: AsRef<str>>(text: T, color: Color) {
    let colored_text = text.as_ref().color(color);
    print!("{}", colored_text);
    // Flush to ensure immediate output
    io::stdout().flush().expect("Failed to flush stdout");
}
