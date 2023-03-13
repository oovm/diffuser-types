use diffuser_parser::DiffuserPrompts;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let prompts = DiffuserPrompts::from(vec!["a", "b", "c"]);
    println!("{:?}", prompts)
}
