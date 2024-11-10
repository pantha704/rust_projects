fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        // Cloning vec0 to create a new vector so that vec0 remains accessible without owning it, just borrowing the values.
        let vec1 = fill_vec(vec0.clone());
        // if we didn't clone vec0, vec0 would be moved into fill_vec and would not be accessible here.

        // Assert that vec0 is still accessible and has not been modified by fill_vec.
        assert_eq!(vec0, [22, 44, 66]);
        // Assert that vec1 has been modified by fill_vec.
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
