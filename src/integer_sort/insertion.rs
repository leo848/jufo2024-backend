use crate::{
    typed::Highlight::{Compare, Correct, Swap},
    IntegerSortContext, SortedNumbers,
};

pub fn insertion(ctx: IntegerSortContext) -> Vec<i64> {
    let IntegerSortContext {
        mut numbers,
        action,
    } = ctx.clone();

    let progress = |outer_index: usize, inner_index: usize| -> f32 {
        (outer_index * ctx.len() + (ctx.len() - inner_index)) as f32
            / (ctx.len() * ctx.len()) as f32
    };

    for i in 0..numbers.len() {
        let next_to_insert = numbers[i];
        let mut index = i;
        while index > 0 && next_to_insert < numbers[index - 1] {
            action.send(
                SortedNumbers::new(&numbers)
                    .highlights([(index, Compare), (index - 1, Compare)])
                    .progress(progress(i, index)),
            );
            numbers.swap(index, index - 1);
            action.send(
                SortedNumbers::new(&numbers)
                    .highlights([(index, Swap), (index - 1, Swap)])
                    .progress(progress(i, index)),
            );
            index -= 1;
        }
        if index > 0 {
            action.send(
                SortedNumbers::new(&numbers)
                    .highlights([(index, Compare), (index - 1, Compare)])
                    .progress(progress(i, index)),
            );
            index -= 1;
        }
        if index > 0 {
            action.send(
                SortedNumbers::new(&numbers)
                    .highlights([(index, Compare), (index - 1, Compare)])
                    .progress(progress(i, index)),
            );
        }
        action.send(
            SortedNumbers::new(&numbers)
                .highlights((0..=i).map(|i| (i, Correct)))
                .progress(progress(i, index)),
        );
    }

    numbers.to_vec()
}
