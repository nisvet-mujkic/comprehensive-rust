fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));

    let array = [10, 20, 30];

    println!("array: {array:?}");

    print!("\nIterating over an array:");
    for n in array {
        print!(" {n}");
    }

    println!();

    let matrix: [[i32; 3]; 3] = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("\nmatrix:");
    pretty_print(&matrix);

    let transposed: [[i32; 3]; 3] = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    unimplemented!()
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    unimplemented!()
}
