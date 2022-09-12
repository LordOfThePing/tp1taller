use tp1taller::cell::Cell;

#[warn(unused_imports)]
#[test]
fn test_cell_increase_empty() {
    let mut cell = Cell::Empty;
    cell = cell.increase();
    cell = cell.increase();
    cell = cell.increase();

    assert_eq!(cell.obtain_ascii(), 51);
}
#[warn(unused_imports)]
#[test]
fn test_cell_increase_bomb() {
    let mut cell = Cell::Bomb;
    cell = cell.increase();
    cell = cell.increase();
    cell = cell.increase();

    assert_eq!(cell.obtain_ascii(), 42);
}
#[warn(unused_imports)]
#[test]
fn test_cell_increase_number() {
    let mut cell = Cell::Number(2);
    cell = cell.increase();
    cell = cell.increase();
    cell = cell.increase();

    assert_eq!(cell.obtain_ascii(), 53);
}
