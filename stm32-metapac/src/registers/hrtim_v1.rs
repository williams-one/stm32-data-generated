
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Hrtim",
            extends: None,
            description: Some(
                "High Resolution Timer",
            ),
            items: &[
                BlockItem {
                    name: "mcr",
                    description: Some(
                        "Master Timer Control Register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "misr",
                    description: Some(
                        "Master Timer Interrupt Status Register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Misr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "micr",
                    description: Some(
                        "Master Timer Interrupt Clear Register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Micr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdier",
                    description: Some(
                        "Master Timer DMA / Interrupt Enable Register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mdier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mcntr",
                    description: Some(
                        "Master Timer Counter Register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mcntr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mper",
                    description: Some(
                        "Master Timer Period Register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mper",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mrep",
                    description: Some(
                        "Master Timer Repetition Register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mrep",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mcmp",
                    description: Some(
                        "Master Timer Compare X Register",
                    ),
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    8,
                                    12,
                                    16,
                                ],
                            },
                        ),
                    ),
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mcmpx",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tim",
                    description: Some(
                        "High Resolution Timer: Timing Unit",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 128,
                            },
                        ),
                    ),
                    byte_offset: 0x80,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "HrtimTimx",
                        },
                    ),
                },
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "High Resolution Timer: Control Register 1",
                    ),
                    array: None,
                    byte_offset: 0x380,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimCr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "High Resolution Timer: Control Register 2",
                    ),
                    array: None,
                    byte_offset: 0x384,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimCr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr",
                    description: Some(
                        "High Resolution Timer: Interrupt Status Register",
                    ),
                    array: None,
                    byte_offset: 0x388,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimIsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icr",
                    description: Some(
                        "High Resolution Timer: Interrupt Clear Register",
                    ),
                    array: None,
                    byte_offset: 0x38c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimIcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier",
                    description: Some(
                        "High Resolution Timer: Interrupt Enable Register",
                    ),
                    array: None,
                    byte_offset: 0x390,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimIer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "oenr",
                    description: Some(
                        "High Resolution Timer: Output Enable Register",
                    ),
                    array: None,
                    byte_offset: 0x394,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimOenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "odisr",
                    description: Some(
                        "High Resolution Timer: Output Disable Register",
                    ),
                    array: None,
                    byte_offset: 0x398,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimOdisr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "odsr",
                    description: Some(
                        "High Resolution Timer: Output Disable Status Register",
                    ),
                    array: None,
                    byte_offset: 0x39c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimOdsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bmcr",
                    description: Some(
                        "High Resolution Timer: Burst Mode Control Register",
                    ),
                    array: None,
                    byte_offset: 0x3a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimBmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bmtrgr",
                    description: Some(
                        "High Resolution Timer: Burst Mode Trigger Register",
                    ),
                    array: None,
                    byte_offset: 0x3a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimBmtrgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bmcmpr",
                    description: Some(
                        "High Resolution Timer: Burst Mode Compare Register",
                    ),
                    array: None,
                    byte_offset: 0x3a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimBmcmpr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bmper",
                    description: Some(
                        "High Resolution Timer: Burst Mode Period Register",
                    ),
                    array: None,
                    byte_offset: 0x3ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimBmper",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eecr1",
                    description: Some(
                        "High Resolution Timer: External Event Control Register 1",
                    ),
                    array: None,
                    byte_offset: 0x3b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimEecr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eecr2",
                    description: Some(
                        "High Resolution Timer: External Event Control Register 2",
                    ),
                    array: None,
                    byte_offset: 0x3b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimEecr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eecr3",
                    description: Some(
                        "High Resolution Timer: External Event Control Register 3",
                    ),
                    array: None,
                    byte_offset: 0x3b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimEecr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc1r",
                    description: Some(
                        "High Resolution Timer: ADC Trigger [1, 3] Register",
                    ),
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    8,
                                ],
                            },
                        ),
                    ),
                    byte_offset: 0x3bc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimAdc1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc2r",
                    description: Some(
                        "High Resolution Timer: ADC Trigger [2, 4] Register",
                    ),
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    8,
                                ],
                            },
                        ),
                    ),
                    byte_offset: 0x3c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimAdc2r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dllcr",
                    description: Some(
                        "High Resolution Timer: DLL Control Register",
                    ),
                    array: None,
                    byte_offset: 0x3cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimDllcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fltinr1",
                    description: Some(
                        "High Resolution Timer: Fault Input Register 1",
                    ),
                    array: None,
                    byte_offset: 0x3d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimFltinr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fltinr2",
                    description: Some(
                        "High Resolution Timer: Fault Input Register 2",
                    ),
                    array: None,
                    byte_offset: 0x3d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimFltinr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdmupr",
                    description: Some(
                        "High Resolution Timer: Burst DMA Master timer update Register",
                    ),
                    array: None,
                    byte_offset: 0x3d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimBdmupr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdtupr",
                    description: Some(
                        "High Resolution Timer: Burst DMA Timer X update Register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x3dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimBdtupr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdmadr",
                    description: Some(
                        "High Resolution Timer: Burst DMA Data Register",
                    ),
                    array: None,
                    byte_offset: 0x3f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "HrtimBdmadr",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "HrtimTimx",
            extends: None,
            description: Some(
                "High Resolution Timer: Timing Unit",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Timer X Control Register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr",
                    description: Some(
                        "Timer X Interrupt Status Register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxisr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icr",
                    description: Some(
                        "Timer X Interrupt Clear Register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxicr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dier",
                    description: Some(
                        "Timer X DMA / Interrupt Enable Register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxdier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "Timer X Counter Register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxcnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "per",
                    description: Some(
                        "Timer X Period Register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxper",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rep",
                    description: Some(
                        "Timer X Repetition Register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxrep",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmp",
                    description: Some(
                        "Timer X Compare X Register",
                    ),
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    8,
                                    12,
                                    16,
                                ],
                            },
                        ),
                    ),
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxcmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmpc",
                    description: Some(
                        "Timer X Compare X Compound Register",
                    ),
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                ],
                            },
                        ),
                    ),
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxcmpc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpt",
                    description: Some(
                        "Timer X Capture X Register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxcpt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dt",
                    description: Some(
                        "Timer X Deadtime Register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "setr",
                    description: Some(
                        "Timer X Output X Set Register",
                    ),
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    8,
                                ],
                            },
                        ),
                    ),
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxsetr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rstr",
                    description: Some(
                        "Timer X Output X Reset Register",
                    ),
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    8,
                                ],
                            },
                        ),
                    ),
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxrstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eef",
                    description: Some(
                        "Timer X External Event Filtering Register 1",
                    ),
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    4,
                                ],
                            },
                        ),
                    ),
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxeef",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rst",
                    description: Some(
                        "Timer X Reset Register",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxrst",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chp",
                    description: Some(
                        "Timer X Chopper Register",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxchp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "Timer X Capture X Control Register",
                    ),
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    4,
                                ],
                            },
                        ),
                    ),
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "outr",
                    description: Some(
                        "Timer X Output Register",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxoutr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt",
                    description: Some(
                        "Timer X Fault Register",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timxflt",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "HrtimAdc1r",
            extends: None,
            description: Some(
                "High Resolution Timer: ADC Trigger 1 Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adcmc",
                    description: Some(
                        "ADC trigger X on Master Compare Y",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "adcmper",
                    description: Some(
                        "ADC trigger X on Master Period",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adceev",
                    description: Some(
                        "ADC trigger X on External Event Y",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "adctc2",
                    description: Some(
                        "ADC trigger X on Timer Y Compare 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    5,
                                    10,
                                    14,
                                    18,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "adctc3",
                    description: Some(
                        "ADC trigger X on Timer Y Compare 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    5,
                                    10,
                                    14,
                                    18,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "adctc4",
                    description: Some(
                        "ADC trigger X on Timer Y Compare 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    5,
                                    10,
                                    14,
                                    18,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "adctper",
                    description: Some(
                        "ADC trigger X on Timer Y Period",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    5,
                                    10,
                                    14,
                                    18,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "adctrst",
                    description: Some(
                        "ADC trigger X on Timer Y Reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    5,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimAdc2r",
            extends: None,
            description: Some(
                "High Resolution Timer: ADC Trigger 2 Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adcmc",
                    description: Some(
                        "ADC trigger X on Master Compare Y",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "adcmper",
                    description: Some(
                        "ADC trigger X on Master Period",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adceev",
                    description: Some(
                        "ADC trigger X on External Event Y",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "adctc2",
                    description: Some(
                        "ADC trigger X on Timer Y Compare 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    4,
                                    8,
                                    13,
                                    18,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "adctc3",
                    description: Some(
                        "ADC trigger X on Timer Y Compare 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    4,
                                    8,
                                    13,
                                    18,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "adctc4",
                    description: Some(
                        "ADC trigger X on Timer Y Compare 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    4,
                                    8,
                                    13,
                                    18,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "adctper",
                    description: Some(
                        "ADC trigger X on Timer Y Period",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    4,
                                    8,
                                    13,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "adctrst",
                    description: Some(
                        "ADC trigger X on Timer Y Reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    5,
                                    9,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimBdmadr",
            extends: None,
            description: Some(
                "High Resolution Timer:  Burst DMA Data Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bdmadr",
                    description: Some(
                        "Burst DMA Data register",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimBdmupr",
            extends: None,
            description: Some(
                "High Resolution Timer: Burst DMA Master timer update Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mcr",
                    description: Some(
                        "MCR register update enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "micr",
                    description: Some(
                        "MICR register update enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mdier",
                    description: Some(
                        "MDIER register update enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mcnt",
                    description: Some(
                        "MCNT register update enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mper",
                    description: Some(
                        "MPER register update enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mrep",
                    description: Some(
                        "MREP register update enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mcmp",
                    description: Some(
                        "MCMP register X update enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimBdtupr",
            extends: None,
            description: Some(
                "High Resolution Timer: Burst DMA Master timer update Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cr",
                    description: Some(
                        "CR register update enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "icr",
                    description: Some(
                        "ICR register update enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dier",
                    description: Some(
                        "DIER register update enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnt",
                    description: Some(
                        "CNT register update enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "per",
                    description: Some(
                        "PER register update enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rep",
                    description: Some(
                        "REP register update enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmp",
                    description: Some(
                        "CMP register X update enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimBmcmpr",
            extends: None,
            description: Some(
                "High Resolution Timer: Burst Mode Compare Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bmcmp",
                    description: Some(
                        "Burst mode compare value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimBmcr",
            extends: None,
            description: Some(
                "High Resolution Timer: Burst Mode Control Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bme",
                    description: Some(
                        "Burst Mode Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bmom",
                    description: Some(
                        "Burst Mode Operating Mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bmclk",
                    description: Some(
                        "Burst Mode Clock source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bmprsc",
                    description: Some(
                        "Burst Mode Prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bmpren",
                    description: Some(
                        "Burst Mode Preload Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mtbm",
                    description: Some(
                        "Master Timer Burst Mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tbm",
                    description: Some(
                        "Timer X Burst Mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "bmstat",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimBmper",
            extends: None,
            description: Some(
                "High Resolution Timer: Burst Mode Period Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bmper",
                    description: Some(
                        "Burst mode period value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimBmtrgr",
            extends: None,
            description: Some(
                "High Resolution Timer: Burst Mode Trigger Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sw",
                    description: Some(
                        "Software start",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mstrst",
                    description: Some(
                        "Master reset or roll-over",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mstrep",
                    description: Some(
                        "Master repetition",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mstcmp",
                    description: Some(
                        "Master Compare X",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "trst",
                    description: Some(
                        "Timer X reset or roll-over",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    4,
                                    8,
                                    12,
                                    16,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "trep",
                    description: Some(
                        "Timer X repetition",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    4,
                                    8,
                                    12,
                                    16,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tcmp1",
                    description: Some(
                        "Timer X compare 1 event",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    4,
                                    8,
                                    12,
                                    16,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tcmp2",
                    description: Some(
                        "Timer X compare 2 event",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    4,
                                    8,
                                    12,
                                    16,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimCr1",
            extends: None,
            description: Some(
                "High Resolution Timer: Control Register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mudis",
                    description: Some(
                        "Master Update Disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tudis",
                    description: Some(
                        "Timer X Update Disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "adusrc",
                    description: Some(
                        "ADC Trigger X Update Source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimCr2",
            extends: None,
            description: Some(
                "High Resolution Timer: Control Register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mswu",
                    description: Some(
                        "Master Timer Software Update",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tswu",
                    description: Some(
                        "Timer X Software Update",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "mrst",
                    description: Some(
                        "Master Counter Software Reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trst",
                    description: Some(
                        "Timer X Counter Software Reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimDllcr",
            extends: None,
            description: Some(
                "High Resolution Timer: DLL Control Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cal",
                    description: Some(
                        "DLL Calibration Start",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "calen",
                    description: Some(
                        "DLL Calibration Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "calrte",
                    description: Some(
                        "DLL Calibration Rate",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimEecr1",
            extends: None,
            description: Some(
                "High Resolution Timer: External Events Control Register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eesrc",
                    description: Some(
                        "External Event X Source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    6,
                                    12,
                                    18,
                                    24,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "eepol",
                    description: Some(
                        "External Event X Polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    6,
                                    12,
                                    18,
                                    24,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "eesns",
                    description: Some(
                        "External Event X Sensitivity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    6,
                                    12,
                                    18,
                                    24,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "eefast",
                    description: Some(
                        "External Event X Fast Mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    6,
                                    12,
                                    18,
                                    24,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimEecr2",
            extends: None,
            description: Some(
                "High Resolution Timer: External Events Control Register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eesrc",
                    description: Some(
                        "External Event X Source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    6,
                                    12,
                                    18,
                                    24,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "eepol",
                    description: Some(
                        "External Event X Polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    6,
                                    12,
                                    18,
                                    24,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "eesns",
                    description: Some(
                        "External Event X Sensitivity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    6,
                                    12,
                                    18,
                                    24,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimEecr3",
            extends: None,
            description: Some(
                "High Resolution Timer: External Events Control Register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eef",
                    description: Some(
                        "External Event X filter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    6,
                                    12,
                                    18,
                                    24,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "eevsd",
                    description: Some(
                        "External Event Sampling Clock Division",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimFltinr1",
            extends: None,
            description: Some(
                "High Resolution Timer: Fault Input Register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flte",
                    description: Some(
                        "Fault X enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    8,
                                    16,
                                    24,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "fltp",
                    description: Some(
                        "Fault X polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    8,
                                    16,
                                    24,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "fltsrc",
                    description: Some(
                        "Fault X source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    8,
                                    16,
                                    24,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "fltf",
                    description: Some(
                        "Fault X filter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    8,
                                    16,
                                    24,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "fltlck",
                    description: Some(
                        "Fault X Lock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    8,
                                    16,
                                    24,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimFltinr2",
            extends: None,
            description: Some(
                "High Resolution Timer: Fault Input Register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flte",
                    description: Some(
                        "Fault X enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "fltp",
                    description: Some(
                        "Fault X polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "fltsrc",
                    description: Some(
                        "Fault X source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "fltf",
                    description: Some(
                        "Fault X filter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "fltlck",
                    description: Some(
                        "Fault X Lock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "fltsd",
                    description: Some(
                        "Fault Sampling clock division",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimIcr",
            extends: None,
            description: Some(
                "High Resolution Timer: Interrupt Clear Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flt",
                    description: Some(
                        "Fault X Interrupt Flag Clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "sysflt",
                    description: Some(
                        "System Fault Interrupt Flag Clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dllrdy",
                    description: Some(
                        "DLL Ready Interrupt Flag Clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bmper",
                    description: Some(
                        "Burst Mode Period Interrupt Flag Clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimIer",
            extends: None,
            description: Some(
                "High Resolution Timer: Interrupt Enable Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flt",
                    description: Some(
                        "Fault X Interrupt Flag Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "sysflt",
                    description: Some(
                        "System Fault Interrupt Flag Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dllrdy",
                    description: Some(
                        "DLL Ready Interrupt Flag Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bmper",
                    description: Some(
                        "Burst Mode Period Interrupt Flag Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimIsr",
            extends: None,
            description: Some(
                "High Resolution Timer: Interrupt Status Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flt",
                    description: Some(
                        "Fault X Interrupt Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "sysflt",
                    description: Some(
                        "System Fault Interrupt Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dllrdy",
                    description: Some(
                        "DLL Ready Interrupt Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bmper",
                    description: Some(
                        "Burst Mode Period Interrupt Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimOdisr",
            extends: None,
            description: Some(
                "High Resolution Timer: Output Disable Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t1odis",
                    description: Some(
                        "Timer X Output Disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    2,
                                    4,
                                    6,
                                    8,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "t2odis",
                    description: Some(
                        "Timer X Complementary Output Disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    2,
                                    4,
                                    6,
                                    8,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimOdsr",
            extends: None,
            description: Some(
                "High Resolution Timer: Output Disable Status Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t1odis",
                    description: Some(
                        "Timer X Output Disable Status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    2,
                                    4,
                                    6,
                                    8,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "t2odis",
                    description: Some(
                        "Timer X Complementary Output Disable Status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    2,
                                    4,
                                    6,
                                    8,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HrtimOenr",
            extends: None,
            description: Some(
                "High Resolution Timer: Output Enable Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t1oen",
                    description: Some(
                        "Timer X Output Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    2,
                                    4,
                                    6,
                                    8,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "t2oen",
                    description: Some(
                        "Timer X Complementary Output Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    2,
                                    4,
                                    6,
                                    8,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mcmpx",
            extends: None,
            description: Some(
                "Master Timer Compare X Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mcmp",
                    description: Some(
                        "Master Timer Compare X value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mcntr",
            extends: None,
            description: Some(
                "Master Timer Counter Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mcnt",
                    description: Some(
                        "Counter value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mcr",
            extends: None,
            description: Some(
                "Master Timer Control Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ckpsc",
                    description: Some(
                        "HRTIM Master Clock prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cont",
                    description: Some(
                        "Master Continuous mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "retrig",
                    description: Some(
                        "Master Re-triggerable mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "half",
                    description: Some(
                        "Half mode enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syncin",
                    description: Some(
                        "Synchronization input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Syncin",
                    ),
                },
                Field {
                    name: "syncrstm",
                    description: Some(
                        "Synchronization Resets Master",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syncstrtm",
                    description: Some(
                        "Synchronization Starts Master",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syncout",
                    description: Some(
                        "Synchronization output",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Syncout",
                    ),
                },
                Field {
                    name: "syncsrc",
                    description: Some(
                        "Synchronization source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Syncsrc",
                    ),
                },
                Field {
                    name: "mcen",
                    description: Some(
                        "Master Counter enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tcen",
                    description: Some(
                        "Timer X counter enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "dacsync",
                    description: Some(
                        "AC Synchronization",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Dacsync",
                    ),
                },
                Field {
                    name: "preen",
                    description: Some(
                        "Preload enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mrepu",
                    description: Some(
                        "Master Timer Repetition update",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "brstdma",
                    description: Some(
                        "Burst DMA Update",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Brstdma",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Mdier",
            extends: None,
            description: Some(
                "Master Timer DMA / Interrupt Enable Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mcmpie",
                    description: Some(
                        "Master Compare X Interrupt Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "mrepie",
                    description: Some(
                        "Master Repetition Interrupt Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syncie",
                    description: Some(
                        "Sync Input Interrupt Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mupdie",
                    description: Some(
                        "Master Update Interrupt Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mcmpde",
                    description: Some(
                        "Master Compare X DMA request Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "mrepde",
                    description: Some(
                        "Master Repetition DMA request Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syncde",
                    description: Some(
                        "Sync Input DMA request Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mupdde",
                    description: Some(
                        "Master Update DMA request Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Micr",
            extends: None,
            description: Some(
                "Master Timer Interrupt Clear Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mcmpc",
                    description: Some(
                        "Master Compare X Interrupt flag clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "mrepc",
                    description: Some(
                        "Repetition Interrupt flag clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syncc",
                    description: Some(
                        "Sync Input Interrupt flag clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mupdc",
                    description: Some(
                        "Master update Interrupt flag clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Misr",
            extends: None,
            description: Some(
                "Master Timer Interrupt Status Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mcmp",
                    description: Some(
                        "Master Compare X Interrupt Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "mrep",
                    description: Some(
                        "Master Repetition Interrupt Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sync",
                    description: Some(
                        "Sync Input Interrupt Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mupd",
                    description: Some(
                        "Master Update Interrupt Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mper",
            extends: None,
            description: Some(
                "Master Timer Period Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mper",
                    description: Some(
                        "Master Timer Period value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mrep",
            extends: None,
            description: Some(
                "Master Timer Repetition Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mrep",
                    description: Some(
                        "Master Timer Repetition counter value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timxccr",
            extends: None,
            description: Some(
                "Timerx Capture 2 Control Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swcpt",
                    description: Some(
                        "Software Capture",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "updcpt",
                    description: Some(
                        "Update Capture",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exevcpt",
                    description: Some(
                        "External Event X Capture",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 10,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "txset",
                    description: Some(
                        "Timer X output Set",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txrst",
                    description: Some(
                        "Timer X output Reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txcmp",
                    description: Some(
                        "Timer X Compare X",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tyset",
                    description: Some(
                        "Timer Y output Set",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tyrst",
                    description: Some(
                        "Timer Y output Reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tycmp",
                    description: Some(
                        "Timer Y Compare X",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tzset",
                    description: Some(
                        "Timer Z output Set",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tzrst",
                    description: Some(
                        "Timer Z output Reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tzcmp",
                    description: Some(
                        "Timer Z Compare X",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ttset",
                    description: Some(
                        "Timer T output Set",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ttrst",
                    description: Some(
                        "Timer T output Reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ttcmp",
                    description: Some(
                        "Timer T Compare X",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timxchp",
            extends: None,
            description: Some(
                "Timerx Chopper Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "carfrq",
                    description: Some(
                        "Timerx carrier frequency value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cardty",
                    description: Some(
                        "Timerx chopper duty cycle value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "strtpw",
                    description: Some(
                        "Timerx start pulsewidth",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timxcmp",
            extends: None,
            description: Some(
                "Timerx Compare X Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp",
                    description: Some(
                        "Timerx Compare X value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timxcmpc",
            extends: None,
            description: Some(
                "Timerx Compare X Compound Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp",
                    description: Some(
                        "Timerx Compare X value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rep",
                    description: Some(
                        "Timerx Repetition value (aliased from HRTIM_REPx register)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timxcnt",
            extends: None,
            description: Some(
                "Timerx Counter Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "Timerx Counter value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timxcpt",
            extends: None,
            description: Some(
                "Timerx Capture X Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cpt",
                    description: Some(
                        "Timerx Capture X value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timxcr",
            extends: None,
            description: Some(
                "Timerx Control Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ckpsc",
                    description: Some(
                        "HRTIM Timer x Clock prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cont",
                    description: Some(
                        "Continuous mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "retrig",
                    description: Some(
                        "Re-triggerable mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "half",
                    description: Some(
                        "Half mode enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pshpll",
                    description: Some(
                        "Push-Pull mode enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syncrst",
                    description: Some(
                        "Synchronization Resets Timer X",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syncstrt",
                    description: Some(
                        "Synchronization Starts Timer X",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "delcmp2",
                    description: Some(
                        "Delayed CMP2 mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Delcmp",
                    ),
                },
                Field {
                    name: "delcmp4",
                    description: Some(
                        "Delayed CMP4 mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Delcmp",
                    ),
                },
                Field {
                    name: "repu",
                    description: Some(
                        "Timer X Repetition update",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rstu",
                    description: Some(
                        "Timer X reset update",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tau",
                    description: Some(
                        "Timer A update",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tbu",
                    description: Some(
                        "Timer B update",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tcu",
                    description: Some(
                        "Timer C update",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tdu",
                    description: Some(
                        "Timer D update",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "teu",
                    description: Some(
                        "Timer E update",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mstu",
                    description: Some(
                        "Master Timer update",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dacsync",
                    description: Some(
                        "AC Synchronization",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Dacsync",
                    ),
                },
                Field {
                    name: "preen",
                    description: Some(
                        "Preload enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "updgat",
                    description: Some(
                        "Update Gating",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Updgat",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Timxdier",
            extends: None,
            description: Some(
                "Timerx DMA / Interrupt Enable Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmpie",
                    description: Some(
                        "Compare X Interrupt Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "repie",
                    description: Some(
                        "Repetition Interrupt Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "updie",
                    description: Some(
                        "Update Interrupt Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cptie",
                    description: Some(
                        "Capture Interrupt Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "setrie",
                    description: Some(
                        "Output X Set Interrupt Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    2,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rstrie",
                    description: Some(
                        "Output X Reset Interrupt Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    2,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rstie",
                    description: Some(
                        "Reset/roll-over Interrupt Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dlyprtie",
                    description: Some(
                        "Delayed Protection Interrupt Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmpde",
                    description: Some(
                        "Compare X DMA request Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "repde",
                    description: Some(
                        "Repetition DMA request Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "updde",
                    description: Some(
                        "Update DMA request Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cptde",
                    description: Some(
                        "Capture X DMA request Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "setrde",
                    description: Some(
                        "Output X Set DMA request Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    2,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rstrde",
                    description: Some(
                        "Output X Reset DMA request Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    2,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rstde",
                    description: Some(
                        "Reset/roll-over DMA request Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dlyprtde",
                    description: Some(
                        "Delayed Protection DMA request Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timxdt",
            extends: None,
            description: Some(
                "Timerx Deadtime Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtr",
                    description: Some(
                        "Deadtime Rising value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdtr",
                    description: Some(
                        "Sign Deadtime Rising value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sdtr",
                    ),
                },
                Field {
                    name: "dtprsc",
                    description: Some(
                        "Deadtime Prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtrslk",
                    description: Some(
                        "Deadtime Rising Sign Lock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtrlk",
                    description: Some(
                        "Deadtime Rising Lock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtf",
                    description: Some(
                        "Deadtime Falling value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdtf",
                    description: Some(
                        "Sign Deadtime Falling value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sdtf",
                    ),
                },
                Field {
                    name: "dtfslk",
                    description: Some(
                        "Deadtime Falling Sign Lock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtflk",
                    description: Some(
                        "Deadtime Falling Lock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timxeef",
            extends: None,
            description: Some(
                "Timer X External Event Filtering Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltch",
                    description: Some(
                        "External Event X latch",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 6,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "fltr",
                    description: Some(
                        "External Event X filter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 6,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Eefltr",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Timxflt",
            extends: None,
            description: Some(
                "Timerx Fault Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flten",
                    description: Some(
                        "Fault X enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "fltlck",
                    description: Some(
                        "Fault sources Lock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timxicr",
            extends: None,
            description: Some(
                "Timerx Interrupt Clear Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmpc",
                    description: Some(
                        "Compare X Interrupt flag Clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "repc",
                    description: Some(
                        "Repetition Interrupt flag Clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "updc",
                    description: Some(
                        "Update Interrupt flag Clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cptc",
                    description: Some(
                        "Capture X Interrupt flag Clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "setrc",
                    description: Some(
                        "Output X Set flag Clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    2,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rstrc",
                    description: Some(
                        "Output X Reset flag Clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    2,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rstc",
                    description: Some(
                        "Reset Interrupt flag Clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dlyprtc",
                    description: Some(
                        "Delayed Protection Flag Clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timxisr",
            extends: None,
            description: Some(
                "Timerx Interrupt Status Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp",
                    description: Some(
                        "Compare X Interrupt Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rep",
                    description: Some(
                        "Repetition Interrupt Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "upd",
                    description: Some(
                        "Update Interrupt Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cpt",
                    description: Some(
                        "Capture X Interrupt Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "setr",
                    description: Some(
                        "Output X Set Interrupt Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    2,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rstr",
                    description: Some(
                        "Output X Reset Interrupt Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    2,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rst",
                    description: Some(
                        "Reset Interrupt Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dlyprt",
                    description: Some(
                        "Delayed Protection Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cppstat",
                    description: Some(
                        "Current Push Pull Status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cppstat",
                    ),
                },
                Field {
                    name: "ippstat",
                    description: Some(
                        "Idle Push Pull Status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ippstat",
                    ),
                },
                Field {
                    name: "ostat",
                    description: Some(
                        "Output X State",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ocpy",
                    description: Some(
                        "Output X Copy",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timxoutr",
            extends: None,
            description: Some(
                "Timerx Output Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pol",
                    description: Some(
                        "Output 1 polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    16,
                                ],
                            },
                        ),
                    ),
                    enumm: Some(
                        "Pol",
                    ),
                },
                Field {
                    name: "idlem",
                    description: Some(
                        "Output X Idle mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    16,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "idles",
                    description: Some(
                        "Output X Idle State",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    16,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "fault",
                    description: Some(
                        "Output X Fault state",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    16,
                                ],
                            },
                        ),
                    ),
                    enumm: Some(
                        "Fault",
                    ),
                },
                Field {
                    name: "chp",
                    description: Some(
                        "Output X Chopper enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    16,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "didl",
                    description: Some(
                        "Output X Deadtime upon burst mode Idle entry",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    16,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "dten",
                    description: Some(
                        "Deadtime enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dlyprten",
                    description: Some(
                        "Delayed Protection Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dlyprt",
                    description: Some(
                        "Delayed Protection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Dlyprt",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Timxper",
            extends: None,
            description: Some(
                "Timerx Period Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "per",
                    description: Some(
                        "Timerx Period value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timxrep",
            extends: None,
            description: Some(
                "Timerx Repetition Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rep",
                    description: Some(
                        "Timerx Repetition counter value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timxrst",
            extends: None,
            description: Some(
                "Timerx Reset Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "updt",
                    description: Some(
                        "Timer X Update reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmp",
                    description: Some(
                        "Timer X compare X reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "mstper",
                    description: Some(
                        "Master timer Period",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mstcmp",
                    description: Some(
                        "Master compare X",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "extevnt",
                    description: Some(
                        "External Event X",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 10,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tcmp1",
                    description: Some(
                        "Timer X compare 1 event",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    3,
                                    6,
                                    9,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tcmp2",
                    description: Some(
                        "Timer X compare 2 event",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    3,
                                    6,
                                    9,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tcmp4",
                    description: Some(
                        "Timer X compare 4 event",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    3,
                                    6,
                                    9,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timxrstr",
            extends: None,
            description: Some(
                "Timerx OutputX Reset Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srt",
                    description: Some(
                        "Software Reset trigger",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "resync",
                    description: Some(
                        "Timer X resynchronizaton",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "per",
                    description: Some(
                        "Timer X Period",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmp",
                    description: Some(
                        "Timer X compare X",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "mstper",
                    description: Some(
                        "Master Period",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mstcmp",
                    description: Some(
                        "Master Compare X",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "timevnt",
                    description: Some(
                        "Timer Event X",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 9,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "extevnt",
                    description: Some(
                        "External Event X",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 10,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "update",
                    description: Some(
                        "Registers update (transfer preload to active)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timxsetr",
            extends: None,
            description: Some(
                "Timerx OutputX Set Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sst",
                    description: Some(
                        "Software Set trigger",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "resync",
                    description: Some(
                        "Timer X resynchronizaton",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "per",
                    description: Some(
                        "Timer X Period",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmp",
                    description: Some(
                        "Timer X compare X",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "mstper",
                    description: Some(
                        "Master Period",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mstcmpx",
                    description: Some(
                        "Master Compare X",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "timevnt",
                    description: Some(
                        "Timer Event X",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 9,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "extevnt",
                    description: Some(
                        "External Event X",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 10,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "update",
                    description: Some(
                        "Registers update (transfer preload to active)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Brstdma",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "INDEPENDENT",
                    description: Some(
                        "Update done independently from the DMA burst transfer completion",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "COMPLETION",
                    description: Some(
                        "Update done when the DMA burst transfer is completed",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ROLLOVER",
                    description: Some(
                        "Update done on master timer roll-over following a DMA burst transfer completion",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Cppstat",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "OUTPUT1ACTIVE",
                    description: Some(
                        "Signal applied on output 1 and output 2 forced inactive",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OUTPUT2ACTIVE",
                    description: Some(
                        "Signal applied on output 2 and output 1 forced inactive",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dacsync",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "No DAC trigger generated",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DACSYNC1",
                    description: Some(
                        "Trigger generated on DACSync1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DACSYNC2",
                    description: Some(
                        "Trigger generated on DACSync2",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DACSYNC3",
                    description: Some(
                        "Trigger generated on DACSync3",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Delcmp",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "STANDARD",
                    description: Some(
                        "CMP register is always active (standard compare mode)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CAPTURE1",
                    description: Some(
                        "CMP is recomputed and is active following a capture 1 event",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CAPTUREX_COMPARE1",
                    description: Some(
                        "CMP is recomputed and is active following a capture 1 event or a Compare 1 match",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CAPTUREX_COMPARE3",
                    description: Some(
                        "CMP is recomputed and is active following a capture 1 event or a Compare 3 match",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Dlyprt",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "OUTPUT1_EE6",
                    description: Some(
                        "Output 1 delayed idle on external event 6",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OUTPUT2_EE6",
                    description: Some(
                        "Output 2 delayed idle on external event 6",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "OUTPUT1_2_EE6",
                    description: Some(
                        "Output 1 and 2 delayed idle on external event 6",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BALANCED_EE6",
                    description: Some(
                        "Balanced idle on external event 6",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "OUTPUT1_EE7",
                    description: Some(
                        "Output 1 delayed idle on external event 7",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "OUTPUT2_EE7",
                    description: Some(
                        "Output 2 delayed idle on external event 7",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "OUTPUT1_2_EE7",
                    description: Some(
                        "Output 1 and 2 delayed idle on external event 7",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "BALANCED_EE7",
                    description: Some(
                        "Balanced idle on external event 7",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Eefltr",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "No filtering",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BLANKRESETTOCOMPARE1",
                    description: Some(
                        "Blanking from counter reset/roll-over to Compare 1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BLANKRESETTOCOMPARE2",
                    description: Some(
                        "Blanking from counter reset/roll-over to Compare 2",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BLANKRESETTOCOMPARE3",
                    description: Some(
                        "Blanking from counter reset/roll-over to Compare 3",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "BLANKRESETTOCOMPARE4",
                    description: Some(
                        "Blanking from counter reset/roll-over to Compare 4",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "BLANKTIMFLTR1",
                    description: Some(
                        "Blanking from another timing unit: TIMFLTR1 source",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "BLANKTIMFLTR2",
                    description: Some(
                        "Blanking from another timing unit: TIMFLTR2 source",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "BLANKTIMFLTR3",
                    description: Some(
                        "Blanking from another timing unit: TIMFLTR3 source",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "BLANKTIMFLTR4",
                    description: Some(
                        "Blanking from another timing unit: TIMFLTR4 source",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "BLANKTIMFLTR5",
                    description: Some(
                        "Blanking from another timing unit: TIMFLTR5 source",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "BLANKTIMFLTR6",
                    description: Some(
                        "Blanking from another timing unit: TIMFLTR6 source",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "BLANKTIMFLTR7",
                    description: Some(
                        "Blanking from another timing unit: TIMFLTR7 source",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "BLANKTIMFLTR8",
                    description: Some(
                        "Blanking from another timing unit: TIMFLTR8 source",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "WINDOWRESETTOCOMPARE2",
                    description: Some(
                        "Windowing from counter reset/roll-over to compare 2",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "WINDOWRESETTOCOMPARE3",
                    description: Some(
                        "Windowing from counter reset/roll-over to compare 3",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "WINDOWTIMWIN",
                    description: Some(
                        "Windowing from another timing unit: TIMWIN source",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Fault",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "No action: the output is not affected by the fault input and stays in run mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SETACTIVE",
                    description: Some(
                        "Output goes to active state after a fault event",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SETINACTIVE",
                    description: Some(
                        "Output goes to inactive state after a fault event",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SETHIGHZ",
                    description: Some(
                        "Output goes to high-z state after a fault event",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Ippstat",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "OUTPUT1ACTIVE",
                    description: Some(
                        "Protection occurred when the output 1 was active and output 2 forced inactive",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OUTPUT2ACTIVE",
                    description: Some(
                        "Protection occurred when the output 2 was active and output 1 forced inactive",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ACTIVEHIGH",
                    description: Some(
                        "Positive polarity (output active high)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVELOW",
                    description: Some(
                        "Negative polarity (output active low)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sdtf",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "POSITIVE",
                    description: Some(
                        "Positive deadtime on falling edge",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NEGATIVE",
                    description: Some(
                        "Negative deadtime on falling edge",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sdtr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "POSITIVE",
                    description: Some(
                        "Positive deadtime on rising edge",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NEGATIVE",
                    description: Some(
                        "Negative deadtime on rising edge",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Syncin",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Disabled. HRTIM is not synchronized and runs in standalone mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INTERNAL",
                    description: Some(
                        "Internal event: the HRTIM is synchronized with the on-chip timer",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "EXTERNAL",
                    description: Some(
                        "External event: a positive pulse on HRTIM_SCIN input triggers the HRTIM",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Syncout",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "POSITIVEPULSE",
                    description: Some(
                        "Positive pulse on SCOUT output (16x f_HRTIM clock cycles)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "NEGATIVEPULSE",
                    description: Some(
                        "Negative pulse on SCOUT output (16x f_HRTIM clock cycles)",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Syncsrc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MASTERSTART",
                    description: Some(
                        "Master timer Start",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MASTERCOMPARE1",
                    description: Some(
                        "Master timer Compare 1 event",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "TIMERASTART",
                    description: Some(
                        "Timer A start/reset",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "TIMERACOMPARE1",
                    description: Some(
                        "Timer A Compare 1 event",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Updgat",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "INDEPENDENT",
                    description: Some(
                        "Update occurs independently from the DMA burst transfer",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DMABURST",
                    description: Some(
                        "Update occurs when the DMA burst transfer is completed",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DMABURST_UPDATE",
                    description: Some(
                        "Update occurs on the update event following DMA burst transfer completion",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "INPUT1",
                    description: Some(
                        "Update occurs on a rising edge of HRTIM update enable input 1",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "INPUT2",
                    description: Some(
                        "Update occurs on a rising edge of HRTIM update enable input 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "INPUT3",
                    description: Some(
                        "Update occurs on a rising edge of HRTIM update enable input 3",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "INPUT1_UPDATE",
                    description: Some(
                        "Update occurs on the update event following a rising edge of HRTIM update enable input 1",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "INPUT2_UPDATE",
                    description: Some(
                        "Update occurs on the update event following a rising edge of HRTIM update enable input 2",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "INPUT3_UPDATE",
                    description: Some(
                        "Update occurs on the update event following a rising edge of HRTIM update enable input 3",
                    ),
                    value: 8,
                },
            ],
        },
    ],
};
                