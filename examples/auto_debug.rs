use macros::AutoDebug;

#[allow(unused)]
#[derive(AutoDebug)]
pub struct RespBulkString {
    inner: String,
    // #[proc_macro_derive(AutoDebug,attributes(debug))]，添加了debug属性宏
    #[debug(skip)]
    nothing: (),
    hello: u32,
}

fn main() {
    let s = RespBulkString {
        inner: "hello".to_string(),
        nothing: (),
        hello: 42,
    };
    println!("{:?}", s);
}
