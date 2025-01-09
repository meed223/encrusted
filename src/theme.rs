use std::path::PathBuf;
use hex_color::HexColor;

pub struct Colour {
    colour: HexColor,
    name: String,
    tags: Vec<str>
}

pub struct Font {
    font: String,
    name: String,
    tags: Vec<str>
}

pub struct Template {

}

pub struct Target {
    name: String,
    path: PathBuf,
    template: Template
}

pub struct Theme {
    name: String,
    colours: Vec<Colour>,
    fonts: Vec<Font>,
    targets: Vec<Target>
}