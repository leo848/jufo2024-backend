use crate::{
    typed::Highlight::{Compare, Correct, Swap},
    IntegerSortContext, SortedNumbers,
};

pub fn bubble(ctx: IntegerSortContext) -> Vec<i64> {
    let IntegerSortContext {
        mut numbers,
        action,
    } = ctx.clone();

    let progress = |outer_index: usize, inner_index: usize| -> f32 {
        (outer_index * ctx.len() + (inner_index - outer_index)) as f32
            / (ctx.len() * ctx.len()) as f32
    };

    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            action.send(
                SortedNumbers::new(&numbers)
                    .highlight(i, Compare)
                    .highlight(j, Compare)
                    .progress(progress(i, j)),
            );
            if numbers[i] > numbers[j] {
                numbers.swap(i, j);
                action.send(
                    SortedNumbers::new(&numbers)
                        .highlight(i, Swap)
                        .highlight(j, Swap)
                        .progress(progress(i, j)),
                );
            }
        }

        action.send(
            SortedNumbers::new(&numbers)
                .highlights((0..=i).map(|i| (i, Correct)))
                .progress(progress(i, numbers.len())),
        );
    }

    numbers.to_vec()
}
