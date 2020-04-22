#[inline]
pub fn div_ceil(a: usize, b: usize) -> usize {
    let (div, rem) = (a / b, a % b);

    if rem == 0 {
        div
    } else {
        div + 1
    }
}

#[inline]
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    assert!(min <= max);
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[inline]
pub fn iter_stop_step(start: usize, stop: usize, step: usize) -> impl Iterator<Item = usize> {
    std::iter::successors(Some(start), move |&i| Some(i + step)).take_while(move |&i| i < stop)
}

#[inline]
pub fn iter_step(start: usize, step: usize) -> impl Iterator<Item = usize> {
    std::iter::successors(Some(start), move |&i| Some(i + step))
}

pub fn iter_signal_last<T, I: Iterator<Item = T>>(iter: I) -> impl Iterator<Item = (bool, T)> {
    let mut iter = iter.peekable();
    std::iter::from_fn(move || {
        if let Some(current) = iter.next() {
            if iter.peek().is_some() {
                Some((false, current))
            } else {
                Some((true, current))
            }
        } else {
            None
        }
    })
}
