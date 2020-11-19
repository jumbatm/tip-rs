#[cfg(test)]
mod tests {
    use std::path::Path;
    use tip::tip_parser;
    #[test]
    fn test_examples_folder() {
        fn try_parse_dir(dir: &Path) -> Result<(), tip_parser::ParseError>{
            assert!(dir.exists());
            assert!(dir.is_dir());
            for path in std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()) {
                if path.is_dir() {
                    try_parse_dir(&path)?
                } else {
                    println!("Parsing {:?}", path);
                    tip_parser::parse(std::fs::read_to_string(path).unwrap())?;
                }
            }
            Ok(())
        }
        let _ = try_parse_dir(Path::new("examples")).unwrap();
    }
}
