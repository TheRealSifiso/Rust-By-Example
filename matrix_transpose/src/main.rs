use std::fmt::{self, Display};

#[derive(Debug)]
//Tuple Struct
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: &Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Debug format: {:?}\n", matrix);
    println!("Display format:\n{}\n", matrix);
    println!("Transpose:\n{}", transpose(&matrix))
}
