use anyhow::Result;
use dashmap::DashMap;
use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CmapMetrics {
    data: Arc<DashMap<String, i64>>,
}

impl CmapMetrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&mut self, key: impl Into<String>) -> Result<()> {
        self.data
            .entry(key.into())
            .and_modify(|v| *v += 1)
            .or_insert(1);
        Ok(())
    }

    pub fn dec(&mut self, key: impl Into<String>) -> Result<()> {
        self.data
            .entry(key.into())
            .and_modify(|v| *v -= 1)
            .or_insert(-1);
        Ok(())
    }
}

impl Default for CmapMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for CmapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
