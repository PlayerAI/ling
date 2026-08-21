use std::collections::BTreeMap;

use ling_bytecode::{
    BYTECODE_MAGIC, FORMAT_VERSION, HEADER_BYTES, LANGUAGE_VERSION, UNICODE_VERSION,
};

pub struct LabeledArtifact {
    pub bytes: Vec<u8>,
    labels: BTreeMap<&'static str, usize>,
}

impl LabeledArtifact {
    pub fn position(&self, label: &'static str) -> usize {
        *self.labels.get(label).expect("fixture label exists")
    }
}

pub fn branch_artifact() -> LabeledArtifact {
    let mut writer = Writer::new();
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

    writer.u32(2);
    writer.record(|record| {
        record.u8(0);
        record.bytes(&[0; 3]);
        record.u32(0);
    });
    writer.record(|record| {
        record.u8(1);
        record.bytes(&[0; 3]);
        record.u32(1);
        record.u8(1);
    });

    writer.u32(1);
    writer.record(|record| {
        record.u32(0);
        record.u32(2);
        record.u64(1);
        record.bytes(&[0; 32]);
    });

    writer.u32(1);
    writer.record(|function| {
        function.u32(0);
        function.u32(1);
        function.u32(1);
        function.u32(0);
        function.u32(0);
        function.u32(0);
        function.u32(5);
        function.u32(4);

        function.record(|block| {
            block.u32(1);
            block.u32(0);
            block.u32(0);
            block.u32(1);
            block.record(|instruction| {
                instruction.u8(0x01);
                instruction.bytes(&[0; 3]);
                instruction.u32(1);
                instruction.u32(1);
            });
            block.record(|terminator| {
                terminator.u8(0x81);
                terminator.bytes(&[0; 3]);
                terminator.u32(1);
                terminator.u32(1);
                terminator.u32(1);
                terminator.mark("branch_true_argument");
                terminator.u32(0);
                terminator.u32(2);
                terminator.u32(1);
                terminator.u32(0);
            });
        });

        function.record(|block| {
            block.u32(1);
            block.mark("block1_parameter_register");
            block.u32(2);
            block.u32(0);
            block.u32(0);
            block.record(|terminator| {
                terminator.u8(0x80);
                terminator.bytes(&[0; 3]);
                terminator.mark("block1_jump_target");
                terminator.u32(3);
                terminator.u32(1);
                terminator.mark("block1_jump_argument");
                terminator.u32(2);
            });
        });

        function.record(|block| {
            block.u32(1);
            block.u32(3);
            block.u32(0);
            block.u32(0);
            block.record(|terminator| {
                terminator.u8(0x80);
                terminator.bytes(&[0; 3]);
                terminator.u32(3);
                terminator.u32(1);
                terminator.u32(3);
            });
        });

        function.record(|block| {
            block.u32(1);
            block.u32(4);
            block.u32(0);
            block.u32(0);
            block.record(|terminator| {
                terminator.u8(0x82);
                terminator.bytes(&[0; 3]);
                terminator.u32(4);
            });
        });
    });

    writer.u32(0);
    writer.u32(5);
    for (block, ordinal) in [(0, 0), (0, 1), (1, 0), (2, 0), (3, 0)] {
        writer.record(|record| {
            record.u32(0);
            record.u32(block);
            record.u32(ordinal);
            record.u32(0);
            record.u64(0);
            record.u64(1);
            record.u8(0);
            record.bytes(&[0; 7]);
        });
    }

    let total = u64::try_from(writer.bytes.len()).expect("test artifact length fits u64");
    writer.bytes[32..40].copy_from_slice(&total.to_le_bytes());
    LabeledArtifact {
        bytes: writer.bytes,
        labels: writer.labels,
    }
}

struct Writer {
    bytes: Vec<u8>,
    labels: BTreeMap<&'static str, usize>,
}

impl Writer {
    fn new() -> Self {
        Self {
            bytes: Vec::new(),
            labels: BTreeMap::new(),
        }
    }

    fn mark(&mut self, label: &'static str) {
        assert!(self.labels.insert(label, self.bytes.len()).is_none());
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

    fn record(&mut self, build: impl FnOnce(&mut Self)) {
        let mut payload = Self::new();
        build(&mut payload);
        self.u32(u32::try_from(payload.bytes.len()).expect("record length fits u32"));
        let base = self.bytes.len();
        self.bytes.extend_from_slice(&payload.bytes);
        for (label, relative) in payload.labels {
            assert!(self.labels.insert(label, base + relative).is_none());
        }
    }
}
