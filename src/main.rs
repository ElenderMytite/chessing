static SIZE: usize = 8;

fn main() {
    let mut index;
    for i in 0..SIZE{
        for j in 0..SIZE {
            index = i * SIZE + j;
            println!("{}", index_to_position(index));
        }
    }  
}
fn index_to_position(index: usize) -> String {
    let column: usize = index % SIZE;
    let row: usize = index / SIZE + 1;
    return format!("{}{}", ((column + 97) as u8) as char, row);
}