use core::fmt;

#[derive(Debug)]
pub struct Empty {
    pub next_file: u8,
    pub data: Vec<u8>,
}

impl Empty {
    pub fn parse(data: &[u8]) -> Empty {
        let (next_file, data) = data.split_at(1);
        Empty {
            next_file: next_file[0],
            data: data.to_vec(),
        }
    }
}

impl fmt::Display for Empty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut combined = Vec::with_capacity(1 + self.data.len());
        combined.push(self.next_file);
        combined.extend(&self.data);

        let out = hex::encode(combined).to_uppercase();
        write!(f, "{}", out)
    }
}
