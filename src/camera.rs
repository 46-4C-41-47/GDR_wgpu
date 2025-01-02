use wgpu::util::DeviceExt;


pub struct Camera {
  bind_group_layout: wgpu::BindGroupLayout,
  bind_group: wgpu::BindGroup,
}


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
  view_proj: [[f32; 4]; 4]
}


impl Camera {
  pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
  );


  pub fn new(device: &wgpu::Device) -> Self {
    let buffer: wgpu::Buffer = Self::create_buffer(device);
    let bind_group_layout: wgpu::BindGroupLayout = Self::create_bind_group_layout(device);
    let bind_group: wgpu::BindGroup = Self::create_bind_group(device, &bind_group_layout, &buffer);
    
    Self {
      bind_group_layout,
      bind_group,
    }
  }


  fn create_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    let camera_uniform: CameraUniform = CameraUniform { view_proj: Self::get_matrix().into() };

    let camera_buffer: wgpu::Buffer = device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[camera_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
      }
    );

    camera_buffer
  }


  fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let bind_group_layout: wgpu::BindGroupLayout = device.create_bind_group_layout(
      &wgpu::BindGroupLayoutDescriptor {
        entries: &[
          wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
              ty: wgpu::BufferBindingType::Uniform,
              has_dynamic_offset: false,
              min_binding_size: None,
            },
            count: None,
          }
        ],
        label: Some("Camera Bind Group Layout"),
      }
    );

    bind_group_layout
  }

  fn create_bind_group(
    device: &wgpu::Device, 
    layout: &wgpu::BindGroupLayout, 
    buffer: &wgpu::Buffer
  ) -> wgpu::BindGroup { 
    let bind_group: wgpu::BindGroup = device.create_bind_group(
      &wgpu::BindGroupDescriptor {
        layout: &layout,
        entries: &[
          wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
          }
        ],
        label: Some("camera_bind_group"),
      }
    );

    bind_group
  }


  pub fn get_matrix() -> cgmath::Matrix4<f32> {
    let view: cgmath::Matrix4<f32> = cgmath::Matrix4::look_at_rh(
      cgmath::Point3::new(0.0, 1.0, 2.0), 
      cgmath::Point3::new(0.0, 0.0, 0.0),
      cgmath::Vector3::unit_y()
    );
    let proj: cgmath::Matrix4<f32> = cgmath::perspective(
      cgmath::Deg(45.0), 
      16.0 / 9.0, 
      0.1, 
      100.0
    );

    return Self::OPENGL_TO_WGPU_MATRIX * proj * view;
  }

  pub fn get_bind_group(&self) -> &wgpu::BindGroup { &self.bind_group }

  pub fn get_bind_group_layout(&self) -> &wgpu::BindGroupLayout { &self.bind_group_layout }
}
