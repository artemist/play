use std::iter::FromIterator;

pub(crate) const MAX_TILE: u32 = 13;
pub(crate) const NUM_WILDCARDS: u32 = 2;
pub(crate) const NUM_COPIES: u32 = 2;
pub(crate) const MIN_GROUP_SIZE: u32 = 3;
pub(crate) const COLORS: [TileColor; 4] = [
    TileColor::Blue,
    TileColor::Black,
    TileColor::Red,
    TileColor::Orange,
];

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) enum TileColor {
    Blue,
    Black,
    Red,
    Orange,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct NormalTile {
    color: TileColor,
    value: u32,
}

impl NormalTile {
    pub(crate) fn new(color: TileColor, value: u32) -> NormalTile {
        NormalTile { color, value }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) enum Tile {
    Normal(NormalTile),
    Wildcard,
}

impl From<Tile> for Option<NormalTile> {
    fn from(t: Tile) -> Self {
        match t {
            Tile::Normal(n) => Some(n),
            Tile::Wildcard => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) enum GroupType {
    Run,
    Colors,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Group {
    typ: GroupType,
    tiles: Vec<NormalTile>,
    count: u32,
    sum: u32,
}
impl Group {
    pub(crate) fn new(typ: GroupType, tiles: Vec<NormalTile>) -> Group {
        Group {
            typ,
            tiles: tiles.clone(),
            count: tiles.len() as u32,
            sum: tiles.iter().map(|tile| tile.value).sum(),
        }
    }
}
pub(crate) fn all_tiles() -> Vec<Tile> {
    let mut tiles = COLORS
        .iter()
        .flat_map(|color| build_run(color.clone(), (1..(MAX_TILE + 1)).collect()))
        .map(|normal_tile| Tile::Normal(normal_tile))
        .collect::<Vec<_>>();

    let orig_tiles = tiles.clone();

    for _ in 1..NUM_COPIES {
        tiles.append(&mut orig_tiles.clone());
    }

    tiles.append(
        &mut std::iter::repeat(Tile::Wildcard)
            .take(NUM_WILDCARDS as usize)
            .collect(),
    );

    tiles
}

fn build_run(color: TileColor, values: Vec<u32>) -> Vec<NormalTile> {
    values
        .iter()
        .map(|value| NormalTile {
            color,
            value: *value,
        })
        .collect()
}

fn remove_instance<I: PartialEq>(vec: &mut Vec<I>, item: I) {
    if let Some(idx) = vec.iter().position(|it| *it == item) {
        vec.remove(idx);
    }
}

fn remove_tiles(tiles: &mut Vec<NormalTile>, run: Vec<NormalTile>) {
    for tile in run {
        remove_instance(tiles, tile);
    }
}

pub(crate) fn find_groups(tiles: &Vec<Tile>) -> Vec<Group> {
    // Note that this is a greedy algorithm, so it is non-optimal.
    // Hopefully the difference isn't too big

    let mut groups = Vec::new();
    let mut tiles_left: Vec<NormalTile> = tiles
        .iter()
        .filter_map(|tile| Option::from(*tile))
        .collect();

    for color in &COLORS {
        let mut tile_numbers: Vec<_> = tiles_left
            .iter()
            .filter_map(|tile| {
                if tile.color == *color {
                    Some(tile.value)
                } else {
                    None
                }
            })
            .collect();

        tile_numbers.sort_unstable();

        if let Some(min_tile) = tile_numbers.first() {
            let mut i = *min_tile;
            let mut current_group = Vec::new();
            while i <= MAX_TILE {
                if tile_numbers.contains(&i) {
                    current_group.push(i);
                } else if current_group.len() as u32 >= MIN_GROUP_SIZE {
                    let run = build_run(*color, current_group.clone());
                    groups.push(Group::new(GroupType::Run, run.clone()));
                    remove_tiles(&mut tiles_left, run.clone());

                    current_group.clear();
                }
                i += 1;
            }
        }
    }

    // Next find the groups of different colors

    for i in 1..=MAX_TILE {
        let mut tiles = Vec::new();
        for color in &COLORS {
            let tile = NormalTile {
                color: color.clone(),
                value: i,
            };
            if tiles_left.contains(&tile) {
                tiles.push(tile);
            }
        }
        if tiles.len() as u32 >= MIN_GROUP_SIZE {
            groups.push(Group::new(GroupType::Colors, tiles.clone()));
            remove_tiles(&mut tiles_left, tiles.clone());
            tiles.clear();
        }
    }

    groups.clone()
}
