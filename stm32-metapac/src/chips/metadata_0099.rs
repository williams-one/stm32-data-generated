
const PERIPHERALS: &'static [Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 1073816576,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "f1",
            block: "ADC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "ADC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "ADC1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN15",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC1",
            channel: Some("DMA1_CH1"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1_2",
        }],
    },
    Peripheral {
        name: "ADC2",
        address: 1073817600,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "f1",
            block: "ADC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "ADC2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "ADC2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN15",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1_2",
        }],
    },
    Peripheral {
        name: "ADC3",
        address: 1073822720,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "f1",
            block: "ADC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "ADC3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "ADC3RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PF9",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PF10",
                signal: "IN8",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC3",
            channel: Some("DMA2_CH5"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC3",
        }],
    },
    Peripheral {
        name: "ADC_COMMON",
        address: 1073816576,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "AFIO",
        address: 1073807360,
        registers: Some(PeripheralRegisters {
            kind: "afio",
            version: "f1",
            block: "AFIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "AFIOEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "AFIORST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BKP",
        address: 1073769472,
        registers: Some(PeripheralRegisters {
            kind: "bkp",
            version: "v1",
            block: "BKP",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "BKPEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "BKPRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CAN",
        address: 1073767424,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "bxcan",
            block: "CAN",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "CANEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "CANRST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PD0",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PD1",
                signal: "TX",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "USB_LP_CAN1_RX0",
            },
            PeripheralInterrupt {
                signal: "RX1",
                interrupt: "CAN1_RX1",
            },
            PeripheralInterrupt {
                signal: "SCE",
                interrupt: "CAN1_SCE",
            },
            PeripheralInterrupt {
                signal: "TX",
                interrupt: "USB_HP_CAN1_TX",
            },
        ],
    },
    Peripheral {
        name: "CRC",
        address: 1073885184,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "v1",
            block: "CRC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "CRCEN",
            }),
            reset: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DAC",
        address: 1073771520,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v1",
            block: "DAC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "DACEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "DACRST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "OUT1",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "OUT2",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "DBGMCU",
        address: 3758366720,
        registers: Some(PeripheralRegisters {
            kind: "dbgmcu",
            version: "f1",
            block: "DBGMCU",
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DMA1",
        address: 1073872896,
        registers: Some(PeripheralRegisters {
            kind: "bdma",
            version: "v1",
            block: "DMA",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "DMA1EN",
            }),
            reset: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA1_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA1_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA1_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA1_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA1_CHANNEL7",
            },
        ],
    },
    Peripheral {
        name: "DMA2",
        address: 1073873920,
        registers: Some(PeripheralRegisters {
            kind: "bdma",
            version: "v1",
            block: "DMA",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "DMA2EN",
            }),
            reset: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA2_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA2_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA2_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA2_CHANNEL4_5",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA2_CHANNEL4_5",
            },
        ],
    },
    Peripheral {
        name: "EXTI",
        address: 1073808384,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "v1",
            block: "EXTI",
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EXTI0",
                interrupt: "EXTI0",
            },
            PeripheralInterrupt {
                signal: "EXTI1",
                interrupt: "EXTI1",
            },
            PeripheralInterrupt {
                signal: "EXTI10",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI11",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI12",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI13",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI14",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI15",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI2",
                interrupt: "EXTI2",
            },
            PeripheralInterrupt {
                signal: "EXTI3",
                interrupt: "EXTI3",
            },
            PeripheralInterrupt {
                signal: "EXTI4",
                interrupt: "EXTI4",
            },
            PeripheralInterrupt {
                signal: "EXTI5",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI6",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI7",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI8",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI9",
                interrupt: "EXTI9_5",
            },
        ],
    },
    Peripheral {
        name: "FLASH",
        address: 1073881088,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "f1",
            block: "FLASH",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "FLASHEN",
            }),
            reset: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FLASH",
        }],
    },
    Peripheral {
        name: "FSMC",
        address: 2684354560,
        registers: Some(PeripheralRegisters {
            kind: "fsmc",
            version: "v1x3",
            block: "FSMC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "FSMCEN",
            }),
            reset: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB7",
                signal: "NL",
                af: None,
            },
            PeripheralPin {
                pin: "PD0",
                signal: "D2",
                af: None,
            },
            PeripheralPin {
                pin: "PD0",
                signal: "DA2",
                af: None,
            },
            PeripheralPin {
                pin: "PD1",
                signal: "D3",
                af: None,
            },
            PeripheralPin {
                pin: "PD1",
                signal: "DA3",
                af: None,
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CLK",
                af: None,
            },
            PeripheralPin {
                pin: "PD4",
                signal: "NOE",
                af: None,
            },
            PeripheralPin {
                pin: "PD5",
                signal: "NWE",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "NWAIT",
                af: None,
            },
            PeripheralPin {
                pin: "PD7",
                signal: "NCE2",
                af: None,
            },
            PeripheralPin {
                pin: "PD7",
                signal: "NE1",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "D13",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "DA13",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
                signal: "D14",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
                signal: "DA14",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "D15",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "DA15",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "A16",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CLE",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "A17",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "ALE",
                af: None,
            },
            PeripheralPin {
                pin: "PD13",
                signal: "A18",
                af: None,
            },
            PeripheralPin {
                pin: "PD14",
                signal: "D0",
                af: None,
            },
            PeripheralPin {
                pin: "PD14",
                signal: "DA0",
                af: None,
            },
            PeripheralPin {
                pin: "PD15",
                signal: "D1",
                af: None,
            },
            PeripheralPin {
                pin: "PD15",
                signal: "DA1",
                af: None,
            },
            PeripheralPin {
                pin: "PE0",
                signal: "NBL0",
                af: None,
            },
            PeripheralPin {
                pin: "PE1",
                signal: "NBL1",
                af: None,
            },
            PeripheralPin {
                pin: "PE2",
                signal: "A23",
                af: None,
            },
            PeripheralPin {
                pin: "PE3",
                signal: "A19",
                af: None,
            },
            PeripheralPin {
                pin: "PE4",
                signal: "A20",
                af: None,
            },
            PeripheralPin {
                pin: "PE5",
                signal: "A21",
                af: None,
            },
            PeripheralPin {
                pin: "PE6",
                signal: "A22",
                af: None,
            },
            PeripheralPin {
                pin: "PE7",
                signal: "D4",
                af: None,
            },
            PeripheralPin {
                pin: "PE7",
                signal: "DA4",
                af: None,
            },
            PeripheralPin {
                pin: "PE8",
                signal: "D5",
                af: None,
            },
            PeripheralPin {
                pin: "PE8",
                signal: "DA5",
                af: None,
            },
            PeripheralPin {
                pin: "PE9",
                signal: "D6",
                af: None,
            },
            PeripheralPin {
                pin: "PE9",
                signal: "DA6",
                af: None,
            },
            PeripheralPin {
                pin: "PE10",
                signal: "D7",
                af: None,
            },
            PeripheralPin {
                pin: "PE10",
                signal: "DA7",
                af: None,
            },
            PeripheralPin {
                pin: "PE11",
                signal: "D8",
                af: None,
            },
            PeripheralPin {
                pin: "PE11",
                signal: "DA8",
                af: None,
            },
            PeripheralPin {
                pin: "PE12",
                signal: "D9",
                af: None,
            },
            PeripheralPin {
                pin: "PE12",
                signal: "DA9",
                af: None,
            },
            PeripheralPin {
                pin: "PE13",
                signal: "D10",
                af: None,
            },
            PeripheralPin {
                pin: "PE13",
                signal: "DA10",
                af: None,
            },
            PeripheralPin {
                pin: "PE14",
                signal: "D11",
                af: None,
            },
            PeripheralPin {
                pin: "PE14",
                signal: "DA11",
                af: None,
            },
            PeripheralPin {
                pin: "PE15",
                signal: "D12",
                af: None,
            },
            PeripheralPin {
                pin: "PE15",
                signal: "DA12",
                af: None,
            },
            PeripheralPin {
                pin: "PF0",
                signal: "A0",
                af: None,
            },
            PeripheralPin {
                pin: "PF1",
                signal: "A1",
                af: None,
            },
            PeripheralPin {
                pin: "PF2",
                signal: "A2",
                af: None,
            },
            PeripheralPin {
                pin: "PF3",
                signal: "A3",
                af: None,
            },
            PeripheralPin {
                pin: "PF4",
                signal: "A4",
                af: None,
            },
            PeripheralPin {
                pin: "PF5",
                signal: "A5",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "NIORD",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "NREG",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "NIOWR",
                af: None,
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CD",
                af: None,
            },
            PeripheralPin {
                pin: "PF10",
                signal: "INTR",
                af: None,
            },
            PeripheralPin {
                pin: "PF11",
                signal: "NIOS16",
                af: None,
            },
            PeripheralPin {
                pin: "PF12",
                signal: "A6",
                af: None,
            },
            PeripheralPin {
                pin: "PF13",
                signal: "A7",
                af: None,
            },
            PeripheralPin {
                pin: "PF14",
                signal: "A8",
                af: None,
            },
            PeripheralPin {
                pin: "PF15",
                signal: "A9",
                af: None,
            },
            PeripheralPin {
                pin: "PG0",
                signal: "A10",
                af: None,
            },
            PeripheralPin {
                pin: "PG1",
                signal: "A11",
                af: None,
            },
            PeripheralPin {
                pin: "PG2",
                signal: "A12",
                af: None,
            },
            PeripheralPin {
                pin: "PG3",
                signal: "A13",
                af: None,
            },
            PeripheralPin {
                pin: "PG4",
                signal: "A14",
                af: None,
            },
            PeripheralPin {
                pin: "PG5",
                signal: "A15",
                af: None,
            },
            PeripheralPin {
                pin: "PG6",
                signal: "INT2",
                af: None,
            },
            PeripheralPin {
                pin: "PG7",
                signal: "INT3",
                af: None,
            },
            PeripheralPin {
                pin: "PG9",
                signal: "NCE3",
                af: None,
            },
            PeripheralPin {
                pin: "PG9",
                signal: "NE2",
                af: None,
            },
            PeripheralPin {
                pin: "PG10",
                signal: "NCE4_1",
                af: None,
            },
            PeripheralPin {
                pin: "PG10",
                signal: "NE3",
                af: None,
            },
            PeripheralPin {
                pin: "PG11",
                signal: "NCE4_2",
                af: None,
            },
            PeripheralPin {
                pin: "PG12",
                signal: "NE4",
                af: None,
            },
            PeripheralPin {
                pin: "PG13",
                signal: "A24",
                af: None,
            },
            PeripheralPin {
                pin: "PG14",
                signal: "A25",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FSMC",
        }],
    },
    Peripheral {
        name: "GPIOA",
        address: 1073809408,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v1",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "GPIOAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "GPIOARST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOB",
        address: 1073810432,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v1",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "GPIOBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "GPIOBRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOC",
        address: 1073811456,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v1",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "GPIOCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "GPIOCRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOD",
        address: 1073812480,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v1",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "GPIODEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "GPIODRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOE",
        address: 1073813504,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v1",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "GPIOEEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "GPIOERST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOF",
        address: 1073814528,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v1",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "GPIOFEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "GPIOFRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOG",
        address: 1073815552,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v1",
            block: "GPIO",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "GPIOGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "GPIOGRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C1",
        address: 1073763328,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v1",
            block: "I2C",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "I2C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "I2C1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB5",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C1_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C1_EV",
            },
        ],
    },
    Peripheral {
        name: "I2C2",
        address: 1073764352,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v1",
            block: "I2C",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "I2C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "I2C2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SMBA",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C2_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C2_EV",
            },
        ],
    },
    Peripheral {
        name: "IWDG",
        address: 1073754112,
        registers: Some(PeripheralRegisters {
            kind: "iwdg",
            version: "v1",
            block: "IWDG",
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PWR",
        address: 1073770496,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "f1",
            block: "PWR",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "PWREN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "PWRRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RCC",
        address: 1073876992,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "f1",
            block: "RCC",
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "MCO",
                af: None,
            },
            PeripheralPin {
                pin: "PC14",
                signal: "OSC32_IN",
                af: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "OSC32_OUT",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RCC",
        }],
    },
    Peripheral {
        name: "RTC",
        address: 1073752064,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "v1",
            block: "RTC",
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PC13",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "TAMPER",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ALARM",
                interrupt: "RTC",
            },
            PeripheralInterrupt {
                signal: "ALARM",
                interrupt: "RTC_ALARM",
            },
            PeripheralInterrupt {
                signal: "SSRU",
                interrupt: "RTC",
            },
            PeripheralInterrupt {
                signal: "STAMP",
                interrupt: "RTC",
            },
            PeripheralInterrupt {
                signal: "TAMP",
                interrupt: "RTC",
            },
            PeripheralInterrupt {
                signal: "TAMP",
                interrupt: "TAMPER",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "RTC",
            },
        ],
    },
    Peripheral {
        name: "SDIO",
        address: 1073840128,
        registers: Some(PeripheralRegisters {
            kind: "sdmmc",
            version: "v1",
            block: "SDMMC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "AHB1",
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "SDIOEN",
            }),
            reset: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB8",
                signal: "D4",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D5",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D6",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D7",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D0",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D1",
                af: None,
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D2",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D3",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CMD",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SDIO",
            channel: Some("DMA2_CH4"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDIO",
        }],
    },
    Peripheral {
        name: "SPI1",
        address: 1073819648,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "f1",
            block: "SPI",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SPI1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI1",
        }],
    },
    Peripheral {
        name: "SPI2",
        address: 1073756160,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "f1",
            block: "SPI",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "SPI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "SPI2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "I2S_WS",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "I2S_CK",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "I2S_SD",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "I2S_MCK",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI2",
        }],
    },
    Peripheral {
        name: "SPI3",
        address: 1073757184,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "f1",
            block: "SPI",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "SPI3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "SPI3RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "I2S_WS",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "I2S_CK",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "I2S_SD",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "I2S_MCK",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
        }],
    },
    Peripheral {
        name: "TIM1",
        address: 1073818624,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_ADV",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PE7",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PE8",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PE9",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PE11",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PE12",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PE13",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PE15",
                signal: "BKIN",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP_TIM10",
            },
        ],
    },
    Peripheral {
        name: "TIM10",
        address: 1073827840,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM10EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM10RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "CH1",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_UP_TIM10",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_UP_TIM10",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_UP_TIM10",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_UP_TIM10",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP_TIM10",
            },
        ],
    },
    Peripheral {
        name: "TIM11",
        address: 1073828864,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM11EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM11RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "CH1",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
        ],
    },
    Peripheral {
        name: "TIM12",
        address: 1073747968,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM12RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB14",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_BRK_TIM12",
            },
        ],
    },
    Peripheral {
        name: "TIM13",
        address: 1073748992,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM13EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM13RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "CH1",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_UP_TIM13",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_UP_TIM13",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_UP_TIM13",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_UP_TIM13",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_UP_TIM13",
            },
        ],
    },
    Peripheral {
        name: "TIM14",
        address: 1073750016,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM14EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM14RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CH1",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
        ],
    },
    Peripheral {
        name: "TIM2",
        address: 1073741824,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH4",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM2",
            },
        ],
    },
    Peripheral {
        name: "TIM3",
        address: 1073742848,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM3RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "ETR",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM3",
            },
        ],
    },
    Peripheral {
        name: "TIM4",
        address: 1073743872,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM4RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PD13",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PD14",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PD15",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM4",
            },
        ],
    },
    Peripheral {
        name: "TIM5",
        address: 1073744896,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM5RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM5",
            },
        ],
    },
    Peripheral {
        name: "TIM6",
        address: 1073745920,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_BASIC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM6RST",
            }),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: Some("DMA2_CH3"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM6",
            },
        ],
    },
    Peripheral {
        name: "TIM7",
        address: 1073746944,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_BASIC",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM7RST",
            }),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: Some("DMA2_CH4"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM7",
            },
        ],
    },
    Peripheral {
        name: "TIM8",
        address: 1073820672,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_ADV",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM8EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM8RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_UP_TIM13",
            },
        ],
    },
    Peripheral {
        name: "TIM9",
        address: 1073826816,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP16",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2_TIM",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM9EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM9RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PE6",
                signal: "CH2",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_BRK_TIM9",
            },
        ],
    },
    Peripheral {
        name: "UART4",
        address: 1073761280,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "UART4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "UART4RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART4",
        }],
    },
    Peripheral {
        name: "UART5",
        address: 1073762304,
        registers: None,
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "UART5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "UART5RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC12",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "RX",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART5",
        }],
    },
    Peripheral {
        name: "UID",
        address: 536868840,
        registers: Some(PeripheralRegisters {
            kind: "uid",
            version: "v1",
            block: "UID",
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USART1",
        address: 1073821696,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v1",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB2",
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "USART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "USART1RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART1",
        }],
    },
    Peripheral {
        name: "USART2",
        address: 1073759232,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v1",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "USART2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "USART2RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PD4",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PD5",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CK",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART2",
        }],
    },
    Peripheral {
        name: "USART3",
        address: 1073760256,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "v1",
            block: "USART",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "USART3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "USART3RST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "RTS",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3",
        }],
    },
    Peripheral {
        name: "USB",
        address: 1073765376,
        registers: Some(PeripheralRegisters {
            kind: "usb",
            version: "v1",
            block: "USB",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "USBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "USBRST",
            }),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "DM",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DP",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "HP",
                interrupt: "USB_HP_CAN1_TX",
            },
            PeripheralInterrupt {
                signal: "LP",
                interrupt: "USB_LP_CAN1_RX0",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "USBWAKEUP",
            },
        ],
    },
    Peripheral {
        name: "USBRAM",
        address: 1073766400,
        registers: Some(PeripheralRegisters {
            kind: "usbram",
            version: "16x1_512",
            block: "USBRAM",
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "WWDG",
        address: 1073753088,
        registers: Some(PeripheralRegisters {
            kind: "wwdg",
            version: "v1",
            block: "WWDG",
        }),
        rcc: Some(PeripheralRcc {
            clock: "APB1",
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "WWDGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "WWDGRST",
            }),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "WWDG",
            },
            PeripheralInterrupt {
                signal: "RST",
                interrupt: "WWDG",
            },
        ],
    },
];
const INTERRUPTS: &'static [Interrupt] = &[
    Interrupt {
        name: "WWDG",
        number: 0,
    },
    Interrupt { name: "PVD", number: 1 },
    Interrupt {
        name: "TAMPER",
        number: 2,
    },
    Interrupt { name: "RTC", number: 3 },
    Interrupt {
        name: "FLASH",
        number: 4,
    },
    Interrupt { name: "RCC", number: 5 },
    Interrupt {
        name: "EXTI0",
        number: 6,
    },
    Interrupt {
        name: "EXTI1",
        number: 7,
    },
    Interrupt {
        name: "EXTI2",
        number: 8,
    },
    Interrupt {
        name: "EXTI3",
        number: 9,
    },
    Interrupt {
        name: "EXTI4",
        number: 10,
    },
    Interrupt {
        name: "DMA1_CHANNEL1",
        number: 11,
    },
    Interrupt {
        name: "DMA1_CHANNEL2",
        number: 12,
    },
    Interrupt {
        name: "DMA1_CHANNEL3",
        number: 13,
    },
    Interrupt {
        name: "DMA1_CHANNEL4",
        number: 14,
    },
    Interrupt {
        name: "DMA1_CHANNEL5",
        number: 15,
    },
    Interrupt {
        name: "DMA1_CHANNEL6",
        number: 16,
    },
    Interrupt {
        name: "DMA1_CHANNEL7",
        number: 17,
    },
    Interrupt {
        name: "ADC1_2",
        number: 18,
    },
    Interrupt {
        name: "USB_HP_CAN1_TX",
        number: 19,
    },
    Interrupt {
        name: "USB_LP_CAN1_RX0",
        number: 20,
    },
    Interrupt {
        name: "CAN1_RX1",
        number: 21,
    },
    Interrupt {
        name: "CAN1_SCE",
        number: 22,
    },
    Interrupt {
        name: "EXTI9_5",
        number: 23,
    },
    Interrupt {
        name: "TIM1_BRK_TIM9",
        number: 24,
    },
    Interrupt {
        name: "TIM1_UP_TIM10",
        number: 25,
    },
    Interrupt {
        name: "TIM1_TRG_COM_TIM11",
        number: 26,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 27,
    },
    Interrupt {
        name: "TIM2",
        number: 28,
    },
    Interrupt {
        name: "TIM3",
        number: 29,
    },
    Interrupt {
        name: "TIM4",
        number: 30,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 31,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 32,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 33,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 34,
    },
    Interrupt {
        name: "SPI1",
        number: 35,
    },
    Interrupt {
        name: "SPI2",
        number: 36,
    },
    Interrupt {
        name: "USART1",
        number: 37,
    },
    Interrupt {
        name: "USART2",
        number: 38,
    },
    Interrupt {
        name: "USART3",
        number: 39,
    },
    Interrupt {
        name: "EXTI15_10",
        number: 40,
    },
    Interrupt {
        name: "RTC_ALARM",
        number: 41,
    },
    Interrupt {
        name: "USBWAKEUP",
        number: 42,
    },
    Interrupt {
        name: "TIM8_BRK_TIM12",
        number: 43,
    },
    Interrupt {
        name: "TIM8_UP_TIM13",
        number: 44,
    },
    Interrupt {
        name: "TIM8_TRG_COM_TIM14",
        number: 45,
    },
    Interrupt {
        name: "TIM8_CC",
        number: 46,
    },
    Interrupt {
        name: "ADC3",
        number: 47,
    },
    Interrupt {
        name: "FSMC",
        number: 48,
    },
    Interrupt {
        name: "SDIO",
        number: 49,
    },
    Interrupt {
        name: "TIM5",
        number: 50,
    },
    Interrupt {
        name: "SPI3",
        number: 51,
    },
    Interrupt {
        name: "UART4",
        number: 52,
    },
    Interrupt {
        name: "UART5",
        number: 53,
    },
    Interrupt {
        name: "TIM6",
        number: 54,
    },
    Interrupt {
        name: "TIM7",
        number: 55,
    },
    Interrupt {
        name: "DMA2_CHANNEL1",
        number: 56,
    },
    Interrupt {
        name: "DMA2_CHANNEL2",
        number: 57,
    },
    Interrupt {
        name: "DMA2_CHANNEL3",
        number: 58,
    },
    Interrupt {
        name: "DMA2_CHANNEL4_5",
        number: 59,
    },
];
const DMA_CHANNELS: &'static [DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH1",
        dma: "DMA2",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH2",
        dma: "DMA2",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH3",
        dma: "DMA2",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH4",
        dma: "DMA2",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH5",
        dma: "DMA2",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
];
