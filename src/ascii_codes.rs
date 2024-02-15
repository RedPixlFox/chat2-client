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
    // B_X sets the background color to color X
    pub const BLACK: &str = "\x1b[30";
    pub const B_BLACK: &str = "\x1b[40";
    pub const RED: &str = "\x1b[31";
    pub const B_RED: &str = "\x1b[41";
    pub const GREEN: &str = "\x1b[32";
    pub const B_GREEN: &str = "\x1b[42";
    pub const YELLOW: &str = "\x1b[33";
    pub const B_YELLOW: &str = "\x1b[43";
    pub const BLUE: &str = "\x1b[34";
    pub const B_BLUE: &str = "\x1b[44";
    pub const MAGENTA: &str = "\x1b[35";
    pub const B_MAGENTA: &str = "\x1b[45";
    pub const CYAN: &str = "\x1b[36";
    pub const B_CYAN: &str = "\x1b[46";
    pub const WHITE: &str = "\x1b[37";
    pub const B_WHITE: &str = "\x1b[47";
    pub const DEFAULT: &str = "\x1b[39";
    pub const B_DEFAULT: &str = "\x1b[49";
    pub const RESET: &str = "\x1b[0";
    pub const B_RESET: &str = "\x1b[0";
}
