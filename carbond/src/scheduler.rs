use std::{future::Future, sync::Arc, time::Duration};

use async_channel::{unbounded, Receiver, Sender};
use log::{debug, error, info};
use tokio::{
    signal::unix::{signal, SignalKind},
    sync::Mutex,
    task::{self, JoinHandle},
    time,
};

use crate::{
    data::{config::internal::Config, state::State},
    errors::CarbondError,
};

/// Scheduler for the execution of carbond jobs in the background at given intervals.
/// Can be awaited to sleep until an error occurs or an interrupt signal is received.
pub struct Scheduler {
    cfg: Arc<Mutex<Config>>,
    state: Arc<Mutex<State>>,
    handles: Vec<JoinHandle<()>>,
    tx: Sender<CarbondError>,
    rx: Receiver<CarbondError>,
}

impl Scheduler {
    pub fn new(cfg: Config, state: State) -> Self {
        let (tx, rx) = unbounded::<CarbondError>();

        Scheduler {
            cfg: Arc::new(Mutex::new(cfg)),
            state: Arc::new(Mutex::new(state)),
            handles: vec![],
            tx,
            rx,
        }
    }

    /// Calls abort on all of this scheduler's JoinHandles.
    /// Then, checks if all jobs have been canceled successfully.
    #[allow(clippy::unwrap_used)]
    pub async fn shutdown(&mut self) {
        for handle in &self.handles {
            handle.abort();
        }

        for handle in &mut self.handles {
            assert!(handle.await.unwrap_err().is_cancelled());
        }
    }

    /// Schedules a carbond job to run in the background at a specified interval.
    /// Adds error handling to the Future and sends errors through a channel.
    #[allow(clippy::unwrap_used)]
    pub fn schedule_job<F, Fut>(&mut self, name: String, duration: Duration, f: F)
    where
        F: FnOnce(Arc<Mutex<Config>>, Arc<Mutex<State>>) -> Fut + Send + Copy + 'static,
        Fut: Future<Output = Result<(), CarbondError>> + Send,
    {
        let config = self.cfg.clone();
        let state = self.state.clone();
        let job_tx = self.tx.clone();
        debug!("Scheduling task: {name}");
        let job = task::spawn(async move {
            let mut interval = time::interval(duration);
            loop {
                interval.tick().await;
                debug!("Running task: {name}");
                let config = config.clone();
                let state = state.clone();
                if let Err(e) = f(config, state).await {
                    job_tx.send(e).await.unwrap();
                };
            }
        });
        self.handles.push(job);
    }

    /// Waits until service is ended by either an interrupt signal or any carbond service runs into an error.
    #[allow(clippy::unwrap_used)]
    pub async fn await_service_end(&self) -> Result<(), CarbondError> {
        let mut res = Ok(());
        let mut sig = signal(SignalKind::interrupt()).unwrap();
        tokio::select! {
            _ = sig.recv() => {
                info!("Interrupt signal received.");
            },
            e = self.rx.recv() => {
                error!("Received through error channel: {:?}", e);
                res = Err(e.unwrap());
            }
        };
        res
    }
}
