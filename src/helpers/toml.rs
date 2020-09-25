use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Read, Write},
    path::Path,
};

use crate::{data::Data, Result};

/// Attempts to deserialize a Data struct from a string
pub fn from_str(s: &str) -> Result<Data> {
    Ok(toml::from_str(s)?)
}

/// Attempts to deserialize a Data struct from a slice of bytes
pub fn from_slice(s: &[u8]) -> Result<Data> {
    Ok(toml::from_slice(s)?)
}

/// Attempts to deserialize a Data struct from something that implements
/// the std::io::Read trait
pub fn from_reader<R: Read>(mut r: R) -> Result<Data> {
    let mut buffer = Vec::new();
    r.read_to_end(&mut buffer)?;
    from_slice(&buffer)
}

/// Attempts to deserialize a Data struct from a file
pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Data> {
    let path = path.as_ref();
    let file = File::open(path)?;
    Ok(from_reader(file)?)
}

/// Attempts to serialize a Data struct to a String
pub fn to_string(data: &Data) -> Result<String> {
    Ok(toml::to_string_pretty(data)?)
}

/// Attempts to serialize a Data struct to a Vec of bytes
pub fn to_vec(data: &Data) -> Result<Vec<u8>> {
    Ok(toml::to_vec(data)?)
}

/// Attempts to serialize a Data struct to something that implements the
/// std::io::Write trait
pub fn to_writer<W: Write>(data: &Data, writer: W) -> Result<()> {
    let mut buf_writer = BufWriter::new(writer);
    let vec = to_vec(data)?;
    buf_writer.write(&vec)?;
    Ok(())
}

/// Attempts to serialize a Data struct to a file
///
/// When opening the file, this will set the `.write(true)` and
/// `.truncate(true)` options, use the next method for more
/// fine-grained control
pub fn to_file<P: AsRef<Path>>(data: &Data, path: P) -> Result<()> {
    let mut options = OpenOptions::new();
    options.create(true).write(true).truncate(true);
    to_file_with_options(data, path, options)?;
    Ok(())
}

/// Attempts to serialize a Data struct to a file
pub fn to_file_with_options<P: AsRef<Path>>(
    data: &Data,
    path: P,
    options: OpenOptions,
) -> Result<()> {
    let path = path.as_ref();
    let file = options.open(path)?;
    to_writer(data, file)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::OpenOptions, io::Cursor};
    use tempfile::{tempdir, NamedTempFile};

    const DOC: &'static str = indoc::indoc!(
        r#"
            base = "https://example.com"
            client_id = "adbc01234"
            client_secret = "0987dcba"
            redirect = "urn:ietf:wg:oauth:2.0:oob"
            token = "fedc5678"
    "#
    );

    #[test]
    fn test_from_str() {
        let desered = from_str(DOC).expect("Couldn't deserialize Data");
        assert_eq!(
            desered,
            Data {
                base: "https://example.com".into(),
                client_id: "adbc01234".into(),
                client_secret: "0987dcba".into(),
                redirect: "urn:ietf:wg:oauth:2.0:oob".into(),
                token: "fedc5678".into(),
            }
        );
    }
    #[test]
    fn test_from_slice() {
        let doc = DOC.as_bytes();
        let desered = from_slice(&doc).expect("Couldn't deserialize Data");
        assert_eq!(
            desered,
            Data {
                base: "https://example.com".into(),
                client_id: "adbc01234".into(),
                client_secret: "0987dcba".into(),
                redirect: "urn:ietf:wg:oauth:2.0:oob".into(),
                token: "fedc5678".into(),
            }
        );
    }
    #[test]
    fn test_from_reader() {
        let doc = DOC.as_bytes();
        let doc = Cursor::new(doc);
        let desered = from_reader(doc).expect("Couldn't deserialize Data");
        assert_eq!(
            desered,
            Data {
                base: "https://example.com".into(),
                client_id: "adbc01234".into(),
                client_secret: "0987dcba".into(),
                redirect: "urn:ietf:wg:oauth:2.0:oob".into(),
                token: "fedc5678".into(),
            }
        );
    }
    #[test]
    fn test_from_file() {
        let mut datafile = NamedTempFile::new().expect("Couldn't create tempfile");
        write!(&mut datafile, "{}", DOC).expect("Couldn't write Data to file");
        let desered = from_file(datafile.path()).expect("Couldn't deserialize Data");
        assert_eq!(
            desered,
            Data {
                base: "https://example.com".into(),
                client_id: "adbc01234".into(),
                client_secret: "0987dcba".into(),
                redirect: "urn:ietf:wg:oauth:2.0:oob".into(),
                token: "fedc5678".into(),
            }
        );
    }
    #[test]
    fn test_to_string() {
        let data = Data {
            base: "https://example.com".into(),
            client_id: "adbc01234".into(),
            client_secret: "0987dcba".into(),
            redirect: "urn:ietf:wg:oauth:2.0:oob".into(),
            token: "fedc5678".into(),
        };
        let s = to_string(&data).expect("Couldn't serialize Data");
        let desered = from_str(&s).expect("Couldn't deserialize Data");
        assert_eq!(data, desered);
    }
    #[test]
    fn test_to_vec() {
        let data = Data {
            base: "https://example.com".into(),
            client_id: "adbc01234".into(),
            client_secret: "0987dcba".into(),
            redirect: "urn:ietf:wg:oauth:2.0:oob".into(),
            token: "fedc5678".into(),
        };
        let v = to_vec(&data).expect("Couldn't write to vec");
        let desered = from_slice(&v).expect("Couldn't deserialize data");
        assert_eq!(data, desered);
    }
    #[test]
    fn test_to_writer() {
        let data = Data {
            base: "https://example.com".into(),
            client_id: "adbc01234".into(),
            client_secret: "0987dcba".into(),
            redirect: "urn:ietf:wg:oauth:2.0:oob".into(),
            token: "fedc5678".into(),
        };
        let mut buffer = Vec::new();
        to_writer(&data, &mut buffer).expect("Couldn't write to writer");
        let reader = Cursor::new(buffer);
        let desered = from_reader(reader).expect("Couldn't deserialize Data");
        assert_eq!(data, desered);
    }
    #[test]
    fn test_to_file() {
        let data = Data {
            base: "https://example.com".into(),
            client_id: "adbc01234".into(),
            client_secret: "0987dcba".into(),
            redirect: "urn:ietf:wg:oauth:2.0:oob".into(),
            token: "fedc5678".into(),
        };
        let tempdir = tempdir().expect("Couldn't create tempdir");
        let filename = tempdir.path().join("mastodon-data.toml");
        to_file(&data, &filename).expect("Couldn't write to file");
        let desered = from_file(&filename).expect("Couldn't deserialize Data");
        assert_eq!(data, desered);
    }
    #[test]
    fn test_to_file_with_options() {
        let data = Data {
            base: "https://example.com".into(),
            client_id: "adbc01234".into(),
            client_secret: "0987dcba".into(),
            redirect: "urn:ietf:wg:oauth:2.0:oob".into(),
            token: "fedc5678".into(),
        };
        let file = NamedTempFile::new().expect("Couldn't create tempfile");
        let mut options = OpenOptions::new();
        options.write(true).create(false).truncate(true);
        to_file_with_options(&data, file.path(), options).expect("Couldn't write to file");
        let desered = from_file(file.path()).expect("Couldn't deserialize Data");
        assert_eq!(data, desered);
    }
}
