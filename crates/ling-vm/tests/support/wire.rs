use ling_bytecode::{
    BYTECODE_MAGIC, FORMAT_VERSION, HEADER_BYTES, LANGUAGE_VERSION, UNICODE_VERSION,
};

pub fn scalar_artifact(
    zero_divisor: bool,
    take_false_branch: bool,
    invalid_format: bool,
) -> Vec<u8> {
    let mut writer = Writer::new();
    header(&mut writer);

    writer.u32(4);
    let template = if invalid_format {
        "value={} {}"
    } else {
        "value={}"
    };
    for value in ["Main", "main", "src/Main.ling", template] {
        writer.record(|record| record.bytes(value.as_bytes()));
    }

    writer.u32(0);
    writer.u32(1);
    writer.record(|record| {
        record.u32(u32::MAX);
        record.u32(0);
        record.u32(1);
        record.u8(1);
    });

    writer.u32(4);
    for tag in 0..4_u8 {
        writer.record(|record| record.u8(tag));
    }

    writer.u32(6);
    constant_unit(&mut writer);
    constant_bool(&mut writer, true);
    constant_int(&mut writer, 0, &[]);
    constant_int(&mut writer, 1, &[3]);
    constant_int(&mut writer, 1, &[7]);
    writer.record(|record| {
        record.u8(3);
        record.bytes(&[0; 3]);
        record.u32(3);
        record.u32(3);
    });

    writer.u32(1);
    writer.record(|record| {
        record.u32(0);
        record.u32(2);
        record.u64(64);
        record.bytes(&[0; 32]);
    });

    writer.u32(1);
    writer.record(|function| {
        function.u32(0);
        function.u32(1);
        function.registers(&[0]);
        function.u32(0);
        function.u32(1);
        function.u8(1);
        function.u32(26);
        function.u32(4);

        function.record(|block| {
            block.u32(1);
            block.u32(0);
            block.u32(0);
            block.u32(25);
            instruction_const(block, 1, 1);
            instruction_const(block, 2, 4);
            instruction_const(block, 3, if zero_divisor { 2 } else { 3 });
            instruction_const(block, 4, 5);
            instruction_unary(block, 5, 0, 2);
            instruction_unary(block, 6, 1, 3);
            instruction_binary(block, 7, 0, 2, 3);
            instruction_binary(block, 8, 1, 2, 3);
            instruction_binary(block, 9, 2, 2, 3);
            instruction_binary(block, 10, 3, 2, 3);
            instruction_binary(block, 11, 4, 2, 3);
            instruction_compare(block, 12, 0, 1, 1);
            instruction_compare(block, 13, 1, 1, 1);
            instruction_compare(block, 14, 2, 2, 3);
            instruction_compare(block, 15, 3, 2, 3);
            instruction_compare(block, 16, 4, 2, 3);
            instruction_compare(block, 17, 5, 2, 3);
            instruction_compare(block, 18, 6, 2, 3);
            instruction_compare(block, 19, 7, 2, 3);
            instruction_intrinsic(block, 20, 1, &[2, 3]);
            instruction_intrinsic(block, 21, 2, &[2, 3]);
            instruction_intrinsic(block, 22, 0, &[4, 7]);
            instruction_compare(block, 23, 8, 4, 4);
            instruction_compare(block, 24, 9, 4, 22);
            block.record(|instruction| {
                instruction.u8(0x20);
                instruction.bytes(&[0; 3]);
                instruction.u32(25);
                instruction.u32(22);
            });
            block.record(|terminator| {
                terminator.u8(0x81);
                terminator.bytes(&[0; 3]);
                terminator.u32(if take_false_branch { 13 } else { 12 });
                terminator.u32(1);
                terminator.registers(&[]);
                terminator.u32(3);
                terminator.registers(&[]);
            });
        });

        function.record(|block| {
            block.u32(0);
            block.u32(0);
            block.record(|terminator| {
                terminator.u8(0x80);
                terminator.bytes(&[0; 3]);
                terminator.u32(2);
                terminator.registers(&[]);
            });
        });

        for _ in 0..2 {
            function.record(|block| {
                block.u32(0);
                block.u32(0);
                block.record(|terminator| {
                    terminator.u8(0x82);
                    terminator.bytes(&[0; 3]);
                    terminator.u32(25);
                });
            });
        }
    });

    writer.u32(0);
    let locations = (0..=25_u32)
        .map(|ordinal| (0_u32, ordinal))
        .chain([(1, 0), (2, 0), (3, 0)]);
    let locations = locations.collect::<Vec<_>>();
    writer.u32(u32::try_from(locations.len()).expect("source-map count fits u32"));
    for (index, (block, ordinal)) in locations.into_iter().enumerate() {
        writer.record(|record| {
            record.u32(0);
            record.u32(block);
            record.u32(ordinal);
            record.u32(0);
            let start = u64::try_from(index).expect("fixture index fits u64");
            record.u64(start);
            record.u64(start + 1);
            record.u8(0);
            record.bytes(&[0; 7]);
        });
    }

    finish(writer)
}

pub fn recursive_artifact() -> Vec<u8> {
    let mut writer = Writer::new();
    header(&mut writer);

    writer.u32(3);
    for value in ["Main", "main", "src/Main.ling"] {
        writer.record(|record| record.bytes(value.as_bytes()));
    }
    writer.u32(0);
    writer.u32(1);
    writer.record(|record| {
        record.u32(u32::MAX);
        record.u32(0);
        record.u32(0);
    });
    writer.u32(4);
    for tag in 0..4_u8 {
        writer.record(|record| record.u8(tag));
    }
    writer.u32(0);
    writer.u32(1);
    writer.record(|record| {
        record.u32(0);
        record.u32(2);
        record.u64(2);
        record.bytes(&[0; 32]);
    });

    writer.u32(1);
    writer.record(|function| {
        function.u32(0);
        function.u32(1);
        function.registers(&[0]);
        function.u32(0);
        function.u32(0);
        function.u32(2);
        function.u32(1);
        function.record(|block| {
            block.u32(1);
            block.u32(0);
            block.u32(0);
            block.u32(1);
            block.record(|instruction| {
                instruction.u8(0x10);
                instruction.bytes(&[0; 3]);
                instruction.u32(1);
                instruction.u32(0);
                instruction.registers(&[0]);
            });
            block.record(|terminator| {
                terminator.u8(0x82);
                terminator.bytes(&[0; 3]);
                terminator.u32(1);
            });
        });
    });
    writer.u32(0);
    writer.u32(2);
    for ordinal in 0..=1_u32 {
        writer.record(|record| {
            record.u32(0);
            record.u32(0);
            record.u32(ordinal);
            record.u32(0);
            record.u64(u64::from(ordinal));
            record.u64(u64::from(ordinal) + 1);
            record.u8(0);
            record.bytes(&[0; 7]);
        });
    }
    finish(writer)
}

fn header(writer: &mut Writer) {
    writer.bytes(&BYTECODE_MAGIC);
    writer.u32(HEADER_BYTES);
    writer.u16(FORMAT_VERSION.major());
    writer.u16(FORMAT_VERSION.minor());
    writer.u16(LANGUAGE_VERSION.major());
    writer.u16(LANGUAGE_VERSION.minor());
    writer.u16(UNICODE_VERSION.major());
    writer.u16(UNICODE_VERSION.minor());
    writer.u16(UNICODE_VERSION.patch());
    writer.u16(0);
    writer.u32(0);
    writer.u64(0);
}

fn constant_unit(writer: &mut Writer) {
    writer.record(|record| {
        record.u8(0);
        record.bytes(&[0; 3]);
        record.u32(0);
    });
}

fn constant_bool(writer: &mut Writer, value: bool) {
    writer.record(|record| {
        record.u8(1);
        record.bytes(&[0; 3]);
        record.u32(1);
        record.u8(u8::from(value));
    });
}

fn constant_int(writer: &mut Writer, sign: u8, magnitude: &[u8]) {
    writer.record(|record| {
        record.u8(2);
        record.bytes(&[0; 3]);
        record.u32(2);
        record.u8(sign);
        record.bytes(&[0; 3]);
        record.u32(u32::try_from(magnitude.len()).expect("magnitude fits u32"));
        record.bytes(magnitude);
    });
}

fn instruction_const(writer: &mut Writer, destination: u32, constant: u32) {
    writer.record(|instruction| {
        instruction.u8(0x01);
        instruction.bytes(&[0; 3]);
        instruction.u32(destination);
        instruction.u32(constant);
    });
}

fn instruction_unary(writer: &mut Writer, destination: u32, operator: u8, operand: u32) {
    writer.record(|instruction| {
        instruction.u8(0x02);
        instruction.bytes(&[0; 3]);
        instruction.u32(destination);
        instruction.u8(operator);
        instruction.bytes(&[0; 3]);
        instruction.u32(operand);
    });
}

fn instruction_binary(writer: &mut Writer, destination: u32, operator: u8, left: u32, right: u32) {
    writer.record(|instruction| {
        instruction.u8(0x03);
        instruction.bytes(&[0; 3]);
        instruction.u32(destination);
        instruction.u8(operator);
        instruction.bytes(&[0; 3]);
        instruction.u32(left);
        instruction.u32(right);
    });
}

fn instruction_compare(writer: &mut Writer, destination: u32, operator: u8, left: u32, right: u32) {
    writer.record(|instruction| {
        instruction.u8(0x04);
        instruction.bytes(&[0; 3]);
        instruction.u32(destination);
        instruction.u8(operator);
        instruction.bytes(&[0; 3]);
        instruction.u32(left);
        instruction.u32(right);
    });
}

fn instruction_intrinsic(writer: &mut Writer, destination: u32, intrinsic: u8, arguments: &[u32]) {
    writer.record(|instruction| {
        instruction.u8(0x11);
        instruction.bytes(&[0; 3]);
        instruction.u32(destination);
        instruction.u8(intrinsic);
        instruction.bytes(&[0; 3]);
        instruction.registers(arguments);
    });
}

fn finish(mut writer: Writer) -> Vec<u8> {
    let total = u64::try_from(writer.bytes.len()).expect("test artifact length fits u64");
    writer.bytes[32..40].copy_from_slice(&total.to_le_bytes());
    writer.bytes
}

struct Writer {
    bytes: Vec<u8>,
}

impl Writer {
    fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    fn u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    fn u16(&mut self, value: u16) {
        self.bytes(&value.to_le_bytes());
    }

    fn u32(&mut self, value: u32) {
        self.bytes(&value.to_le_bytes());
    }

    fn u64(&mut self, value: u64) {
        self.bytes(&value.to_le_bytes());
    }

    fn bytes(&mut self, value: &[u8]) {
        self.bytes.extend_from_slice(value);
    }

    fn registers(&mut self, values: &[u32]) {
        self.u32(u32::try_from(values.len()).expect("register count fits u32"));
        for value in values {
            self.u32(*value);
        }
    }

    fn record(&mut self, build: impl FnOnce(&mut Self)) {
        let mut payload = Self::new();
        build(&mut payload);
        self.u32(u32::try_from(payload.bytes.len()).expect("record length fits u32"));
        self.bytes.extend_from_slice(&payload.bytes);
    }
}
