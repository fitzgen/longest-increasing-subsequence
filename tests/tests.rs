use longest_increasing_subsequence::lis;

#[test]
fn a_bunch_of_tests() {
    for (input, expected) in vec![
        (vec![], vec![]),
        (vec![3], vec![0]),
        (vec![3, 2], vec![1]),
        (vec![3, 3], vec![0]),
        (vec![3, 4], vec![0, 1]),
        (vec![7, 8, 9], vec![0, 1, 2]),
        (vec![9, 8, 7], vec![2]),
        (vec![1, 7, 2, 8, 3, 9, 4, 5], vec![0, 2, 4, 6, 7]),
    ] {
        eprintln!("=====================================================");
        let input = dbg!(input);
        let actual = dbg!(lis(&input));
        assert_eq!(actual, expected);
    }
}
