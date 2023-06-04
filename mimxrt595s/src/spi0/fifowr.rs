#[doc = "Register `FIFOWR` writer"]
pub struct W(crate::W<FIFOWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FIFOWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATA` writer - Transmit Data to the FIFO"]
pub type TXDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFOWR_SPEC, u16, u16, 16, O>;
#[doc = "Transmit Slave Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXSSEL0_N_AW {
    #[doc = "0: SSEL0 is asserted"]
    ASSERTED = 0,
    #[doc = "1: SSEL0 is not asserted"]
    NOT_ASSERTED = 1,
}
impl From<TXSSEL0_N_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSSEL0_N_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSSEL0_N` writer - Transmit Slave Select 0"]
pub type TXSSEL0_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOWR_SPEC, TXSSEL0_N_AW, O>;
impl<'a, const O: u8> TXSSEL0_N_W<'a, O> {
    #[doc = "SSEL0 is asserted"]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL0_N_AW::ASSERTED)
    }
    #[doc = "SSEL0 is not asserted"]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL0_N_AW::NOT_ASSERTED)
    }
}
#[doc = "Transmit Slave Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXSSEL1_N_AW {
    #[doc = "0: SSEL1 is asserted"]
    ASSERTED = 0,
    #[doc = "1: SSEL1 is not asserted"]
    NOT_ASSERTED = 1,
}
impl From<TXSSEL1_N_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSSEL1_N_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSSEL1_N` writer - Transmit Slave Select 1"]
pub type TXSSEL1_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOWR_SPEC, TXSSEL1_N_AW, O>;
impl<'a, const O: u8> TXSSEL1_N_W<'a, O> {
    #[doc = "SSEL1 is asserted"]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL1_N_AW::ASSERTED)
    }
    #[doc = "SSEL1 is not asserted"]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL1_N_AW::NOT_ASSERTED)
    }
}
#[doc = "Transmit Slave Select 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXSSEL2_N_AW {
    #[doc = "0: SSEL2 is asserted"]
    ASSERTED = 0,
    #[doc = "1: SSEL2 is not asserted"]
    NOT_ASSERTED = 1,
}
impl From<TXSSEL2_N_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSSEL2_N_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSSEL2_N` writer - Transmit Slave Select 2"]
pub type TXSSEL2_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOWR_SPEC, TXSSEL2_N_AW, O>;
impl<'a, const O: u8> TXSSEL2_N_W<'a, O> {
    #[doc = "SSEL2 is asserted"]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL2_N_AW::ASSERTED)
    }
    #[doc = "SSEL2 is not asserted"]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL2_N_AW::NOT_ASSERTED)
    }
}
#[doc = "Transmit Slave Select 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXSSEL3_N_AW {
    #[doc = "0: SSEL3 is asserted"]
    ASSERTED = 0,
    #[doc = "1: SSEL3 is not asserted"]
    NOT_ASSERTED = 1,
}
impl From<TXSSEL3_N_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSSEL3_N_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSSEL3_N` writer - Transmit Slave Select 3"]
pub type TXSSEL3_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOWR_SPEC, TXSSEL3_N_AW, O>;
impl<'a, const O: u8> TXSSEL3_N_W<'a, O> {
    #[doc = "SSEL3 is asserted"]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL3_N_AW::ASSERTED)
    }
    #[doc = "SSEL3 is not asserted"]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL3_N_AW::NOT_ASSERTED)
    }
}
#[doc = "End of Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOT_AW {
    #[doc = "0: SSEL is not deasserted. This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    NOT_DEASSERTED = 0,
    #[doc = "1: SSEL is deasserted. This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data."]
    DEASSERTED = 1,
}
impl From<EOT_AW> for bool {
    #[inline(always)]
    fn from(variant: EOT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOT` writer - End of Transfer"]
pub type EOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOWR_SPEC, EOT_AW, O>;
impl<'a, const O: u8> EOT_W<'a, O> {
    #[doc = "SSEL is not deasserted. This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    #[inline(always)]
    pub fn not_deasserted(self) -> &'a mut W {
        self.variant(EOT_AW::NOT_DEASSERTED)
    }
    #[doc = "SSEL is deasserted. This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data."]
    #[inline(always)]
    pub fn deasserted(self) -> &'a mut W {
        self.variant(EOT_AW::DEASSERTED)
    }
}
#[doc = "End of Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOF_AW {
    #[doc = "0: Data not EOF. This piece of data transmitted is not treated as the end of a frame."]
    NOT_EOF = 0,
    #[doc = "1: Data EOF. This piece of data is treated as the end of a frame, causing the Frame_delay time to be inserted before subsequent data is transmitted."]
    EOF = 1,
}
impl From<EOF_AW> for bool {
    #[inline(always)]
    fn from(variant: EOF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOF` writer - End of Frame"]
pub type EOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOWR_SPEC, EOF_AW, O>;
impl<'a, const O: u8> EOF_W<'a, O> {
    #[doc = "Data not EOF. This piece of data transmitted is not treated as the end of a frame."]
    #[inline(always)]
    pub fn not_eof(self) -> &'a mut W {
        self.variant(EOF_AW::NOT_EOF)
    }
    #[doc = "Data EOF. This piece of data is treated as the end of a frame, causing the Frame_delay time to be inserted before subsequent data is transmitted."]
    #[inline(always)]
    pub fn eof(self) -> &'a mut W {
        self.variant(EOF_AW::EOF)
    }
}
#[doc = "Receive Ignore\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXIGNORE_AW {
    #[doc = "0: Read received data. Received data must be read, to allow transmission to proceed. SPI transmit will halt when the receive data FIFO is full. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    READ = 0,
    #[doc = "1: Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    IGNORE = 1,
}
impl From<RXIGNORE_AW> for bool {
    #[inline(always)]
    fn from(variant: RXIGNORE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIGNORE` writer - Receive Ignore"]
pub type RXIGNORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOWR_SPEC, RXIGNORE_AW, O>;
impl<'a, const O: u8> RXIGNORE_W<'a, O> {
    #[doc = "Read received data. Received data must be read, to allow transmission to proceed. SPI transmit will halt when the receive data FIFO is full. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(RXIGNORE_AW::READ)
    }
    #[doc = "Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(RXIGNORE_AW::IGNORE)
    }
}
#[doc = "Transmit Ignore\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXIGNORE_AW {
    #[doc = "0: Write transmit data"]
    WRITETXDATA = 0,
    #[doc = "1: Ignore transmit data"]
    IGNORETXDATA = 1,
}
impl From<TXIGNORE_AW> for bool {
    #[inline(always)]
    fn from(variant: TXIGNORE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIGNORE` writer - Transmit Ignore"]
pub type TXIGNORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOWR_SPEC, TXIGNORE_AW, O>;
impl<'a, const O: u8> TXIGNORE_W<'a, O> {
    #[doc = "Write transmit data"]
    #[inline(always)]
    pub fn writetxdata(self) -> &'a mut W {
        self.variant(TXIGNORE_AW::WRITETXDATA)
    }
    #[doc = "Ignore transmit data"]
    #[inline(always)]
    pub fn ignoretxdata(self) -> &'a mut W {
        self.variant(TXIGNORE_AW::IGNORETXDATA)
    }
}
#[doc = "Data Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEN_AW {
    #[doc = "3: Data transfer is 4 bits in length"]
    LEN_4BITS = 3,
    #[doc = "4: Data transfer is 5 bits in length"]
    LEN_5BITS = 4,
    #[doc = "15: Data transfer is 16 bits in length"]
    LEN_16BITS = 15,
}
impl From<LEN_AW> for u8 {
    #[inline(always)]
    fn from(variant: LEN_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `LEN` writer - Data Length"]
pub type LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFOWR_SPEC, u8, LEN_AW, 4, O>;
impl<'a, const O: u8> LEN_W<'a, O> {
    #[doc = "Data transfer is 4 bits in length"]
    #[inline(always)]
    pub fn len_4bits(self) -> &'a mut W {
        self.variant(LEN_AW::LEN_4BITS)
    }
    #[doc = "Data transfer is 5 bits in length"]
    #[inline(always)]
    pub fn len_5bits(self) -> &'a mut W {
        self.variant(LEN_AW::LEN_5BITS)
    }
    #[doc = "Data transfer is 16 bits in length"]
    #[inline(always)]
    pub fn len_16bits(self) -> &'a mut W {
        self.variant(LEN_AW::LEN_16BITS)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Data to the FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TXDATA_W<0> {
        TXDATA_W::new(self)
    }
    #[doc = "Bit 16 - Transmit Slave Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn txssel0_n(&mut self) -> TXSSEL0_N_W<16> {
        TXSSEL0_N_W::new(self)
    }
    #[doc = "Bit 17 - Transmit Slave Select 1"]
    #[inline(always)]
    #[must_use]
    pub fn txssel1_n(&mut self) -> TXSSEL1_N_W<17> {
        TXSSEL1_N_W::new(self)
    }
    #[doc = "Bit 18 - Transmit Slave Select 2"]
    #[inline(always)]
    #[must_use]
    pub fn txssel2_n(&mut self) -> TXSSEL2_N_W<18> {
        TXSSEL2_N_W::new(self)
    }
    #[doc = "Bit 19 - Transmit Slave Select 3"]
    #[inline(always)]
    #[must_use]
    pub fn txssel3_n(&mut self) -> TXSSEL3_N_W<19> {
        TXSSEL3_N_W::new(self)
    }
    #[doc = "Bit 20 - End of Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn eot(&mut self) -> EOT_W<20> {
        EOT_W::new(self)
    }
    #[doc = "Bit 21 - End of Frame"]
    #[inline(always)]
    #[must_use]
    pub fn eof(&mut self) -> EOF_W<21> {
        EOF_W::new(self)
    }
    #[doc = "Bit 22 - Receive Ignore"]
    #[inline(always)]
    #[must_use]
    pub fn rxignore(&mut self) -> RXIGNORE_W<22> {
        RXIGNORE_W::new(self)
    }
    #[doc = "Bit 23 - Transmit Ignore"]
    #[inline(always)]
    #[must_use]
    pub fn txignore(&mut self) -> TXIGNORE_W<23> {
        TXIGNORE_W::new(self)
    }
    #[doc = "Bits 24:27 - Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<24> {
        LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Write Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifowr](index.html) module"]
pub struct FIFOWR_SPEC;
impl crate::RegisterSpec for FIFOWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fifowr::W](W) writer structure"]
impl crate::Writable for FIFOWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOWR to value 0"]
impl crate::Resettable for FIFOWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
