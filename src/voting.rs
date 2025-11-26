WIP

use std::{fmt::Debug, iter::Sum, ops::Mul};

fn dot<T: TryInto<V>, U: TryInto<V>, V: Sum + Mul<Output = V>>(a: Vec<T>, b: Vec<U>) -> V
where
    <T as TryInto<V>>::Error: Debug,
    <U as TryInto<V>>::Error: Debug,
{
    assert_eq!(a.len(), b.len());
    a.into_iter()
        .zip(b)
        .map(move |(a, b)| a.try_into().unwrap() * b.try_into().unwrap())
        .sum()
}

pub fn winner(a: f32, b: f32) -> bool {
    a > b
}

pub fn seats_per(seats: usize, shares: Vec<f32>) -> Vec<usize> {
    let shares_total: f32 = shares.iter().sum();
    let shares_prop = shares.iter().map(|x| x / shares_total).collect::<Vec<_>>();
    let seats_naive = shares_prop
        .iter()
        .map(|x| (x * seats as f32) as usize)
        .collect();

    let seats_iterative: Vec<usize> = shares.iter().map(|_| 0).collect();
    let best_score
    for option_id in 0..shares.len() {
        let option = seats_iterative.clone();
        option[option_id] += 1;
    }

    seats_naive
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seats_per_test() {
        let seats = 5;
        let seats_per = seats_per(5, vec![1.0, 2.0, 4.0]);
        assert_eq!(seats_per.iter().count(), seats);
    }
}


use vote_core::winner;
use vote_core::seats_per;

fn main() {
    println!("Hello, world {}!", winner(1.0, 2.0));
    let seats = seats_per(5, vec![1.0, 2.0, 4.0]);
    println!("{:?}!", seats);
}
