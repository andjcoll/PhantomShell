use core::fmt;

#[derive(Debug)]
pub struct Folder {
    pub next_available: u8,
    pub next_bad: u8,
    pub next_file: u8,
    pub name: Vec<u8>,
    pub rest: Vec<u8>,
}

impl Folder {
    pub fn parse(data: &[u8]) -> Option<Folder> {
        let (next_available, data) = data.split_at(1);
        let (next_bad, data) = data.split_at(1);
        let (next_file, data) = data.split_at(1);

        let pos = data.iter().position(|window| *window == 0)?;
        let (name, data) = data.split_at(pos);

        Some(Folder {
            next_available: next_available[0],
            next_bad: next_bad[0],
            next_file: next_file[0],
            name: name.to_vec(),
            rest: data.to_vec(),
        })
    }
}

impl fmt::Display for Folder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut combined = Vec::with_capacity(3 + self.rest.len());
        combined.push(self.next_available);
        combined.push(self.next_bad);
        combined.push(self.next_file);
        combined.extend(&self.name);
        combined.extend(&self.rest);

        let out = hex::encode(combined).to_uppercase();
        write!(f, "{}", out)
    }
}
