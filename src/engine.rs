use anyhow::Error;
use deno_core::{
    error::AnyError as DenoError,
    include_js_files, op, resolve_url,
    v8::{self, HandleScope, Local},
    Extension, FsModuleLoader, JsRuntime, ModuleId, OpState, RuntimeOptions,
};
use nannou::color::PLUM;
use std::rc::Rc;

pub struct Engine {
    js: JsRuntime,
    module_id: Option<ModuleId>,
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

        Self {
            js,
            module_id: None,
        }
    }

    pub async fn compile(&mut self, code: String) -> Result<(), Error> {
        let module_url = resolve_url("file:///main.js").unwrap();
        let module_id = self.js.load_side_module(&module_url, Some(code)).await?;
        let receiver = self.js.mod_evaluate(module_id);
        self.js.run_event_loop(false).await?;
        let _ = receiver.await;

        self.module_id = Some(module_id);

        Ok(())
    }

    pub fn run(&mut self, time: f32) -> Result<(), Error> {
        let module_ns = self
            .js
            .get_module_namespace(self.module_id.unwrap())
            .unwrap();

        let context = self.js.global_context();
        let isolate = self.js.v8_isolate();

        let module_ns = module_ns.open(isolate);
        let mut scope = HandleScope::with_context(isolate, context);

        let main_export_name = v8::String::new(&mut scope, "main").unwrap();
        let main_export = module_ns.get(&mut scope, main_export_name.into()).unwrap();

        let time_js = v8::Number::new(&mut scope, time as f64);
        let this = v8::undefined(&mut scope).into();

        let main_export_function = Local::<v8::Function>::try_from(main_export)?;
        main_export_function
            .call(&mut scope, this, &[time_js.into()])
            .unwrap();

        Ok(())
    }
}

#[op]
fn op_shapes_rect(state: &mut OpState, x: f32, y: f32) -> Result<(), DenoError> {
    let draw = state.borrow::<nannou::Draw>().clone();

    draw.rect().x_y(x, y).w_h(4_f32, 4_f32).color(PLUM);

    Ok(())
}
