use crate::{
    typed::Highlight::{Compare, Correct, Swap},
    IntegerSortContext, SortedNumbers,
};

pub fn bubble(ctx: IntegerSortContext) -> Vec<u64> {
    let IntegerSortContext {
        mut numbers,
        action,
    } = ctx;

    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            action.send(
                SortedNumbers::new(&numbers)
                    .highlight(i, Compare)
                    .highlight(j, Compare),
            );
            if numbers[i] > numbers[j] {
                numbers.swap(i, j);
                action.send(
                    SortedNumbers::new(&numbers)
                        .highlight(i, Swap)
                        .highlight(j, Swap),
                );
            }
        }

        action.send(SortedNumbers::new(&numbers).highlights((0..=i).map(|i| (i, Correct))));
    }

    numbers.to_vec()
}
