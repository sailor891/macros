use anyhow::Result;

// macro_export属性可以将这个宏暴露给其他crate使用
#[macro_export]
macro_rules! my_vec {
    // ()还可以是[]和{}
    // 括号里面的$运算符可以匹配一个表达式，如$n，$elem，$x。且括号里面的进行正则匹配
    () => {
        Vec::new()
    };
    // 对应my_vec![1;5]，$运算符就像ex表达式
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    // 对应my_vec!["1".parse::<i32>()?,"2".parse::<i32>()?,"3".parse::<i32>()?,]
    // $($x:expr),+ 匹配表达式，+表示至少一个，?表示0个或多个
    // $(,)? 表示逗号在最后一个元素之后可选
    ($($x:expr),+ $(,)?) => {{
        // let mut temp_vec = Vec::new();
        // $(
        //   temp_vec.push($x);
        // )*
        // temp_vec
        <[_]>::into_vec(Box::new([$($x),*]))
    }};
}

fn main() -> Result<()> {
    let vec = my_vec![1, 2, 3];
    println!("{:?}", vec);
    let vec = my_vec!(1;3);
    println!("{:?}", vec);
    let vec = my_vec!(
        "1".parse::<i32>()?,
        "2".parse::<i32>()?,
        "3".parse::<i32>()?,
    );
    println!("{:?}", vec);
    Ok(())
}
