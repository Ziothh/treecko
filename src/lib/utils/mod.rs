pub type DirData = Vec<std::fs::DirEntry>;

pub fn read_dir(path: &std::path::Path) -> std::io::Result<DirData> {
    let dir_items: Vec<_> = std::fs::read_dir(path)?.into_iter().flatten().collect();

    return Ok(dir_items);
}
