use std::collections::HashMap;

fn main() {
    let result = add(3, 7);
    println!("{result}");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn return_median_mode(v: &Vec<i32>) -> (i32, i32) {
    let mut frequencies = HashMap::new();
    let median: i32;
    let mut mode: i32 = 0;
    let mut max_freq: i32 = 0;

    for i in v {
        let count = frequencies.entry(*i).or_insert(0);
        *count += 1;
        if *count > max_freq {
            max_freq = *count;
            mode = *i;
        }
    }

    let mut sorted_vec: Vec<i32> = v.clone();
    sorted_vec.sort();

    let num_elements = sorted_vec.len();

    if num_elements.is_multiple_of(2) {
        median = (sorted_vec[num_elements / 2] + sorted_vec[(num_elements - 1) / 2]) / 2;
    } else {
        median = sorted_vec[num_elements / 2];
    }

    (median, mode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_median_mode_1() {
        let v = vec![1,2,2,3,4];
        assert_eq!(
            return_median_mode(&v),
            (2,2)
        )
    }

    #[test]
    fn test_median_mode_2() {
        let v = vec![48, 48, 91, 86, 59, 5, 27, 0, 59, 10, 44, 22, 61, 8, 49, 53, 90, 35, 8, 70, 65, 78, 10, 84, 39, 46, 73, 96, 64, 85, 55, 59, 37, 75, 4, 88, 44, 63, 69, 93, 86, 49, 6, 85, 10, 97, 64, 59, 30, 32];
        assert_eq!(
            return_median_mode(&v),
            (57,59)
        )
    }

}
