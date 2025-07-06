mod external;
mod util;

use std::collections::HashMap;

use external::JsInputValue;
use tweak_shader::{RenderContext, TextureDesc, input_type::InputVariant, wgpu};
use util::WgpuContext;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, ImageData};

// This is bad form, but we are in JS land. When in rome.
const DATA_SURFACE_ID: &str = "data-surface-id";

#[wasm_bindgen]
pub struct TweakShader {
    context: RenderContext,
    surface_map: HashMap<String, wgpu::Surface<'static>>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    instance: wgpu::Instance,
}

#[wasm_bindgen]
pub async fn initialize_library() -> WgpuContext {
    util::set_up_wgpu().await
}

#[wasm_bindgen]
impl TweakShader {
    /// Create a new TweakShader instance with a shader source and output format
    #[wasm_bindgen(constructor)]
    pub fn new(shader_source: String, ctx: &WgpuContext) -> Result<TweakShader, JsError> {
        let WgpuContext {
            device,
            queue,
            instance,
            ..
        } = ctx;

        Ok(Self {
            context: RenderContext::new(
                shader_source,
                wgpu::TextureFormat::Rgba8Unorm,
                &device,
                &queue,
            )?,
            device: device.clone(),
            queue: queue.clone(),
            instance: instance.clone(),
            surface_map: Default::default(),
        })
    }

    pub fn update_src(
        &mut self,
        shader_source: &str,
        wgpu_context: &WgpuContext,
    ) -> Result<(), JsError> {
        let mut new_ctx = RenderContext::new(
            shader_source,
            wgpu::TextureFormat::Rgba8Unorm,
            &self.device,
            &self.queue,
        )?;
        // well that's a confusing api...
        self.context
            .copy_resources_into(&mut new_ctx, &wgpu_context.device, &wgpu_context.queue);
        self.context = new_ctx;
        Ok(())
    }

    /// Check if the loaded shader is a compute shader
    #[wasm_bindgen(getter)]
    pub fn is_compute(&self) -> bool {
        self.context.is_compute()
    }

    /// Set the target texture for compute shaders
    pub fn set_compute_target(&mut self, uniform_name: String) -> Result<(), JsError> {
        self.context.set_compute_target(&uniform_name)?;
        Ok(())
    }

    /// Render to a canvas or buffer
    pub fn render(&mut self, target: HtmlCanvasElement) -> Result<(), JsError> {
        let surface = self.retrieve_surface(target)?;

        let tex = surface
            .get_current_texture()
            .map_err(|e| JsError::new(&format!("{e:?}")))?;

        let view = tex.texture.create_view(&wgpu::TextureViewDescriptor {
            label: None,
            format: None,
            dimension: None,
            usage: None,
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });

        let mut command_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.context.render(
            &self.queue,
            &self.device,
            &mut command_encoder,
            Some(view),
            tex.texture.width(),
            tex.texture.height(),
        );

        self.queue.submit([command_encoder.finish()]);
        Ok(())
    }

    fn retrieve_surface(
        &mut self,
        target: HtmlCanvasElement,
    ) -> Result<&wgpu::Surface<'_>, JsError> {
        let surface_id = js_sys::Reflect::get(&target, &DATA_SURFACE_ID.into())
            .map_err(|_| JsError::new("could not retrieve surface ID field"))?;

        if surface_id.is_undefined() {
            let new_surface = util::create_surface(target.clone(), &self.instance, &self.device)?;
            let id = self.surface_map.len().to_string();

            self.surface_map.insert(id.clone(), new_surface);

            js_sys::Reflect::set(
                &target,
                &DATA_SURFACE_ID.into(),
                &JsValue::try_from(id.clone()).unwrap(),
            )
            .expect("Failed to set SurfaceID property");

            let Some(entry) = self.surface_map.get(&id) else {
                return Err(JsError::new("Surface entry not Found."));
            };

            Ok(entry)
        } else {
            let Some(id) = surface_id.as_string() else {
                return Err(JsError::new("Stored surface id was not a string."));
            };

            let Some(entry) = self.surface_map.get(&id) else {
                return Err(JsError::new("Surface entry not Found."));
            };

            Ok(entry)
        }
    }

    /// Load a texture from ImageData or Uint8Array
    pub fn load_texture(&mut self, name: &str, image: ImageData) -> Result<(), JsValue> {
        let desc = TextureDesc {
            width: image.width(),
            height: image.height(),
            stride: None,
            data: &image.data(),
            format: wgpu::TextureFormat::Rgba8Unorm,
        };

        self.context
            .load_texture(name, desc, &self.device, &self.queue);

        Ok(())
    }

    /// Remove a texture by name
    pub fn remove_texture(&mut self, name: &str) -> bool {
        self.context.remove_texture(name)
    }

    /// Get an input value by name
    pub fn get_input(&self, name: &str) -> Result<JsValue, JsError> {
        let Some(val) = self.context.get_input(name) else {
            return Ok(JsValue::null());
        };

        let out = external::JsInputValue::from(val);

        serde_wasm_bindgen::to_value(&out).map_err(|_| JsError::new("Serde Error!"))
    }

    /// Set an input value by name
    pub fn set_input(&mut self, name: &str, value: &JsValue) -> Result<(), JsError> {
        let Some(mut input) = self.context.get_input_mut(name) else {
            return Err(JsError::new("Input not found"));
        };

        // Depending on the type of input, deserialize the value appropriately
        match input.variant() {
            InputVariant::Float => {
                if let Some(float_val) = input.as_float() {
                    float_val.current = serde_wasm_bindgen::from_value(value.clone())?;
                }
            }
            InputVariant::Int => {
                if let Some(int_input) = input.as_int() {
                    int_input.value.current = serde_wasm_bindgen::from_value(value.clone())?;
                }
            }
            InputVariant::Point => {
                if let Some(point_val) = input.as_point() {
                    point_val.current = serde_wasm_bindgen::from_value(value.clone())?;
                }
            }
            InputVariant::Bool => {
                if let Some(bool_val) = input.as_bool() {
                    let js_bool: bool = serde_wasm_bindgen::from_value(value.clone())?;
                    bool_val.current = if js_bool {
                        tweak_shader::input_type::ShaderBool::True
                    } else {
                        tweak_shader::input_type::ShaderBool::False
                    };
                }
            }
            InputVariant::Color => {
                if let Some(color_val) = input.as_color() {
                    color_val.current = serde_wasm_bindgen::from_value(value.clone())?;
                }
            }
            InputVariant::Bytes => {
                let bytes = input.as_unknown_bytes();
                let vec: Vec<u8> = serde_wasm_bindgen::from_value(value.clone())?;
                let copy_len = usize::min(bytes.len(), vec.len());
                bytes[..copy_len].copy_from_slice(&vec[..copy_len]);
            }

            _ => return Err(JsError::new("Unsupported input type for setting value")),
        }

        Ok(())
    }

    /// Check if shader has persistent state
    pub fn is_stateful(&mut self) -> bool {
        self.context.is_stateful()
    }

    /// Get list of all inputs
    pub fn get_input_list(&self) -> js_sys::Map {
        let mut map = js_sys::Map::new();
        self.context.iter_inputs().for_each(|i| {
            let key = JsValue::from_str(&i.0);
            let val = serde_wasm_bindgen::to_value(&JsInputValue::from(i.1)).unwrap();
            map = map.set(&key, &val);
        });
        map
    }

    /// Get list of storage texture targets
    pub fn get_targets(&self) -> Vec<String> {
        self.context.iter_targets().map(|i| i.name.into()).collect()
    }

    /// Set mouse position
    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.context.set_mouse_input([x, y]);
    }

    /// Set mouse down state
    pub fn set_mouse_down(&mut self) {
        self.context.set_mouse_down();
    }

    /// Set mouse up state
    pub fn set_mouse_up(&mut self) {
        self.context.set_mouse_up();
    }

    /// Update time
    pub fn update_time(&mut self, time: f32) {
        self.context.update_time(time);
    }

    /// Update time delta
    pub fn update_delta(&mut self, delta: f32) {
        self.context.update_delta(delta);
    }

    /// Update frame count
    pub fn update_frame_count(&mut self, frame: u32) {
        self.context.update_frame_count(frame);
    }

    /// Update resolution
    pub fn update_resolution(&mut self, width: u32, height: u32) {
        self.context
            .update_resolution([width as f32, height as f32]);
    }

    /// Update date/time
    pub fn update_datetime(&mut self, year: f32, month: f32, day: f32, seconds: f32) {
        self.context.update_datetime([year, month, day, seconds]);
    }
}
