use std::io;

use io::Read;

use io::BufWriter;
use io::Write;

use zip::CompressionMethod;
use zip::read::ZipFile;

#[derive(serde::Serialize)]
pub enum Method {
    Stored,
    Deflated,
    Bzip2,
    Zstd,
    Lzma,
    Xz,
    Unsupported(u16),
    Unknown,
}

impl From<CompressionMethod> for Method {
    fn from(m: CompressionMethod) -> Self {
        match m {
            CompressionMethod::Stored => Self::Stored,
            CompressionMethod::Deflated => Self::Deflated,
            _ => Self::Unknown,
        }
    }
}

#[derive(serde::Serialize)]
pub struct ZipItemInfo {
    pub name: String,
    pub comment: String,
    pub method: Method,
    pub encrypted: bool,
    pub size_compressed: u64,
    pub size_original: u64,
    pub modified_time: String,
    pub is_dir: bool,
    pub is_symlink: bool,
    pub is_file: bool,
    pub crc32: u32,
    pub data_start: u64,
    pub header_start: u64,
    pub central_header_start: u64,
}

impl ZipItemInfo {
    pub fn to_json(&self) -> Result<String, io::Error> {
        serde_json::to_string(self).map_err(io::Error::other)
    }
}

impl<'a, R> From<&ZipFile<'a, R>> for ZipItemInfo
where
    R: Read,
{
    fn from(z: &ZipFile<'a, R>) -> Self {
        Self {
            name: z.name().into(),
            comment: z.comment().into(),
            method: z.compression().into(),
            encrypted: z.encrypted(),
            size_compressed: z.compressed_size(),
            size_original: z.size(),
            modified_time: z.last_modified().unwrap_or_default().to_string(),
            is_dir: z.is_dir(),
            is_symlink: z.is_symlink(),
            is_file: z.is_file(),
            crc32: z.crc32(),
            data_start: 0, // this may cause panic
            header_start: z.header_start(),
            central_header_start: z.central_header_start(),
        }
    }
}

pub fn reader2zipitems2metadata2jsons<R>(
    mut rdr: R,
) -> impl Iterator<Item = Result<String, io::Error>>
where
    R: Read,
{
    std::iter::from_fn(move || {
        let rslt = zip::read::read_zipfile_from_stream(&mut rdr);
        let mapd = rslt.map_err(io::Error::other);
        match mapd {
            Ok(None) => None,
            Ok(Some(zfile)) => {
                let zinfo: ZipItemInfo = (&zfile).into();
                match zinfo.to_json() {
                    Ok(zjson) => Some(Ok(zjson)),
                    Err(e) => Some(Err(e)),
                }
            }
            Err(e) => Some(Err(e)),
        }
    })
}

pub fn stdin2zipitems2metadata2jsons() -> impl Iterator<Item = Result<String, io::Error>> {
    let i = io::stdin();
    let l = i.lock();
    reader2zipitems2metadata2jsons(l)
}

pub fn jsons2writer<W, I>(mut w: W) -> impl FnMut(I) -> Result<(), io::Error>
where
    W: Write,
    I: Iterator<Item = Result<String, io::Error>>,
{
    move |jsons| {
        let mut bw = BufWriter::new(&mut w);
        for rjson in jsons {
            let jstr: String = rjson?;
            writeln!(&mut bw, "{jstr}")?;
        }
        bw.flush()?;
        drop(bw);

        w.flush()
    }
}

pub fn jsons2stdout<I>(jsons: I) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<String, io::Error>>,
{
    let o = io::stdout();
    let l = o.lock();
    jsons2writer(l)(jsons)
}

pub fn stdin2zipitems2metadata2jsons2stdout() -> Result<(), io::Error> {
    let jsons = stdin2zipitems2metadata2jsons();
    jsons2stdout(jsons)
}
