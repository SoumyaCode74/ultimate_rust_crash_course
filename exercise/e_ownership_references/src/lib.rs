pub fn inspect(s: &String){
    if s.ends_with("s") {
        println!("Plural");
    }
    else{
        println!("Singular");
    }
}
pub fn change(s: &mut String){
    if !s.ends_with("s"){
        s.push_str("s");
    }
}
pub fn eat(s: String) -> bool{
    if s.starts_with("b") && s.contains("a") {
        true
    }
    else {
        false
    }
}