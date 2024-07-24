fn main() {
    let matrix_a = [
        [1,2,3],
        [4,5,6]
    ];

    let matrix_b = [
        [1,2,3],
        [4,5,6],
        [7,8,9]
    ];

    let mut result = [
        [0,0,0],
        [0,0,0]
    ];

    for i in 0..2{
        for j in 0..=2{
            for k in 0..=2{
                result[i][j] += matrix_a[i][k] * matrix_b[k][j];
            }
        }
    }

    //result
    println!("Result Matrix: ");
    for row in &result{
        println!("{:?}", row);
    }
}
