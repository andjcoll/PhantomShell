#[cfg(test)]
mod tests {
    use crate::vfs::block::{block::Block, folder::Folder};

    #[test]
    fn test_parse_empty() {
        let data = "1000000";
        let parsed = Block::parse(data).unwrap();

        let actual = parsed.to_string();

        assert_eq!(data, actual);
    }

    #[test]
    #[should_panic]
    fn test_bad_datatype() {
        let data = "9000000";
        Block::parse(data).unwrap();
    }

    #[test]
    fn test_folder_parse() {
        let data = "010000";
        let data = hex::decode(data).unwrap();
        let block = Folder::parse(&data);

        assert_eq!(block.next_file, 1);

        for i in block.rest {
            assert_eq!(i, 0);
        }
    }
}
