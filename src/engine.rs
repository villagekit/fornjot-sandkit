use anyhow::Error;
use deno_core::{
    error::AnyError as DenoError, include_js_files, op, resolve_url, Extension, FsModuleLoader,
    JsRuntime, OpState, RuntimeOptions,
};
use nannou::color::PLUM;
use std::rc::Rc;

pub struct Engine {
    js: JsRuntime,
}

impl Engine {
    pub fn new(draw: nannou::Draw) -> Self {
        let js_extension = Extension::builder("runjs")
            .esm(include_js_files!("runtime.js",))
            .ops(vec![op_shapes_rect::decl()])
            .state(move |state| {
                state.put::<nannou::Draw>(draw.clone());
            })
            .build();

        let js = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(FsModuleLoader)),
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

        let main_mod_url = resolve_url("file:///main.js").unwrap();
        let mod_id = self.js.load_side_module(&main_mod_url, Some(code)).await?;
        let _result = self.js.mod_evaluate(mod_id);
        self.js.run_event_loop(false).await?;

        Ok(())
    }
}

#[op]
fn op_shapes_rect(state: &mut OpState, x: f32, y: f32) -> Result<(), DenoError> {
    let draw = state.borrow::<nannou::Draw>().clone();

    draw.rect().x_y(x, y).w(0.1_f32).color(PLUM);

    Ok(())
}
