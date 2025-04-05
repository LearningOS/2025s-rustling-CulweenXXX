// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// String 拥有它的数据。
// &str 只是借用数据，不拥有它。

/*
String 提供了更多方法来构建和操作字符串，例如 push_str, push, pop, remove, replace 等等。
&str 提供了一些基本的方法来访问和检查字符串内容，如 len(), is_empty(), trim() 等。
*/
fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut ans=input.trim().to_string();
    ans
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut ans=String::new();
    ans=input.to_string()+" world!";

    ans
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    // cars ---> balloons
    let mut ans=String::from(input.to_string()).replace("cars","balloons");
    
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
