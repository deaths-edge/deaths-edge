use tracing_subscriber::fmt::{writer::MutexGuardWriter, MakeWriter};

use std::{
    collections::VecDeque,
    io,
    ops::Deref,
    sync::{Arc, Mutex},
};

#[derive(Default)]
pub struct Logs {
    logs: VecDeque<String>,
    capacity: usize,
}

impl Logs {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            logs: VecDeque::with_capacity(capacity),
            capacity,
        }
    }
}

impl Deref for Logs {
    type Target = VecDeque<String>;

    fn deref(&self) -> &Self::Target {
        &self.logs
    }
}

#[derive(Clone)]
pub struct SharedLogs(Arc<Mutex<Logs>>);

impl Deref for SharedLogs {
    type Target = Mutex<Logs>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SharedLogs {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(Mutex::new(Logs::with_capacity(capacity))))
    }
}

impl<'a> MakeWriter<'a> for SharedLogs {
    type Writer = MutexGuardWriter<'a, Logs>;

    fn make_writer(&'a self) -> Self::Writer {
        self.0.make_writer()
    }
}

impl io::Write for Logs {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = buf.len();
        let vec = String::from_utf8(buf.to_vec())
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        if self.logs.len() > self.capacity {
            self.logs.pop_front();
        }
        self.logs.push_back(vec);
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
