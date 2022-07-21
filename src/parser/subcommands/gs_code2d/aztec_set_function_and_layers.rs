use crate::parser::*;

#[derive(Clone)]
pub struct Handler;

impl CommandHandler for Handler {
    fn apply_context(&self, command: &Command, context: &mut Context) {
        let n1 = *command.data.get(0).unwrap_or(&0u8);
        let n2 = *command.data.get(1).unwrap_or(&0u8);

        let mode = match n1 {
            0 | 48 => 0,
            1 | 49 => 1,
            _ => 0
        };

        let mut layers = 0;
        if n2 > 0 && n2 < 33 { layers = n2 }

        context.code2d.aztec_mode = mode;
        context.code2d.aztec_layers = layers;
    }
}

pub fn new() -> Command {
    Command::new(
        "Aztec Set Function and Layers",
        vec![53, 66],
        CommandType::Subcommand,
        DataType::Subcommand,
        Box::new(Handler),
    )
}