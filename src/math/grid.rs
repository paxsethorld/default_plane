//each plane is sliced into a 15x15 grid
//index 0 is mapped to (-7, 7) while index 224 is (7, 7)

use std::collections::HashMap;


const PLANE_LENGTH: i32 = 15;
pub struct Coordinate{
    x: i32,
    y: i32,
}

pub struct Index{
    index: i32,
}

pub fn index_to_coord(index: Index) -> Coordinate {
    let modulus: i32 = index.index % PLANE_LENGTH;
    let x: i32 = ((PLANE_LENGTH - 1) * -1 / 2) + modulus;

    let divide: i32 = index.index / PLANE_LENGTH;
    let y: i32 = ((PLANE_LENGTH - 1) * -1 / 2) + divide;

    Coordinate{
        x,
        y,
    }
}

pub fn coord_to_index(coord: Coordinate) -> Index {
    let mut mods: HashMap<i32, Vec<i32>> =  HashMap::new();
    mods.insert(0, vec![0, 15, 30, 45, 60, 75, 90, 105, 120, 135, 150, 165, 180, 195, 210]);

    let mut quotients: HashMap<i32, Vec<i32>> = HashMap::new();
    quotients.insert(0, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);

    let modulus: i32 = (((2 * coord.x) + PLANE_LENGTH - 1) / 2).abs();
    let divide: i32 = (((2 * coord.y) + PLANE_LENGTH - 1) / 2).abs();

    let vec1 = mods.get(&modulus).unwrap();
    let vec2 = quotients.get(&divide).unwrap();

    let mut index = 0;
    for i in vec1.iter() {
        if vec2.contains(i) {
            index = *i;
            break;
        }
    }

    Index{
        index
    }
}