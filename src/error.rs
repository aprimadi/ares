use std::error;
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug, derive_more::From)]
pub enum AError {
    UiError(ui::Error),
    //SceneError(zscene::Error),
    RonDeserializeError {
        error: ron::de::Error,
        path: PathBuf,
    },
    IOError(io::Error),
    MqFileError(mq::file::FileError),
    MqFontError(mq::text::FontError),
}

impl AError {
    pub fn from_ron_de_error(error: ron::de::Error, path: PathBuf) -> Self {
        AError::RonDeserializeError { error, path }
    }
}

impl fmt::Display for AError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AError::UiError(ref e) => write!(f, "ZGUI Error: {}", e),
            //AError::SceneError(ref e) => write!(f, "ZScene Error: {}", e),
            AError::RonDeserializeError { error, path } => {
                let s = path.to_str().unwrap_or("<no path>");
                write!(f, "Can't deserialize '{}': {}", s, error)
            }
            AError::IOError(ref e) => write!(f, "IO Error: {}", e),
            AError::MqFileError(ref e) => write!(f, "Macroquad File error: {}", e),
            AError::MqFontError(ref e) => write!(f, "Macroquad Font error: {}", e),
        }
    }
}

impl error::Error for AError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            AError::UiError(ref e) => Some(e),
            //AError::SceneError(ref e) => Some(e),
            AError::RonDeserializeError { error, .. } => Some(error),
            AError::IOError(ref e) => Some(e),
            AError::MqFileError(ref e) => Some(e),
            AError::MqFontError(ref e) => Some(e),
        }
    }
}

