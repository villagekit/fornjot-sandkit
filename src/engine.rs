use anyhow::Error;
use deno_core::{
    error::AnyError as DenoError, include_js_files, op, resolve_url as deno_resolve_url,
    Extension as DenoExtension, FsModuleLoader as DenoFsModuleLoader, JsRuntime as DenoRuntime,
    RuntimeOptions as DenoRuntimeOptions,
};
use std::rc::Rc;

pub struct Engine {
    js: DenoRuntime,
}

impl Engine {
    pub fn new() -> Self {
        let js_extension = DenoExtension::builder("runjs")
            .esm(include_js_files!("runtime.js",))
            .ops(vec![op_shapes_rect::decl()])
            .build();

        let mut js = DenoRuntime::new(DenoRuntimeOptions {
            module_loader: Some(Rc::new(DenoFsModuleLoader)),
            extensions: vec![js_extension],
            ..Default::default()
        });

        Self { js }
    }

    pub async fn run(&mut self, code: String, time: f32) -> Result<(), Error> {
        // set current time in JavaScript global context
        let time_script = format!("globalThis.time = {}", time);
        self.js
            .execute_script("[mycelia:time.js]", time_script.as_ref())
            .unwrap();

        let main_mod_url = deno_resolve_url("file:///main.js").unwrap();
        let mod_id = self.js.load_side_module(&main_mod_url, Some(code)).await?;
        let result = self.js.mod_evaluate(mod_id);
        self.js.run_event_loop(false).await?;

        Ok(())
    }
}

#[op]
fn op_shapes_rect(x: f64, y: f64) -> Result<(), DenoError> {
    Ok(())
}
