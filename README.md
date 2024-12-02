# FocusBuild
A CLI focus timer that helps you stay focused and take balanced breaks.
It was written using [ratatui](https://ratatui.rs/) and [rusqlite](https://docs.rs/rusqlite/latest/rusqlite/).

![image](https://github.com/user-attachments/assets/d3b49b5f-659c-4eec-8299-b805ea64d98a)

## Premise
The idea is based on [focumon](https://www.focumon.com/), which has been helping me focus and control how much time I spend doing other things when trying to focus.
However, since Focumon can only be used on the browser or mobile app, I wanted a CLI alternative!
FocusBuild is a simple implementation of the same idea, where you can start your focus session and visualize a graph with the last focus sessions.
![image](https://github.com/user-attachments/assets/68334fa3-3a87-4741-ae5b-cfde779d2994)

## How to install
To install, simply clone the repository:
- `git clone https://github.com/LukeDias42/focusbuild`
And run with cargo:
- `cargo run --release`

## How to use
![image](https://github.com/user-attachments/assets/18773092-e285-4a56-8f1c-f8a6375d2d51)


Just start a session from the main menu, and change between focus and break accordingly.
Then press Q to save and stop the current session.
You can go to data from the main menu and visualize other days and specific sessions.
A SQLite database will be generated, currently changing that is the only way to change the settings.
