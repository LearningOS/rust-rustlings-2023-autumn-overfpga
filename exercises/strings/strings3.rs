// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    // let bytes = input.as_bytes();
    let ss = input.to_string();
    let mut out = String::new();
    let mut tmp = String::new();
    let mut i =0;
    let mut h = -1;
    let mut e = 0;
    let mut end = String::new();
    for s in ss.chars(){
        if s == ' ' && h == -1 {
            h = 1;
        }
        else{
            h = 0;
        }
        if s != ' ' && h ==1 {
            h = 0;
        }
        if h == 0 && e == 0 && s == ' '{
            e = 1;
        }
        if e == 1 && s != ' '{
            e = 0;
            out.push_str(end.as_str());
        }
        // if e == 1 {
        //     end.push(s);
        // }
        if h == 0 && e == 0 {
            
            out.push(s);
        }
    }
    out
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut out:String = input.to_string();
    out.push_str(" world!");
    out 
}

fn replace_me(input: &str) -> String {
    let mut out = input.replace("cars","balloons");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What'sup!");
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
