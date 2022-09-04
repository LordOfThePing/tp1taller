
use tp1taller::read_file::read_file;

#[test]
fn opens_file() {
    
    assert_eq!(read_file(&"file/texto.txt".to_string()).as_bytes(), [46, 42, 46, 42, 46, 10, 
                                                                46, 46, 42, 46, 46, 10, 
                                                                46, 46, 42, 46, 46, 10, 
                                                                46, 46, 46, 46, 46]);
}

