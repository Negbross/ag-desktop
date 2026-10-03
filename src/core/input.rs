use enigo::{Enigo, Mouse, Keyboard, Settings, Button, Direction::{Click, Press, Release}, Coordinate};

fn new_enigo() -> anyhow::Result<Enigo> {
    Ok(Enigo::new(&Settings::default())?)
}

pub fn move_mouse(x: i32, y: i32) -> anyhow::Result<()> {
    let mut e = new_enigo()?;
    e.move_mouse(x, y, Coordinate::Abs)?;
    Ok(())
}

pub fn click(button: &str) -> anyhow::Result<()> {
    let mut e = new_enigo()?;
    let btn = match button {
        "left" => Button::Left,
        "right" => Button::Right,
        "middle" => Button::Middle,
        _ => anyhow::bail!("button harus: left/right/middle"),
    };
    e.button(btn, Click)?;
    Ok(())
}

pub fn drag(from: (i32, i32), to: (i32, i32)) -> anyhow::Result<()> {
    let mut e = new_enigo()?;
    e.move_mouse(from.0, from.1, Coordinate::Abs)?;
    e.button(Button::Left, Press)?;
    e.move_mouse(to.0, to.1, Coordinate::Abs)?;
    e.button(Button::Left, Release)?;
    Ok(())
}

pub fn scroll(amount: i32) -> anyhow::Result<()> {
    let mut e = new_enigo()?;
    e.scroll(amount, enigo::Axis::Vertical)?;
    Ok(())
}

pub fn type_text(text: &str) -> anyhow::Result<()> {
    let mut e = new_enigo()?;
    e.text(text)?;
    Ok(())
}

pub fn key_press(key_name: &str) -> anyhow::Result<()> {
    let mut e = new_enigo()?;
    let key = match key_name.to_lowercase().as_str() {
        "enter" => enigo::Key::Return,
        "tab" => enigo::Key::Tab,
        "escape" | "esc" => enigo::Key::Escape,
        "backspace" => enigo::Key::Backspace,
        "space" => enigo::Key::Space,
        _ => anyhow::bail!("key belum dipetakan: {key_name}"),
    };
    e.key(key, Click)?;
    Ok(())
}