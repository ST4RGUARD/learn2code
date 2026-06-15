/// Parses a command-line-like string into arguments, respecting quoted substrings.
/// Supports both single and double quotes.
pub fn parse_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut single_quote = false;
    let mut double_quote = false;

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(next_char) = chars.peek() {
                    current.push(*next_char);
                    chars.next();
                }
            }
            '\'' if !double_quote => {
                single_quote = !single_quote;
            } 
            '"' if !single_quote => {
                double_quote = !double_quote;
            } 
            ' ' | '\t' if !single_quote && !double_quote => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(c);
            }
        }
    }
    if !current.is_empty() {
        args.push(current);
    }
    args
}
