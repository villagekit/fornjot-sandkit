use anyhow::{bail, Context, Error, Result};
use deno_core::{
    include_js_files, serde_v8, url::Url, v8, Extension, JsRuntime, ModuleId, RuntimeOptions,
};
use fj::{Circle, Difference2d, Shape, Shape2d, Sketch, Sweep};
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

fn into_shape<'a, 'b, 's>(
    scope: &'b mut v8::HandleScope<'s>,
    value: v8::Local<'a, v8::Value>,
) -> Result<Shape, Error> {
    let object =
        v8::Local::<v8::Object>::try_from(value).context("Shape value is not an object")?;
    let type_name = v8::String::new(scope, "type")
        .context("Unable to create JavaScript string \"type\".")?
        .into();
    let type_value = object
        .get(scope, type_name)
        .context("Unable to get \"type\" from shape object")?;
    let type_id: String =
        serde_v8::from_v8(scope, type_value).context("Unable to convert shape type to String")?;

    let shape: Shape = match type_id.as_str() {
        "Difference2d" => into_difference2d(scope, value)?.into(),
        "Sketch" => into_sketch(scope, value)?.into(),
        "Sweep" => into_sweep(scope, value)?.into(),
        _ => {
            bail!("Unknown shape type: {}", type_id);
        }
    };

    Ok(shape)
}

fn into_sweep<'a, 'b, 's>(
    scope: &'b mut v8::HandleScope<'s>,
    value: v8::Local<'a, v8::Value>,
) -> Result<Sweep, Error> {
    let object =
        v8::Local::<v8::Object>::try_from(value).context("Sweep value is not an object")?;
    let shape_name = v8::String::new(scope, "shape")
        .context("Unable to create JavaScript string \"shape\".")?
        .into();
    let shape_value = object
        .get(scope, shape_name)
        .context("Unable to get \"shape\" from Sweep object")?;
    let shape = into_shape2d(scope, shape_value).context("Sweep shape is not a shape")?;
    let path_name = v8::String::new(scope, "path")
        .context("Unable to create JavaScript string \"path\".")?
        .into();
    let path_value = object
        .get(scope, path_name)
        .context("Unable to get \"path\" from Sweep object")?;
    let path = serde_v8::from_v8::<[f64; 3]>(scope, path_value)
        .context("Unable to parse Sweep path value")?;
    let sweep = Sweep::from_path(shape, path);
    Ok(sweep)
}

fn into_shape2d<'a, 'b, 's>(
    scope: &'b mut v8::HandleScope<'s>,
    value: v8::Local<'a, v8::Value>,
) -> Result<Shape2d, Error> {
    let object =
        v8::Local::<v8::Object>::try_from(value).context("Shape2d value is not an object")?;
    let type_name = v8::String::new(scope, "type")
        .context("Unable to create JavaScript string \"type\".")?
        .into();
    let type_value = object
        .get(scope, type_name)
        .context("Unable to get \"type\" from Shape2d object")?;
    let type_id = serde_v8::from_v8::<String>(scope, type_value)
        .context("Unable to convert shape2d type to String")?;

    let shape: Shape2d = match type_id.as_str() {
        "Difference2d" => into_difference2d(scope, value)?.into(),
        "Sketch" => into_sketch(scope, value)?.into(),
        _ => {
            bail!("Unknown shape2d type: {}", type_id);
        }
    };

    Ok(shape)
}

fn into_difference2d<'a, 'b, 's>(
    scope: &'b mut v8::HandleScope<'s>,
    value: v8::Local<'a, v8::Value>,
) -> Result<Difference2d, Error> {
    let object =
        v8::Local::<v8::Object>::try_from(value).context("Difference2d value is not an object")?;
    let shapes_name = v8::String::new(scope, "shapes")
        .context("Unable to create JavaScript string \"shapes\".")?
        .into();
    let shapes_value = object
        .get(scope, shapes_name)
        .context("Unable to get \"shapes\" from Difference2d object")?;
    let shapes_array = v8::Local::<v8::Array>::try_from(shapes_value)
        .context("Difference2d shapes is not an Array")?;
    let shapes_a_value = shapes_array
        .get_index(scope, 0)
        .context("Unable to get shapes[0] value from Difference2d shapes object")?;
    let shapes_a =
        into_shape2d(scope, shapes_a_value).context("Difference2d shapes[0] is not a shape")?;
    let shapes_b_value = shapes_array
        .get_index(scope, 1)
        .context("Unable to get shapes[1] value from Difference2d shapes object")?;
    let shapes_b =
        into_shape2d(scope, shapes_b_value).context("Difference2d shapes[1] is not a shape")?;
    let difference2d = Difference2d::from_shapes([shapes_a.into(), shapes_b.into()]);
    Ok(difference2d)
}

fn into_sketch<'a, 'b, 's>(
    scope: &'b mut v8::HandleScope<'s>,
    value: v8::Local<'a, v8::Value>,
) -> Result<Sketch, Error> {
    let object =
        v8::Local::<v8::Object>::try_from(value).context("Sketch value is not an object")?;
    let chain_name = v8::String::new(scope, "chain")
        .context("Unable to create JavaScript string \"chain\".")?
        .into();
    let chain_value = object
        .get(scope, chain_name)
        .context("Unable to get \"chain\" from Sketch object")?;
    let chain_object =
        v8::Local::<v8::Object>::try_from(chain_value).context("Sketch chain is not an object")?;
    let chain_type_name = v8::String::new(scope, "type")
        .context("Unable to create JavaScript string \"type\".")?
        .into();
    let chain_type_value = chain_object
        .get(scope, chain_type_name)
        .context("Unable to get \"type\" from Sketch chain object")?;
    let chain_type = serde_v8::from_v8::<String>(scope, chain_type_value)
        .context("Unable to convert chain.type to String")?;
    println!("chain type: {}", chain_type);
    let sketch = match chain_type.as_str() {
        "Circle" => {
            let circle = into_circle(scope, chain_value)?;
            Sketch::from_circle(circle)
        }
        _ => {
            bail!("Unknown Chain type: {}", chain_type);
        }
    };
    Ok(sketch)
}

fn into_circle<'a, 'b, 's>(
    scope: &'b mut v8::HandleScope<'s>,
    value: v8::Local<'a, v8::Value>,
) -> Result<Circle, Error> {
    let object =
        v8::Local::<v8::Object>::try_from(value).context("Circle value is not an object")?;
    let radius_name = v8::String::new(scope, "radius")
        .context("Unable to create JavaScript string \"radius\".")?
        .into();
    let radius_value = object
        .get(scope, radius_name)
        .context("Unable to get \"radius\" from Circle object")?;
    let radius = serde_v8::from_v8::<f64>(scope, radius_value)
        .context("Unable to convert radius value to f64")?;
    let circle = Circle::from_radius(radius);
    Ok(circle)
}
