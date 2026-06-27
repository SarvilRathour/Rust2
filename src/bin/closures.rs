// fn with_counting<F>(inner:F)->impl FnMut(i32)->i32
// where 
//     F:Fn(i32)->i32
// {
//     let mut count = 0;
//     move |x| {
//         count += 1;
//         println!("Count: {}, value: {}", count, x);
//         inner(x)
//     }
// }
// fn main(){
//     let increment=|x:i32| x+1;//closure
//     let mut counted_increment=with_counting(increment);
//     println!("{}", counted_increment(5));
//     println!("{}", counted_increment(10));
    
// }
use std::fmt;

fn tracker<F>(inner:F)->impl FnMut(i32)->i32
where 
    F:Fn(i32)->i32
{
    let mut count = 0;
    move |x| {
        count+=1;
        println!("{}",count); 
        inner(x)
    }
}
// impl fmt::Debug for F{
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.F);
//     }
// }

fn main(){
    let double=|x:i32| x*2;
    let mut a=tracker(double);
    let mut b=tracker(double);
    println!("{}", a(5));
    println!("{}", b(10));
    
}