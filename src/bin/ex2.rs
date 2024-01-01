

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result_matrix = [[0; 3]; 3];
    for (i, row) in matrix.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            result_matrix[j][i] = *val;
        }
    }
    result_matrix
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
   for row in matrix.iter() {
        println!("{:?}", row);
   }
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}