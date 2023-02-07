pub mod ioutils {
    use std::io::{BufReader, BufRead};
    use std::fs::File;

    pub fn get_lines(path: &str) -> impl Iterator<Item = String> {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                panic!("open faild: {e}");
            }
        };
        
        let reader = BufReader::new(file);

        reader.lines().into_iter().map(|line| {
            match line {
                Ok(l) => l,
                Err(e) => panic!("read line failed: {e}"),
            }
        })

    }
}

