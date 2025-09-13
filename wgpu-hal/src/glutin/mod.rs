mod adapter;
use adapter::Adapter;

mod command;
use command::{CommandBuffer, CommandEncoder};

mod device;
use device::Device;

mod instance;
use instance::Instance;

mod queue;
use queue::Queue;

mod surface;
use surface::Surface;

mod texture;
use texture::{Texture, TextureView};

#[derive(Clone, Debug)]
pub struct Api;

impl crate::Api for Api {
    type Instance = Instance;
    type Surface = Surface;
    type Adapter = Adapter;
    type Device = Device;

    type Queue = Queue;
    type CommandEncoder = CommandEncoder;
    type CommandBuffer = CommandBuffer;

    type Buffer = Buffer;
    type Texture = Texture;
    type SurfaceTexture = Texture;
    type TextureView = TextureView;
    type Sampler = Sampler;
    type QuerySet = QuerySet;
    type Fence = Fence;
    type PipelineCache = PipelineCache;
    type AccelerationStructure = AccelerationStructure;

    type BindGroupLayout = BindGroupLayout;
    type BindGroup = BindGroup;
    type PipelineLayout = PipelineLayout;
    type ShaderModule = ShaderModule;
    type RenderPipeline = RenderPipeline;
    type ComputePipeline = ComputePipeline;
}

crate::impl_dyn_resource!(
    Adapter,
    AccelerationStructure,
    BindGroup,
    BindGroupLayout,
    Buffer,
    CommandBuffer,
    CommandEncoder,
    ComputePipeline,
    Device,
    Fence,
    Instance,
    PipelineCache,
    PipelineLayout,
    QuerySet,
    Queue,
    RenderPipeline,
    Sampler,
    ShaderModule,
    Texture,
    TextureView,
    Surface
);
#[derive(Debug)]
pub struct Fence;
impl crate::DynFence for Fence {}

#[derive(Debug)]
pub struct Buffer;
impl crate::DynBuffer for Buffer {}

#[derive(Debug)]
pub struct Sampler {}
impl crate::DynSampler for Sampler {}

#[derive(Debug)]
pub struct BindGroupLayout {}
impl crate::DynBindGroupLayout for BindGroupLayout {}

#[derive(Debug)]
pub struct BindGroup {}
impl crate::DynBindGroup for BindGroup {}

#[derive(Debug)]
pub struct RenderPipeline {}
impl crate::DynRenderPipeline for RenderPipeline {}

#[cfg(send_sync)]
unsafe impl Sync for RenderPipeline {}
#[cfg(send_sync)]
unsafe impl Send for RenderPipeline {}

#[derive(Debug)]
pub struct ComputePipeline {}
impl crate::DynComputePipeline for ComputePipeline {}

#[cfg(send_sync)]
unsafe impl Sync for ComputePipeline {}
#[cfg(send_sync)]
unsafe impl Send for ComputePipeline {}

#[derive(Debug)]
pub struct ShaderModule {}
impl crate::DynShaderModule for ShaderModule {}

#[derive(Debug)]
pub struct QuerySet {}
impl crate::DynQuerySet for QuerySet {}

#[derive(Debug)]
pub struct AccelerationStructure;
impl crate::DynAccelerationStructure for AccelerationStructure {}

#[derive(Debug)]
pub struct PipelineCache;
impl crate::DynPipelineCache for PipelineCache {}

#[derive(Debug)]
pub struct PipelineLayout {}
impl crate::DynPipelineLayout for PipelineLayout {}
