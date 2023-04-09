use ndarray::{ArrayBase, DataMut, Dimension, NdIndex};

pub fn flood_fill<S, D, I, C, F>(
    array: &mut ArrayBase<S, D>,
    mut stack: Vec<I>,
    new_color: C,
    get_neighbors: F,
) where
    S: DataMut<Elem = C>,
    D: Dimension,
    I: NdIndex<D> + Copy,
    C: Copy + PartialEq,
    F: Fn(&I) -> Vec<I>,
{
    while let Some(coordinate) = stack.pop() {
        match array.get_mut(coordinate) {
            Some(color) if *color != new_color => {
                *color = new_color;
                stack.extend(get_neighbors(&coordinate));
            }
            _ => (),
        }
    }
}
