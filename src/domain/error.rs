//! 领域错误：工具全链路的错误语义（thiserror 派生）。
//!
//! 基础设施层的外部错误在此转译为领域错误，附上下文。
//! 组合层据此输出「错误：{err}」与退出码 1。

use std::io;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateError {
    #[error(
        "项目名只能包含小写字母、数字、连字符、下划线。首字符须为小写字母。示例：my-cool-app。"
    )]
    InvalidProjectName,

    #[error("不支持的语言：{0}（支持：python | typescript | rust）")]
    UnsupportedLanguage(String),

    #[error("无法定位模板目录 templates/（可执行文件同级与当前目录均未找到）。")]
    TemplatesNotFound,

    #[error("模板目录不存在：{0}")]
    TemplateMissing(PathBuf),

    #[error("目标目录不存在：{0}")]
    TargetMissing(PathBuf),

    #[error("目标目录已存在：{0}")]
    DestinationExists(PathBuf),

    #[error("Python 模板缺少包目录：{0}")]
    PackageDirMissing(PathBuf),

    #[error("{context}：{source}")]
    Io {
        context: String,
        #[source]
        source: io::Error,
    },
}
