#[derive(Clone)]
pub struct Barcode {
  pub points: Vec<u8>,
  pub text: String,
}

#[derive(Clone)]
pub struct Rectangle {
  _width: u32,
  _height: u32 
}

#[derive(Clone)]
pub struct Line {
  _ax: u32,
  _ay: u32,
  _bx: u32,
  _by: u32 
}

#[derive(Clone)]
pub struct Code2D {
  pub points: Vec<u8>,
  pub width: u32
}

#[derive(Clone)]
pub enum PixelType{
  Monochrome(u8), //1 bit per pixel one color, the u8 selects the color (1 - 4)
  MultipleTone(u8, u8), //the first u8 selects the color (1 - 4), second how many colors are in the data
  Unknown
}

#[derive(Clone)]
pub struct Image {
  pub pixels: Vec<u8>,
  pub width: u32,
  pub height: u32,
  pub pixel_type: PixelType,
  pub stretch: (u8, u8),
}

impl Image {

  pub fn as_pbm(&self) -> Vec<u8> {
    let dim = format!("{} {}", self.width, self.height);
    let dimbytes = dim.as_bytes();

    let mut data: Vec<u8> = vec![0x50, 0x34, 0x0A];

    for b in dimbytes { data.push(*b) }

    data.extend(self.pixels.clone());
    data
  }

  pub fn from_raster_data(data: &Vec<u8>) -> Option<Image> {
    if data.len() < 8 { return None };
    
    let a       = *data.get(0).unwrap();
    let bx     = *data.get(1).unwrap();
    let by     = *data.get(2).unwrap();
    let c       = *data.get(3).unwrap();
    let x1      = *data.get(4).unwrap();
    let x2      = *data.get(5).unwrap();
    let y1      = *data.get(6).unwrap();
    let y2      = *data.get(7).unwrap();
    let width  = x1 as u32 + x2 as u32 * 256;
    let height = y1 as u32 + y2 as u32 * 256;

    let pixel_type = match a {
      48 => PixelType::Monochrome(c),
      52 => PixelType::MultipleTone(c, 1),
      _ => PixelType::Unknown
    };

    let stretch = (bx, by);

    let mut pixels = data.clone();
    pixels.drain(0..8);

    Some(Image{ pixels, width, height, pixel_type, stretch })
  }

  pub fn from_raster_data_with_ref(data: &Vec<u8>, storage: ImageRefStorage) -> Option<(ImageRef, Image)> {
    if data.len() < 8 { return None };
    
    let a       = *data.get(0).unwrap();
    let kc1     = *data.get(1).unwrap();
    let kc2     = *data.get(2).unwrap();
    let b       = *data.get(3).unwrap();
    let x1      = *data.get(4).unwrap();
    let x2      = *data.get(5).unwrap();
    let y1      = *data.get(6).unwrap();
    let y2      = *data.get(7).unwrap();
    let width  = x1 as u32 + x2 as u32 * 256;
    let height = y1 as u32 + y2 as u32 * 256;

    let pixel_type = match a {
      48 => PixelType::Monochrome(1),
      52 => PixelType::MultipleTone(1, b),
      _ => PixelType::Unknown
    };

    let stretch = (1, 1);

    let mut pixels = data.clone();
    pixels.drain(0..8);

    Some((ImageRef{ kc1, kc2, storage }, Image{ pixels, width, height, pixel_type, stretch }))
  }

  pub fn from_table_data(_data: &Vec<u8>) -> Option<Image> {
    //TODO
    None
  }

  pub fn from_table_data_with_ref(_data: &Vec<u8>, _storage: ImageRefStorage) -> Option<(ImageRef, Image)> {
    //TODO
    None
  }

}

//Images that were added tostorage can be 
//referenced with an ImageRef
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ImageRef {
  pub kc1: u8,
  pub kc2: u8,
  pub storage: ImageRefStorage
}

impl ImageRef {
  pub fn from_data(data: &Vec<u8>, storage: ImageRefStorage) -> Option<ImageRef> {
    if data.len() < 2 { return None }
    Some(ImageRef{
      kc1: *data.get(0).unwrap(),
      kc2: *data.get(1).unwrap(),
      storage
    })
  }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ImageRefStorage {
  Disc,
  Ram
}

pub enum GraphicsCommand {
  Code2D(Code2D),
  Barcode(Barcode),
  Image(Image),
  Rectangle(Rectangle),
  Line(Line)
}