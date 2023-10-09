
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Spdifrx",
        extends: None,
        description: Some("Receiver Interface"),
        items: &[
            BlockItem {
                name: "cr",
                description: Some("Control register"),
                array: None,
                byte_offset: 0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "imr",
                description: Some("Interrupt mask register"),
                array: None,
                byte_offset: 4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Imr"),
                }),
            },
            BlockItem {
                name: "sr",
                description: Some("Status register"),
                array: None,
                byte_offset: 8,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Sr"),
                }),
            },
            BlockItem {
                name: "ifcr",
                description: Some("Interrupt Flag Clear register"),
                array: None,
                byte_offset: 12,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ifcr"),
                }),
            },
            BlockItem {
                name: "dr",
                description: Some("Data input register"),
                array: None,
                byte_offset: 16,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Dr"),
                }),
            },
            BlockItem {
                name: "csr",
                description: Some("Channel Status register"),
                array: None,
                byte_offset: 20,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Csr"),
                }),
            },
            BlockItem {
                name: "dir",
                description: Some("Debug Information register"),
                array: None,
                byte_offset: 24,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Dir"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Ifcr",
            extends: None,
            description: Some("Interrupt Flag Clear register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "perrcf",
                    description: Some("Clears the Parity error flag"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovrcf",
                    description: Some("Clears the Overrun error flag"),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sbdcf",
                    description: Some("Clears the Synchronization Block Detected flag"),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syncdcf",
                    description: Some("Clears the Synchronization Done flag"),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dr",
            extends: None,
            description: Some("Data input register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dr",
                    description: Some("Parity Error bit"),
                    bit_offset: 0,
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pe",
                    description: Some("Parity Error bit"),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "v",
                    description: Some("Validity bit"),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "u",
                    description: Some("User bit"),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c",
                    description: Some("Channel Status bit"),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pt",
                    description: Some("Preamble Type"),
                    bit_offset: 28,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("Control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spdifen",
                    description: Some("Peripheral Block Enable"),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxdmaen",
                    description: Some("Receiver DMA ENable for data flow"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxsteo",
                    description: Some("STerEO Mode"),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drfmt",
                    description: Some("RX Data format"),
                    bit_offset: 4,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pmsk",
                    description: Some("Mask Parity error bit"),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vmsk",
                    description: Some("Mask of Validity bit"),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cumsk",
                    description: Some("Mask of channel status and user bits"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ptmsk",
                    description: Some("Mask of Preamble Type bits"),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cbdmaen",
                    description: Some("Control Buffer DMA ENable for control flow"),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chsel",
                    description: Some("Channel Selection"),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nbtr",
                    description: Some("Maximum allowed re-tries during synchronization phase"),
                    bit_offset: 12,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wfa",
                    description: Some("Wait For Activity"),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel",
                    description: Some("input selection"),
                    bit_offset: 16,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some("Status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxne",
                    description: Some("Read data register not empty"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csrne",
                    description: Some("Control Buffer register is not empty"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "perr",
                    description: Some("Parity error"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovr",
                    description: Some("Overrun error"),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sbd",
                    description: Some("Synchronization Block Detected"),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syncd",
                    description: Some("Synchronization Done"),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ferr",
                    description: Some("Framing error"),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "serr",
                    description: Some("Synchronization error"),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terr",
                    description: Some("Time-out error"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "width",
                    description: Some("Duration of 5 symbols counted with SPDIF_CLK"),
                    bit_offset: 16,
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Imr",
            extends: None,
            description: Some("Interrupt mask register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxneie",
                    description: Some("RXNE interrupt enable"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csrneie",
                    description: Some("Control Buffer Ready Interrupt Enable"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "perrie",
                    description: Some("Parity error interrupt enable"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovrie",
                    description: Some("Overrun error Interrupt Enable"),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sblkie",
                    description: Some("Synchronization Block Detected Interrupt Enable"),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syncdie",
                    description: Some("Synchronization Done"),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ifeie",
                    description: Some("Serial Interface Error Interrupt Enable"),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dir",
            extends: None,
            description: Some("Debug Information register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "thi",
                    description: Some("Threshold HIGH"),
                    bit_offset: 0,
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tlo",
                    description: Some("Threshold LOW"),
                    bit_offset: 16,
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some("Channel Status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usr",
                    description: Some("User data information"),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cs",
                    description: Some("Channel A status information"),
                    bit_offset: 16,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sob",
                    description: Some("Start Of Block"),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
