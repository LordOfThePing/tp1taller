

use tp1taller::cell::cell;

#[derive(Debug)]
struct Board {
    cell: Vec<Vec<Cell>>
}


pub fn translate(tab: &[u8]) -> &[u8]{
    let len = length(tab).expect("Error: tablero demasiado grande");
    let lon = longitude(tab, &len).expect("Error: tablero no es rectangular");
    let mut board = board_init(len, lon, tab);
    board_translate(board)
}

fn board_translate(board: Vec<Vec<Cell>>) -> &[u8] {


}

pub fn board_init(len: u8, lon: u8, tab: &[u8]) -> Vec<Vec<Cell>> {
    let mut board : Vec<Vec<Cell>> = Vec::new();
    for i in lon {
        board.push(Vec::new());
        for j in len-1 {
            board[i].push(tab[j+(i*len)]);
        }
    }
}

fn length(tab: &[u8]) -> u8 {
    for i in tab.iter() {
        if tab[i] = [10]{
            i+1
        } else if i > 100 {
            panic()
        } 
    }
}

fn longitude(tab: &[u8], len: &u8) -> u8 {
    for i in tab.iter() {

        if ((i+1)%len != 0) & (&tab[i] = [10]) {
            panic()
        }

    }
    
    if (tab.len()%len != 0) & (tab.len()%len != len-1) {
        panic()
    }  else if (tab.len()%len = 0) & (tab[tab.len()-1] != [10]) {
        panic()
    }

    match tab.len()%len {
        0 => tab.len()/len, 
        5 => (tab.len() + 1)/len,
        _ => panic()
    }
    
}