use thiserror::Error;
use cron_clock::{error as cron_error};


#[derive(Error, Debug)]
pub enum FrequencyAnalyzeError {
    #[error("线程本地存储访问失败")]
    AccessError(#[from] std::thread::AccessError),
    #[error("cron 表达式解析不正确")]
    ParseError(#[from] cron_error::Error),
    #[error("初始化时间错误")]
    InitTimeError,
}

#[derive(Error, Debug)]
pub enum CommandChildError {
    #[error("条件不满足")]
    ConditionError(String),
}