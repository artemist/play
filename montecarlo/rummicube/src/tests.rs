use crate::tile::*;

#[test]
fn test_trivial_run() {
    let tiles: Vec<NormalTile> = (1..=MIN_GROUP_SIZE)
        .map(|i| NormalTile::new(TileColor::Blue, i as u32))
        .collect();

    let groups = find_groups(&tiles.iter().map(|n| Tile::Normal(*n)).collect());
    let expected = vec![Group::new(GroupType::Run, tiles)];
    assert_eq!(groups, expected)
}

#[test]
fn test_5_run() {
    let tiles: Vec<NormalTile> = (1..=5)
        .map(|i| NormalTile::new(TileColor::Blue, i as u32))
        .collect();

    let groups = find_groups(&tiles.iter().map(|n| Tile::Normal(*n)).collect());
    let expected = vec![Group::new(GroupType::Run, tiles)];
    assert_eq!(groups, expected)

}

#[test]
fn test_too_short_run() {
    let tiles: Vec<NormalTile> = (1..=2)
        .map(|i| NormalTile::new(TileColor::Blue, i as u32))
        .collect();

    let groups = find_groups(&tiles.iter().map(|n| Tile::Normal(*n)).collect());
    let expected = vec![];
    assert_eq!(groups, expected)
}
