
extern crate src;

use src::read_file::read_file;

#[cfg(test)]
mod tests {
    #[test]
    fn opens_file() {
        
        assert_eq!(read_file("texto.txt").as_bytes(), [102, 111, 111]);
    }

}