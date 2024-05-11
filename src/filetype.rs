use crate::filetype::Filetype::{Json, Nix};
use std::convert::TryFrom;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum Filetype {
    Nix,
    Json,
}

impl TryFrom<&OsStr> for Filetype {
    type Error = String;

    fn try_from(ext: &OsStr) -> Result<Self, Self::Error> {
        match ext.as_bytes() {
            b"nix" => Ok(Nix),
            b"json" => Ok(Json),
            &_ => Err(format!("Unrecognized file extension '{:#?}'", ext)),
        }
    }
}

impl TryFrom<&Path> for Filetype {
    type Error = String;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        match path.extension() {
            None => Err("Must provide a file ending in '.nix' or '.json'".to_string()),
            Some(ext) => Filetype::try_from(ext),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Filetype;
    use std::ffi::OsStr;
    use std::path::Path;

    #[test]
    fn filetype_osstr() {
        assert_eq!(Filetype::try_from(OsStr::new("nix")), Ok(Filetype::Nix));
        assert_eq!(Filetype::try_from(OsStr::new("json")), Ok(Filetype::Json))
    }

    #[test]
    fn filetype_path() {
        assert_eq!(Filetype::try_from(Path::new("ok.nix")), Ok(Filetype::Nix));
        assert_eq!(Filetype::try_from(Path::new("ok.json")), Ok(Filetype::Json));
    }
}
