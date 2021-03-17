use {
    crate::*,
    anyhow::*,
    char_reader::CharReader,
    std::{
        fs::File,
        path::Path,
    },
};

pub fn read(
    path: &Path,
    h: &mut Histogram,
) -> anyhow::Result<usize> {
    let file = File::open(&path)?;
    let mut reader = CharReader::new(file);
    let mut line = String::new();
    let mut max_len = 0;
    while reader.read_line(&mut line, N*10, N*10)? {
        let len = line.chars().count();
        if len > max_len {
            max_len = len;
        }
        let idx = (len + 9) / 10;
        if idx >= N {
            bail!("line too long: {:?}", len);
        }
        h.bars[idx] += 1;
        line.clear();
    }
    Ok(max_len)
}
