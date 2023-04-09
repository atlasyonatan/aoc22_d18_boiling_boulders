use ndarray::Array;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let file_path = "../input.txt";
    let path = Path::new(file_path);
    let file = File::open(path).unwrap();
    let cubes: Vec<[usize; 3]> = io::BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();
    let mut mins = [usize::MAX; 3];
    let mut maxs = [usize::MIN; 3];
    for cube in cubes.iter() {
        for i in 0..cube.len() {
            mins[i] = mins[i].min(cube[i]);
            maxs[i] = maxs[i].max(cube[i]);
        }
    }
    let shape: [usize; 3] = maxs
        .iter()
        .zip(mins.iter())
        .map(|(max, min)| max - min + 1)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let mut space = Array::from_shape_simple_fn(shape, || false);
    let coordinates: Vec<[usize; 3]> = cubes
        .into_iter()
        .map(|cube| {
            cube.iter()
                .zip(mins.iter())
                .map(|(coordinate, offset)| coordinate - offset)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    for &coordinate in coordinates.iter() {
        space[coordinate] = true;
    }
    let cardinals: [[i32; 3]; 6] = [
        [1, 0, 0],
        [0, 1, 0],
        [0, 0, 1],
        [-1, 0, 0],
        [0, -1, 0],
        [0, 0, -1],
    ];
    let occupied_neighbors_count = coordinates
        .iter()
        .flat_map(|coordinates| {
            cardinals
                .iter()
                .filter_map::<[usize; 3], _>(move |direction| {
                    Some(
                        coordinates
                            .iter()
                            .zip(direction.iter())
                            .map(|(&coordinate, &step)| ((coordinate as i32) + step).try_into())
                            .collect::<Result<Vec<usize>, <i32 as TryInto<usize>>::Error>>()
                            .ok()?
                            .try_into()
                            .unwrap(),
                    )
                })
        })
        .filter_map(|coordinates| space.get(coordinates))
        .filter(|&cell_value| *cell_value)
        .count();
    let surface_area = coordinates.into_iter().count() * 6 - occupied_neighbors_count;
    println!("Part 1: {}", surface_area);
}
