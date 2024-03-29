use crate::{
    typed::Highlight::{Compare, Correct, Swap},
    IntegerSortContext, SortedNumbers,
};

pub fn selection(ctx: IntegerSortContext) -> Vec<i64> {
    let IntegerSortContext {
        mut numbers,
        action,
    } = ctx.clone();

    let progress = |outer_index: usize, inner_index: usize| -> f32 {
        (outer_index * ctx.len() + (inner_index - outer_index)) as f32
            / (ctx.len() * ctx.len()) as f32
    };

    for i in 0..numbers.len() {
        let mut min_index = i;
        for j in i..numbers.len() {
            action.send(
                SortedNumbers::new(&numbers)
                    .highlights([(j, Compare), (min_index, Compare)])
                    .progress(progress(i, j)),
            );
            if numbers[j] < numbers[min_index] {
                min_index = j;
                action.send(
                    SortedNumbers::new(&numbers)
                        .highlight(min_index, Compare)
                        .progress(progress(i, j)),
                );
            }
        }
        if i != min_index {
            action.send(
                SortedNumbers::new(&numbers)
                    .highlights([(i, Swap), (min_index, Swap)])
                    .progress(progress(i, numbers.len())),
            );
            numbers.swap(i, min_index);
            action.send(
                SortedNumbers::new(&numbers)
                    .highlights([(i, Swap), (min_index, Swap)])
                    .progress(progress(i, numbers.len())),
            );
        } else if i != numbers.len() - 1 {
            action.send(
                SortedNumbers::new(&numbers)
                    .highlight(i, Compare)
                    .progress(progress(i, numbers.len())),
            );
        }
        action.send(
            SortedNumbers::new(&numbers)
                .highlights((0..=i).map(|i| (i, Correct)))
                .progress(progress(i, numbers.len())),
        );
    }

    numbers.to_vec()
}
