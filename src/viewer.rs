use fj_interop::processed_shape::ProcessedShape;
use kiss3d::{
    light::Light,
    nalgebra::{Point3, UnitQuaternion, Vector3},
    resource::Mesh,
    scene::SceneNode,
    window::Window,
};
use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Error;

pub struct Viewer {
    window: Window,
    shape_node: SceneNode,
    rot: UnitQuaternion<f32>,
}

impl Viewer {
    pub fn new(shape: ProcessedShape) -> Result<Self, Error> {
        let mut window = Window::new("Sandkit");

        let shape_mesh = shape_to_mesh(shape);
        let mut shape_node = window.add_mesh(shape_mesh, Vector3::new(0.1, 0.1, 0.1));
        shape_node.set_color(1.0, 0., 0.);

        window.set_light(Light::StickToCamera);

        let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

        Ok(Self {
            window,
            shape_node,
            rot,
        })
    }

    pub fn render(&mut self) -> bool {
        self.window.render()
    }

    pub fn step(&mut self) {
        self.shape_node.prepend_to_local_rotation(&self.rot)
    }
}

fn shape_to_mesh(shape: ProcessedShape) -> Rc<RefCell<Mesh>> {
    let mut coords = Vec::<Point3<f32>>::new();
    let mut faces = Vec::<Point3<u16>>::new();

    shape.mesh.triangles().for_each(|triangle| {
        let mut face_indices = Vec::<u16>::new();
        triangle.inner.points().into_iter().for_each(|point| {
            let coord = Point3::new(point.x.into_f32(), point.y.into_f32(), point.z.into_f32());
            coords.push(coord);
            face_indices.push(coords.len() as u16 - 1);
        });
        let face = Point3::new(face_indices[0], face_indices[1], face_indices[2]);
        faces.push(face);
    });

    let mesh = Mesh::new(coords, faces, None, None, false);
    Rc::new(RefCell::new(mesh))
}
