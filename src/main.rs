/*
    tree!{
        sum => {
            mul => { a, c },
            b
        }
    };
    ******
    sum(mul(a, c), b)
*/


#[derive(Debug)]
struct My {
    a: i32,
    b: i32
}


fn procedure<'res, 'a: 'res, 'b: 'res>(
    a: &'a My, b: &'b My) -> &'res My
{
    if a.a > b.b { a } else { b }
}


fn main() {
    let a = My { a: 13, b: -1 };
    let b = My { a: 10, b: 6 };
    let x = procedure(&a, &b);
    println!("{:?}", x);
}
