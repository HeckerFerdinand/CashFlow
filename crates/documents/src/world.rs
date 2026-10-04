//! A minimal Typst "world": embedded templates, fonts and logo plus the
//! document's data as the virtual file `data.json`. No file system access.

use chrono::{Datelike, NaiveDate};
use std::sync::LazyLock;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime, Duration};
use typst::syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

const COMMON: &str = include_str!("../templates/common.typ");
static FONT_FILES: [&[u8]; 2] = [
    include_bytes!("../../../assets/fonts/Carlito-Regular.ttf"),
    include_bytes!("../../../assets/fonts/Carlito-Bold.ttf"),
];
static LOGO: &[u8] = include_bytes!("../../../assets/logo.png");

static LIBRARY: LazyLock<LazyHash<Library>> = LazyLock::new(|| LazyHash::new(Library::builder().build()));

static FONTS: LazyLock<(LazyHash<FontBook>, Vec<Font>)> = LazyLock::new(|| {
    let fonts: Vec<Font> = FONT_FILES.iter().flat_map(|data| Font::iter(Bytes::new(*data))).collect();
    (LazyHash::new(FontBook::from_fonts(&fonts)), fonts)
});

fn file_id(path: &str) -> FileId {
    RootedPath::new(VirtualRoot::Project, VirtualPath::new(path).expect("valid virtual path")).intern()
}

pub(crate) struct DocumentWorld {
    main: Source,
    common: Source,
    data: Bytes,
    today: NaiveDate,
}

impl DocumentWorld {
    pub(crate) fn new(template: &str, data: Vec<u8>, today: NaiveDate) -> Self {
        Self {
            main: Source::new(file_id("/main.typ"), template.to_string()),
            common: Source::new(file_id("/common.typ"), COMMON.to_string()),
            data: Bytes::new(data),
            today,
        }
    }
}

impl World for DocumentWorld {
    fn library(&self) -> &LazyHash<Library> {
        &LIBRARY
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &FONTS.0
    }

    fn main(&self) -> FileId {
        self.main.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main.id() {
            Ok(self.main.clone())
        } else if id == self.common.id() {
            Ok(self.common.clone())
        } else {
            Err(not_found(id))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        match id.get().vpath().get_without_slash() {
            "data.json" => Ok(self.data.clone()),
            "logo.png" => Ok(Bytes::new(LOGO)),
            "common.typ" => Ok(Bytes::from_string(COMMON)),
            _ => Err(not_found(id)),
        }
    }

    fn font(&self, index: usize) -> Option<Font> {
        FONTS.1.get(index).cloned()
    }

    fn today(&self, _offset: Option<Duration>) -> Option<Datetime> {
        Datetime::from_ymd(self.today.year(), self.today.month() as u8, self.today.day() as u8)
    }
}

fn not_found(id: FileId) -> FileError {
    FileError::NotFound(id.get().vpath().get_with_slash().into())
}
