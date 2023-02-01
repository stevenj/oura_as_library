use std::{io::stdout, sync::Arc};

use serde::Deserialize;

use oura::{
    pipelining::{BootstrapResult, SinkProvider, StageReceiver}, utils::Utils
};

use super::run::jsonl_writer_loop;

#[derive(Debug, Deserialize, Clone)]
pub enum Format {
    JSONL,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Output {
    Stdout,
    FileRotate,
}

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    pub format: Option<Format>,
}

/// This is copied from oura `src/utils/mod.rs` better to just use it from there.
/// But I couldn't work out how...
///
/// Most of the stage bootstrapping processes will require a custom config value
/// and a reference to the shared utilities singleton. This is a quality-of-life
/// artifact to wrap other structs (usually configs) and attach the utilities
/// singleton entrypoint.
#[derive(Clone)]
pub struct StdoutUtils<C> {
    pub utils: Arc<Utils>,
    pub inner: C,
}

impl<C> StdoutUtils<C> {
    pub fn new(inner: C, utils: Arc<Utils>) -> Self {
        StdoutUtils { utils, inner }
    }

    /*
    pub fn attach_utils_to<T>(&self, target: T) -> StdoutUtils<T> {
        StdoutUtils {
            inner: target,
            utils: self.utils.clone(),
        }
    }
    */
}


impl SinkProvider for StdoutUtils<Config> {
    fn bootstrap(&self, input: StageReceiver) -> BootstrapResult {
        let format = self.inner.format.as_ref().cloned().unwrap_or(Format::JSONL);

        let mut output = stdout();

        let utils = self.utils.clone();

        let handle = std::thread::spawn(move || match format {
            Format::JSONL => {
                jsonl_writer_loop(input, &mut output, utils).expect("writer sink loop failed")
            }
        });

        Ok(handle)
    }
}
