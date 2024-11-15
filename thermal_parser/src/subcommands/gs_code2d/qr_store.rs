extern crate qr_code;

use crate::context::QrModel::Micro;
use crate::{command::*, context::*, graphics};
use qr_code::{EcLevel, QrCode, Version};

#[derive(Clone)]
pub struct Handler;

//Versions 1 - 40. We are using a lookup instead of
//a calculation since it seems each module jumps up
//in a way that can't be determined
const VERSION_CAPACITIES: [i16; 40] = [
    17, 32, 53, 78, 106, 134, 154, 192, 230, 271,
    321, 367, 425, 458, 520, 586, 644, 718, 792, 858,
    929, 1003, 1091, 1171, 1273, 1367, 1465, 1528, 1628, 1732,
    1840, 1952, 2068, 2188, 2303, 2431, 2563, 2699, 2809, 2953,
];

// Determine a minimum module version given the
// length of bytes. This doesn't account for error
// correction levels
fn minimum_version_for_bytes(byte_len: i16, max_version: i16) -> i16 {
    if max_version > 40 { return 40 };
    
    for v in 0..max_version {
        if VERSION_CAPACITIES[v as usize] >= byte_len {
            return v + 1;
        } 
    }

    max_version
}

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let data = command.data.to_owned();
        
        //Max number of modules per version
        let max_modules: i16 = match &context.code2d.qr_model {
            QrModel::Model1 => 14,
            QrModel::Model2 => 40,
            Micro => 4,
        };
        
        //Minimum number of modules required to encode the binary length
        let min_modules = minimum_version_for_bytes(data.len() as i16, max_modules);
        
        let version = match &context.code2d.qr_model {
            QrModel::Model1 => Version::Normal(min_modules),
            QrModel::Model2 => Version::Normal(min_modules),
            Micro => Version::Micro(min_modules),
        };

        let error_correction = match context.code2d.qr_error_correction {
            QrErrorCorrection::M => EcLevel::M,
            QrErrorCorrection::Q => EcLevel::Q,
            QrErrorCorrection::H => EcLevel::H,
            _ => EcLevel::L,
        };

        let result = QrCode::with_version(data, version, error_correction);

        match result {
            Ok(qr) => {
                let raw = qr.to_vec();
                let mut converted_points = Vec::<u8>::with_capacity(raw.capacity());

                for b in raw {
                    let v = if b { 1 } else { 0 };
                    converted_points.push(v);
                }

                let qrcode = graphics::Code2D {
                    points: converted_points,
                    width: qr.width() as u32,
                    point_width: context.code2d.qr_size as u32,
                    point_height: context.code2d.qr_size as u32,
                };

                context.code2d.symbol_storage = Some(qrcode);
            }
            Err(e) => {
                println!("QR ERROR {} data: {:?}", e, String::from_utf8(command.data.clone()).unwrap_or("".to_string()));
            }
        }
    }
}

pub fn new() -> Command {
    Command::new(
        "QR Store the Code2D data",
        vec![49, 80],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
