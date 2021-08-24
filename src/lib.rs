mod number_of_way_to_make_change;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use crate::number_of_way_to_make_change::solution;
    #[test]
    fn test_number_of_way_to_make_change() {
        assert_eq!(solution(6, vec![1,5]), 2);
        assert_eq!(solution(0, vec![2,3,4,7]), 1);
        assert_eq!(solution(9, vec![5]), 0);
        assert_eq!(solution(7, vec![2,4]), 0);
        assert_eq!(solution(4, vec![1,5,10,25]), 1);
        assert_eq!(solution(5, vec![1,5,10,25]), 2);
        assert_eq!(solution(10, vec![1,5,10,25]), 4);
        assert_eq!(solution(25, vec![1,5,10,25]), 13);
        assert_eq!(solution(12, vec![2,3,7]), 4);
        assert_eq!(solution(7, vec![2,3,4,7]), 3);
    }
}
