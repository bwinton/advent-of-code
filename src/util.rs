
/// Calculate the indices around x and y to the specified depth, handling (0,0), and return them in a vec.
/// Note: includes (x,y)
pub fn ring(x: usize, y: usize, depth: usize) -> Vec<(usize,usize)> {
    let mut rv = vec![];
    for i in x.saturating_sub(depth)..=x + depth {
        for j in y.saturating_sub(depth)..=y + depth {
            rv.push((i, j));
        }
    }
    rv
}
