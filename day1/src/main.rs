use std::cmp::Reverse;

use itertools::Itertools;
use tap::prelude::*;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("input.txt");
    part1(input)?.pipe(|out| println!("{out}"));
    part2(input)?.pipe(|out| println!("{out}"));

    Ok(())
}

fn part1(input: &str) -> color_eyre::Result<u32> {
    Ok(input
        .lines()
        .map(|v| v.parse::<u32>().ok())
        .batching(|it| it.map_while(|x| x).sum1::<u32>())
        .max()
        .unwrap_or_default())
}

fn part2(input: &str) -> color_eyre::Result<u32> {
    Ok(input
        .lines()
        .map(|v| v.parse::<u32>().ok())
        .batching(|it| it.map_while(|x| x).sum1::<u32>())
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u32>())
}
