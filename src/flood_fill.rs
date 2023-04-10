use ndarray::{ArrayBase, DataMut, Dimension, NdIndex};

pub fn flood_fill<S, D, I, C, F>(
    array: &mut ArrayBase<S, D>,
    start: I,
    new_color: C,
    get_neighbors: F,
) where
    S: DataMut<Elem = C>,
    D: Dimension,
    I: NdIndex<D> + Copy,
    C: PartialEq + Copy,
    F: Fn(&I) -> Vec<I>,
{
    match array.get(start) {
        Some(&filling_color) => {
            let mut stack = vec![start];
            while let Some(coordinate) = stack.pop() {
                match array.get_mut(coordinate) {
                    Some(color) if *color == filling_color => {
                        *color = new_color;
                        stack.extend(get_neighbors(&coordinate));
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    }
}
