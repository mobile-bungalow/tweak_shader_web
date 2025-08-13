use tweak_shader::wgpu::{self, Device, Instance, Queue, Surface};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::external::log;

#[wasm_bindgen]
pub struct WgpuContext {
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) instance: Instance,
}

pub async fn set_up_wgpu() -> WgpuContext {
    // Check if already initialized - return early if so

    console_error_panic_hook::set_once();

    log("Initializing wgpu");

    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .expect("Failed to find an appropriate adapter");

    log("Adapter initialized.");

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .expect("Failed to create device");

    log("Device and queue initialized.");

    device.on_uncaptured_error(Box::new(|e| match e {
        wgpu::Error::OutOfMemory { .. } => {
            log("GPU Out of Memory error occurred");
        }
        wgpu::Error::Validation {
            description,
            source,
        } => {
            log(&format!("GPU Validation error: {description} : {source}"));
        }
        wgpu::Error::Internal {
            source,
            description,
        } => log(&format!("GPU Internal error: {description} : {source}")),
    }));

    WgpuContext {
        device,
        queue,
        instance,
    }
}

pub fn create_surface<'a>(
    canvas: HtmlCanvasElement,
    instance: &Instance,
    device: &Device,
) -> Result<Surface<'a>, JsError> {
    let width = canvas.width();
    let height = canvas.height();

    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
        .map_err(|e| JsError::new(&format!("Failed to create surface: {:?}", e)))?;

    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::STORAGE_BINDING,
        format: wgpu::TextureFormat::Rgba8Unorm,
        width,
        height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![],
        desired_maximum_frame_latency: 3,
    };

    surface.configure(&device, &surface_config);

    Ok(surface)
}

// Get the current texture from the surface to render to
//let frame = surface
//    .get_current_texture()
//    .map_err(|e| JsValue::from_str(&format!("Failed to get current texture: {:?}", e)))?;

// Create a view for the texture
//let view = frame
//    .texture
//    .create_view(&wgpu::TextureViewDescriptor::default());
