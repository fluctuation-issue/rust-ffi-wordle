//! Brief ansi escape code utils.

/// Clear the entire screen and place the cursor at top left position.
pub fn clear_screen() {
    print!("\x1b[2J[\x1b[H");
}

pub fn switch_to_alternate_screen() {
    print!("\x1b[?1049h");
    clear_screen();
}

pub fn switch_from_alternate_screen() {
    print!("\x1b[?1049l");
}

pub fn format_green_bg<D: std::fmt::Display>(content: D) -> String {
    format!("\x1b[48;5;2m{}\x1b[0m", content)
}

pub fn format_yellow_bg<D: std::fmt::Display>(content: D) -> String {
    format!("\x1b[48;5;3m{}\x1b[0m", content)
}

pub fn move_cursor_up(n: usize) {
    print!("\x1b[{}A", n);
}

pub fn clear_to_end_of_screen() {
    print!("\x1b[0J");
}
