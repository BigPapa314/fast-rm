use anyhow::Result;

pub struct Config {
    path: String,
}

impl Config {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

pub struct FastRm {
    config: Config,
}

impl FastRm {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(self) -> Result<()> {
        tokio::fs::remove_dir_all(self.config.path).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn run() {
        let config = crate::Config::new("D:/Temp/rm");
        let frm = crate::FastRm::new(config);
        assert_eq!(frm.run().await.unwrap(), ());
    }
}
