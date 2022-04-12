use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("打开excel文件失败")]
    OpenFail(#[from] calamine::XlsxError),
    #[error("保存word文件失败： `{0}`")]
    SaveFail(String),
}
