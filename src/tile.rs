#[derive(PartialEq, Debug)]
pub(crate) struct Tile {
    pub(crate) number: u8,
    pub(crate) colour: Colour
}

#[derive(PartialEq, Debug)]
pub(crate) enum Colour {
    RED,
    BLUE,
    BLACK,
    ORANGE,
}