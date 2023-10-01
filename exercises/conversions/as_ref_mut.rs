// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.


// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
// fn byte_counter<T>(arg: T) -> usize {
//     arg.as_ref().as_bytes().len()
// }


// 对于byte_counter和char_counter，我们想要将T转换为一个str引用
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}


// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
// fn char_counter<T>(arg: T) -> usize {
//     arg.as_ref().chars().count()
// }
// Obtain the number of characters (not bytes) in the given argument.
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}


// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.
// fn num_sq<T>(arg: &mut T) {
//     // TODO: Implement the function body.
//     ???
// }
// 对于num_sq，我们想要将T转换为一个u32的可变引用
// Squares a number using as_mut().
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    let val = arg.as_mut();
    *val *= *val;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}