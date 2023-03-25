use anyhow::{bail, Context, Error, Result};
use deno_core::{
    include_js_files, op, serde_v8, url::Url, v8, Extension, JsRuntime, ModuleId, RuntimeOptions,
};
use fj::{Circle, Difference2d, Group, Shape, Sketch, Sweep, Transform};
use std::rc::Rc;

use crate::module::ModuleLoader;

pub struct ModelLoader {
    js: JsRuntime,
    module_id: Option<ModuleId>,
}

impl ModelLoader {
    pub fn new() -> Self {
        let js_extension = Extension::builder("sandkit")
            .esm(include_js_files!("runtime.js",))
            .ops(vec![
                op_sketch_from_circle::decl(),
                op_circle_from_radius::decl(),
                op_difference2d_from_shapes_sketch_sketch::decl(),
                op_difference2d_from_shapes_difference2d_sketch::decl(),
                op_difference2d_from_shapes_sketch_difference2d::decl(),
                op_difference2d_from_shapes_difference2d_difference2d::decl(),
                op_sweep_from_paths_sketch::decl(),
                op_sweep_from_paths_difference2d::decl(),
            ])
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

            let param_values = v8::Object::new(scope);

            let outer_name = v8::String::new(scope, "outer")
                .context("Unable to create JavaScript string \"outer\".")?
                .into();
            let outer_value = v8::Number::new(scope, 1_f64).into();
            param_values.set(scope, outer_name, outer_value);

            let inner_name = v8::String::new(scope, "inner")
                .context("Unable to create JavaScript string \"inner\".")?
                .into();
            let inner_value = v8::Number::new(scope, 0.5_f64).into();
            param_values.set(scope, inner_name, inner_value);

            let height_name = v8::String::new(scope, "height")
                .context("Unable to create JavaScript string \"height\".")?
                .into();
            let height_value = v8::Number::new(scope, 1_f64).into();
            param_values.set(scope, height_name, height_value);

            let result = shape_export_function
                .call(scope, this, &[param_values.into()])
                .context("Unable to call shape export function")?;

            result
        };

        let shape = into_shape(scope, result)?;

        println!("Rust shape: {:?}", shape);

        Ok(shape)
    }
}

#[op]
fn op_circle_from_radius(radius: f64) -> Circle {
    Circle::from_radius(radius)
}

#[op]
fn op_sketch_from_circle(circle: Circle) -> Sketch {
    Sketch::from_circle(circle)
}

#[op]
fn op_difference2d_from_shapes_sketch_sketch(a: Sketch, b: Sketch) -> Difference2d {
    Difference2d::from_shapes([a.into(), b.into()])
}

#[op]
fn op_difference2d_from_shapes_difference2d_sketch(a: Difference2d, b: Sketch) -> Difference2d {
    Difference2d::from_shapes([a.into(), b.into()])
}

#[op]
fn op_difference2d_from_shapes_sketch_difference2d(a: Sketch, b: Difference2d) -> Difference2d {
    Difference2d::from_shapes([a.into(), b.into()])
}

#[op]
fn op_difference2d_from_shapes_difference2d_difference2d(
    a: Difference2d,
    b: Difference2d,
) -> Difference2d {
    Difference2d::from_shapes([a.into(), b.into()])
}

#[op]
fn op_sweep_from_paths_sketch(shape: Sketch, path: [f64; 3]) -> Sweep {
    Sweep::from_path(shape.into(), path)
}

#[op]
fn op_sweep_from_paths_difference2d(shape: Difference2d, path: [f64; 3]) -> Sweep {
    Sweep::from_path(shape.into(), path)
}

fn into_shape<'a, 'b, 's>(
    scope: &'b mut v8::HandleScope<'s>,
    value: v8::Local<'a, v8::Value>,
) -> Result<Shape, Error> {
    let result = serde_v8::from_v8::<Group>(scope, value);
    if result.is_ok() {
        return Ok(result?.into());
    }
    let result = serde_v8::from_v8::<Difference2d>(scope, value);
    if result.is_ok() {
        return Ok(result?.into());
    }
    let result = serde_v8::from_v8::<Sketch>(scope, value);
    if result.is_ok() {
        return Ok(result?.into());
    }
    let result = serde_v8::from_v8::<Sweep>(scope, value);
    if result.is_ok() {
        return Ok(result?.into());
    }
    let result = serde_v8::from_v8::<Transform>(scope, value);
    if result.is_ok() {
        return Ok(result?.into());
    }
    bail!("Unable to convert value into Shape");
}
