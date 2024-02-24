use std::{cmp::Ordering, ops::Range};

use crate::{
    action::ActionContext,
    typed::Highlight::{Consider, Correct, Larger, Pivot, Smaller},
    IntegerSortContext, SortedNumbers,
};

pub fn quick(ctx: IntegerSortContext) -> Vec<i64> {
    let IntegerSortContext {
        action,
        mut numbers,
    } = ctx;

    let len = numbers.len();
    quick_rec(&mut numbers, 0..len, action);

    numbers
}

pub fn quick_rec(numbers: &mut [i64], bounds: Range<usize>, action: ActionContext) {
    if bounds.len() <= 1 {
        return;
    }

    let pivot = numbers[bounds.start];
    let mut lt = Vec::with_capacity(bounds.len());
    let mut ge = Vec::with_capacity(bounds.len());

    for (index, &number) in numbers[bounds.clone()].iter().enumerate() {
        action.send(
            SortedNumbers::new(&numbers)
                .highlights(bounds.clone().map(|i| (i, Consider)))
                .highlights(
                    (bounds.start + 1..index + 1 + bounds.start)
                        .map(|i| (i, if numbers[i] < pivot { Smaller } else { Larger })),
                )
                .highlight(bounds.start, Pivot),
        );
        if number < pivot {
            lt.push(number);
        } else {
            ge.push(number);
        }
    }

    for (index, &number) in lt.iter().enumerate() {
        numbers[bounds.start + index] = number;
    }
    for (index, &number) in ge.iter().enumerate() {
        numbers[bounds.start + index + lt.len()] = number;
    }

    action.send(
        SortedNumbers::new(&numbers)
            .highlights(bounds.clone().map(|i| {
                (
                    i,
                    match (i - bounds.start).cmp(&lt.len()) {
                        Ordering::Less => Smaller,
                        Ordering::Equal => Pivot,
                        Ordering::Greater => Larger,
                    },
                )
            }))
            .highlights([(lt.len() + bounds.start, Pivot)]),
    );

    quick_rec(
        numbers,
        bounds.start..bounds.start + lt.len(),
        action.clone(),
    );
    quick_rec(
        numbers,
        bounds.start + lt.len() + 1..bounds.end,
        action.clone(),
    );

    action.send(SortedNumbers::new(&numbers).highlights(bounds.clone().map(|i| (i, Correct))));
}
