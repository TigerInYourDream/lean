use std::fmt::Debug;

trait Execute<D>
where
    D: Debug + Sized,
{
    fn execute(&self) -> D;
}

impl Execute<i32> for i32 {
    fn execute(&self) -> i32 {
        *self
    }
}

#[derive(Debug)]
struct Geed {
    value: i32,
}

impl Execute<Geed> for Geed {
    fn execute(&self) -> Geed {
        Geed { value: self.value }
    }
}

enum Executable {
    Int(Box<i32>),
    Geed(Box<Geed>),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_i32() {
        let mut v: Vec<Executable> = Vec::new();
        let a = Executable::Int(Box::new(12));
        let b = Executable::Geed(Box::new(Geed { value: 17 }));
        v.push(a);
        v.push(b);


        for i in v {
            match i {
                Executable::Int(int) => println!("{:?}", int.execute()),
                Executable::Geed(geed) => println!("{:?}", geed.execute()),
            }
        }
    }
}
