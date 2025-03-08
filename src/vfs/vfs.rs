use std::fmt;

use super::block::{block::Block, empty::Empty, folder::Folder};

#[derive(Debug)]
pub struct VFS {
    pub blocks: Vec<Block>,
}

#[derive(Debug)]
pub enum VFSError {
    NotDivisibleByTwo,
    ColsTooSmall,
    RowsTooSmall,
    VolumeNameTooLarge,
    FileNotFound,
    DataNotFound,
}

impl fmt::Display for VFSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VFSError::NotDivisibleByTwo => write!(f, "cols is not divisible by 2"),
            VFSError::ColsTooSmall => {
                write!(f, "number of cols is too small (must be greater than 11)")
            }
            VFSError::RowsTooSmall => {
                write!(f, "number of rows is too small (must be greater than 1)")
            }
            VFSError::VolumeNameTooLarge => write!(f, "volume name is too large"),
            VFSError::FileNotFound => write!(f, "file in vfs was not found"),
            VFSError::DataNotFound => write!(f, "disk is missing data"),
        }
    }
}

impl VFS {
    pub fn new(name: &str, rows: usize, cols: usize) -> Result<VFS, VFSError> {
        if cols < 11 {
            return Err(VFSError::ColsTooSmall);
        }
        if cols % 2 != 0 {
            return Err(VFSError::NotDivisibleByTwo);
        }
        if rows < 2 {
            return Err(VFSError::RowsTooSmall);
        }

        // one nibble for start and end bit
        // divide by 2 for number of bytes
        let cols = (cols - 1) / 2;
        let name = name.as_bytes().to_vec();

        if name.len() > cols - 3 {
            return Err(VFSError::VolumeNameTooLarge);
        }

        let root = Block::Folder(Folder {
            next_available: 1,
            next_bad: 0,
            next_file: 0,
            name: name.clone(),
            rest: vec![0; cols - 3 - name.len()],
        });

        let mut blocks = vec![root];

        for i in 1..rows {
            let mut next_file = i + 1;
            if i == rows - 1 {
                next_file = 0
            }

            let block = Block::Empty(Empty {
                next_file: next_file.try_into().unwrap(),
                data: vec![0; cols - 1],
            });

            blocks.push(block);
        }

        let vfs = VFS { blocks };

        Ok(vfs)
    }

    pub fn parse(input: &str) -> Option<VFS> {
        let mut blocks = Vec::new();
        for line in input.lines().skip(2) {
            if line.len() < 11 {
                eprintln!("Malformatted disk. Line isn't long enough.");
                return None;
            }

            let line = &line[3..];

            let block = Block::parse(line)?;
            blocks.push(block);
        }

        if blocks.len() < 1 {
            return None;
        }

        let vfs = VFS { blocks };
        Some(vfs)
    }

    pub fn print(&self) -> String {
        let mut output = String::new();

        let mut tens = String::from("XX: ");
        let mut ones = String::from("XX:");

        let string_blocks: Vec<String> =
            self.blocks.iter().map(|block| block.to_string()).collect();

        // Create column numbering
        let length = string_blocks[0].len();
        let mut ten = 0;
        for i in 0..length {
            let one = i % 16;
            let hex = format!("{:X}", one);
            ones.push_str(&hex);

            if i != 0 && one == 0 {
                ten += 1;
                tens.push_str(&" ".repeat(15));
                let hex = format!("{:X}", ten);
                tens.push_str(&hex);
            }
        }

        output.push_str(&tens);
        output.push_str("\n");
        output.push_str(&ones);
        output.push_str("\n");

        // Create row numbering
        for (i, line) in string_blocks.clone().into_iter().enumerate() {
            let label = format!("{:02X}:", i);
            output.push_str(&label);
            output.push_str(&line);
            output.push_str("\n");
        }
        output.to_string()
    }

    pub fn get_files(&self) -> Vec<(String, usize)> {
        let mut files = Vec::new();

        let root = match self.blocks.get(0) {
            Some(Block::Folder(folder)) => folder,
            _ => panic!(),
        };

        let mut next = root.next_file;

        while next != 0 {
            match self.blocks.get(next as usize).unwrap() {
                Block::FileHeader(file_header) => {
                    files.push((
                        String::from_utf8(file_header.name.clone()).unwrap(),
                        next.try_into().unwrap(),
                    ));
                    next = file_header.next_file;
                }
                Block::Empty(empty) => {
                    next = empty.next_file;
                }
                _ => break,
            };
        }

        files
    }

    pub fn cat_file(&self, filename: &str) -> Result<String, VFSError> {
        let files = self.get_files();
        let pos = files
            .iter()
            .find(|file| file.0 == filename)
            .ok_or(VFSError::FileNotFound)?
            .1;

        let block = &self.blocks[pos];
        let mut data: Vec<u8> = Vec::new();
        let mut next_data: u8;
        match block {
            Block::FileHeader(file_header) => {
                next_data = file_header.next_data;
                let curr_data = file_header.data.clone();
                data.extend(&curr_data[..curr_data.len() - 1])
            }
            _ => {
                return Err(VFSError::FileNotFound);
            }
        }

        while next_data != 0 {
            let block = &self.blocks[next_data as usize];
            match block {
                Block::FileData(file_data) => {
                    next_data = file_data.next_data;

                    let mut curr_data = file_data.data.clone();
                    let null_pos = curr_data.iter().position(|byte| *byte == 0);
                    if let Some(pos) = null_pos {
                        curr_data = curr_data[..pos].to_vec();
                    }

                    data.extend(&curr_data);
                }
                _ => {
                    return Err(VFSError::DataNotFound);
                }
            }
        }

        Ok(String::from_utf8_lossy(&data).to_string())
    }
}
