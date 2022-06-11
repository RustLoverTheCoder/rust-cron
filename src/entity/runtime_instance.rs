use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::runtime::{Runtime, Builder as TokioBuilder};


use crate::entity::runtime_kind::RuntimeKind;


#[derive(Clone, Debug)]
pub(crate) struct RuntimeInstance {
    pub(crate) inner: Option<Arc<Runtime>>,
    pub(crate) kind: RuntimeKind,
}

impl RuntimeInstance {
    fn init_runtime() -> RuntimeInstance {
        let inner = Some(Arc::new(
            Self::tokio_support().expect("初始化tokio运行时失败"),
        ));
        let kind = RuntimeKind::Tokio;
        RuntimeInstance { inner, kind }
    }

    pub(crate) fn tokio_support() -> Option<Runtime> {
        TokioBuilder::new_multi_thread()
            .enable_all()
            .thread_name_fn(|| {
                static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
                let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
                format!("tokio-{}", id)
            })
            .on_thread_start(|| {
                info!("tokio-thread started");
            })
            .build()
            .ok()
    }
}