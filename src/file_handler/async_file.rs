use futures::future::join_all;
use tokio::{fs::remove_file, io};

pub async fn file_exists(file_name: &str) -> Result<bool, io::Error> {
    tokio::fs::File::open(file_name).await?;
    Ok(true)
}

async fn remove_file_async(file: &str) -> Result<(), io::Error> {
    if file_exists(file).await? {
        remove_file(file).await
    } else {
        Ok(())
    }
}

pub async fn remove_files(files: Vec<&str>) -> Result<(), io::Error> {
    let mut futures = Vec::new();
    for file in files {
        futures.push(remove_file_async(file));
    }
    join_all(futures).await;
    Ok(())
}
