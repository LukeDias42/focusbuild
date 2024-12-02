use crate::models::settings::{FontSize, Settings};

pub struct TimerFont {
    pub title: Title,
    pub digits: Digits,
}

pub struct Title {
    pub offset: i16,
    pub width: u16,
    pub height: u16,
    pub text: String,
}

impl Title {
    pub fn new(title: String) -> Self {
        let height = title.lines().count() as u16;
        let width = title.lines().last().unwrap_or("").chars().count() as u16;
        Self {
            width,
            height,
            text: title,
            offset: 0,
        }
    }
}

impl Digits {
    fn new(digits_map: [&str; 10], chars_map: [&str; 2], gap: u16) -> Self {
        let digit_map: Vec<String> = digits_map.iter().map(|digit| digit.to_string()).collect();
        let height = digit_map[0].lines().count() as u16;
        let digit_width = digit_map[0].lines().last().unwrap_or("").chars().count() as u16;

        let colon = chars_map[0].to_string();
        let colon_width = colon.lines().last().unwrap_or("").chars().count() as u16;

        let minus_sign = chars_map[1].to_string();
        let minus_sign_width = minus_sign.lines().last().unwrap_or("").chars().count() as u16;

        let stopwatch_width = (digit_width + gap) * 4 + (colon_width + gap) + gap;
        let balance_width = stopwatch_width + (minus_sign_width + gap);

        Self {
            digit_map,
            digit_width,
            height,
            colon,
            colon_width,
            minus_sign,
            minus_sign_width,
            stopwatch_width,
            balance_width,
            gap,
        }
    }
}

pub struct Digits {
    pub digit_map: Vec<String>,
    pub digit_width: u16,
    pub colon: String,
    pub colon_width: u16,
    pub minus_sign: String,
    pub minus_sign_width: u16,
    pub height: u16,
    pub stopwatch_width: u16,
    pub balance_width: u16,
    pub gap: u16,
}
impl TimerFont {
    pub fn new(width: u16, height: u16, is_focusing: bool, settings: Settings) -> TimerFont {
        let large_font = TimerFont::new_large(is_focusing);
        let small_font = TimerFont::new_small(is_focusing);
        let tiny_font = TimerFont::new_tiny(is_focusing);
        match settings.font_size {
            FontSize::Auto => {
                if width
                    > (large_font.digits.stopwatch_width + 2) * 2
                        + (large_font.digits.balance_width + 2)
                        + 4
                    && height > large_font.digits.height + 4 + large_font.title.height
                {
                    large_font
                } else if width > small_font.title.width
                    && height > small_font.digits.height + 4 + small_font.title.height
                {
                    small_font
                } else {
                    tiny_font
                }
            }
            FontSize::Large => large_font,
            FontSize::Small => small_font,
            FontSize::Tiny => tiny_font,
        }
    }

    fn new_large(is_focusing: bool) -> TimerFont {
        let text = if is_focusing {
            FOCUS_TITLE.to_string()
        } else {
            BREAK_TITLE.to_string()
        };
        TimerFont {
            title: Title::new(text),
            digits: Digits::new(DIGITS_MAP_LARGE, TIMER_CHARS_LARGE, 1),
        }
    }

    fn new_small(is_focusing: bool) -> TimerFont {
        let text = if is_focusing {
            FOCUS_TITLE.to_string()
        } else {
            BREAK_TITLE.to_string()
        };
        TimerFont {
            title: Title::new(text),
            digits: Digits::new(DIGITS_MAP_SMALL, TIMER_CHARS_SMALL, 0),
        }
    }

    fn new_tiny(is_focusing: bool) -> TimerFont {
        let text = if is_focusing {
            FOCUS_TITLE_TINY.to_string()
        } else {
            BREAK_TITLE_TINY.to_string()
        };
        TimerFont {
            title: Title::new(text),
            digits: Digits::new(DIGITS_MAP_TINY, TIMER_CHARS_TINY, 0),
        }
    }
}

const FOCUS_TITLE: &str = "███████╗ ██████╗  ██████╗██╗   ██╗███████╗██╗
██╔════╝██╔═══██╗██╔════╝██║   ██║██╔════╝██║
█████╗  ██║   ██║██║     ██║   ██║███████╗██║
██╔══╝  ██║   ██║██║     ██║   ██║╚════██║╚═╝
██║     ╚██████╔╝╚██████╗╚██████╔╝███████║██╗
╚═╝      ╚═════╝  ╚═════╝ ╚═════╝ ╚══════╝╚═╝";
const BREAK_TITLE: &str = " ██████╗██╗  ██╗██╗██╗     ██╗     ██╗
██╔════╝██║  ██║██║██║     ██║     ██║
██║     ███████║██║██║     ██║     ██║
██║     ██╔══██║██║██║     ██║     ╚═╝
╚██████╗██║  ██║██║███████╗███████╗██╗
 ╚═════╝╚═╝  ╚═╝╚═╝╚══════╝╚══════╝╚═╝";
const DIGITS_MAP_LARGE: [&str; 10] = [
    " ████ 
██  ██
██  ██
██  ██
 ████ ",
    "████  
  ██  
  ██  
  ██  
██████",
    " ████ 
██  ██
   ██ 
 ██   
██████",
    " ████ 
██  ██
   ██
██  ██
 ████ ",
    "██  ██
██  ██
██████
    ██
    ██",
    "██████
██    
█████ 
    ██
█████ ",
    " ████ 
██    
█████ 
██  ██
 ████ ",
    "██████
   ██ 
  ██  
 ██   
██    ",
    " ████ 
██  ██
 ████ 
██  ██
 ████ ",
    " ████ 
██  ██
 █████
    ██
 ████ ",
];

const TIMER_CHARS_LARGE: [&str; 2] = [
    "  
██
  
██",
    "   
   
███
   ",
];

const DIGITS_MAP_SMALL: [&str; 10] = [
    "┏┓
┃┫
┗┛",
    "┓ 
┃ 
┻ ",
    "┏┓
┏┛
┗━",
    "┏┓
 ┫
┗┛",
    "┏┓
┃┃
┗╋",
    "┏━
┗┓
┗┛",
    "┏┓
┣┓
┗┛",
    "━┓
 ┃
 ╹",
    "┏┓
┣┫
┗┛",
    "┏┓
┗┫
┗┛",
];

const TIMER_CHARS_SMALL: [&str; 2] = [
    " 
•
•",
    "  
━━
  ",
];

const FOCUS_TITLE_TINY: &str = "╔═╗┌─┐┌─┐┬ ┬┌─┐┬
╠╣ │ ││  │ │└─┐│
╚  └─┘└─┘└─┘└─┘o";
const BREAK_TITLE_TINY: &str = "╔═╗┬ ┬┬┬  ┬  ┬
║  ├─┤││  │  │
╚═╝┴ ┴┴┴─┘┴─┘o";

const DIGITS_MAP_TINY: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

const TIMER_CHARS_TINY: [&str; 2] = [":", "-"];
