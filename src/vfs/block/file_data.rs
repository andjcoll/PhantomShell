use core::fmt;

#[derive(Debug)]
pub struct FileData {
    pub next_data: u8,
    pub data: Vec<u8>,
}

impl FileData {
    pub fn parse(data: &[u8]) -> FileData {
        let (next_data, rest) = data.split_at(1);

        FileData {
            next_data: next_data[0],
            data: rest.to_vec(),
        }
    }
}

impl fmt::Display for FileData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut combined = Vec::with_capacity(1 + self.data.len());
        combined.push(self.next_data);
        combined.extend(&self.data);

        let out = hex::encode(combined).to_uppercase();
        write!(f, "{}", out)
    }
}
