use crate::models::settings::Settings;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MainMenuFont {
    pub logo: Logo,
    pub keybinding: KeyBinding,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Logo {
    pub width: u16,
    pub height: u16,
    pub offset: u16,
    pub text: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KeyBinding {
    pub width: u16,
    pub height: u16,
    pub total_height: u16,
    pub longest_description: u16,
    pub key_desc_map: Vec<(String, String)>,
    pub vertical_gap: u16,
}

impl Logo {
    fn new(logo: String) -> Self {
        let height = logo.lines().count() as u16;
        let width = logo.lines().last().unwrap_or("").chars().count() as u16;
        Self {
            width,
            height,
            text: logo,
            offset: 0,
        }
    }
}

impl KeyBinding {
    fn new(keybinds: [(&str, &str); 3], gap: u16) -> Self {
        let height = keybinds[0].0.lines().count() as u16;
        let width = keybinds[0]
            .0
            .to_string()
            .lines()
            .last()
            .unwrap_or("")
            .chars()
            .count() as u16;
        let total_height = (height + gap) * keybinds.len() as u16;
        let longest_description = keybinds
            .map(|(_, desc)| {
                desc.lines()
                    .map(|line| line.chars().count())
                    .max()
                    .unwrap_or(0)
            })
            .iter()
            .max()
            .unwrap_or(&0)
            .to_owned() as u16;
        let key_desc_map = keybinds
            .map(|(key, desc)| (key.to_string(), desc.to_string()))
            .to_vec();

        Self {
            width,
            height,
            longest_description,
            key_desc_map,
            total_height,
            vertical_gap: gap,
        }
    }
}

impl MainMenuFont {
    pub fn new(width: u16, height: u16, settings: Settings) -> Self {
        let large = Self::new_large();
        let small = Self::new_small();
        let tiny = Self::new_tiny();
        match settings.font_size {
            crate::models::settings::FontSize::Auto => {
                if large.logo.width < width
                    && large.keybinding.longest_description + 6 < width
                    && large.keybinding.total_height + 6 + large.logo.height < height
                {
                    return large;
                } else if small.logo.width < width
                    && small.keybinding.longest_description + 6 < width
                    && small.keybinding.total_height + 6 + small.logo.height < height
                {
                    return small;
                }
                return tiny;
            }
            crate::models::settings::FontSize::Large => large,
            crate::models::settings::FontSize::Small => small,
            crate::models::settings::FontSize::Tiny => tiny,
        }
    }

    fn new_large() -> MainMenuFont {
        MainMenuFont {
            logo: Logo::new(POMO_BUILD_LOGO_LARGE.into()),
            keybinding: KeyBinding::new(KEYBIND_STRINGS_LARGE, GAP_LARGE),
        }
    }

    fn new_small() -> MainMenuFont {
        MainMenuFont {
            logo: Logo::new(POMO_BUILD_LOGO_SMALL.into()),
            keybinding: KeyBinding::new(KEYBIND_STRINGS_SMALL, GAP_SMALL),
        }
    }

    fn new_tiny() -> MainMenuFont {
        MainMenuFont {
            logo: Logo::new(POMO_BUILD_LOGO_TINY.into()),
            keybinding: KeyBinding::new(KEYBIND_STRINGS_TINY, GAP_TINY),
        }
    }
}

// ANSII SHADOW
const POMO_BUILD_LOGO_LARGE: &str =
    "███████╗ ██████╗  ██████╗██╗   ██╗███████╗██████╗ ██╗   ██╗██╗██╗     ██████╗ 
██╔════╝██╔═══██╗██╔════╝██║   ██║██╔════╝██╔══██╗██║   ██║██║██║     ██╔══██╗
█████╗  ██║   ██║██║     ██║   ██║███████╗██████╔╝██║   ██║██║██║     ██║  ██║
██╔══╝  ██║   ██║██║     ██║   ██║╚════██║██╔══██╗██║   ██║██║██║     ██║  ██║
██║     ╚██████╔╝╚██████╗╚██████╔╝███████║██████╔╝╚██████╔╝██║███████╗██████╔╝
╚═╝      ╚═════╝  ╚═════╝ ╚═════╝ ╚══════╝╚═════╝  ╚═════╝ ╚═╝╚══════╝╚═════╝ 
                                                                              ";

const GAP_LARGE: u16 = 1;
// RubiFont
const KEYBIND_STRINGS_LARGE: [(&str, &str); 3] = [
    (
        " ▗▄▄▖
▐▌   
 ▝▀▚▖
▗▄▄▞▘",
        " ▗▄▄▖▗▄▄▄▖▗▄▖ ▗▄▄▖▗▄▄▄▖
▐▌     █ ▐▌ ▐▌▐▌ ▐▌ █  
 ▝▀▚▖  █ ▐▛▀▜▌▐▛▀▚▖ █  
▗▄▄▞▘  █ ▐▌ ▐▌▐▌ ▐▌ █  ",
    ),
    (
        "▗▄▄▄ 
▐▌  █
▐▌  █
▐▙▄▄▀",
        "▗▄▄▄  ▗▄▖▗▄▄▄▖▗▄▖ 
▐▌  █▐▌ ▐▌ █ ▐▌ ▐▌
▐▌  █▐▛▀▜▌ █ ▐▛▀▜▌
▐▙▄▄▀▐▌ ▐▌ █ ▐▌ ▐▌",
    ),
    (
        "▗▄▄▄▖ 
▐▌ ▐▌ 
▐▌ ▐▌ 
▐▙▄▟▙▖",
        "▗▄▄▄▖ ▗▖ ▗▖▗▄▄▄▖▗▄▄▄▖
▐▌ ▐▌ ▐▌ ▐▌  █    █  
▐▌ ▐▌ ▐▌ ▐▌  █    █  
▐▙▄▟▙▖▝▚▄▞▘▗▄█▄▖  █  ",
    ),
];

// Small Slant
const POMO_BUILD_LOGO_SMALL: &str = "   ____                  ___       _ __   __
  / __/__  ______ _____ / _ )__ __(_) /__/ /
 / _// _ \\/ __/ // (_-</ _  / // / / / _  / 
/_/  \\___/\\__/\\_,_/___/____/\\_,_/_/_/\\_,_/  
                                            ";
const GAP_SMALL: u16 = 0;
// Big Fig
const KEYBIND_STRINGS_SMALL: [(&str, &str); 3] = [
    (
        " __
(_ 
__)",
        " __            
(_ _|_ _  ___|_
__) |_(_| |  |_",
    ),
    (
        " _ 
| \\
|_/",
        " _          
| \\ _ _|_ _ 
|_/(_| |_(_|",
    ),
    (
        " _ 
/ \\
\\_X",
        " _          
/ \\    o _|_
\\_X|_| |  |_",
    ),
];

const POMO_BUILD_LOGO_TINY: &str = "╔═╗┌─┐┌─┐┬ ┬┌─┐╔╗ ┬ ┬┬┬  ┌┬┐
╠╣ │ ││  │ │└─┐╠╩╗│ │││   ││
╚  └─┘└─┘└─┘└─┘╚═╝└─┘┴┴─┘─┴┘";

// Calvin S
const KEYBIND_STRINGS_TINY: [(&str, &str); 3] = [
    (
        "╔═╗
╚═╗
╚═╝",
        "╔═╗┌┬┐┌─┐┬─┐┌┬┐
╚═╗ │ ├─┤├┬┘ │ 
╚═╝ ┴ ┴ ┴┴└─ ┴ ",
    ),
    (
        "╔╦╗
 ║║
═╩╝",
        "╔╦╗┌─┐┌┬┐┌─┐
 ║║├─┤ │ ├─┤
═╩╝┴ ┴ ┴ ┴ ┴",
    ),
    (
        "╔═╗ 
║═╬╗
╚═╝╚",
        "╔═╗ ┬ ┬┬┌┬┐
║═╬╗│ ││ │ 
╚═╝╚└─┘┴ ┴ ",
    ),
];

const GAP_TINY: u16 = 0;
