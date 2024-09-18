use macros::AutoDeref;

#[allow(unused)]
#[derive(Debug, AutoDeref)]
// #[proc_macro_derive(AutoDeref,attributes(deref))]添加了属性deref
#[deref(field = "inner")]
pub struct RespBulkString {
    inner: String,
    nothing: (),
}

#[allow(unused)]
#[derive(Debug, AutoDeref)]
#[deref(field = "name", mutable = true)]
pub struct Person {
    name: String,
    age: i32,
}
fn main() {
    let s = RespBulkString {
        inner: "hello".to_string(),
        nothing: (),
    };
    println!("{:?}", *s);

    let mut person = Person {
        name: "John".to_string(),
        age: 30,
    };
    *person = String::from("Cow");
    println!("{:?}", *person);
}
