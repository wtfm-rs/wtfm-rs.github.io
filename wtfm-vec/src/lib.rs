#![doc(html_playground_url = "https://play.rust-lang.org/")]

//! Explore `std::vec::Vec` ([RTFM](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html))
//!
//! # [vec_push_pop](fn.vec_push_pop.html)
//! # [vec_extend](fn.vec_extend.html)
//!
/// ```
/// pub fn vec_push_pop() {
///     let mut vec = std::vec::Vec::new();
///     vec.push(1);
///     vec.push(2);
///
///     assert_eq!(vec.len(), 2);
///     assert_eq!(vec[0], 1);
///
///     assert_eq!(vec.pop(), Some(2));
///     assert_eq!(vec.len(), 1);
/// }
/// vec_push_pop();
/// ```
pub fn vec_push_pop() {
    let mut vec = std::vec::Vec::new();
    vec.push(1);
    vec.push(2);

    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);
}

/// ```
/// pub fn vec_extend() {
///    let mut vec = std::vec::Vec::new();
///    vec.push(1);
///    assert_eq!(vec[0], 1);
///    vec[0] = 7;
///    assert_eq!(vec[0], 7);
///    vec.extend([1, 2, 3]);
///    assert_eq!(vec, [7, 1, 2, 3]);
/// }
/// vec_extend();
/// ```
pub fn vec_extend() {
    let mut vec = std::vec::Vec::new();
    vec.push(1);
    assert_eq!(vec[0], 1);
    vec[0] = 7;
    assert_eq!(vec[0], 7);
    vec.extend([1, 2, 3]);
    assert_eq!(vec, [7, 1, 2, 3]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_push_pop() {
        crate::vec_push_pop();
    }
    #[test]
    fn test_vec_extend() {
        crate::vec_extend();
    }
}
