use std::path::{Path, PathBuf};

pub fn get_prefix(path: &Path) -> &str {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let first_dot_index = file_name.find('.').unwrap_or(file_name.len());
    &file_name[..first_dot_index]
}

pub fn get_suffix(path: &Path) -> &str {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let first_dot_index = file_name.find('.').unwrap_or(file_name.len());
    &file_name[first_dot_index..]
}

pub fn get_unique_target(source_path: &Path, target_dir: &Path) -> PathBuf {
    let mut target_file = target_dir.join(source_path.file_name().unwrap());
    let mut index = 1;

    while target_file.exists() {
        let file_name = get_prefix(source_path);
        let file_ext = get_suffix(source_path);
        let link_name = format!("{} ({}){}", file_name, index, file_ext);
        target_file = target_dir.join(link_name);
        index += 1;
    }

    target_file
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_prefix() {
        assert_eq!(get_prefix(Path::new("test_dir/files/a.txt")), "a");
        assert_eq!(get_prefix(Path::new("test_dir/files/b.ext.txt")), "b");
        assert_eq!(get_prefix(Path::new("test_dir/files/")), "files");
        assert_eq!(get_prefix(Path::new("test_dir/files/  a.txt  ")), "  a");
        assert_eq!(get_prefix(Path::new("test_dir/files/  a  ")), "  a  ");
    }

    #[test]
    fn test_get_suffix() {
        assert_eq!(get_suffix(Path::new("test_dir/files/a.txt")), ".txt");
        assert_eq!(
            get_suffix(Path::new("test_dir/files/b.ext.txt")),
            ".ext.txt"
        );
        assert_eq!(get_suffix(Path::new("test_dir/files/a.txt   ")), ".txt   ");
        assert_eq!(get_suffix(Path::new("test_dir/files/a")), "");
    }

    #[test]
    fn test_get_unique_target() {
        let source_path = Path::new("test_dir/files/a.txt");
        let target_dir = Path::new("test_dir");
        assert_eq!(
            get_unique_target(source_path, target_dir),
            Path::new("test_dir/a.txt")
        );

        let source_path = Path::new("test_dir/files/a.txt");
        let target_dir = Path::new("test_dir/files");
        assert_eq!(
            get_unique_target(source_path, target_dir),
            Path::new("test_dir/files/a (1).txt")
        );

        let source_path = Path::new("test_dir/files/b.ext.txt");
        let target_dir = Path::new("test_dir/files");
        assert_eq!(
            get_unique_target(source_path, target_dir),
            Path::new("test_dir/files/b (2).ext.txt")
        );

        let source_path = Path::new("test_dir/files/");
        let target_dir = Path::new("test_dir/");
        assert_eq!(
            get_unique_target(source_path, target_dir),
            Path::new("test_dir/files (1)")
        );
    }
}
