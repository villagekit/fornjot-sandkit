use deno_core::{error::AnyError, JsRuntime, RuntimeOptions};

pub struct Engine {
    js: JsRuntime,
}

impl Engine {
    pub fn new() -> Self {
        let js = JsRuntime::new(RuntimeOptions {
            ..Default::default()
        });
        Self { js }
    }

    pub async fn run(&mut self, code: &str) -> Result<(), AnyError> {
        let _value = self.js.execute_script("", code)?;
        self.js.run_event_loop(false).await?;
        Ok(())
    }
}
