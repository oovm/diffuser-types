use diffuser_scheduler::DiffuserScheduler;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let ddim1: DiffuserScheduler = serde_json::from_str("\"ddim\"").unwrap();
    println!("{:#?}", ddim1);
    let ddim2: DiffuserScheduler = serde_json::from_str(include_str!("ddim.json")).unwrap();
    println!("{:#?}", ddim2)
}
