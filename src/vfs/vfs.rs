pub struct VFS {
    pub blocks: Vec<String>,
}

impl VFS {
    pub fn parse(input: &str) -> Option<VFS> {
        let mut blocks = Vec::new();
        for line in input.lines().skip(2) {
            if line.len() <= 3 {
                eprintln!("Malformatted disk. Line isn't long enough.");
                return None;
            }
            blocks.push(line[3..].to_string());
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

        // Create column numbering
        let length = self.blocks[0].len();
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
        for (i, line) in self.blocks.clone().into_iter().enumerate() {
            let label = format!("{:02X}:", i);
            output.push_str(&label);
            output.push_str(&line);
            output.push_str("\n");
        }
        output.to_string()
    }

    pub fn get_files(&self) -> Vec<String> {
        let mut files = Vec::new();

        let root = self.blocks.get(0).unwrap();
        let mut next = &root[5..=6];

        while next != "00" {
            let i = *hex::decode(next).unwrap().get(0).unwrap();
            let curr = self.blocks.get(i as usize).unwrap();

            let mut name = &curr[5..];
            let name_end = name.find("00").unwrap();
            name = &name[..name_end];

            let name = hex::decode(name).unwrap();
            files.push(String::from_utf8(name).unwrap());

            next = &curr[1..=2];
        }

        files
    }
}
