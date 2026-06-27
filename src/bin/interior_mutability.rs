
fn main(){
    let mut x=3;
    let mut y= &mut x;
    *y=4;
    println!("{}",y);
    println!("{}",x);
}