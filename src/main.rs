use std::io::{ self, prelude::*, BufReader };
use std::fs::File;

struct Junker<'a, I>
where
    I: Iterator<Item = Result<u8, std::io::Error>>
{
    source: &'a mut I,
    value: u8,
    length: usize,
    index: usize,
}

impl<'a, I> Junker<'a, I>
where
    I: Iterator<Item = Result<u8, std::io::Error>>
{
    fn new(source: &'a mut I, length: usize) -> Result<Self, std::io::Error> {
        let value = match source.next() {
            Some(v) => match v {
                Ok(x) => x,
                Err(_) => 0,
            },
            None => 0,
        };
        Ok(Self {
            source,
            value,
            length,
            index: 0,
        })
    }
}

impl<'a, I> Iterator for Junker<'a, I>
where
    I: Iterator<Item = Result<u8, std::io::Error>>
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.length {
            // Try to get a new value from source
            match self.source.next() {
                Some(res) => {
                    match res {
                        Ok(val) => {
                            self.value = val;
                            self.index = 1;
                            return Some(val);
                        },
                        Err(e) => {
                            return None;
                        },
                    }
                },
                None => {
                    return None;
                }
            }
        } else {
            self.index += 1;
            Some(self.value)
        }
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("Cargo.toml")?;
    let mut bytes = BufReader::new(file).bytes();

    let mut junk = Junker::new(&mut bytes, 50)?;

    let mut inflated: Vec<u8> = junk.collect();

    let mut output = File::create("output.rawr")?;
    output.write_all(&inflated);

    Ok(())
}
