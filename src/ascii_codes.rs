pub mod style {
    pub const RESET: &str = "\x1b[0m"; // reset all modes (styles and colors)

    pub const BOLD: &str = "\x1b[1m"; // set bold mode
    pub const BOLD_END: &str = "\x1b[22m"; // reset bold mode

    pub const DIM: &str = "\x1b[2m"; // set dim/faint mode
    pub const DIM_END: &str = "\x1b[22m"; // reset dim/faint mode

    pub const ITALIC: &str = "\x1b[3m"; // set italic mode
    pub const ITALIC_END: &str = "\x1b[23m"; // reset italic mode

    pub const UNDERLINE: &str = "\x1b[4m"; // set underline mode
    pub const UNDERLINE_END: &str = "\x1b[24m"; // reset underline mode

    pub const BLINK: &str = "\x1b[5m"; // set blinking mode
    pub const BLINK_END: &str = "\x1b[25m"; // reset blinking mode

    pub const INVERSE: &str = "\x1b[7m"; // set inverse/reverse mode
    pub const INVERSE_END: &str = "\x1b[27m"; // reset inverse/reverse mode

    pub const HIDDEN: &str = "\x1b[8m"; // set hidden/invisible mode
    pub const HIDDEN_END: &str = "\x1b[28m"; // reset hidden/invisible mode

    pub const STRIKETHROUGH: &str = "\x1b[9m"; // set strikethrough mode
    pub const STRIKETHROUGH_END: &str = "\x1b[29m"; // reset strikethrough mode
}

pub mod erase {
    pub const CURSOR_SCREENEND: &str = "\x1b[0J"; // erase from cursor until end of screen
    pub const CURSOR_SCREENSTART: &str = "\x1b[1J"; // erase from cursor to beginning of screen
    pub const ENTIRE_SCREEN: &str = "\x1b[2J"; // erase entire screen
    pub const CURSOR_LINEEND: &str = "\x1b[0K"; // erase from cursor to end of line
    pub const CURSOR_LINESTART: &str = "\x1b[1K"; // erase start of line to the cursor
    pub const ENTIRE_LINE: &str = "\x1b[2K"; // erase the entire line
}

pub mod color {
    // F -> Foreground
    // B -> Background
    pub const F_BLACK: &str = "\x1b[30m";
    pub const B_BLACK: &str = "\x1b[40m";
    pub const F_RED: &str = "\x1b[31m";
    pub const B_RED: &str = "\x1b[41m";
    pub const F_GREEN: &str = "\x1b[32m";
    pub const B_GREEN: &str = "\x1b[42m";
    pub const F_YELLOW: &str = "\x1b[33m";
    pub const B_YELLOW: &str = "\x1b[43m";
    pub const F_BLUE: &str = "\x1b[34m";
    pub const B_BLUE: &str = "\x1b[44m";
    pub const F_MAGENTA: &str = "\x1b[35m";
    pub const B_MAGENTA: &str = "\x1b[45m";
    pub const F_CYAN: &str = "\x1b[36m";
    pub const B_CYAN: &str = "\x1b[46m";
    pub const F_WHITE: &str = "\x1b[37m";
    pub const B_WHITE: &str = "\x1b[47m";
    pub const F_DEFAULT: &str = "\x1b[39m";
    pub const B_DEFAULT: &str = "\x1b[49m";
}
