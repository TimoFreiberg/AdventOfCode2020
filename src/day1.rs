use eyre::{bail, eyre};
use itertools::Itertools;
use std::fs;

pub fn solve() -> eyre::Result<()> {
    let input = fs::read_to_string("input1.csv")?;

    let lines = input
        .lines()
        .map(|it| it.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    solve_for_combinations(&lines, 2)?;
    solve_for_combinations(&lines, 3)?;

    Ok(())
}

fn solve_for_combinations(lines: &[i32], combinations: usize) -> eyre::Result<()> {
    let result: i32 = lines
        .into_iter()
        .combinations(combinations)
        .find(|it| it.iter().copied().sum::<i32>() == 2020)
        .ok_or(eyre!("no pair found"))?
        .iter()
        .copied()
        .product();
    println!("{}", result);
    Ok(())
}
