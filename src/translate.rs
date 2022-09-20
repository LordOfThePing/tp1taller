use crate::cell::Cell;

pub fn translate(tab: &[u8]) -> Vec<u8> {
    let len = length(tab).expect("Error: board too big");
    let lon = longitude(tab, &len).expect("Error: board not rectangular");
    let mut board = board_init(len, lon, tab).expect("Error: board has incorrect format");
    board = board_translate(board);

    board_to_slice(board)
}

pub fn board_to_slice(board: Vec<Vec<Cell>>) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::with_capacity(board.len() + board[0].len());
    for row in board.iter() {
        for cell in row.iter() {
            vec.push(cell.obtain_ascii());
        }
        vec.push(10);
    }

    vec
}

pub fn board_translate(mut board: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == Cell::Bomb {
                if i > 0 {
                    board[i - 1][j] = board[i - 1][j].increase();
                    if j > 0 {
                        board[i - 1][j - 1] = board[i - 1][j - 1].increase();
                    }
                    if j + 1 < board[i].len() {
                        board[i - 1][j + 1] = board[i - 1][j + 1].increase();
                    }
                }

                if j > 0 {
                    board[i][j - 1] = board[i][j - 1].increase();
                }
                if i + 1 < board.len() {
                    board[i + 1][j] = board[i + 1][j].increase();
                    if j > 0 {
                        board[i + 1][j - 1] = board[i + 1][j - 1].increase();
                    }
                    if j + 1 < board[i].len() {
                        board[i + 1][j + 1] = board[i + 1][j + 1].increase();
                    }
                }
                if j + 1 < board[i].len() {
                    board[i][j + 1] = board[i][j + 1].increase();
                }
            }
        }
    }
    board
}

pub fn board_init(len: u8, lon: u8, tab: &[u8]) -> Result<Vec<Vec<Cell>>, u8> {
    let mut board: Vec<Vec<Cell>> = Vec::new();
    for i in 0..lon {
        board.push(Vec::new());
        for j in 0..(len - 1) {
            match tab[(j + (i * len)) as usize] {
                42 => board[i as usize].push(Cell::Bomb),
                46 => board[i as usize].push(Cell::Empty),
                _ => return Err(1),
            }
        }
    }
    Ok(board)
}

fn length(tab: &[u8]) -> Result<u8, ()> {
    for (i, item) in tab.iter().enumerate() {
        if *item == 10 {
            return Ok((i + 1) as u8);
        } else if i > 100 {
            return Err(());
        }
    }
    Ok(0)
}

pub fn longitude(tab: &[u8], len: &u8) -> Result<u8, u8> {
    for (i, item) in tab.iter().enumerate() {
        if ((i + 1) as u8 % (*len) != 0) & (*item == 10) {
            return Err(1);
        }
    }

    let l =
        u8::try_from(tab.len()).expect("Error: not all integers can be shown with 'try_from()'");

    if (l % len != 0) & (l % len != len - 1) {
        return Err(2);
    } else if (l % len == 0) & (tab[(l - 1) as usize] != 10) {
        return Err(3);
    }

    if l % *len == 0 {
        Ok(l / *len)
    } else if l % *len == *len - 1 {
        Ok((l + 1) / *len)
    } else {
        Err(4)
    }
    /*
    match l%len {
        0 => Ok(l/len),
        => Ok((l + 1)/len),
        _ => Err(4),
    }
    */
}
