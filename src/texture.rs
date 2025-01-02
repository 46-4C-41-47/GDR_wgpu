use image::{ DynamicImage, ImageReader, RgbaImage };
use wgpu::{BindGroup, BindGroupLayout, Sampler, TextureView};


pub struct Texture {
  path: String,
  dimension: (u32, u32),
  image_buffer: RgbaImage,
  view: TextureView,
  sampler: Sampler,
  bind_group_layout: BindGroupLayout,
  bind_group: BindGroup,
}


impl Texture {
  pub fn new(path: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
    let diffuse_image: DynamicImage = ImageReader::open(path).unwrap().decode().unwrap();
    let image_buffer: RgbaImage = diffuse_image.to_rgba8();

    use image::GenericImageView;
    let dimensions: (u32, u32) = diffuse_image.dimensions();
    let view_and_sampler: (TextureView, Sampler) = Texture::create_and_send_texture_to_gpu(
      dimensions, device, queue, &image_buffer
    );
    let bindgroup_and_layout: (BindGroupLayout, BindGroup) = Texture::create_bind_group(
      device, &view_and_sampler.1, &view_and_sampler.0
    );

    Self {
      path: String::from(path),
      dimension: (dimensions.0, dimensions.1),
      image_buffer,
      view: view_and_sampler.0,
      sampler: view_and_sampler.1,
      bind_group_layout: bindgroup_and_layout.0,
      bind_group: bindgroup_and_layout.1,
    }
  }


  fn create_and_send_texture_to_gpu(
    size: (u32, u32), 
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    image_buffer: &RgbaImage
  ) -> (TextureView, Sampler) {
    let texture_size = wgpu::Extent3d {
      width: size.0,
      height: size.1,
      depth_or_array_layers: 1,
    };
    let texture = device.create_texture(
      &wgpu::TextureDescriptor {
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        label: Some("diffuse_texture"),
        view_formats: &[],
      }
    );
    queue.write_texture(
      wgpu::ImageCopyTexture {
        texture: &texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
      },
      &image_buffer,
      wgpu::ImageDataLayout {
        offset: 0,
        bytes_per_row: Some(size.0 * 4),
        rows_per_image: Some(size.1),
      },
      texture_size
    );
    
    let texture_view: TextureView = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler: Sampler = device.create_sampler(&wgpu::SamplerDescriptor {
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Nearest,
      min_filter: wgpu::FilterMode::Nearest,    
      mipmap_filter: wgpu::FilterMode::Nearest,
      ..Default::default()
    });

    (texture_view, sampler)
  }


  fn create_bind_group(
    device: &wgpu::Device, 
    sampler: &Sampler, 
    texture_view: &TextureView
  ) -> (BindGroupLayout, BindGroup) {
    let texture_bind_group_layout: BindGroupLayout = device.create_bind_group_layout(
  &wgpu::BindGroupLayoutDescriptor {
          entries: &[
            wgpu::BindGroupLayoutEntry {
              binding: 0,
              visibility: wgpu::ShaderStages::FRAGMENT,
              ty: wgpu::BindingType::Texture {
                multisampled: false,
                view_dimension: wgpu::TextureViewDimension::D2,
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
              },
              count: None,
            },
            wgpu::BindGroupLayoutEntry {
              binding: 1,
              visibility: wgpu::ShaderStages::FRAGMENT,
              ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
              count: None,
            },
          ],
          label: Some("texture_bind_group_layout"),
        }
      );
  
      let diffuse_bind_group = device.create_bind_group(
        &wgpu::BindGroupDescriptor {
        layout: &texture_bind_group_layout,
        entries: &[
          wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&texture_view),
          },
          wgpu::BindGroupEntry {
            binding: 1,
            resource: wgpu::BindingResource::Sampler(&sampler),
          }
        ],
        label: Some("diffuse_bind_group"),
      }
    );

    (texture_bind_group_layout, diffuse_bind_group)
  }


  pub fn get_path(&self) -> String { self.path.clone() }

  pub fn get_dimension(&self) -> (u32, u32) { self.dimension }

  pub fn get_bind_group_layout(&self) -> &BindGroupLayout { &self.bind_group_layout }

  pub fn get_bind_group(&self) -> &BindGroup { &self.bind_group }
}
