use anyhow::{Context, Error, Result};
use deno_core::{
    error::AnyError as DenoError, include_js_files, op, url::Url, v8, Extension, JsRuntime,
    ModuleId, OpState, RuntimeOptions,
};
use fj::{Circle, Shape, Shape2d, Sketch};
use std::rc::Rc;

use crate::loader::ModuleLoader;

pub struct Engine {
    js: JsRuntime,
    module_id: Option<ModuleId>,
}

impl Engine {
    pub fn new() -> Self {
        let js_extension = Extension::builder("runjs")
            .esm(include_js_files!("runtime.js",))
            // .ops(vec![op_shapes_rect::decl()])
            .state(move |state| {
                // state.put::<nannou::Draw>(draw.clone());
            })
            .build();

        let js = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(ModuleLoader)),
            extensions: vec![js_extension],
            ..Default::default()
        });

        Self {
            js,
            module_id: None,
        }
    }

    pub async fn load(&mut self, module_url: Url) -> Result<(), Error> {
        let module_id = self.js.load_side_module(&module_url, None).await?;
        let receiver = self.js.mod_evaluate(module_id);
        self.js.run_event_loop(false).await?;
        let _ = receiver.await;

        self.module_id = Some(module_id);

        Ok(())
    }

    pub async fn shape(&mut self) -> Result<Shape, Error> {
        let module_id = self
            .module_id
            .context("Module id not available. Have you called .compile yet?")?;

        let module_ns = self
            .js
            .get_module_namespace(module_id)
            .with_context(|| format!("Module id not loaded: {}", module_id))?;
        let isolate = self.js.v8_isolate();

        let module_ns = module_ns.open(isolate);
        let result = {
            let scope = &mut self.js.handle_scope();

            let shape_export_name = v8::String::new(scope, "shape")
                .context("Unable to create JavaScript string \"shape\".")?;
            let shape_export = module_ns
                .get(scope, shape_export_name.into())
                .context("Unable to get export named \"shape\" from module.")?;
            let shape_export_function = v8::Local::<v8::Function>::try_from(shape_export)
                .context("Export named \"shape\" is not a function")?;

            let this = v8::undefined(scope).into();

            let result = shape_export_function
                .call(scope, this, &[])
                .context("Unable to call shape export function")?;

            v8::Global::new(scope, result)
        };

        let _value = self.js.resolve_value(result).await?;

        Ok(Shape::Shape2d(Shape2d::Sketch(Sketch::from_circle(
            Circle::from_radius(10_f64),
        ))))
    }
}

/*
#[op(fast)]
fn op_shapes_rect(state: &mut OpState, x: f32, y: f32) -> Result<(), DenoError> {
    let draw = state.borrow::<nannou::Draw>().clone();

    draw.rect().x_y(x, y).w_h(4_f32, 4_f32).color(PLUM);

    Ok(())
}
*/
