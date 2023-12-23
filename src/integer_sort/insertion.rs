use crate::{
    typed::Highlight::{Compare, Correct, Swap},
    IntegerSortContext, SortedNumbers,
};

pub fn insertion(ctx: IntegerSortContext) -> Vec<u64> {
    let IntegerSortContext {
        mut numbers,
        action,
    } = ctx;

    for i in 0..numbers.len() {
        let next_to_insert = numbers[i];
        let mut index = i;
        while index > 0 && next_to_insert < numbers[index - 1] {
            action.send(
                SortedNumbers::new(&numbers).highlights([(index, Compare), (index - 1, Compare)]),
            );
            numbers.swap(index, index - 1);
            action
                .send(SortedNumbers::new(&numbers).highlights([(index, Swap), (index - 1, Swap)]));
            index -= 1;
        }
        if index > 0 {
            action.send(
                SortedNumbers::new(&numbers).highlights([(index, Compare), (index - 1, Compare)]),
            );
        }
        action.send(SortedNumbers::new(&numbers).highlights((0..=i).map(|i| (i, Correct))));
    }

    numbers.to_vec()
}
