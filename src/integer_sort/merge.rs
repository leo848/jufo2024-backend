use std::ops::Range;

use itertools::Itertools;

use crate::{action::ActionContext, typed::Highlight::Correct, IntegerSortContext, SortedNumbers};
pub fn merge(ctx: IntegerSortContext) -> Vec<i64> {
    let IntegerSortContext { action, numbers } = ctx;

    merge_rec(numbers.as_slice(), 0..numbers.len(), action)
}

fn merge_rec(numbers: &[i64], range: Range<usize>, action: ActionContext) -> Vec<i64> {
    let mut numbers = numbers.to_vec();

    if range.len() <= 1 {
        return numbers[range].to_vec();
    }

    action.send(SortedNumbers::new(&numbers).consider(&range));

    let to_merge_1 = merge_rec(
        &numbers,
        range.start..range.start + range.len() / 2,
        action.clone(),
    );
    for (index, &number) in to_merge_1.iter().enumerate() {
        numbers[index + range.start] = number;
    }

    action.send(
        SortedNumbers::new(&numbers)
            .highlights((range.start..range.start + range.len() / 2).map(|i| (i, Correct))),
    );

    let to_merge_2 = merge_rec(
        &numbers,
        range.start + range.len() / 2..range.end,
        action.clone(),
    );
    for (index, &number) in to_merge_2.iter().enumerate() {
        numbers[index + range.start + range.len() / 2] = number;
    }

    action.send(
        SortedNumbers::new(&numbers)
            .highlights((range.start + range.len() / 2..range.end).map(|i| (i, Correct))),
    );

    for index in range.clone() {
        let Some(min_index) = numbers[index..range.end].into_iter().position_min() else {
            break;
        };
        numbers.swap(index, min_index + index);
        action.send(
            SortedNumbers::new(&numbers).highlights((range.start..index).map(|i| (i, Correct))),
        );
    }

    merge_vectors(to_merge_1, to_merge_2)
}

fn merge_vectors(a: Vec<i64>, b: Vec<i64>) -> Vec<i64> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let (mut a_iter, mut b_iter) = (a.iter().copied().peekable(), b.iter().copied().peekable());

    while result.len() < a.len() + b.len() {
        let value = match (a_iter.peek(), b_iter.peek()) {
            (None, None) => panic!("Invariant broken"),
            (Some(&value), None) => {
                a_iter.next();
                value
            }
            (None, Some(&value)) => {
                b_iter.next();
                value
            }
            (Some(&value_a), Some(&value_b)) => {
                if value_a < value_b {
                    a_iter.next();
                    value_a
                } else {
                    b_iter.next();
                    value_b
                }
            }
        };
        result.push(value);
    }

    result
}
