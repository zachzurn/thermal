extern crate barcoders;

use std::str::from_utf8;

use barcoders::sym::codabar::Codabar;
use barcoders::sym::code128::Code128;
use barcoders::sym::code39::Code39;
use barcoders::sym::code93::Code93;
use barcoders::sym::ean13::EAN13;
use barcoders::sym::ean13::UPCA;
use barcoders::sym::ean8::EAN8;
use barcoders::sym::tf::TF;

use crate::{command::*, constants::*, context::*, graphics::*};


#[derive(Clone)]
enum BarcodeType {
    UpcA,
    UpcE,
    Ean13,
    Ean8,
    Code39,
    Itf,
    Nw7Codabar,
    Code93,
    Code128,
    Gs1128,
    Gs1DatabarOmni,
    Gs1DatabarTruncated,
    Gs1DatabarLimited,
    Gs1DatabarExpanded,
    Code128Auto,
    Unknown,
}

#[derive(Clone)]
enum EncodingFunction {
    NulTerminated,
    ExplicitSize,
    Unknown,
}

#[derive(Clone)]
struct BarcodeHandler {
    kind: BarcodeType,
    kind_id: u8,
    encoding: EncodingFunction,
    capacity: u8,
    has_capacity: bool,
    accept_data: bool,
}

impl CommandHandler for BarcodeHandler {
    fn get_graphics(&self, command: &Command, context: &Context) -> Option<GraphicsCommand> {
        let data = from_utf8(&command.data as &[u8]).unwrap_or("");
        let point_width = context.barcode.width;
        let point_height = context.barcode.height;
        let hri = context.barcode.human_readable.clone();

        match self.kind {
            BarcodeType::Code128 => {
                if let Ok(barcode) = Code128::new(data.to_string()) {
                    //all code128 data has two bytes that set the type, we are converting this to the barcoders format
                    let adjusted_data = data.replace("{A", "À").replace("{B", "Ɓ").replace("{C", "Ć");
                    return Some(GraphicsCommand::Barcode(Barcode { points: barcode.encode(), text: adjusted_data, point_width, point_height, hri }));
                }
            }
            BarcodeType::Nw7Codabar => {
                if let Ok(barcode) = Codabar::new(data.to_string()) {
                    return Some(GraphicsCommand::Barcode(Barcode { points: barcode.encode(), text: data.to_string(), point_width, point_height, hri }));
                }
            }
            BarcodeType::Code39 => {
                if let Ok(barcode) = Code39::new(data.to_string()) {
                    return Some(GraphicsCommand::Barcode(Barcode { points: barcode.encode(), text: data.to_string(), point_width, point_height, hri }));
                }
            }
            BarcodeType::Code93 => {
                if let Ok(barcode) = Code93::new(data.to_string()) {
                    return Some(GraphicsCommand::Barcode(Barcode { points: barcode.encode(), text: data.to_string(), point_width, point_height, hri }));
                }
            }
            BarcodeType::Ean13 => {
                if let Ok(barcode) = EAN13::new(data.to_string()) {
                    return Some(GraphicsCommand::Barcode(Barcode { points: barcode.encode(), text: data.to_string(), point_width, point_height, hri }));
                }
            }
            BarcodeType::UpcA => {
                if let Ok(barcode) = UPCA::new(data.to_string()) {
                    return Some(GraphicsCommand::Barcode(Barcode { points: barcode.encode(), text: data.to_string(), point_width, point_height, hri }));
                }
            }
            BarcodeType::Ean8 => {
                if let Ok(barcode) = EAN8::new(data.to_string()) {
                    return Some(GraphicsCommand::Barcode(Barcode { points: barcode.encode(), text: data.to_string(), point_width, point_height, hri }));
                }
            }
            BarcodeType::Itf => {
                if let Ok(barcode) = TF::interleaved(data.to_string()) {
                    return Some(GraphicsCommand::Barcode(Barcode { points: barcode.encode(), text: data.to_string(), point_width, point_height, hri }));
                }
            }
            _ => return None
        }
        None
    }

    fn debug(&self, command: &Command, _context: &Context) -> String {
        let encoding_str = match self.encoding {
            EncodingFunction::NulTerminated => "Nul Terminated",
            EncodingFunction::ExplicitSize => "Explicit Size",
            EncodingFunction::Unknown => "Unknown",
        };

        let type_str = match self.kind {
            BarcodeType::UpcA => "UPC A",
            BarcodeType::UpcE => "UPC E",
            BarcodeType::Ean13 => "EAN 13",
            BarcodeType::Ean8 => "EAN 8",
            BarcodeType::Code39 => "CODE 39",
            BarcodeType::Itf => "ITF",
            BarcodeType::Nw7Codabar => "Nw7Codabar",
            BarcodeType::Code93 => "Code93",
            BarcodeType::Code128 => "Code128",
            BarcodeType::Gs1128 => "GS1 128",
            BarcodeType::Gs1DatabarOmni => "GS1 Omni",
            BarcodeType::Gs1DatabarTruncated => "GS1 Truncated",
            BarcodeType::Gs1DatabarLimited => "GS1 Kimited",
            BarcodeType::Gs1DatabarExpanded => "GS1 Expanded",
            BarcodeType::Code128Auto => "Code 128 Auto",
            BarcodeType::Unknown => "Unknown",
        };

        if matches!(self.kind, BarcodeType::Unknown) {
            return format!("Unknown Barcode Format with {} encoding and a type id of {} and data {:02X?}", encoding_str, self.kind_id, command.data);
        }
        format!("{} Barcode with {} bytes: {}", type_str, command.data.len(), from_utf8(&command.data as &[u8]).unwrap_or("[No Data]"))
    }

    fn push(&mut self, data: &mut Vec<u8>, byte: u8) -> bool {
        let data_len = data.len();

        //Gather metadata
        if !self.accept_data {
            self.kind_id = byte;
            self.kind = match self.kind_id {
                0 | 65 => BarcodeType::UpcA,
                1 | 66 => BarcodeType::UpcE,
                2 | 67 => BarcodeType::Ean13,
                3 | 68 => BarcodeType::Ean8,
                4 | 69 => BarcodeType::Code39,
                5 | 70 => BarcodeType::Itf,
                6 | 71 => BarcodeType::Nw7Codabar,
                72 => BarcodeType::Code93,
                73 => BarcodeType::Code128,
                80 => BarcodeType::Gs1128,
                81 => BarcodeType::Gs1DatabarOmni,
                82 => BarcodeType::Gs1DatabarTruncated,
                83 => BarcodeType::Gs1DatabarLimited,
                84 => BarcodeType::Gs1DatabarExpanded,
                85 => BarcodeType::Code128Auto,
                _ => BarcodeType::Unknown
            };

            //I'm seeing some conflicting implementations for function definitions
            if byte <= 6 { self.encoding = EncodingFunction::NulTerminated; } else if byte >= 65 && byte <= 79 { self.encoding = EncodingFunction::ExplicitSize; } else { self.encoding = EncodingFunction::Unknown }
            self.accept_data = true;

            return true;
        }

        return match self.encoding {
            EncodingFunction::NulTerminated => {
                if *data.last().unwrap_or(&0x01) == NUL {
                    data.pop();
                    return false;
                }
                data.push(byte);
                true
            }
            EncodingFunction::ExplicitSize => {
                if !self.has_capacity {
                    self.capacity = byte;
                    self.has_capacity = true;
                    return true;
                } else if data_len < self.capacity as usize {
                    data.push(byte);
                    return true;
                }
                false
            }
            EncodingFunction::Unknown => false,
        };
    }
}

pub fn new() -> Command {
    Command::new(
      "Barcode",
      vec![GS, 'k' as u8],
      CommandType::Graphics,
      DataType::Custom,
      Box::new(BarcodeHandler {
            kind: BarcodeType::Unknown,
            kind_id: 0,
            encoding: EncodingFunction::Unknown,
            capacity: 0,
            has_capacity: false,
            accept_data: false,
      }),
    )
}