pub fn strtok<'a, 'b>(s: &'a mut &'b str, delimiter: char) -> &'a str {
    if let Some(n) = s.find(delimiter) {
        let prefix = &s[..n];
        let suffix = &s[(n + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        // why not work with last line
        let mut x = "hello world";
        let _ = strtok(&mut x, ' ');
        assert_eq!(x, "world");
    }

    #[test]
    fn another_example() {
        // why can that work
        let s = String::new();
        let x: &'static str = "hello world";
        let mut _y /* : &'a str */ = &*s;
        _y = x; /* 'static str --> &'a str */
    }
}

// T:U
// T至少像是U 一样有用
// 'static: 'a
// IN rust Variance
//
// &'a str
// &'static str more useful beacause It can use in more places
//
//
// Fn(&'a str) -> &'a str  This more usefull because it checks input wider
// Fn(&'static str) -> &'static str
//
// Covariant
// &'static <: &'a
// &'static T <: &'a T
//
// Contravariant
// &'static <: &'a
// Fn(&'a T) :< Fn(&'static T)
//
// fn fn(s: &mut &'a str)
// 这个的参数的意思  一个对于 &'a str 的可变引用
//
// invariant
//
// fn foo(s: &mut &'a str, &'a str) {
//    *s = x
// }
//
// let mut x : &'static = "hello world";
// let z = String::new();
// foo(&mut x, &z);
// drop(z);
// println!("{}", x);
