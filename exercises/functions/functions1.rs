// functions1.rs
//
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    call_me();
}

fn call_me(){
    let unit_like_struct = String::from("UnitLikeStruct");
        
    let message = format!("{:?} are fun!", unit_like_struct);

    println!("call_me {}",message);
}

 