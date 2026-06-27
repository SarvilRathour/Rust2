use std::mem::drop;
struct CustomSmartPointer{
    data:String,
}
impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CSP with data `{}`",self.data);
    }
}
fn main(){
    let c=CustomSmartPointer{
        data:String::from("My stuff"),
    };
    drop(c);
    let d=CustomSmartPointer{
        data:String::from("Other stuff"),
    };
    println!("CSP created");
}