use futures::future::{ExecuteError, Executor, Future};
use futures_util::future::{FutureExt, TryFutureExt};
use std::io;
use tokio_compat::runtime::Builder;

pub struct Runtime {
    rt: tokio_compat::runtime::Runtime,
}

impl Runtime {
    pub fn new() -> io::Result<Self> {
        Ok(Runtime {
            rt: tokio_compat::runtime::Runtime::new()?,
        })
    }

    pub fn single_threaded() -> io::Result<Self> {
        Self::with_thread_count(1)
    }

    pub fn with_thread_count(number: usize) -> io::Result<Self> {
        Ok(Runtime {
            rt: Builder::new().core_threads(number).build()?,
        })
    }

    pub fn spawn<F>(&mut self, future: F) -> &mut Self
    where
        F: Future<Item = (), Error = ()> + Send + 'static,
    {
        self.rt.spawn(future);
        self
    }

    pub fn executor(&self) -> TaskExecutor {
        TaskExecutor {
            inner: self.rt.executor(),
        }
    }

    pub fn block_on<F, R, E>(&mut self, future: F) -> Result<R, E>
    where
        F: Send + 'static + Future<Item = R, Error = E>,
        R: Send + 'static,
        E: Send + 'static,
    {
        self.rt.block_on(future)
    }

    pub fn shutdown_on_idle(self) -> impl Future<Item = (), Error = ()> {
        self.rt.shutdown_on_idle().unit_error().boxed().compat()
    }

    pub fn shutdown_now(self) -> impl Future<Item = (), Error = ()> {
        self.rt.shutdown_now().unit_error().boxed().compat()
    }
}

#[derive(Clone, Debug)]
pub struct TaskExecutor {
    inner: tokio_compat::runtime::TaskExecutor,
}

impl TaskExecutor {
    pub fn spawn(&self, f: impl Future<Item = (), Error = ()> + Send + 'static) {
        self.execute(f).unwrap()
    }
}

impl<F> Executor<F> for TaskExecutor
where
    F: Future<Item = (), Error = ()> + Send + 'static,
{
    fn execute(&self, future: F) -> Result<(), ExecuteError<F>> {
        self.inner.spawn(future);
        Ok(())
    }
}
