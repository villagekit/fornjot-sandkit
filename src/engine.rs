use anyhow::Error;
use deno_core::{JsRuntime, RuntimeOptions};

pub struct Engine {
    js: JsRuntime,
}

impl Engine {
    pub fn new() -> Self {
        let mut js = JsRuntime::new(RuntimeOptions {
            ..Default::default()
        });
        js.execute_script("[mycelia:runtime.js]", include_str!("./runtime.js"))
            .unwrap();
        Self { js }
    }

    pub fn run(&mut self, code: &str, time: f32) -> Result<(), Error> {
        // set current time in JavaScript global context
        let time_script = format!("globalThis.time = {}", time);
        self.js
            .execute_script("[mycelia:time.js]", time_script.as_ref())
            .unwrap();

        let _value = self.js.execute_script("", code)?;

        Ok(())
    }
}
