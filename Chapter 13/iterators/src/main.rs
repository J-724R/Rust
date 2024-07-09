fn main() {
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    // Methods that Produce Other Iterators
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
