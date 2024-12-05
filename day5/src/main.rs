use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

fn main() {
    let time = std::time::Instant::now();
    let mut input = include_str!("../input.txt").lines();

    let mut pre_reqs: HashMap<i32, Vec<i32>> = HashMap::default();

    while let Some(s) = input.next() {
        let s = s.trim();
        if s.is_empty() { break; }
        let (l, r) = s.split_once("|").unwrap();
        let (l, r) = (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap());
        pre_reqs.entry(r).or_default().push(l);
    }

    let parse_sequence = |s: &str| -> Vec<i32> { s.split(",").map(|s| s.parse::<i32>().unwrap()).collect() };
    let middle_page = |seq: Vec<i32>| -> i32 { *seq.get(seq.len() / 2).unwrap() };
    let check_sequence = |seq: &Vec<i32>| -> bool {
        let mut seen: HashSet<i32> = HashSet::default();
        seq.iter().all(|&a| {
            seen.insert(a);
            pre_reqs.get(&a).is_none() || pre_reqs.get(&a).unwrap().iter().all(|&b| !seq.contains(&b) || seen.contains(&b))
        })
    };
    
    let reorder_sequence = |mut seq: Vec<i32>| -> Vec<i32> { 
        let sub_set: HashMap<i32, usize> = pre_reqs.iter().filter(|(k, _)| seq.contains(k)).map(|(&k, v)| (k, v.iter().filter(|n| seq.contains(n)).count())).collect();    
        seq.sort_unstable_by(|a, b| {
            let a = sub_set.get(a).unwrap_or(&0);
            let b = sub_set.get(b).unwrap_or(&0);
            a.cmp(&b)
        });
        seq
    };

    let (correct, incorrect): (Vec<_>, Vec<_>) = input.map(parse_sequence).partition(check_sequence);
    let part1 = correct.into_iter().map(middle_page).sum::<i32>();
    let part2 = incorrect.into_iter().map(reorder_sequence).map(middle_page).sum::<i32>();

    println!("Part 1: {part1}, Part 2: {part2}, in {:?}", time.elapsed()); 
}
