use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1:1H";

struct Unbound;

struct Bounded {
    bound: usize,
    delimiter: (char, char),
}

trait DisplayProgress: Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

impl DisplayProgress for Unbound {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}", "*".repeat(progress.i));
    }
}

impl DisplayProgress for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!(
            "{}{}{}{}",
            self.delimiter.0,
            "*".repeat(progress.i),
            " ".repeat(self.bound - progress.i),
            self.delimiter.1
        );
    }
}

struct Progress<Iter, B> {
    iter: Iter,
    i: usize,
    bound: B,
}

impl<Iter> Progress<Iter, Unbound> {
    fn new(iter: Iter) -> Self {
        Self {
            iter,
            i: 0,
            bound: Unbound,
        }
    }
}

impl<Iter> Progress<Iter, Unbound>
where
    Iter: ExactSizeIterator,
{
    pub fn with_bound(self) -> Progress<Iter, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delimiter: ('[', ']'),
        };
        Progress {
            iter: self.iter,
            i: self.i,
            bound,
        }
    }
}

impl<Iter> Progress<Iter, Bounded>
where
    Iter: Iterator,
{
    pub fn with_delimiter(mut self, start: char, end: char) -> Self {
        self.bound.delimiter = (start, end);
        self
    }
}

impl<Iter, Bound> Iterator for Progress<Iter, Bound>
where
    Iter: Iterator,
    Bound: DisplayProgress,
{
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);
        self.bound.display(self);
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self, Unbound>;
}

impl<Iter> ProgressIteratorExt for Iter {
    fn progress(self) -> Progress<Self, Unbound> {
        Progress::new(self)
    }
}

fn expensive_caculation<T>(_n: &T) {
    sleep(Duration::from_secs(1))
}

#[cfg(test)]
mod test {
    #[test]
    fn test_progress() {
        use super::*;
        for n in (0..).progress() {
            expensive_caculation(&n)
        }

        // let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // for n in v.iter().progress().with_bound().with_delimiter('<', '>') {
        //     expensive_caculation(n);
        // }

    }
} 