use {
    crossterm::style::{Attribute::*, Color::*},
    termimad::*,
};

pub fn make_skin(color: bool) -> MadSkin {
    if color {
        make_color_skin()
    } else {
        make_no_color_skin()
    }
}

fn make_color_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(AnsiValue(217));
    skin.italic.remove_attr(Italic);
    skin.italic.set_fg(AnsiValue(30));
    skin
}

fn make_no_color_skin() -> MadSkin {
    MadSkin::no_style()
}
