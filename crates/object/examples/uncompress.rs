use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::{self, Read};

// 这个文件不一定在哦
fn read_git_object(sha: &str) -> io::Result<Vec<u8>> {
    // 构造对象文件路径
    let path = format!(".git/objects/{}/{}", &sha[..2], &sha[2..]);
    dbg!(&path);

    // 打开对象文件
    let file = File::open(path)?;

    // 读取文件内容
    let mut compressed_data = Vec::new();
    file.take(1024 * 1024).read_to_end(&mut compressed_data)?; // 读取最多1MB

    // 解压缩数据
    let mut decoder = ZlibDecoder::new(&compressed_data[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;

    Ok(decompressed_data)
}

fn main() -> io::Result<()> {
    let sha = "95db6b87121c1a2cd63762518be0c00635e8b502"; // 替换为实际的 SHA-1 哈希
    match read_git_object(sha) {
        Ok(data) => {
            println!("Decompressed data: {:?}", data);
            // 你可以在这里解析对象数据
        }
        Err(e) => {
            eprintln!("Error reading object: {}", e);
        }
    }
    Ok(())
}
