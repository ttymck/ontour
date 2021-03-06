use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum MapTile {
    Tee,
    TeeBox,
    Fairway,
    Green,
    Flag,
    Rough,
    DeepRough,
}

impl MapTile {
    pub fn from_char(c: &char) -> MapTile {
        match c {
            'T' => MapTile::Tee,
            'D' => MapTile::TeeBox,
            '=' => MapTile::Fairway,
            '@' => MapTile::Green,
            'F' => MapTile::Flag,
            _ => MapTile::DeepRough,
        }
    }

    pub fn glyph(self) -> u16 {
        let c = match self {
            MapTile::Tee => 'T',
            MapTile::TeeBox => '█',
            MapTile::Fairway => '█',
            MapTile::Green => '█',
            MapTile::Flag => 'F',
            MapTile::Rough => '░',
            MapTile::DeepRough => '░',
        };
        to_cp437(c)
    }

    pub fn color_pair(self) -> ColorPair {
        ColorPair::new(self.color(), self.bg())
    }

    pub fn color(self) -> (u8, u8, u8) {
        match self {
            MapTile::Tee => WHITE,
            MapTile::TeeBox => DARKGREEN,
            MapTile::Fairway => FOREST_GREEN,
            MapTile::Green => GREEN,
            MapTile::Flag => RED,
            MapTile::Rough => DARKGREEN,
            MapTile::DeepRough => DARKOLIVEGREEN,
        }
    }

    pub fn bg(self) -> (u8, u8, u8) {
        match self {
            MapTile::Tee => DARKGREEN,
            MapTile::Flag => LIGHTGREEN,
            MapTile::Rough => BLACK,
            MapTile::DeepRough => BLACK,
            _ => self.color(),
        }
    }
}
