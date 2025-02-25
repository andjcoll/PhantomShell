use core::fmt;

#[derive(Debug)]
pub struct FileHeader {
    pub next_file: u8,
    pub next_data: u8,
    pub name: Vec<u8>,
    pub data: Vec<u8>,
}

impl FileHeader {
    pub fn parse(data: &[u8]) -> Option<FileHeader> {
        let (next_file, data) = data.split_at(1);
        let (next_data, rest) = data.split_at(1);

        let pos = rest.iter().position(|window| *window == 0)?;
        let (name, rest) = rest.split_at(pos);

        Some(FileHeader {
            next_file: next_file[0],
            next_data: next_data[0],
            name: name.to_vec(),
            data: rest[1..].to_vec(),
        })
    }
}

impl fmt::Display for FileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut combined = Vec::with_capacity(3 + self.data.len());
        combined.push(self.next_file);
        combined.push(self.next_data);
        combined.extend(&self.name);
        combined.extend(&self.data);

        let out = hex::encode(combined).to_uppercase();
        write!(f, "{}", out)
    }
}
