// Copyright 2021 Simeon Miteff
// This Rust implementation of Conway's Game Of Life is optimised for speed and memory use
// and doesn't use heap allocation. My first attempt used a bitset (basically a translation
// of the C++ code at https://fdi.sk/posts/life/) but the bit operations were expensive.
//
// This "coordinate map" implementation is quite common. The rabbit hole goes deep:
//
// - https://conwaylife.com/wiki/Xlife
// - http://dotat.at/prog/life/life.html
// - http://www.jagregory.com/abrash-black-book/#chapter-17-the-game-of-life
//
// I adapted the Rust version from Rosetta code:
//
// -  https://rosettacode.org/wiki/Conway%27s_Game_of_Life#Rust
//
// The original is licensed under the GDL (what a horrible license!). My changes
// are extensive (so not "copied verbatim" as described here:
// https://rosettacode.org/wiki/Rosetta_Code:Copyrights) but I'm not a lawyer
// so I don't know if it is still technically a derivative work. If you're
// User:Stephenw, please drop me an email!

#![no_std]

const WIDTH: u8 = 128;
const HEIGHT: u8 = 64;
const MAX_CELLS: usize = 32;
const MAX_NEIGHBOURS: usize = 128;

use heapless::FnvIndexMap;
use heapless::FnvIndexSet;

type Cell = (u8, u8);
type NeighbourMap = FnvIndexMap<Cell, u8, MAX_NEIGHBOURS>;
pub type Colony = FnvIndexSet<Cell, MAX_CELLS>;

/// neighbours returns the row/col coordinates for the 8 neighbours of a given cell
fn neighbours(&(x, y): &Cell) -> [Cell; 8] {
    // Wrap around the grid (toroidal surface)
    let above = if y == 0 { HEIGHT - 1 } else { y - 1 };
    let below = if y == HEIGHT - 1 { 0 } else { y + 1 };
    let left = if x == 0 { WIDTH - 1 } else { x - 1 };
    let right = if x == WIDTH - 1 { 0 } else { x + 1 };

    [
        (left, above),
        (x, above),
        (right, above),
        (left, y),
        (right, y),
        (left, below),
        (x, below),
        (right, below),
    ]
}

/// neighbour_counts returns a map of neighbour coordinate -> count for lives cells in a Colony
fn neighbour_counts(colony: &Colony) -> NeighbourMap {
    let mut counts = FnvIndexMap::new();
    for neighbour_cell in colony.iter().flat_map(neighbours) {
        counts
            .insert(
                neighbour_cell,
                match counts.get(&neighbour_cell) {
                    Some(value) => value + 1,
                    None => 1,
                },
            )
            .unwrap();
    }
    counts
}

/// generation computes a Colony at generation n+1
pub fn generation(col: Colony) -> Colony {
    let mut new_colony = Colony::new();
    for cell in neighbour_counts(&col)
        .into_iter()
        .filter_map(|(cell, cnt)| match (cnt, col.contains(cell)) {
            (2, true) | (3, ..) => Some(cell),
            _ => None,
        })
    {
        new_colony.insert(*cell).unwrap();
    }
    new_colony
}

/// seed_glider sets cells live in the shape of a glider, with top/left corner of x, y
pub fn seed_glider(col: &mut Colony, x: u8, y: u8) {
    for cell in [
        (x + 1, y),
        (x + 2, y + 1),
        (x, y + 2),
        (x + 1, y + 2),
        (x + 2, y + 2),
    ] {
        col.insert(cell).ok();
    }
}

/// seed_gliders makes 4 gliders in formation, 10 pixels apart (horizontally)
pub fn seed_gliders(col: &mut Colony) {
    seed_glider(col, 0, 0);
    seed_glider(col, 10, 0);
    seed_glider(col, 20, 0);
    seed_glider(col, 30, 0);
}
