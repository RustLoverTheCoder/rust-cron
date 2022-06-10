use crate::core::task::Task;

#[derive(Debug)]
pub enum TimerEvent {
    /// 停止定时器
    StopTimer,
    /// 新增一个新的任务
    AddTask(Box<Task>),
    /// 插入一个新的任务
    InsertTask(Box<Task>, TaskInstancesChainMaintainer),
    /// 在定时器中更新一个任务
    UpdateTask(Box<Task>),
    /// 在定时器中删除一个任务
    RemoveTask(u64),
    /// 在定时器取消一个 运行中的任务实例
    CancelTask(u64, i64),
    /// 在定时器取消一个 超时的任务实例
    TimeoutTask(u64, i64),
    /// 在定时器完成了一个 运行的任务实例
    FinishTask(FinishTaskBody),
    /// 附加一个正在运行的任务的新实例
    AppendTaskHandle(u64, DelayTaskHandlerBox),
    /// 主动执行一次 任务
    AdvanceTask(u64),
}