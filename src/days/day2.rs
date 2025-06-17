fn is_safe(report: &Vec<u64>) -> bool {
    return is_strictly_monotonic(report);
}

fn is_strictly_monotonic(report: &Vec<u64>) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut increasing = false;
    let mut decreasing = false;
    let mut previous_level = &report[0];

    for level in &report[1..] {
        let diff = (*level as i64) - (*previous_level as i64);
        if diff.abs() < 1 {
            return false;
        } else if diff.abs() > 3 {
            return false;
        } else {
            if diff < 0 && increasing {
                return false;
            } else if diff > 0 && decreasing {
                return false;
            }
            if diff < 0 {
                decreasing = true;
            } else if diff > 0 {
                increasing = true;
            }
        }
        previous_level = level;
    }

    return true;
}

fn is_safe_with_dampener(report: &Vec<u64>) -> bool {
    if report.len() < 2 {
        return true;
    }

    if is_strictly_monotonic(report) {
        return true;
    }

    for (i, _) in report.iter().enumerate() {
        let mut report_clone: Vec<u64> = Vec::new();
        for (j, value) in report.iter().enumerate() {
            if i != j {
                report_clone.push(*value);
            }
        }
        if is_strictly_monotonic(&report_clone) {
            return true;
        }
    }

    return false;
}

fn count_safe_reports(reports: &Vec<Vec<u64>>) -> u64 {
    let mut count = 0;
    for report in reports {
        if is_safe(report) {
            count += 1
        }
    }
    count
}

fn count_safe_reports_with_dampener(reports: &Vec<Vec<u64>>) -> u64 {
    let mut count = 0;
    for report in reports {
        if is_safe_with_dampener(report) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::{fs, path::Path};

    #[test]
    fn it_can_tell_if_a_report_is_safe_if_numbers_are_decreasing() {
        let report = vec![7, 6, 4, 2, 1];
        assert!(is_safe(&report));
    }

    #[test]
    fn it_can_tell_if_a_report_is_safe_if_numbers_are_increasing() {
        let report = vec![1, 3, 6, 7, 9];
        assert!(is_safe(&report));
    }

    #[test]
    fn it_can_tell_if_a_report_is_not_safe_if_increase_greater_than_3() {
        let report = vec![1, 2, 7, 8, 9];
        assert!(!is_safe(&report));
    }

    #[test]
    fn it_can_count_safe_reports_on_example() {
        let reports = example_data();
        let result = count_safe_reports(&reports);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_can_count_safe_reports_on_part_1() {
        let reports = read_test_file().unwrap();
        let result = count_safe_reports(&reports);
        assert_eq!(result, 472);
    }

    #[test]
    fn it_can_count_safe_reports_with_dampener_on_example() {
        let reports = example_data();
        let result = count_safe_reports_with_dampener(&reports);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_can_count_safe_reports_on_part_2() {
        let reports = read_test_file().unwrap();
        let result = count_safe_reports_with_dampener(&reports);
        assert_eq!(result, 520);
    }

    fn example_data() -> Vec<Vec<u64>> {
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]
    }

    fn read_test_file() -> Result<Vec<Vec<u64>>, Box<dyn Error>> {
        let path = Path::new("inputs/day2.txt");
        let content = fs::read_to_string(path)?;
        let mut output = Vec::new();

        for line in content.lines() {
            let levels: Vec<u64> = line.split(" ").map(|part| part.parse().unwrap()).collect();
            output.push(levels);
        }

        Ok(output)
    }
}
