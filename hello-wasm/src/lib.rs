use wasm_bindgen::prelude::*;
use duckdb::Connection;

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
pub fn dcount() -> u32 {
    let db = Connection::open_in_memory();
    if let Ok(k) = db {
        if let Ok(v) = k.query_row("SELECT count(1) read_parquet('https://github.com/mnsrulz/hpqdata/releases/download/v1.0/FY2021_Q1.parquet');",[], |row| row.get(0)) {
            v
        } else {
            40
        }
    } else {
        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
