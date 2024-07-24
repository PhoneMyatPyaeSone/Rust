fn main() {
    let rows = 8;

    let mut triangle = vec![vec![1]];

    for i in 1..rows {
        let mut row = vec![1];
        let previous_row = &triangle[i - 1];
        for j in 1..i {
            row.push(previous_row[j - 1] + previous_row[j]);
        }
        row.push(1);
        triangle.push(row);
    }

    for i in 0..rows {
        for _ in 0..(rows - i - 1) {
            print!(" ");
        }
        for num in &triangle[i] {
            print!("{} ", num);
        }
        println!();
    }
}
