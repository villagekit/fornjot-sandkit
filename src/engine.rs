use anyhow::{bail, Context, Error, Result};
use deno_core::{
    error::AnyError as DenoError, include_js_files, op, serde_v8, url::Url, v8, Extension,
    JsRuntime, ModuleId, OpState, RuntimeOptions,
};
use fj::{Circle, Difference2d, Group, Shape, Shape2d, Sketch};
use std::rc::Rc;

use crate::loader::ModuleLoader;

pub struct Engine {
    js: JsRuntime,
    module_id: Option<ModuleId>,
}

impl Engine {
    pub fn new() -> Self {
        let js_extension = Extension::builder("sandkit")
            .esm(include_js_files!("runtime.js",))
            .ops(vec![
                op_sketch_from_circle::decl(),
                op_circle_from_radius::decl(),
            ])
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

    pub async fn get_shape(&mut self) -> Result<Shape, Error> {
        let module_id = self
            .module_id
            .context("Module id not available. Have you called .compile yet?")?;

        let module_ns = self
            .js
            .get_module_namespace(module_id)
            .with_context(|| format!("Module id not loaded: {}", module_id))?;
        let isolate = self.js.v8_isolate();

        let module_ns = module_ns.open(isolate);
        let scope = &mut self.js.handle_scope();
        let result = {
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

            // v8::Global::new(scope, result)
            result
        };

        /*
        let value = self.js.resolve_value(result_global).await?;
        */

        let shape = into_shape(scope, result)?;

        println!("value: {:?}", shape);

        Ok(shape)
    }
}

#[op]
fn op_sketch_from_circle(circle: Circle) -> Sketch {
    Sketch::from_circle(circle)
}

#[op]
fn op_circle_from_radius(radius: f64) -> Circle {
    Circle::from_radius(radius)
}

fn into_shape<'a, 'b, 's>(
    scope: &'b mut v8::HandleScope<'s>,
    value: v8::Local<'a, v8::Value>,
) -> Result<Shape, Error> {
    let result = serde_v8::from_v8::<Group>(scope, value);
    if result.is_ok() {
        return Ok(result.unwrap().into());
    }
    let result = serde_v8::from_v8::<Difference2d>(scope, value);
    if result.is_ok() {
        return Ok(result.unwrap().into());
    }
    let result = serde_v8::from_v8::<Sketch>(scope, value);
    if result.is_ok() {
        return Ok(result.unwrap().into());
    }
    bail!("Unable to convert value into Shape");
}

/*
#[op(fast)]
fn op_shapes_rect(state: &mut OpState, x: f32, y: f32) -> Result<(), DenoError> {
    let draw = state.borrow::<nannou::Draw>().clone();

    draw.rect().x_y(x, y).w_h(4_f32, 4_f32).color(PLUM);

    Ok(())
}
*/
