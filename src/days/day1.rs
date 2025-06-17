use std::collections::HashMap;

pub fn distance(left: &mut Vec<u64>, right: &mut Vec<u64>) -> u64 {
    assert_eq!(left.len(), right.len(), "left and right should have the same size");
    
    left.sort();
    right.sort();

    let mut sum = 0;
    for i in 0..left.len() {
        let diff = left[i].abs_diff(right[i]);
        sum += diff;
    }
    sum
}

pub fn similarity(left: &Vec<u64>, right: &Vec<u64>) -> u64 {
    let mut counts = HashMap::new();

    for num in right {
        *counts.entry(num).or_insert(0) += 1;
    }

    let mut sum = 0;

    for num in left {
        let n: &u64 = counts.get(&num).unwrap_or(&0);
        sum += num * n;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::Path};
    use std::error::Error;

    #[test]
    fn it_should_find_the_distance_in_example() {
        let mut left: Vec<u64> = vec![3, 4, 2, 1, 3, 3];
        let mut right: Vec<u64> = vec![4, 3, 5, 3, 9, 3];

        let result = distance(&mut left, &mut right);
        assert_eq!(result, 11);
    }

    #[test]
    fn it_should_find_the_distance_in_part_1() {
        let (mut left, mut right) = read_test_file().unwrap();
        let result = distance(&mut left, &mut right);
        assert_eq!(result, 2057374);
    }

    #[test]
    fn it_should_find_the_similarity_in_example() {
        let (left, right) = example_data();
        let result = similarity(&left, &right);
        assert_eq!(result, 31);
    }

    #[test]
    fn it_should_find_the_similarity_in_part2() {
        let (left, right) = read_test_file().unwrap();
        let result = similarity(&left, &right);
        assert_eq!(result, 23177084);
    }

    fn example_data() -> (Vec<u64>, Vec<u64>) {
        let left: Vec<u64> = vec![3, 4, 2, 1, 3, 3];
        let right: Vec<u64> = vec![4, 3, 5, 3, 9, 3];
        (left, right)
    }

    fn read_test_file() -> Result<(Vec<u64>, Vec<u64>), Box<dyn Error>> {
        let path = Path::new("inputs/day1.txt");
        let content = fs::read_to_string(path)?;
        let mut left: Vec<u64> = Vec::new();
        let mut right: Vec<u64> = Vec::new();
        for line in content.lines().into_iter() {
            let mut parts = line.split("   ");
            let left_number = parts.next().unwrap().parse()?;
            let right_number = parts.next().unwrap().parse()?;
            left.push(left_number);
            right.push(right_number);
        }
        Ok((left, right))
    }
}
