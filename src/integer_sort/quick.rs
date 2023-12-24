use std::ops::Range;

use crate::{
    action::ActionContext,
    typed::Highlight::{Compare, Consider, Correct, Larger, Smaller},
    IntegerSortContext, SortedNumbers,
};

pub fn quick(ctx: IntegerSortContext) -> Vec<u64> {
    let IntegerSortContext {
        action,
        mut numbers,
    } = ctx;

    let len = numbers.len();
    quick_rec(&mut numbers, 0..len, action);

    numbers
}

pub fn quick_rec(numbers: &mut [u64], bounds: Range<usize>, action: ActionContext) {
    if bounds.len() <= 1 {
        return;
    }

    let pivot = numbers[bounds.start];
    let mut lt = Vec::with_capacity(bounds.len());
    let mut ge = Vec::with_capacity(bounds.len());

    for (index, &number) in numbers[bounds.clone()].iter().enumerate() {
        if number < pivot {
            action.send(
                SortedNumbers::new(&numbers)
                    .highlights(bounds.clone().map(|i| (i, Consider)))
                    .highlights([(index + bounds.start, Smaller), (bounds.start, Compare)]),
            );
            lt.push(number);
        } else {
            action.send(
                SortedNumbers::new(&numbers)
                    .highlights(bounds.clone().map(|i| (i, Consider)))
                    .highlights([(index + bounds.start, Larger), (bounds.start, Compare)]),
            );
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
            .highlights(
                bounds
                    .clone()
                    .map(|i| (i, if i < lt.len() { Smaller } else { Larger })),
            )
            .highlights([(lt.len() + bounds.start, Correct)]),
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
    // for actual_index in 0..lt.len() {
    //     let (orig_index, number) = lt[actual_index];
    //     numbers.swap(actual_index, orig_index);
    //     lt.swap(actual_index, orig_index);
    // }

    // for actual_index in 0..ge.len() {
    //     let (orig_index, number) = ge[actual_index];
    //     numbers.swap(actual_index + lt.len(), orig_index);
    //     ge.swap(actual_index, orig_index - lt.len());
    // }
}
