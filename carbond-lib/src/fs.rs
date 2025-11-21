use std::path::Path;

use tokio::io;

pub async fn create_file(path: &Path) -> io::Result<bool> {
    if !path.is_file() {
        let prefix = path.parent().ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "Parent of path not found.",
        ))?;
        tokio::fs::create_dir_all(prefix).await?;
        return Ok(true);
    }
    Ok(false)
}
