pub fn main() {
    let answer = generate(4);
    println!("Answer: {:?}", answer);
}

fn generate(num: i32) -> Vec<Vec<i32>> {
    let mut sut = vec![];
    let mut current = vec![];

    for _ in 0..num {
        let next = calc_next(current);
        sut.push(next.clone());
        current = next;
    }

    return sut
}

fn calc_next(row: Vec<i32>) -> Vec<i32> {
    if row.len() == 0 {
        return vec![1];
    } else if row.len() == 1 {
        return vec![1, 1];
    } else if row.len() == 2 {
        return vec![1, 2, 1];
    }

    let mut next = vec![1];

    for i in 0..row.len()-1 {
        let sum = row[i] + row[i+1];
        next.push(sum);
    }
    next.push(1);

    return next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_one() {
        let result = generate(1);
        assert_eq!(result, vec![vec![1]]);
    }

    #[test]
    fn test_generate_two() {
        let result = generate(2);
        assert_eq!(result, vec![vec![1], vec![1, 1]]);
    }

    #[test]
    fn test_generate_three() {
        let result = generate(3);
        assert_eq!(result, vec![vec![1], vec![1, 1], vec![1, 2, 1]]);
    }

    #[test]
    fn test_generate_four() {
        let result = generate(4);
        assert_eq!(result, vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]]);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_generate_five() {
            let result = generate(5);
            assert_eq!(result, vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]);
        }

        #[test]
        fn test_generate_eight() {
            let result = generate(8);
            assert_eq!(result, vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
                vec![1, 5, 10, 10, 5, 1],
                vec![1, 6, 15, 20, 15, 6, 1],
                vec![1, 7, 21, 35, 35, 21, 7, 1]
            ]);
        }

        #[test]
        fn test_generate_ten() {
            let result = generate(10);
            assert_eq!(result, vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
                vec![1, 5, 10, 10, 5, 1],
                vec![1, 6, 15, 20, 15, 6, 1],
                vec![1, 7, 21, 35, 35, 21, 7, 1],
                vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
                vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1]
            ]);
        }

        #[test]
        fn test_generate_thirteen() {
            let result = generate(13);
            assert_eq!(result, vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
                vec![1, 5, 10, 10, 5, 1],
                vec![1, 6, 15, 20, 15, 6, 1],
                vec![1, 7, 21, 35, 35, 21, 7, 1],
                vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
                vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
                vec![1, 10, 45, 120, 210, 252, 210, 120, 45, 10, 1],
                vec![1, 11, 55, 165, 330, 462, 462, 330, 165, 55, 11, 1],
                vec![1, 12, 66, 220, 495, 792, 924, 792, 495, 220, 66, 12, 1]
            ]);
        }

        #[test]
        fn test_generate_seventeen() {
            let result = generate(17);
            assert_eq!(result, vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
                vec![1, 5, 10, 10, 5, 1],
                vec![1, 6, 15, 20, 15, 6, 1],
                vec![1, 7, 21, 35, 35, 21, 7, 1],
                vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
                vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
                vec![1, 10, 45, 120, 210, 252, 210, 120, 45, 10, 1],
                vec![1, 11, 55, 165, 330, 462, 462, 330, 165, 55, 11, 1],
                vec![1, 12, 66, 220, 495, 792, 924, 792, 495, 220, 66, 12, 1],
                vec![1, 13, 78, 286, 715, 1287, 1716, 1716, 1287, 715, 286, 78, 13, 1],
                vec![1, 14, 91, 364, 1001, 2002, 3003, 3432, 3003, 2002, 1001, 364, 91, 14, 1],
                vec![1, 15, 105, 455, 1365, 3003, 5005, 6435, 6435, 5005, 3003, 1365, 455, 105, 15, 1],
                vec![1, 16, 120, 560, 1820, 4368, 8008, 11440, 12870, 11440, 8008, 4368, 1820, 560, 120, 16, 1]
            ]);
        }

        #[test]
        fn test_generate_twenty() {
            let result = generate(20);
            assert_eq!(result, vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
                vec![1, 5, 10, 10, 5, 1],
                vec![1, 6, 15, 20, 15, 6, 1],
                vec![1, 7, 21, 35, 35, 21, 7, 1],
                vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
                vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
                vec![1, 10, 45, 120, 210, 252, 210, 120, 45, 10, 1],
                vec![1, 11, 55, 165, 330, 462, 462, 330, 165, 55, 11, 1],
                vec![1, 12, 66, 220, 495, 792, 924, 792, 495, 220, 66, 12, 1],
                vec![1, 13, 78, 286, 715, 1287, 1716, 1716, 1287, 715, 286, 78, 13, 1],
                vec![1, 14, 91, 364, 1001, 2002, 3003, 3432, 3003, 2002, 1001, 364, 91, 14, 1],
                vec![1, 15, 105, 455, 1365, 3003, 5005, 6435, 6435, 5005, 3003, 1365, 455, 105, 15, 1],
                vec![1, 16, 120, 560, 1820, 4368, 8008, 11440, 12870, 11440, 8008, 4368, 1820, 560, 120, 16, 1],
                vec![1, 17, 136, 680, 2380, 6188, 12376, 19448, 24310, 24310, 19448, 12376, 6188, 2380, 680, 136, 17, 1],
                vec![1, 18, 153, 816, 3060, 8568, 18564, 31824, 43758, 48620, 43758, 31824, 18564, 8568, 3060, 816, 153, 18, 1],
                vec![1, 19, 171, 969, 3876, 11628, 27132, 50388, 75582, 92378, 92378, 75582, 50388, 27132, 11628, 3876, 969, 171, 19, 1]
            ]);
        }
    }

    #[test]
    fn test_calc_next_single_element() {
        let result = calc_next(vec![1]);
        assert_eq!(result, vec![1, 1]);
    }

    #[test]
    fn test_calc_next_two_elements() {
        let result = calc_next(vec![1, 1]);
        assert_eq!(result, vec![1, 2, 1]);
    }

    #[test]
    fn test_calc_next_multiple_elements() {
        let result = calc_next(vec![1, 2, 1]);
        assert_eq!(result, vec![1, 3, 3, 1]);
    }

    #[test]
    fn test_calc_next_more_elements() {
        let result = calc_next(vec![1, 3, 3, 1]);
        assert_eq!(result, vec![1, 4, 6, 4, 1]);
    }
}