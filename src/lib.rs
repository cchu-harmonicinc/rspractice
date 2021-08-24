mod max_subset_sum_no_adjacent;
mod number_of_way_to_make_change;


#[cfg(test)]
mod tests {
    use crate::max_subset_sum_no_adjacent;
    #[test]
    fn test_max_subset_sum_no_adjacent() {
        assert_eq!(max_subset_sum_no_adjacent::solution(vec![75, 105, 120, 75, 90, 135]), 330);
        assert_eq!(max_subset_sum_no_adjacent::solution(vec![]), 0);
        assert_eq!(max_subset_sum_no_adjacent::solution(vec![1]), 1);
        assert_eq!(max_subset_sum_no_adjacent::solution(vec![1,2]), 2);
        assert_eq!(max_subset_sum_no_adjacent::solution(vec![1, 2, 3]), 4);
        assert_eq!(max_subset_sum_no_adjacent::solution(vec![1, 15, 3]), 15);
        assert_eq!(max_subset_sum_no_adjacent::solution(vec![7, 10, 12, 7, 9, 14]), 33);
        assert_eq!(max_subset_sum_no_adjacent::solution(vec![4, 3, 5, 200, 5, 3]), 207);
        assert_eq!(max_subset_sum_no_adjacent::solution(vec![10, 5, 20, 25, 15, 5, 5, 15]), 60);
        assert_eq!(max_subset_sum_no_adjacent::solution(vec![10, 5, 20, 25, 15, 5, 5, 15, 3, 15, 5, 5, 15]), 90);
        assert_eq!(max_subset_sum_no_adjacent::solution(vec![100, 10, 20, 10000]), 10100);
    }

    use crate::number_of_way_to_make_change;
    #[test]
    fn test_number_of_way_to_make_change() {
        assert_eq!(number_of_way_to_make_change::solution(6, vec![1,5]), 2);
        assert_eq!(number_of_way_to_make_change::solution(0, vec![2,3,4,7]), 1);
        assert_eq!(number_of_way_to_make_change::solution(9, vec![5]), 0);
        assert_eq!(number_of_way_to_make_change::solution(7, vec![2,4]), 0);
        assert_eq!(number_of_way_to_make_change::solution(4, vec![1,5,10,25]), 1);
        assert_eq!(number_of_way_to_make_change::solution(5, vec![1,5,10,25]), 2);
        assert_eq!(number_of_way_to_make_change::solution(10, vec![1,5,10,25]), 4);
        assert_eq!(number_of_way_to_make_change::solution(25, vec![1,5,10,25]), 13);
        assert_eq!(number_of_way_to_make_change::solution(12, vec![2,3,7]), 4);
        assert_eq!(number_of_way_to_make_change::solution(7, vec![2,3,4,7]), 3);
    }
}
