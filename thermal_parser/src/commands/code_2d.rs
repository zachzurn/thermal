//See: https://reference.epson-biz.com/modules/ref_escpos/index.php?content_id=130
use crate::{command::*, constants::*, subcommands};

pub fn new() -> Command {
    Command::new(
      "QR Code",
      vec![GS, '(' as u8, 'k' as u8],
      CommandType::Subcommand,
      DataType::Custom,
      subcommands::new(false, true, subcommands::gs_code2d::all()),
    )
}
