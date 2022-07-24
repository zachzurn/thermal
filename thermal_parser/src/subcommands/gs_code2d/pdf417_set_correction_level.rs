use crate::{command::*, context::*};

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let m = *command.data.get(0).unwrap_or(&48u8);
        let n = *command.data.get(1).unwrap_or(&48u8);

        let codeword_count = command.data.len() - 2; //Assuming it's one byte per codeword
        let mut level = m - 48;

        //Ratio based error correction
        if m == 49 {
            let a = codeword_count as f32 * n as f32 * 0.01;
            if a < 4f32 { level = 1 }
            if a < 11f32 { level = 2 }
            if a < 21f32 { level = 3 }
            if a < 46f32 { level = 4 }
            if a < 101f32 { level = 5 }
            if a < 201f32 { level = 6 }
            if a < 401f32 { level = 7 }
            if a > 400f32 { level = 8 }
        }

        context.code2d.pdf417_err_correction = level;
    }
}

pub fn new() -> Command {
    Command::new(
        "PDF417 Set Error Correction Level",
        vec![48, 69],
        CommandType::Context,
        DataType::Subcommand,
        Box::new(Handler),
    )
}
