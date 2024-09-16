use macros::EnumFrom;

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(u32),
    Right { a: u32 },
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

fn main() {
    // 将一个值转换成枚举类型，使用into()方法前提是要实现From trait
    let up = DirectionUp::new(42);

    let up: Direction<i32> = up.into();
    let left: Direction<i32> = 10.into();
    println!("{:?}, {:?}", up, left);
}

impl<T> DirectionUp<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}

// ident: Direction, var: Up, ty: DirectionUp
// impl From<DirectionUp> for Direction {
//     fn from(v: DirectionUp) -> Self {
//         Direction::Up(v)
//     }
// }
