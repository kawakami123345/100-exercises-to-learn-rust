use std::ops::Add;

fn intro() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to learn about traits!"
}
// fn main() {
//     let a: i32 = 3;
//     let b: i32 = 5;
//     let c = a.add(b);
//     println!("{}", c);
// }
#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to learn about traits!");
    }
}
