#[allow(dead_code)]
// Gradient function based on CSES number spiral problem
pub fn number_spiral(row : u32, col : u32) -> u32 {
    let num : u32;
    if row > col {
        if row % 2 == 0 {num = row * row - col + 1;}
        else {num = (row - 1) * (row - 1) + col;}
    } else if col > row {
        if col % 2 == 1 {num = col * col - row + 1;}
        else {num = (col - 1) * (col - 1) + row;}
    } else {
        if row % 2 == 0 {num = row * row - col + 1;}
        else {num = col * col - row + 1;}
    }
    return num;
}

#[allow(dead_code)]
// Gradient function based on CSES number grid problem
pub fn number_grid(row : u32, col : u32) -> u32 {
    return (row-1)^(col-1);
}
