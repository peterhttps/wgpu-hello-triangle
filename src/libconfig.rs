use winit::window::Window;
use winit::dpi::PhysicalSize;

struct State {
  surface: wgpu::Surface,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  size: winit::dpi::PhysicalSize<u32>,
  window: Window,
}

pub impl State {
  pub async fn new(window: Window) -> Self {
    let size = window.inner_size();
    
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      ..Default::default()
    });

    let surface = unsafe { instance.create_surface(&window) }.unwrap();

    let adapter = instance.request_adapter(
      &wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
      }
    ).await.unwrap();

    let (device, queue) = adapter.request_device(
      &wgpu::DeviceDescriptor {
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::default(),
        shader_validation: true,
      },
      None,
    ).await.unwrap();

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
      .compatible_formats
      .iter()
      .copied()
      .filter(|f| f.is_srgb())
      .next()
      .unwrap()
      .unwrap_or(surface_caps.formats[0]);

    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: surface_caps.present_modes[0],
      alpha_mode: surface_caps.alpha_modes[0],
      view_formats: vec![]
    };

    surface.configure(&device, &config);

    Self {
      surface,
      device,
      queue,
      config,
      size,
      window,
    }
  }

  pub fn window(&self) -> &Window {
      &self.window
  }

  fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);
    }
  }

  fn input(&mut self, event: &WindowEvent) -> bool {
      todo!()
  }

  fn update(&mut self) {
      todo!()
  }

  fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
      todo!()
  }
}