extern crate term;

fn main() {
    let s = String::from("Test");
    heap_example(&mut s);
    println!("{}", s);
    //color_text();

}
fn heap_example(input:&mut String){
    let mystr = input;
    let _otherstr = mystr.clone();
    println!("{}", mystr);
}
fn color_text(){
    let mut t = term::stdout().unwrap();
    t.fg(term::color::GREEN).unwrap();
    write!(t, "Hello ").unwrap();
    
    t.fg(term::color::RED).unwrap();
    write!(t, "World! ").unwrap();
    
    t.reset().unwrap();
}
  
  


