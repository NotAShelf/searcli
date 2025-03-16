/// Wraps a string into lines with a maximum width
pub fn wrap_text(text: &str, max_line_length: usize) -> String {
    let mut wrapped_text = String::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_line_length {
            wrapped_text.push_str(&current_line);
            wrapped_text.push('\n');
            current_line = String::new();
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    // Append any remaining text abd trim any leading/trailing whitespaces
    // from the wrapped text.
    wrapped_text.push_str(&current_line);
    wrapped_text.trim_end().to_string()
}
