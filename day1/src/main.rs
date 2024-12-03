use rustc_hash::FxHashMap as HashMap;

fn main() {
    let time = std::time::Instant::now();
    let input = include_str!("../input.txt");

    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut right_counts: HashMap<u32, u32> = HashMap::default();

    for line in input.lines()
    {
        let mut nums = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
        let (left_num, right_num) = (nums.next().unwrap(), nums.next().unwrap());

        left.push(left_num);
        right.push(right_num);
        right_counts.entry(right_num).and_modify(|n| *n += 1).or_insert(1);
    }

    left.sort_unstable();
    right.sort_unstable();

    let part1: u32 = left.iter().zip(right.iter()).map(|(l, r)| l.abs_diff(*r)).sum();
    let part2: u32 = left.iter().map(|n| n * right_counts.get(n).or(Some(&0)).unwrap()).sum();

    println!("Part 1: {part1}, Part 2: {part2}, in {:?}", time.elapsed());
}
