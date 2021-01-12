#![feature(str_split_once)]
use regex::Regex;
use std::{collections::HashMap, usize};

const THE_BAG: &str = "shiny gold";

type Bag<'t> = HashMap<&'t str, usize>;

fn main() {
    let re_bag = Regex::new(r"(\d+) (.+?) bags?[,.]").unwrap();
    let bags = include_str!("../input")
        .lines()
        .map(|l| {
            let (bag, contents) = l.split_once(" bags contain ").unwrap();
            (
                bag,
                re_bag
                    .captures_iter(contents)
                    .map(|cs| (cs.get(2).unwrap().as_str(), cs[1].parse::<usize>().unwrap()))
                    .collect::<Bag>(),
            )
        })
        .collect::<HashMap<_, _>>();

    println!(
        "p1= {}",
        bags.iter()
            .filter(|(&colour, _)| can_contain_the_bag(&bags, colour))
            .count()
    );

    println!("p2= {}", bag_size(&bags, THE_BAG));
}

fn can_contain_the_bag(bags: &HashMap<&str, Bag>, colour: &str) -> bool {
    let bag = &bags[colour];
    bag.contains_key(THE_BAG) || bag.iter().any(|(&c, _)| can_contain_the_bag(bags, c))
}

fn bag_size(bags: &HashMap<&str, Bag>, colour: &str) -> usize {
    bags[colour].iter().fold(0, |acc, (&c, count)| {
        acc + (count * (1 + bag_size(bags, c)))
    })
}
