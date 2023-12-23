use crate::tile::Tile;
use crate::tile::Colour;

#[derive(Debug)]
pub(crate) enum Consecutive {
    True { colour: Colour },
    False { number: u8, colours: Vec<Colour>},
}

// dont need to sort numbers in order, just ensure that the added number == start_number + length
#[derive(Debug)]
pub(crate) struct Run {
    tiles: Vec<Tile>,
    consecutive: Consecutive,
}

impl Run {
    fn add_tile(&mut self, tile: Tile) -> bool {
        return match &self.consecutive {
            Consecutive::True { colour } => {
                if tile.colour != *colour {
                    return false
                }

                match self.tiles.first() {
                    None => {
                        self.tiles.push(tile);
                        true
                    },
                    Some(first_tile) => {
                        if usize::from(tile.number) == usize::from(first_tile.number) + (&self.tiles.len()) {
                            self.tiles.push(tile);
                            true
                        } else {
                            false
                        }
                    }
                }
            },
            Consecutive::False { number, colours } => {
                colours.contains(&tile.colour) &&
                    tile.number == *number
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::run::{Consecutive, Run};
    use crate::tile::{Colour, Tile};

    #[test]
    fn fresh_add() {
        let tile = Tile {number: 1, colour: Colour::ORANGE };
        let mut run: Run = Run { tiles: vec![], consecutive: Consecutive::True {colour: Colour::ORANGE }};

        assert!(run.add_tile(tile));
    }

    #[test]
    fn consecutive_add() {
        let tile1 = Tile { number: 1, colour: Colour::RED };
        let tile2 = Tile { number: 2, colour: Colour::RED };
        let tile3 = Tile { number: 3, colour: Colour::RED };
        let mut run: Run = Run { tiles: vec![tile1, tile2], consecutive: Consecutive::True { colour: Colour::RED }};

        assert!(run.add_tile(tile3));
        assert_eq!(run.tiles.last().unwrap().number, 3);
        assert_eq!(run.tiles.last().unwrap().colour, Colour::RED);
    }

    #[test]
    fn consecutive_dup_num_add() {
        let tile1 = Tile { number: 1, colour: Colour::RED };
        let tile2 = Tile { number: 2, colour: Colour::RED };
        let tile3 = Tile { number: 3, colour: Colour::RED };
        let tile3_clone = Tile { number: 3, colour: Colour::RED };
        let mut run: Run = Run { tiles: vec![tile1, tile2, tile3], consecutive: Consecutive::True { colour: Colour::RED }};

        assert_eq!(run.add_tile(tile3_clone), false);
    }

    #[test]
    fn consecutive_missed_num_add() {
        let tile1 = Tile { number: 1, colour: Colour::RED };
        let tile3 = Tile { number: 3, colour: Colour::RED };
        let mut run: Run = Run { tiles: vec![tile1], consecutive: Consecutive::True { colour: Colour::RED }};

        assert_eq!(run.add_tile(tile3), false);
    }

    #[test]
    fn consecutive_bad_colour_add() {
        let tile1 = Tile { number: 1, colour: Colour::RED };
        let tile2 = Tile { number: 2, colour: Colour::ORANGE };
        let mut run: Run = Run { tiles: vec![tile1], consecutive: Consecutive::True { colour: Colour::RED }};

        assert_eq!(run.add_tile(tile2), false);
    }
}
