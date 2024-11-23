# Logging

DDS-Edit provides informative logging to help users understand the processing steps and any issues that arise.

## Log Levels

The application logs various messages, including:

- **Information**: Details about processing steps.
- **Warnings**: Non-critical issues, such as skipping invalid target widths.
- **Errors**: Critical issues that prevent processing, such as file I/O errors.

## Colored Output

Logging messages are color-coded for better readability:

- **White**: General information.
- **Bright Cyan**: Paths and important parameters.
- **Bright Blue**: Processing steps.
- **Green**: Successful operations.
- **Yellow**: Warnings.
- **Red**: Errors.

## Log Queue

DDS-Edit uses a `LogQueue` to manage log messages, which ensures thread-safe logging from asynchronous tasks.

### Example Usage

```rust
let log_queue = LogQueue::new();
log_queue.enqueue(LogType::Println, "Processing started".to_string(), Color::Green);
log_queue.process_queue();
```

## Custom Logging Functions

- `smnprint`: Prints colored text without a newline.
- `smnprintln`: Prints colored text with a newline.

### Example

```rust
smnprint("Processing file: ", Color::White);
smnprintln(file_name, Color::BrightCyan);
```

