use thiserror::Error;
use crate::error::cron::FrequencyAnalyzeError;
use tokio::sync::mpsc::error::{TrySendError, TryRecvError};
use tokio::sync::watch::error;
use crate::core::timer_event::TimerEvent;

/// task
#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Cron表达式解析失败")]
    FrequencyAnalyzeError(#[from] FrequencyAnalyzeError),
    #[error("任务发送失败")]
    SendError(#[from] TrySendError<TimerEvent>),
    #[error("获取任务事件失败")]
    GetEventError(#[from] TryRecvError),
}

/// task instance
#[derive(Error, Debug)]
pub enum TaskInstanceError {
    #[error("实例发送失败")]
    SendError(#[from] TrySendError<TimerEvent>),
    #[error("获取实例事件失败")]
    GetEventError(#[from] TryRecvError),
    #[error("取消失败，无法取消")]
    CancelError,
    #[error("取消超时")]
    CancelTimeOutError,
    #[error("缺少事件发送者")]
    MisEventSenderError,
    #[error("实例通道异常")]
    TaskInstanceChannelError(#[from] error::RecvError),
    #[error("到期")]
    ExpiredError,
}