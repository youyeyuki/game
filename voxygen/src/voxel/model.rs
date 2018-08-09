use gfx::{
    self,
    traits::{FactoryExt, Pod},
    IndexBuffer, Slice,
};
use gfx_device_gl;

use consts::{ConstHandle, GlobalConsts};
use renderer::{ColorFormat, DepthFormat, Renderer};
use voxel::{Mesh, Vertex};

type PipelineData = pipeline::Data<gfx_device_gl::Resources>;
type VertexBuffer = gfx::handle::Buffer<gfx_device_gl::Resources, Vertex>;

gfx_defines! {
    constant ModelConsts {
        model_mat: [[f32; 4]; 4] = "model_mat",
    }

    pipeline pipeline {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        model_consts: gfx::ConstantBuffer<ModelConsts> = "model_consts",
        global_consts: gfx::ConstantBuffer<GlobalConsts> = "global_consts",
        out_color: gfx::RenderTarget<ColorFormat> = "target",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

pub struct Model {
    vbuf: VertexBuffer,
    const_handle: ConstHandle<ModelConsts>,
    vert_count: u32,
}

impl Model {
    pub fn new(renderer: &mut Renderer, mesh: &Mesh) -> Model {
        Model {
            vbuf: renderer.factory_mut().create_vertex_buffer(&mesh.vertices()),
            const_handle: ConstHandle::new(renderer),
            vert_count: mesh.vert_count(),
        }
    }

    pub fn const_handle(&self) -> &ConstHandle<ModelConsts> { &self.const_handle }

    pub fn get_pipeline_data(&self, renderer: &mut Renderer, global_consts: &ConstHandle<GlobalConsts>) -> PipelineData {
        PipelineData {
            vbuf: self.vbuf.clone(),
            model_consts: self.const_handle.buffer().clone(),
            global_consts: global_consts.buffer().clone(),
            out_color: renderer.color_view().clone(),
            out_depth: renderer.depth_view().clone(),
        }
    }

    pub fn slice(&self) -> Slice<gfx_device_gl::Resources> {
        Slice::<gfx_device_gl::Resources> {
            start: 0,
            end: self.vert_count,
            base_vertex: 0,
            instances: None,
            buffer: IndexBuffer::Auto,
        }
    }
}
