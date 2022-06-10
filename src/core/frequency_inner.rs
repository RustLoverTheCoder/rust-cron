/// 用于执行时间的任务内部控制的迭代器
#[derive(Debug, Clone)]
pub(crate) enum FrequencyInner {
    /// cron-expression 的无限的重复类型
    CronExpressionRepeated(DelayTimerScheduleIteratorOwned),
    /// cron-expression 的倒计时类型。
    CronExpressionCountDown(u64, DelayTimerScheduleIteratorOwned),
    /// 秒 的 无限重复类型
    SecondsRepeated(SecondsState),
    /// 倒计时状态
    SecondsCountDown(u64, SecondsState),
}