use super::{empty::Empty, file_data::FileData, file_header::FileHeader, folder::Folder};

#[derive(Debug)]
pub enum Block {
    Folder(Folder),
    Empty(Empty),
    Damaged(Empty),
    FileHeader(FileHeader),
    FileData(FileData),
}

impl Block {
    pub fn parse(data: &str) -> Option<Block> {
        let (block_type, data) = data.split_at(1);
        let data = &data[..data.len() - 1];
        let data = hex::decode(data).ok()?;

        match block_type {
            "0" => Some(Block::Folder(Folder::parse(&data)?)),
            "1" => Some(Block::Empty(Empty::parse(&data))),
            "2" => Some(Block::Empty(Empty::parse(&data))),
            "3" => Some(Block::FileHeader(FileHeader::parse(&data)?)),
            "4" => Some(Block::FileData(FileData::parse(&data))),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Block::Folder(folder) => format!("0{}0", folder),
            Block::Empty(empty) => format!("1{}0", empty),
            Block::Damaged(empty) => format!("2{}0", empty),
            Block::FileHeader(file_header) => format!("3{}0", file_header),
            Block::FileData(file_data) => format!("4{}0", file_data),
        }
    }
}
