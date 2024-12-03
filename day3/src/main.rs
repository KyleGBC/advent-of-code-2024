use nom::{branch::alt, bytes::complete::tag, character::complete::anychar, combinator::map, multi::many0, sequence::{delimited, separated_pair}, IResult};

enum Instruction { Mul(i32, i32), Do, Dont }

fn parse_mul(input: &str) -> IResult<&str, (i32, i32)>
{
    delimited(tag("mul("),separated_pair(nom::character::complete::i32, tag(","), nom::character::complete::i32), tag(")"))(input)
}

fn parse_garbage(input: &str) -> IResult<&str, ()>
{
    map(anychar, drop)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Option<Instruction>>
{
    alt((
        map(parse_mul, |x| Some(Instruction::Mul(x.0, x.1))), 
        map(tag("do()"), |_| Some(Instruction::Do)),
        map(tag("don't()"), |_| Some(Instruction::Dont)),
        map(parse_garbage, |_| None)
    ))(input)
}

fn main() {
    let time = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let (_, intrs) = many0(parse_instruction)(input).unwrap();

    let part1 = intrs.iter().filter_map(|x| match x { Some(Instruction::Mul(l, r)) => Some(l * r), _ => None }).sum::<i32>();

    let mut enabled = true;
    let part2 = intrs.iter().filter_map(|x| {
        match x 
        {
            Some(Instruction::Mul(l, r)) => if enabled { Some(l * r) } else { None }
            Some(Instruction::Do) => { enabled = true; None },
            Some(Instruction::Dont) => { enabled = false; None },
            _ => None
        }
    }).sum::<i32>();

    println!("Part 1: {part1}, Part 2: {part2}, in {:?}", time.elapsed());
}