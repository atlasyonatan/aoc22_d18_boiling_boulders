use ndarray::Array;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub mod flood_fill;
use crate::flood_fill::flood_fill;

fn main() {
    let file_path = "../test.txt";
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
    let padding = 1;
    let shape: [usize; 3] = maxs
        .iter()
        .zip(mins.iter())
        .map(|(max, min)| max - min + 1 + padding)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let mut space = Array::zeros(shape);
    let coordinates: Vec<[usize; 3]> = cubes
        .into_iter()
        .map(|cube| {
            cube.iter()
                .zip(mins.iter())
                .map(|(coordinate, offset)| coordinate - offset + padding)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();
    let formation_color = 1;
    for &coordinate in coordinates.iter() {
        space[coordinate] = formation_color;
    }
    let cardinals = vec![
        [1, 0, 0],
        [0, 1, 0],
        [0, 0, 1],
        [-1, 0, 0],
        [0, -1, 0],
        [0, 0, -1],
    ];

    //part 1
    let occupied_neighbors_count = coordinates
        .iter()
        .flat_map(|coordinate| get_neighbors(coordinate, &cardinals))
        .filter_map(|neighbor| match space.get(neighbor) {
            Some(&color) if color == formation_color => Some(neighbor),
            _ => None,
        })
        .count();
    let surface_area = coordinates.into_iter().count() * 6 - occupied_neighbors_count;
    println!("Part 1: {}", surface_area);

    //part 2
    let outside_color = 2;
    flood_fill(&mut space, vec![[0; 3]], outside_color, |coordinate| {
        get_neighbors(coordinate, &cardinals)
    });
    let outside_coordinates: Vec<_> = space
        .indexed_iter()
        .filter_map(|(coordinate, &color)| (color == outside_color).then(|| coordinate))
        .map(|coordinate| [coordinate.0, coordinate.1, coordinate.2])
        .collect();
    // println!("outside_coordinates count = {}", outside_coordinates.iter().count());

    let occupied_neighbors_count = outside_coordinates
        .iter()
        .flat_map(|coordinate| get_neighbors(&coordinate, &cardinals))
        .filter_map(|neighbor| match space.get(neighbor) {
            Some(&color) if color == outside_color => Some(neighbor),
            _ => None,
        })
        .count();
    // println!("occupied_neighbors_count = {}", occupied_neighbors_count);
    //let outside_surface = 2 * (shape[0] * shape[1] + shape[0] * shape[2] + shape[1] * shape[2]);
    // println!("outside_surface = {}", outside_surface);
    let surface_area = outside_coordinates.into_iter().count() * 6 - occupied_neighbors_count;
    println!("Part 2: {}", surface_area);
}

fn get_neighbors(coordinate: &[usize; 3], steps: &Vec<[i32; 3]>) -> Vec<[usize; 3]> {
    steps
        .iter()
        .filter_map::<[usize; 3], _>(move |step| {
            Some(
                coordinate
                    .iter()
                    .zip(step.iter())
                    .map(|(&coordinate, &step)| ((coordinate as i32) + step).try_into())
                    .collect::<Result<Vec<usize>, <i32 as TryInto<usize>>::Error>>()
                    .ok()?
                    .try_into()
                    .unwrap(),
            )
        })
        .collect()
}
