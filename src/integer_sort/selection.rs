use crate::{
    typed::Highlight::{Compare, Correct, Swap},
    IntegerSortContext, SortedNumbers,
};

pub fn selection(ctx: IntegerSortContext) -> Vec<i64> {
    let IntegerSortContext {
        mut numbers,
        action,
    } = ctx;

    for i in 0..numbers.len() {
        let mut min_index = i;
        for j in i..numbers.len() {
            action.send(
                SortedNumbers::new(&numbers).highlights([(j, Compare), (min_index, Compare)]),
            );
            if numbers[j] < numbers[min_index] {
                min_index = j;
                action.send(SortedNumbers::new(&numbers).highlight(min_index, Compare));
            }
        }
        if i != min_index {
            action.send(SortedNumbers::new(&numbers).highlights([(i, Swap), (min_index, Swap)]));
            numbers.swap(i, min_index);
            action.send(SortedNumbers::new(&numbers).highlights([(i, Swap), (min_index, Swap)]));
        } else if i != numbers.len() - 1 {
            action.send(SortedNumbers::new(&numbers).highlight(i, Compare));
        }
        action.send(SortedNumbers::new(&numbers).highlights((0..=i).map(|i| (i, Correct))));
    }

    numbers.to_vec()
}
