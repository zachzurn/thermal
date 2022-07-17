
//Barcodes have an array of line drawing flags
//That allow us to render them however we want
//text can be used based on the current Context
pub struct Barcode {
  pub points: Vec<u8>,
  pub text: String,
}

pub struct Qrcode {
  pub points: Vec<u8>,
  pub width: u32
}

pub enum PixelType{
  Byte, //1 pixel per byte
  Bit //8 pixels per byte
}

//Images can be marked as storage graphics
//by specifying a storage_id
pub struct Image {
  pub pixels: Vec<u8>,
  pub width: u32,
  pub height: u32,
  pub pixel_type: PixelType,
  pub storage_id: Option<u8>,
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

}

//Images that were added tostorage can be 
//referenced with an ImageRef
pub struct ImageRef {
  pub storage_id: u8
}

pub enum GraphicsCommand {
  Qrcode(Qrcode),
  Barcode(Barcode),
  Image(Image),
  ImageRef(ImageRef)
}