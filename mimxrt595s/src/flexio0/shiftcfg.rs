#[doc = "Register `SHIFTCFG[%s]` reader"]
pub struct R(crate::R<SHIFTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTCFG[%s]` writer"]
pub struct W(crate::W<SHIFTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTCFG_SPEC>;
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
impl From<crate::W<SHIFTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSTART` reader - Shifter Start bit"]
pub type SSTART_R = crate::FieldReader<u8, SSTART_A>;
#[doc = "Shifter Start bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SSTART_A {
    #[doc = "0: Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable"]
    VALUE00 = 0,
    #[doc = "1: Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
    VALUE01 = 1,
    #[doc = "2: Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
    VALUE10 = 2,
    #[doc = "3: Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
    VALUE11 = 3,
}
impl From<SSTART_A> for u8 {
    #[inline(always)]
    fn from(variant: SSTART_A) -> Self {
        variant as _
    }
}
impl SSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSTART_A {
        match self.bits {
            0 => SSTART_A::VALUE00,
            1 => SSTART_A::VALUE01,
            2 => SSTART_A::VALUE10,
            3 => SSTART_A::VALUE11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE00`"]
    #[inline(always)]
    pub fn is_value00(&self) -> bool {
        *self == SSTART_A::VALUE00
    }
    #[doc = "Checks if the value of the field is `VALUE01`"]
    #[inline(always)]
    pub fn is_value01(&self) -> bool {
        *self == SSTART_A::VALUE01
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == SSTART_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == SSTART_A::VALUE11
    }
}
#[doc = "Field `SSTART` writer - Shifter Start bit"]
pub type SSTART_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SHIFTCFG_SPEC, u8, SSTART_A, 2, O>;
impl<'a, const O: u8> SSTART_W<'a, O> {
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable"]
    #[inline(always)]
    pub fn value00(self) -> &'a mut W {
        self.variant(SSTART_A::VALUE00)
    }
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
    #[inline(always)]
    pub fn value01(self) -> &'a mut W {
        self.variant(SSTART_A::VALUE01)
    }
    #[doc = "Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(SSTART_A::VALUE10)
    }
    #[doc = "Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(SSTART_A::VALUE11)
    }
}
#[doc = "Field `SSTOP` reader - Shifter Stop bit"]
pub type SSTOP_R = crate::FieldReader<u8, SSTOP_A>;
#[doc = "Shifter Stop bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SSTOP_A {
    #[doc = "0: Stop bit disabled for transmitter/receiver/match store"]
    VALUE00 = 0,
    #[doc = "1: Stop bit disabled for transmitter/receiver/match store, receiver/match store will store receive data on the configured shift edge when timer in stop condition"]
    VALUE01 = 1,
    #[doc = "2: Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0, receiver/match store will also store receive data on the configured shift edge when timer in stop condition"]
    VALUE10 = 2,
    #[doc = "3: Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1, receiver/match store will also store receive data on the configured shift edge when timer in stop condition"]
    VALUE11 = 3,
}
impl From<SSTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: SSTOP_A) -> Self {
        variant as _
    }
}
impl SSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSTOP_A {
        match self.bits {
            0 => SSTOP_A::VALUE00,
            1 => SSTOP_A::VALUE01,
            2 => SSTOP_A::VALUE10,
            3 => SSTOP_A::VALUE11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE00`"]
    #[inline(always)]
    pub fn is_value00(&self) -> bool {
        *self == SSTOP_A::VALUE00
    }
    #[doc = "Checks if the value of the field is `VALUE01`"]
    #[inline(always)]
    pub fn is_value01(&self) -> bool {
        *self == SSTOP_A::VALUE01
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == SSTOP_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == SSTOP_A::VALUE11
    }
}
#[doc = "Field `SSTOP` writer - Shifter Stop bit"]
pub type SSTOP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SHIFTCFG_SPEC, u8, SSTOP_A, 2, O>;
impl<'a, const O: u8> SSTOP_W<'a, O> {
    #[doc = "Stop bit disabled for transmitter/receiver/match store"]
    #[inline(always)]
    pub fn value00(self) -> &'a mut W {
        self.variant(SSTOP_A::VALUE00)
    }
    #[doc = "Stop bit disabled for transmitter/receiver/match store, receiver/match store will store receive data on the configured shift edge when timer in stop condition"]
    #[inline(always)]
    pub fn value01(self) -> &'a mut W {
        self.variant(SSTOP_A::VALUE01)
    }
    #[doc = "Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0, receiver/match store will also store receive data on the configured shift edge when timer in stop condition"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(SSTOP_A::VALUE10)
    }
    #[doc = "Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1, receiver/match store will also store receive data on the configured shift edge when timer in stop condition"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(SSTOP_A::VALUE11)
    }
}
#[doc = "Field `INSRC` reader - Input Source"]
pub type INSRC_R = crate::BitReader<INSRC_A>;
#[doc = "Input Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INSRC_A {
    #[doc = "0: Pin"]
    PIN = 0,
    #[doc = "1: Shifter N+1 Output"]
    SHIFTER_NPLUS1 = 1,
}
impl From<INSRC_A> for bool {
    #[inline(always)]
    fn from(variant: INSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl INSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INSRC_A {
        match self.bits {
            false => INSRC_A::PIN,
            true => INSRC_A::SHIFTER_NPLUS1,
        }
    }
    #[doc = "Checks if the value of the field is `PIN`"]
    #[inline(always)]
    pub fn is_pin(&self) -> bool {
        *self == INSRC_A::PIN
    }
    #[doc = "Checks if the value of the field is `SHIFTER_NPLUS1`"]
    #[inline(always)]
    pub fn is_shifter_nplus1(&self) -> bool {
        *self == INSRC_A::SHIFTER_NPLUS1
    }
}
#[doc = "Field `INSRC` writer - Input Source"]
pub type INSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHIFTCFG_SPEC, INSRC_A, O>;
impl<'a, const O: u8> INSRC_W<'a, O> {
    #[doc = "Pin"]
    #[inline(always)]
    pub fn pin(self) -> &'a mut W {
        self.variant(INSRC_A::PIN)
    }
    #[doc = "Shifter N+1 Output"]
    #[inline(always)]
    pub fn shifter_nplus1(self) -> &'a mut W {
        self.variant(INSRC_A::SHIFTER_NPLUS1)
    }
}
#[doc = "Field `LATST` reader - Late Store"]
pub type LATST_R = crate::BitReader<LATST_A>;
#[doc = "Late Store\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LATST_A {
    #[doc = "0: Shift register stores the pre-shift register state."]
    PRESHIFT = 0,
    #[doc = "1: Shift register stores the post-shift register state."]
    POSTSHIFT = 1,
}
impl From<LATST_A> for bool {
    #[inline(always)]
    fn from(variant: LATST_A) -> Self {
        variant as u8 != 0
    }
}
impl LATST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LATST_A {
        match self.bits {
            false => LATST_A::PRESHIFT,
            true => LATST_A::POSTSHIFT,
        }
    }
    #[doc = "Checks if the value of the field is `PRESHIFT`"]
    #[inline(always)]
    pub fn is_preshift(&self) -> bool {
        *self == LATST_A::PRESHIFT
    }
    #[doc = "Checks if the value of the field is `POSTSHIFT`"]
    #[inline(always)]
    pub fn is_postshift(&self) -> bool {
        *self == LATST_A::POSTSHIFT
    }
}
#[doc = "Field `LATST` writer - Late Store"]
pub type LATST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHIFTCFG_SPEC, LATST_A, O>;
impl<'a, const O: u8> LATST_W<'a, O> {
    #[doc = "Shift register stores the pre-shift register state."]
    #[inline(always)]
    pub fn preshift(self) -> &'a mut W {
        self.variant(LATST_A::PRESHIFT)
    }
    #[doc = "Shift register stores the post-shift register state."]
    #[inline(always)]
    pub fn postshift(self) -> &'a mut W {
        self.variant(LATST_A::POSTSHIFT)
    }
}
#[doc = "Field `SSIZE` reader - Shifter Size"]
pub type SSIZE_R = crate::BitReader<SSIZE_A>;
#[doc = "Shifter Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSIZE_A {
    #[doc = "0: Shift register is 32-bit."]
    WIDTH32 = 0,
    #[doc = "1: Shift register is 24-bit."]
    WIDTH24 = 1,
}
impl From<SSIZE_A> for bool {
    #[inline(always)]
    fn from(variant: SSIZE_A) -> Self {
        variant as u8 != 0
    }
}
impl SSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSIZE_A {
        match self.bits {
            false => SSIZE_A::WIDTH32,
            true => SSIZE_A::WIDTH24,
        }
    }
    #[doc = "Checks if the value of the field is `WIDTH32`"]
    #[inline(always)]
    pub fn is_width32(&self) -> bool {
        *self == SSIZE_A::WIDTH32
    }
    #[doc = "Checks if the value of the field is `WIDTH24`"]
    #[inline(always)]
    pub fn is_width24(&self) -> bool {
        *self == SSIZE_A::WIDTH24
    }
}
#[doc = "Field `SSIZE` writer - Shifter Size"]
pub type SSIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHIFTCFG_SPEC, SSIZE_A, O>;
impl<'a, const O: u8> SSIZE_W<'a, O> {
    #[doc = "Shift register is 32-bit."]
    #[inline(always)]
    pub fn width32(self) -> &'a mut W {
        self.variant(SSIZE_A::WIDTH32)
    }
    #[doc = "Shift register is 24-bit."]
    #[inline(always)]
    pub fn width24(self) -> &'a mut W {
        self.variant(SSIZE_A::WIDTH24)
    }
}
#[doc = "Field `PWIDTH` reader - Parallel Width"]
pub type PWIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWIDTH` writer - Parallel Width"]
pub type PWIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHIFTCFG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Shifter Start bit"]
    #[inline(always)]
    pub fn sstart(&self) -> SSTART_R {
        SSTART_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Shifter Stop bit"]
    #[inline(always)]
    pub fn sstop(&self) -> SSTOP_R {
        SSTOP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Input Source"]
    #[inline(always)]
    pub fn insrc(&self) -> INSRC_R {
        INSRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Late Store"]
    #[inline(always)]
    pub fn latst(&self) -> LATST_R {
        LATST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Shifter Size"]
    #[inline(always)]
    pub fn ssize(&self) -> SSIZE_R {
        SSIZE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Parallel Width"]
    #[inline(always)]
    pub fn pwidth(&self) -> PWIDTH_R {
        PWIDTH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Shifter Start bit"]
    #[inline(always)]
    #[must_use]
    pub fn sstart(&mut self) -> SSTART_W<0> {
        SSTART_W::new(self)
    }
    #[doc = "Bits 4:5 - Shifter Stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn sstop(&mut self) -> SSTOP_W<4> {
        SSTOP_W::new(self)
    }
    #[doc = "Bit 8 - Input Source"]
    #[inline(always)]
    #[must_use]
    pub fn insrc(&mut self) -> INSRC_W<8> {
        INSRC_W::new(self)
    }
    #[doc = "Bit 9 - Late Store"]
    #[inline(always)]
    #[must_use]
    pub fn latst(&mut self) -> LATST_W<9> {
        LATST_W::new(self)
    }
    #[doc = "Bit 12 - Shifter Size"]
    #[inline(always)]
    #[must_use]
    pub fn ssize(&mut self) -> SSIZE_W<12> {
        SSIZE_W::new(self)
    }
    #[doc = "Bits 16:19 - Parallel Width"]
    #[inline(always)]
    #[must_use]
    pub fn pwidth(&mut self) -> PWIDTH_W<16> {
        PWIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Configuration N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftcfg](index.html) module"]
pub struct SHIFTCFG_SPEC;
impl crate::RegisterSpec for SHIFTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftcfg::R](R) reader structure"]
impl crate::Readable for SHIFTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftcfg::W](W) writer structure"]
impl crate::Writable for SHIFTCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHIFTCFG[%s]
to value 0"]
impl crate::Resettable for SHIFTCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
