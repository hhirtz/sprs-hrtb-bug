// If this line is commented, then "cargo build" succeeds
extern crate sprs;

fn f<T>(_arr: &[T])
where
    for<'a> &'a T: std::ops::Sub<Output = T>,
{
}

pub fn g() {
    let w = [1, 2, 3, 4, 5, 6];
    f(&w);
}
