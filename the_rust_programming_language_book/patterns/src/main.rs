
fn pattern1(run: bool) {
    if !run {
        return;
    }
    println!("-------------------- Pattern 1: --------------------\n");

    let n: i32 = 10;
    let mut condition: bool;

    for i in 1..=n {
        for j in 1..=n {
            condition = j <= i;
            if condition {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        println!();
    }
    println!();
}

fn pattern2(run: bool) {
    if !run {
        return;
    }
    println!("-------------------- Pattern 2: --------------------\n");

    let n: i32 = 10;
    let mut condition: bool;

    for i in 1..=n {
        for j in 1..=n {
            condition = j >= n+ 1 -i;
            if condition {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        println!();
    }
    println!();
}

fn pattern3(run: bool) {
    if !run {
        return;
    }
    println!("-------------------- Pattern 3: --------------------\n");

    let n: i32 = 10;
    let mut condition: bool;

    for i in 1..=n {
        for j in 1..=n {
            condition = i <= j;
            if condition {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        println!();
    }
    println!();
}

fn pattern4(run: bool) {
    if !run {
        return;
    }
    println!("-------------------- Pattern 4: --------------------\n");

    let n: i32 = 10;
    let mut condition: bool;

    for i in 1..=n {
        for j in 1..=n {
            condition = j <= n + 1 - i;
            if condition {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        println!();
    }

    println!();
}

fn pattern5(run: bool) {
    if !run {
        return;
    }
    println!("-------------------- Pattern 5: --------------------\n");

    let row: i32 = 10;
    let column: i32 = (row * 2) - 1;
    let mut condition: bool;

    for i in 1..=row {
        for j in 1..=column {
            condition = j >= row + 1 - i && j <= row - 1 + i;
            if condition {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        println!();
    }
    println!();
}

fn pattern6(run: bool) {
    if !run {
        return;
    }
    println!("-------------------- Pattern 6: --------------------\n");

    let row: i32 = 11;
    let column: i32 = (row * 2) - 1;
    let mut condition: bool;

    for i in 1..=row {
        for j in 1..=column {
            condition = (j >= row + 1 - i && j <= row + i) && ((i % 2 == 1 && j % 2 == 1) || (i % 2 == 0 && j % 2 == 0));
            if condition {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        println!();
    }
    println!();
}

fn pattern7(run: bool) {
    if !run {
        return;
    }
    println!("-------------------- Pattern 7: --------------------\n");

    let row: i32 = 5;
    let column: i32 = (row * 2) - 1;
    let mut condition: bool;

    for i in 1..=row {
        for j in 1..= column {
            condition = j <= row + 1 - i || j >= row - 1 + i;
            if condition {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        println!();
    }
    println!();
}

fn pattern8(run: bool) {
    if !run {
        return;
    }
    println!("-------------------- Pattern 8: --------------------\n");

    let row: i32 = 5;
    let column: i32 = (row * 2) - 1;
    let mut condition: bool;
    let mut value: i32;

    for i in 1..=row {
        value = 1;
        for j in 1..=column {
            condition = j >= row + 1 - i && j <= row - 1 + i;
            if condition {
                print!("{value} ");
                if j < row {
                    value += 1;
                } else {
                    value -= 1;
                }
            } else {
                print!("  ");
            }
        }
        println!();
    }
    println!();
}

fn pattern9(run: bool) {
    if !run {
        return;
    }
    println!("-------------------- Pattern 9: --------------------\n");

    let row: i32 = 5;
    let column: i32 = (row * 2) - 1;
    let mut condition: bool;
    let mut value: u8;
    let mut character: char;

    for i in 1..=row {
        value = 65;
        for j in 1..=column {
            condition = j <= row + 1 - i || j >= row - 1 + i;
            if condition {
                character = value as char;
                print!("{character} ");
                if j < (row + 1 - i) {
                    value += 1;
                } else if j >= (row - 1 + i) {
                    value -= 1;
                }
            } else {
                print!("  ");
            }
        }
        println!();
    }
    println!();
}

fn pattern10(run: bool) {
    if !run {
        return;
    }
    println!("-------------------- Pattern 10: --------------------\n");

    let row: i32 = 7;
    let column: i32 = row;
    let middle: i32 = (row + 1) / 2;
    let mut condition: bool;

    for i in 1..=row {
        for j in 1..=column {
            condition = (i <= middle && (j >= middle + 1 - i && j <= middle - 1 + i)) || (i > middle && (j >= i - middle + 1 && j <= column - (i - middle)));
            if condition {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        println!();
    }
    println!();
}

fn pattern11(run: bool) {
    if !run {
        return;
    }
    println!("-------------------- Pattern 11: --------------------\n");

    let row: i32 = 11;
    let column: i32 = (row + 1) / 2;
    let mut condition: bool;

    for i in 1..=row {
        for j in 1..=column {
            condition = (i <= column && j <= i) || (i > column && j <= row - i + 1);
            if condition {
                print!("* ")
            } else {
                print!("  ")
            }
        }
        println!();
    }

    println!();
}

fn pattern12(run: bool) {
    if !run {
        return;
    }
    println!("-------------------- Pattern 12: --------------------\n");

    let row: i32 = 6;
    let column: i32 = (row * 2) - 1;
    let mut condition: bool;

    for i in 1..=row {
        for j in 1..=column {
            condition = (j <= row && j >= i) || (j > row && i <= column - j + 1);
            if condition {
                print!("* ")
            } else {
                print!("  ")
            }
        }
        println!();
    }

    println!();
}

fn pattern13(run: bool) {
    if !run {
        return;
    }
    println!("-------------------- Pattern 13: --------------------\n");

    let row: i32 = 5;
    let column: i32 = (row * 2) - 1;
    let mut condition: bool;
    let mut value: i32; 

    for i in 1..=row {
        value = i;
        for j in 1..=column {
            condition = j >= row + 1 - i && j <= row - 1 + i;
            if condition {
                print!("{value} ");

                if j < row {
                    value += 1;
                } else if j >= row {
                    value -= 1;
                }
            } else {
                print!("  ")
            }
        }
        println!();
    }

    println!();
}

fn pattern14(run: bool) {
    if !run {
        return;
    }
    println!("-------------------- Pattern 14: --------------------\n");

    let row: i32 = 7;
    let column: i32 = row;
    let mut condition: bool;
    let mut value: i32;

    for i in 1..=row {
        value = row - i;
        for j in 1..=column {
            condition = j <= row + 1 - i;
            if condition {
                print!("{value} ");
                value -= 1;
            } else {
                print!("  ");
            }
        }
        println!();
    }

    println!();
}

fn pattern15(run: bool) {
    if !run {
        return;
    }

    let column: i32 = 5;
    let row: i32 = (column * 2) - 1;
    let mut condition: bool;
    let mut value: i32;

    for i in 1..=row {
        value = 1;
        for j in 1..=column{
            condition = i >= column + 1 - j && i <= column - 1 + j;
            if condition {
                print!("{value} ");
                value += 1;
            } else {
                print!("  ")
            }
        }
        println!();
    }

    println!();
}

fn main() {
    println!("-------------------- Patterns in Rust! --------------------\n\n");

    pattern1(true);
    pattern2(true);
    pattern3(true);
    pattern4(true);
    pattern5(true);
    pattern6(true);
    pattern7(true);
    pattern8(true);
    pattern9(true);
    pattern10(true);
    pattern11(true);
    pattern12(true);
    pattern13(true);
    pattern14(true);
    pattern15(true);
}
