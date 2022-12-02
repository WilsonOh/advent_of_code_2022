use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let f = std::fs::read_to_string("input.txt")?;
    let ans: u32 = f
        .split("\n\n")
        .map(|i| {
            i.split_whitespace()
                .fold(0, |acc, curr| acc + curr.parse::<u32>().unwrap())
        })
        .sorted()
        .rev()
        .take(3)
        .sum();
    println!("{:?}", ans);
    Ok(())
}
