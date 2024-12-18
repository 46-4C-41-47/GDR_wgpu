use image::{DynamicImage, RgbaImage};

pub struct Texture {
  path: String,
  dimension: (u32, u32),
  image_buffer: RgbaImage,
}


impl Texture {
  pub fn new(path: &str) -> Self {
    let diffuse_bytes = include_bytes!("../res/textures/happy-tree.png");
    let diffuse_image: DynamicImage = image::load_from_memory(diffuse_bytes).unwrap();
    let image_buffer: RgbaImage = diffuse_image.to_rgba8();

    use image::GenericImageView;
    let dimensions: (u32, u32) = diffuse_image.dimensions();

    Self {
      path: String::from(path),
      dimension: (dimensions.0, dimensions.1),
      image_buffer,
    }
  }


  pub fn get_path(&self) -> String { self.path.clone() }

  pub fn get_dimension(&self) -> (u32, u32) { self.dimension }

  pub fn get_image_buffer(&self) -> &RgbaImage { &self.image_buffer }
}
