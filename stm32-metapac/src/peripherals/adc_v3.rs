#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Analog-to-Digital Converter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "interrupt and status register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "interrupt enable register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "configuration register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "configuration register"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "sample time register 1"]
    #[inline(always)]
    pub const fn smpr(self, n: usize) -> crate::common::Reg<regs::Smpr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize + n * 4usize) as _) }
    }
    #[doc = "watchdog threshold register 1"]
    #[inline(always)]
    pub const fn tr(self, n: usize) -> crate::common::Reg<regs::Tr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "regular sequence register 1"]
    #[inline(always)]
    pub const fn sqr1(self) -> crate::common::Reg<regs::Sqr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "regular sequence register 2"]
    #[inline(always)]
    pub const fn sqr2(self) -> crate::common::Reg<regs::Sqr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "regular sequence register 3"]
    #[inline(always)]
    pub const fn sqr3(self) -> crate::common::Reg<regs::Sqr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "regular sequence register 4"]
    #[inline(always)]
    pub const fn sqr4(self) -> crate::common::Reg<regs::Sqr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "regular Data Register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "injected sequence register"]
    #[inline(always)]
    pub const fn jsqr(self) -> crate::common::Reg<regs::Jsqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "offset register 1"]
    #[inline(always)]
    pub const fn ofr(self, n: usize) -> crate::common::Reg<regs::Ofr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "injected data registers"]
    #[inline(always)]
    pub const fn jdr(self, n: usize) -> crate::common::Reg<regs::Jdr, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Analog Watchdog 2 Configuration Register"]
    #[inline(always)]
    pub const fn awd2cr(self) -> crate::common::Reg<regs::Awd2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "Analog Watchdog 3 Configuration Register"]
    #[inline(always)]
    pub const fn awd3cr(self) -> crate::common::Reg<regs::Awd3cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "Differential Mode Selection Register 2"]
    #[inline(always)]
    pub const fn difsel(self) -> crate::common::Reg<regs::Difsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "Calibration Factors"]
    #[inline(always)]
    pub const fn calfact(self) -> crate::common::Reg<regs::Calfact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Analog Watchdog 2 Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd2cr(pub u32);
    impl Awd2cr {
        #[doc = "AWD2CH"]
        #[inline(always)]
        pub const fn awd2ch(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "AWD2CH"]
        #[inline(always)]
        pub fn set_awd2ch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 1usize)) | (((val as u32) & 0x0003_ffff) << 1usize);
        }
    }
    impl Default for Awd2cr {
        #[inline(always)]
        fn default() -> Awd2cr {
            Awd2cr(0)
        }
    }
    #[doc = "Analog Watchdog 3 Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd3cr(pub u32);
    impl Awd3cr {
        #[doc = "AWD3CH"]
        #[inline(always)]
        pub const fn awd3ch(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "AWD3CH"]
        #[inline(always)]
        pub fn set_awd3ch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 1usize)) | (((val as u32) & 0x0003_ffff) << 1usize);
        }
    }
    impl Default for Awd3cr {
        #[inline(always)]
        fn default() -> Awd3cr {
            Awd3cr(0)
        }
    }
    #[doc = "Calibration Factors"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calfact(pub u32);
    impl Calfact {
        #[doc = "CALFACT_S"]
        #[inline(always)]
        pub const fn calfact_s(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "CALFACT_S"]
        #[inline(always)]
        pub fn set_calfact_s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "CALFACT_D"]
        #[inline(always)]
        pub const fn calfact_d(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "CALFACT_D"]
        #[inline(always)]
        pub fn set_calfact_d(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Calfact {
        #[inline(always)]
        fn default() -> Calfact {
            Calfact(0)
        }
    }
    #[doc = "configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "DMAEN"]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMAEN"]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Direct memory access configuration"]
        #[inline(always)]
        pub const fn dmacfg(&self) -> super::vals::Dmacfg {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Dmacfg::from_bits(val as u8)
        }
        #[doc = "Direct memory access configuration"]
        #[inline(always)]
        pub fn set_dmacfg(&mut self, val: super::vals::Dmacfg) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "RES"]
        #[inline(always)]
        pub const fn res(&self) -> super::vals::Res {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Res::from_bits(val as u8)
        }
        #[doc = "RES"]
        #[inline(always)]
        pub fn set_res(&mut self, val: super::vals::Res) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "ALIGN"]
        #[inline(always)]
        pub const fn align(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ALIGN"]
        #[inline(always)]
        pub fn set_align(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "EXTSEL"]
        #[inline(always)]
        pub const fn extsel(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x0f;
            val as u8
        }
        #[doc = "EXTSEL"]
        #[inline(always)]
        pub fn set_extsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
        }
        #[doc = "EXTEN"]
        #[inline(always)]
        pub const fn exten(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "EXTEN"]
        #[inline(always)]
        pub fn set_exten(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "OVRMOD"]
        #[inline(always)]
        pub const fn ovrmod(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "OVRMOD"]
        #[inline(always)]
        pub fn set_ovrmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CONT"]
        #[inline(always)]
        pub const fn cont(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "CONT"]
        #[inline(always)]
        pub fn set_cont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "AUTDLY"]
        #[inline(always)]
        pub const fn autdly(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "AUTDLY"]
        #[inline(always)]
        pub fn set_autdly(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "AUTOFF"]
        #[inline(always)]
        pub const fn autoff(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "AUTOFF"]
        #[inline(always)]
        pub fn set_autoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "DISCEN"]
        #[inline(always)]
        pub const fn discen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DISCEN"]
        #[inline(always)]
        pub fn set_discen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DISCNUM"]
        #[inline(always)]
        pub const fn discnum(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "DISCNUM"]
        #[inline(always)]
        pub fn set_discnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "JDISCEN"]
        #[inline(always)]
        pub const fn jdiscen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "JDISCEN"]
        #[inline(always)]
        pub fn set_jdiscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "JQM"]
        #[inline(always)]
        pub const fn jqm(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "JQM"]
        #[inline(always)]
        pub fn set_jqm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "AWD1SGL"]
        #[inline(always)]
        pub const fn awd1sgl(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "AWD1SGL"]
        #[inline(always)]
        pub fn set_awd1sgl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "AWD1EN"]
        #[inline(always)]
        pub const fn awd1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "AWD1EN"]
        #[inline(always)]
        pub fn set_awd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "JAWD1EN"]
        #[inline(always)]
        pub const fn jawd1en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "JAWD1EN"]
        #[inline(always)]
        pub fn set_jawd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "JAUTO"]
        #[inline(always)]
        pub const fn jauto(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "JAUTO"]
        #[inline(always)]
        pub fn set_jauto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "AWDCH1CH"]
        #[inline(always)]
        pub const fn awdch1ch(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[doc = "AWDCH1CH"]
        #[inline(always)]
        pub fn set_awdch1ch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    #[doc = "configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "DMAEN"]
        #[inline(always)]
        pub const fn rovse(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMAEN"]
        #[inline(always)]
        pub fn set_rovse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMACFG"]
        #[inline(always)]
        pub const fn jovse(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMACFG"]
        #[inline(always)]
        pub fn set_jovse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RES"]
        #[inline(always)]
        pub const fn ovsr(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[doc = "RES"]
        #[inline(always)]
        pub fn set_ovsr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[doc = "ALIGN"]
        #[inline(always)]
        pub const fn ovss(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "ALIGN"]
        #[inline(always)]
        pub fn set_ovss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "EXTSEL"]
        #[inline(always)]
        pub const fn tovs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "EXTSEL"]
        #[inline(always)]
        pub fn set_tovs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "EXTEN"]
        #[inline(always)]
        pub const fn rovsm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "EXTEN"]
        #[inline(always)]
        pub fn set_rovsm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    #[doc = "control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "ADEN"]
        #[inline(always)]
        pub const fn aden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADEN"]
        #[inline(always)]
        pub fn set_aden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADDIS"]
        #[inline(always)]
        pub const fn addis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADDIS"]
        #[inline(always)]
        pub fn set_addis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADSTART"]
        #[inline(always)]
        pub const fn adstart(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ADSTART"]
        #[inline(always)]
        pub fn set_adstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "JADSTART"]
        #[inline(always)]
        pub const fn jadstart(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "JADSTART"]
        #[inline(always)]
        pub fn set_jadstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ADSTP"]
        #[inline(always)]
        pub const fn adstp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ADSTP"]
        #[inline(always)]
        pub fn set_adstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JADSTP"]
        #[inline(always)]
        pub const fn jadstp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JADSTP"]
        #[inline(always)]
        pub fn set_jadstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADVREGEN"]
        #[inline(always)]
        pub const fn advregen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ADVREGEN"]
        #[inline(always)]
        pub fn set_advregen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DEEPPWD"]
        #[inline(always)]
        pub const fn deeppwd(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DEEPPWD"]
        #[inline(always)]
        pub fn set_deeppwd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "ADCALDIF"]
        #[inline(always)]
        pub const fn adcaldif(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ADCALDIF"]
        #[inline(always)]
        pub fn set_adcaldif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "ADCAL"]
        #[inline(always)]
        pub const fn adcal(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ADCAL"]
        #[inline(always)]
        pub fn set_adcal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "Differential Mode Selection Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Difsel(pub u32);
    impl Difsel {
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub const fn difsel_1_15(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x7fff;
            val as u16
        }
        #[doc = "Differential mode for channels 15 to 1"]
        #[inline(always)]
        pub fn set_difsel_1_15(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 1usize)) | (((val as u32) & 0x7fff) << 1usize);
        }
        #[doc = "Differential mode for channels 18 to 16"]
        #[inline(always)]
        pub const fn difsel_16_18(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Differential mode for channels 18 to 16"]
        #[inline(always)]
        pub fn set_difsel_16_18(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Difsel {
        #[inline(always)]
        fn default() -> Difsel {
            Difsel(0)
        }
    }
    #[doc = "regular Data Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "regularDATA"]
        #[inline(always)]
        pub const fn regular_data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "regularDATA"]
        #[inline(always)]
        pub fn set_regular_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    #[doc = "interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "ADRDYIE"]
        #[inline(always)]
        pub const fn adrdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADRDYIE"]
        #[inline(always)]
        pub fn set_adrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EOSMPIE"]
        #[inline(always)]
        pub const fn eosmpie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EOSMPIE"]
        #[inline(always)]
        pub fn set_eosmpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "EOCIE"]
        #[inline(always)]
        pub const fn eocie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EOCIE"]
        #[inline(always)]
        pub fn set_eocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EOSIE"]
        #[inline(always)]
        pub const fn eosie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EOSIE"]
        #[inline(always)]
        pub fn set_eosie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OVRIE"]
        #[inline(always)]
        pub const fn ovrie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OVRIE"]
        #[inline(always)]
        pub fn set_ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JEOCIE"]
        #[inline(always)]
        pub const fn jeocie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JEOCIE"]
        #[inline(always)]
        pub fn set_jeocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "JEOSIE"]
        #[inline(always)]
        pub const fn jeosie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "JEOSIE"]
        #[inline(always)]
        pub fn set_jeosie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "AWD1IE"]
        #[inline(always)]
        pub const fn awd1ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "AWD1IE"]
        #[inline(always)]
        pub fn set_awd1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "AWD2IE"]
        #[inline(always)]
        pub const fn awd2ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "AWD2IE"]
        #[inline(always)]
        pub fn set_awd2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "AWD3IE"]
        #[inline(always)]
        pub const fn awd3ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "AWD3IE"]
        #[inline(always)]
        pub fn set_awd3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "JQOVFIE"]
        #[inline(always)]
        pub const fn jqovfie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "JQOVFIE"]
        #[inline(always)]
        pub fn set_jqovfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    #[doc = "interrupt and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "ADRDY"]
        #[inline(always)]
        pub const fn adrdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADRDY"]
        #[inline(always)]
        pub fn set_adrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EOSMP"]
        #[inline(always)]
        pub const fn eosmp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EOSMP"]
        #[inline(always)]
        pub fn set_eosmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "EOC"]
        #[inline(always)]
        pub const fn eoc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EOC"]
        #[inline(always)]
        pub fn set_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EOS"]
        #[inline(always)]
        pub const fn eos(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EOS"]
        #[inline(always)]
        pub fn set_eos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OVR"]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OVR"]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JEOC"]
        #[inline(always)]
        pub const fn jeoc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JEOC"]
        #[inline(always)]
        pub fn set_jeoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "JEOS"]
        #[inline(always)]
        pub const fn jeos(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "JEOS"]
        #[inline(always)]
        pub fn set_jeos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "AWD1"]
        #[inline(always)]
        pub const fn awd(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "AWD1"]
        #[inline(always)]
        pub fn set_awd(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "JQOVF"]
        #[inline(always)]
        pub const fn jqovf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "JQOVF"]
        #[inline(always)]
        pub fn set_jqovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    #[doc = "injected data register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jdr(pub u32);
    impl Jdr {
        #[doc = "JDATA1"]
        #[inline(always)]
        pub const fn jdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "JDATA1"]
        #[inline(always)]
        pub fn set_jdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Jdr {
        #[inline(always)]
        fn default() -> Jdr {
            Jdr(0)
        }
    }
    #[doc = "injected sequence register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jsqr(pub u32);
    impl Jsqr {
        #[doc = "JL"]
        #[inline(always)]
        pub const fn jl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "JL"]
        #[inline(always)]
        pub fn set_jl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "JEXTSEL"]
        #[inline(always)]
        pub const fn jextsel(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[doc = "JEXTSEL"]
        #[inline(always)]
        pub fn set_jextsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[doc = "JEXTEN"]
        #[inline(always)]
        pub const fn jexten(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "JEXTEN"]
        #[inline(always)]
        pub fn set_jexten(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "JSQ1"]
        #[inline(always)]
        pub const fn jsq(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 8usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "JSQ1"]
        #[inline(always)]
        pub fn set_jsq(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 8usize + n * 6usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Jsqr {
        #[inline(always)]
        fn default() -> Jsqr {
            Jsqr(0)
        }
    }
    #[doc = "offset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ofr(pub u32);
    impl Ofr {
        #[inline(always)]
        pub const fn offset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[inline(always)]
        pub const fn offset_ch(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_offset_ch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
        }
        #[inline(always)]
        pub const fn offset_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_offset_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ofr {
        #[inline(always)]
        fn default() -> Ofr {
            Ofr(0)
        }
    }
    #[doc = "sample time register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpr(pub u32);
    impl Smpr {
        #[doc = "Channel 0 sampling time selection"]
        #[inline(always)]
        pub const fn smp(&self, n: usize) -> super::vals::SampleTime {
            assert!(n < 10usize);
            let offs = 0usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "Channel 0 sampling time selection"]
        #[inline(always)]
        pub fn set_smp(&mut self, n: usize, val: super::vals::SampleTime) {
            assert!(n < 10usize);
            let offs = 0usize + n * 3usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
    }
    impl Default for Smpr {
        #[inline(always)]
        fn default() -> Smpr {
            Smpr(0)
        }
    }
    #[doc = "regular sequence register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr1(pub u32);
    impl Sqr1 {
        #[doc = "Regular channel sequence length"]
        #[inline(always)]
        pub const fn l(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Regular channel sequence length"]
        #[inline(always)]
        pub fn set_l(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "SQ1"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 6usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "SQ1"]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 6usize + n * 6usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Sqr1 {
        #[inline(always)]
        fn default() -> Sqr1 {
            Sqr1(0)
        }
    }
    #[doc = "regular sequence register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr2(pub u32);
    impl Sqr2 {
        #[doc = "SQ5"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "SQ5"]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Sqr2 {
        #[inline(always)]
        fn default() -> Sqr2 {
            Sqr2(0)
        }
    }
    #[doc = "regular sequence register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr3(pub u32);
    impl Sqr3 {
        #[doc = "SQ10"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "SQ10"]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Sqr3 {
        #[inline(always)]
        fn default() -> Sqr3 {
            Sqr3(0)
        }
    }
    #[doc = "regular sequence register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr4(pub u32);
    impl Sqr4 {
        #[doc = "SQ15"]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "SQ15"]
        #[inline(always)]
        pub fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 0usize + n * 6usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Sqr4 {
        #[inline(always)]
        fn default() -> Sqr4 {
            Sqr4(0)
        }
    }
    #[doc = "watchdog threshold register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tr(pub u32);
    impl Tr {
        #[doc = "LT1"]
        #[inline(always)]
        pub const fn lt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "LT1"]
        #[inline(always)]
        pub fn set_lt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "HT1"]
        #[inline(always)]
        pub const fn ht(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "HT1"]
        #[inline(always)]
        pub fn set_ht(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Tr {
        #[inline(always)]
        fn default() -> Tr {
            Tr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dmacfg {
        #[doc = "DMA One Shot mode selected"]
        ONESHOT = 0x0,
        #[doc = "DMA Circular mode selected"]
        CIRCULAR = 0x01,
    }
    impl Dmacfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dmacfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dmacfg {
        #[inline(always)]
        fn from(val: u8) -> Dmacfg {
            Dmacfg::from_bits(val)
        }
    }
    impl From<Dmacfg> for u8 {
        #[inline(always)]
        fn from(val: Dmacfg) -> u8 {
            Dmacfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Res {
        #[doc = "12-bit resolution"]
        BITS12 = 0x0,
        #[doc = "10-bit resolution"]
        BITS10 = 0x01,
        #[doc = "8-bit resolution"]
        BITS8 = 0x02,
        #[doc = "6-bit resolution"]
        BITS6 = 0x03,
    }
    impl Res {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Res {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Res {
        #[inline(always)]
        fn from(val: u8) -> Res {
            Res::from_bits(val)
        }
    }
    impl From<Res> for u8 {
        #[inline(always)]
        fn from(val: Res) -> u8 {
            Res::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SampleTime {
        #[doc = "2.5 ADC cycles"]
        CYCLES2_5 = 0x0,
        #[doc = "6.5 ADC cycles"]
        CYCLES6_5 = 0x01,
        #[doc = "12.5 ADC cycles"]
        CYCLES12_5 = 0x02,
        #[doc = "24.5 ADC cycles"]
        CYCLES24_5 = 0x03,
        #[doc = "47.5 ADC cycles"]
        CYCLES47_5 = 0x04,
        #[doc = "92.5 ADC cycles"]
        CYCLES92_5 = 0x05,
        #[doc = "247.5 ADC cycles"]
        CYCLES247_5 = 0x06,
        #[doc = "640.5 ADC cycles"]
        CYCLES640_5 = 0x07,
    }
    impl SampleTime {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SampleTime {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SampleTime {
        #[inline(always)]
        fn from(val: u8) -> SampleTime {
            SampleTime::from_bits(val)
        }
    }
    impl From<SampleTime> for u8 {
        #[inline(always)]
        fn from(val: SampleTime) -> u8 {
            SampleTime::to_bits(val)
        }
    }
}
