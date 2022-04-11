use calamine::{open_workbook, Reader, Xlsx};

use crate::error::Error;

pub struct Builder(String);

impl Builder {
    pub fn new(path: &str) -> Self {
        Self(path.to_string())
    }

    pub fn build(&self) -> Result<(), Error> {
        let mut workbook: Xlsx<_> = open_workbook(&*self.0)?;

        for (title_count, range) in workbook.worksheets() {
            let mut split = title_count.split('|');
            let title = split.next().unwrap_or_default();
            let count = split.next().unwrap_or("0").parse::<usize>().unwrap();
            println!("{}：{}", title, count);

            for row in range.rows() {
                for cell in row {
                    print!("{}\t", cell);
                }
                println!();
            }
        }

        Ok(())
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let builder = Builder::new("test.xlsx");
        builder.build().unwrap();
    }
}
