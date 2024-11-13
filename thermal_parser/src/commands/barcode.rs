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

use crate::text::TextSpan;
use crate::{command::*, constants::*, context::*, graphics::*};
use crate::utils::barcodes::upce::UPCE;

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

impl BarcodeHandler {
    fn decorate_error(&self, error: String, command: &Command) -> Option<GraphicsCommand> {
        Some(GraphicsCommand::Error(format!(
            "{} {} --> {}",
            self.kind_to_string().to_string(),
            error,
            from_utf8(&command.data as &[u8]).unwrap_or("[Error parsing data as utf8]")
        )))
    }

    fn kind_to_string(&self) -> &str {
        match self.kind {
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
        }
    }
}

impl CommandHandler for BarcodeHandler {
    fn get_graphics(&self, command: &Command, context: &Context) -> Option<GraphicsCommand> {
        let raw_data = &command.data.clone() as &[u8];
        let data = from_utf8(raw_data).unwrap_or("");
        let point_width = context.barcode.width;
        let point_height = context.barcode.height;
        let hri = context.barcode.human_readable.clone();

        match self.kind {
            BarcodeType::Code128 => {
                //all code128 data has two bytes that set the type, we are converting this to the barcoders format
                let adjusted_data = data
                    .replace("{A", "À")
                    .replace("{B", "Ɓ")
                    .replace("{C", "Ć");

                let hri_data: String = data
                    .replace("{A", "")
                    .replace("{B", "")
                    .replace("{C", "");

                return match Code128::new(adjusted_data.to_string()) {

                    Ok(barcode) => Some(GraphicsCommand::Barcode(Barcode {
                        points: barcode.encode(),
                        text: TextSpan::new_for_barcode(hri_data.to_string(), context),
                        point_width,
                        point_height,
                        hri,
                    })),
                    Err(error) => self.decorate_error(error.to_string(), command),
                };
            }
            BarcodeType::Nw7Codabar => {
                return match Codabar::new(data.to_string()) {
                    Ok(barcode) => Some(GraphicsCommand::Barcode(Barcode {
                        points: barcode.encode(),
                        text: TextSpan::new_for_barcode(data.to_string(), context),
                        point_width,
                        point_height,
                        hri,
                    })),
                    Err(error) => self.decorate_error(error.to_string(), command),
                };
            }
            BarcodeType::Code39 => {
                //Data can be surrounded with * or not
                //The data that was provided should be shown as
                //it was provided.
                //Code39 doesn't want asterisks in the data
                let text = data.to_string();
                let data = text.replace("*", "");

                return match Code39::new(data) {
                    Ok(barcode) => Some(GraphicsCommand::Barcode(Barcode {
                        points: barcode.encode(),
                        text: TextSpan::new_for_barcode(text, context),
                        point_width,
                        point_height,
                        hri,
                    })),
                    Err(error) => self.decorate_error(error.to_string(), command),
                };
            }
            BarcodeType::Code93 => {
                return match Code93::new(data.to_string()) {
                    Ok(barcode) => Some(GraphicsCommand::Barcode(Barcode {
                        points: barcode.encode(),
                        text: TextSpan::new_for_barcode(data.to_string(), context),
                        point_width,
                        point_height,
                        hri,
                    })),
                    Err(error) => self.decorate_error(error.to_string(), command),
                };
            }
            BarcodeType::Ean13 => {
                //Prevent a panic and instead return a graphics error
                if data.len() < 12 {
                    return self.decorate_error("Too few digits provided for EAN 13".to_string(), command);     
                }
                
                let data_sp = &data[..12];
                return match EAN13::new(data_sp.to_string()) {
                    Ok(barcode) => Some(GraphicsCommand::Barcode(Barcode {
                        points: barcode.encode(),
                        text: TextSpan::new_for_barcode(data.to_string(), context),
                        point_width,
                        point_height,
                        hri,
                    })),
                    Err(error) => self.decorate_error(error.to_string(), command),
                };
            }
            BarcodeType::UpcA => {
                let mut data_sp = data.to_string();
                let data_len = data.len();

                match data_len {
                    11 => {
                        if let Some(first_char) = data.chars().next() {
                            if first_char != '0' {
                                data_sp = format!("0{}", &data[..11]);
                            }
                        }
                    }
                    12 => {
                        data_sp = format!("0{}", &data[..11]);
                    }
                    _ => {}
                }

                return match UPCA::new(data_sp) {
                    Ok(barcode) => Some(GraphicsCommand::Barcode(Barcode {
                        points: barcode.encode(),
                        text: TextSpan::new_for_barcode(data.to_string(), context),
                        point_width,
                        point_height,
                        hri,
                    })),
                    Err(error) => self.decorate_error(error.to_string(), command),
                };
            }
            BarcodeType::UpcE => {
                return match UPCE::new(data.to_string()) {
                    Ok(barcode) => Some(GraphicsCommand::Barcode(Barcode {
                        points: barcode.encode(),
                        text: TextSpan::new_for_barcode(data.to_string(), context),
                        point_width,
                        point_height,
                        hri,
                    })),
                    Err(error) => self.decorate_error(error.to_string(), command),
                };
            }
            BarcodeType::Ean8 => {
                return match EAN8::new(data.to_string()) {
                    Ok(barcode) => Some(GraphicsCommand::Barcode(Barcode {
                        points: barcode.encode(),
                        text: TextSpan::new_for_barcode(data.to_string(), context),
                        point_width,
                        point_height,
                        hri,
                    })),
                    Err(error) => self.decorate_error(error.to_string(), command),
                };
            }
            BarcodeType::Itf => {
                return match TF::interleaved(data.to_string()) {
                    Ok(barcode) => Some(GraphicsCommand::Barcode(Barcode {
                        points: barcode.encode(),
                        text: TextSpan::new_for_barcode(data.to_string(), context),
                        point_width,
                        point_height,
                        hri,
                    })),
                    Err(error) => self.decorate_error(error.to_string(), command),
                };
            }
            _ => return self.decorate_error("Unknown barcode type".to_string(), command),
        }
    }

    fn debug(&self, command: &Command, _context: &Context) -> String {
        let encoding_str = match self.encoding {
            EncodingFunction::NulTerminated => "Nul Terminated",
            EncodingFunction::ExplicitSize => "Explicit Size",
            EncodingFunction::Unknown => "Unknown",
        };

        if matches!(self.kind, BarcodeType::Unknown) {
            return format!(
                "Unknown Barcode Format with {} encoding and a type id of {} and data {:02X?}",
                encoding_str, self.kind_id, command.data
            );
        }
        format!(
            "{} Barcode with {} bytes: {}",
            self.kind_to_string(),
            command.data.len(),
            from_utf8(&command.data as &[u8]).unwrap_or("[No Data]")
        )
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
                _ => BarcodeType::Unknown,
            };

            //I'm seeing some conflicting implementations for function definitions
            if byte <= 6 {
                self.encoding = EncodingFunction::NulTerminated;
            } else if byte >= 65 && byte <= 79 {
                self.encoding = EncodingFunction::ExplicitSize;
            } else {
                self.encoding = EncodingFunction::Unknown
            }
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
