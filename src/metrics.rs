use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn inc(&mut self, key: impl Into<String>) -> Result<()> {
        self.data
            .write()
            .map_err(|e| anyhow!(e.to_string()))?
            .entry(key.into())
            .and_modify(|v| *v += 1)
            .or_insert(1);
        Ok(())
    }

    pub fn dec(&mut self, key: impl Into<String>) -> Result<()> {
        self.data
            .write()
            .map_err(|e| anyhow!(e.to_string()))?
            .entry(key.into())
            .and_modify(|v| *v -= 1)
            .or_insert(-1);
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        Ok(self
            .data
            .read()
            .map_err(|e| anyhow!(e.to_string()))?
            .clone())
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.data.read().map_err(|_| std::fmt::Error)?;
        for (k, v) in data.iter() {
            writeln!(f, "{}: {}", k, v)?;
        }
        Ok(())
    }
}
