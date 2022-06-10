use std::fmt;
use std::fmt::Pointer;
use crate::core::frequency_inner::FrequencyInner;

#[derive(Debug)]
/// 任务结构
pub struct Task {
    /// 任务id
    pub task_id: u64,
    /// 例程
    pub(crate) routine: SafeStructBoxRoutine,
    /// 频率
    frequency: FrequencyInner,
    /// 最长执行时间(可选)
    maximum_running_time: Option<u64>,
    /// 时钟周期
    cylinder_line: u64,
    /// 有效期
    valid: bool,
    /// 最大并行可运行数（可选）。
    pub(crate) maximum_parallel_runnable_num: Option<u64>,
}

pub(crate) struct SafeStructBoxRoutine(pub(crate) SafeBoxRoutine);

impl fmt::Debug for SafeStructBoxRoutine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <&Self as Pointer>::fmt(&self, f)
    }
}