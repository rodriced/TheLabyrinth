use std::ops::Range;

pub fn centered_range(center: usize, limit: usize) -> Range<usize> {
    let start = if center < 2 { 0 } else { center - 2 };
    let end = if center + 2 >= limit {
        limit
    } else {
        center + 3
    };
    //    println!("{} {}", start, end);
    start..end
}
