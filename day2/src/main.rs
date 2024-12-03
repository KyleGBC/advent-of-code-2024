fn is_safe(report: &Vec<i32>) -> bool {
    let increasing = report[1] - report[0] > 0; 
    report.windows(2).all(|r| {
       let diff = r[1] - r[0];
       if diff.abs() < 1 || diff.abs() > 3 { return false }
       if (diff > 0) != increasing { return false } 
       true
    })
}

fn is_almost_safe(report: &Vec<i32>) -> bool {
    if is_safe(report) { return true }

    for i in 0..report.len() {
        let mut subset = report.clone();
        subset.remove(i);
        if is_safe(&subset) { return true }
    }
    return false;
}

fn main() {
    let time = std::time::Instant::now();
    let input = std::include_str!("../input.txt");

    let reports: Vec<Vec<i32>> = input.lines().map(|line| line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect()).collect();

    let part1 = reports.iter().filter(|report| is_safe(report)).count();
    let part2 = reports.iter().filter(|report| is_almost_safe(report)).count();
    println!("Part 1: {part1}, Part 2: {part2}, in {:?}", time.elapsed());
}
