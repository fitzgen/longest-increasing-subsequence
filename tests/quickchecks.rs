use longest_increasing_subsequence::lis;
use quickcheck::quickcheck;

quickcheck! {
    fn output_sequence_length_is_less_than_or_equal_input_length(xs: Vec<u32>) -> bool {
        let seq = lis(&xs);
        seq.len() <= xs.len()
    }

    fn output_seq_indices_in_increasing_order(xs: Vec<u32>) -> bool {
        let seq = lis(&xs);
        seq.windows(2).all(|w| w[0] < w[1])
    }

    fn output_seq_items_in_increasing_order(xs: Vec<u32>) -> bool {
        let seq = lis(&xs);
        seq.windows(2).all(|w| xs[w[0]] < xs[w[1]])
    }

    fn lis_of_non_empty_input_is_non_empty(xs: Vec<u32>) -> bool {
        let seq = lis(&xs);
        xs.is_empty() || !seq.is_empty()
    }
}
