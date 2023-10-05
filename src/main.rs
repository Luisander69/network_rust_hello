extern crate term;

fn main() {
    




    //let v1 = vec![1, 2, 3, 4];
    //let v2 = vec![1, 2];
    //println!("{:?}", longer_vector(&v1, &v2));


    //let mut s = String::from("Test");
    //heap_example(&mut s);
    //color_text();

}
fn longer_vector<'a>(x: &'a[i32], y :&'a[i32]) -> &'a[i32]{
    if x.len() > y.len() {x} else {y}
}

fn heap_example(input:&mut String){
    let mut mystr = input;
    *mystr =  String::from("Test2");
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
  
  


