#[doc = "Register `DEBUG0` reader"]
pub struct R(crate::R<DEBUG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG0` writer"]
pub struct W(crate::W<DEBUG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG0_SPEC>;
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
impl From<crate::W<DEBUG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEBUG_INTERFACE_HOLD` reader - Debug interface"]
pub type DEBUG_INTERFACE_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `DEBUG_INTERFACE_HOLD` writer - Debug interface"]
pub type DEBUG_INTERFACE_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG0_SPEC, bool, O>;
#[doc = "Field `HSTPULLDOWN` reader - HS DP/DM pulldown resistance select."]
pub type HSTPULLDOWN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSTPULLDOWN` writer - HS DP/DM pulldown resistance select."]
pub type HSTPULLDOWN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEBUG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `ENHSTPULLDOWN` reader - Enable Host pulldown"]
pub type ENHSTPULLDOWN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENHSTPULLDOWN` writer - Enable Host pulldown"]
pub type ENHSTPULLDOWN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEBUG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX2RXCOUNT` reader - TX2RXCOUNT"]
pub type TX2RXCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX2RXCOUNT` writer - TX2RXCOUNT"]
pub type TX2RXCOUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEBUG0_SPEC, u8, u8, 4, O>;
#[doc = "Field `ENTX2RXCOUNT` reader - Set this bit to allow a countdown to transition in between TX and RX."]
pub type ENTX2RXCOUNT_R = crate::BitReader<bool>;
#[doc = "Field `ENTX2RXCOUNT` writer - Set this bit to allow a countdown to transition in between TX and RX."]
pub type ENTX2RXCOUNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG0_SPEC, bool, O>;
#[doc = "Field `SQUELCHRESETCOUNT` reader - Squelch reset count"]
pub type SQUELCHRESETCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQUELCHRESETCOUNT` writer - Squelch reset count"]
pub type SQUELCHRESETCOUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG0_SPEC, u8, u8, 5, O>;
#[doc = "Field `ENSQUELCHRESET` reader - Enable squelch reset"]
pub type ENSQUELCHRESET_R = crate::BitReader<bool>;
#[doc = "Field `ENSQUELCHRESET` writer - Enable squelch reset"]
pub type ENSQUELCHRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG0_SPEC, bool, O>;
#[doc = "Field `SQUELCHRESETLENGTH` reader - Squelch reset length"]
pub type SQUELCHRESETLENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQUELCHRESETLENGTH` writer - Squelch reset length"]
pub type SQUELCHRESETLENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG0_SPEC, u8, u8, 4, O>;
#[doc = "Field `HOST_RESUME_DEBUG` reader - Host resume"]
pub type HOST_RESUME_DEBUG_R = crate::BitReader<bool>;
#[doc = "Field `HOST_RESUME_DEBUG` writer - Host resume"]
pub type HOST_RESUME_DEBUG_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG0_SPEC, bool, O>;
#[doc = "Field `CLKGATE` reader - Test clock gate"]
pub type CLKGATE_R = crate::BitReader<bool>;
#[doc = "Field `CLKGATE` writer - Test clock gate"]
pub type CLKGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Debug interface"]
    #[inline(always)]
    pub fn debug_interface_hold(&self) -> DEBUG_INTERFACE_HOLD_R {
        DEBUG_INTERFACE_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - HS DP/DM pulldown resistance select."]
    #[inline(always)]
    pub fn hstpulldown(&self) -> HSTPULLDOWN_R {
        HSTPULLDOWN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Enable Host pulldown"]
    #[inline(always)]
    pub fn enhstpulldown(&self) -> ENHSTPULLDOWN_R {
        ENHSTPULLDOWN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - TX2RXCOUNT"]
    #[inline(always)]
    pub fn tx2rxcount(&self) -> TX2RXCOUNT_R {
        TX2RXCOUNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    pub fn entx2rxcount(&self) -> ENTX2RXCOUNT_R {
        ENTX2RXCOUNT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Squelch reset count"]
    #[inline(always)]
    pub fn squelchresetcount(&self) -> SQUELCHRESETCOUNT_R {
        SQUELCHRESETCOUNT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Enable squelch reset"]
    #[inline(always)]
    pub fn ensquelchreset(&self) -> ENSQUELCHRESET_R {
        ENSQUELCHRESET_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - Squelch reset length"]
    #[inline(always)]
    pub fn squelchresetlength(&self) -> SQUELCHRESETLENGTH_R {
        SQUELCHRESETLENGTH_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Host resume"]
    #[inline(always)]
    pub fn host_resume_debug(&self) -> HOST_RESUME_DEBUG_R {
        HOST_RESUME_DEBUG_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Test clock gate"]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Debug interface"]
    #[inline(always)]
    #[must_use]
    pub fn debug_interface_hold(&mut self) -> DEBUG_INTERFACE_HOLD_W<1> {
        DEBUG_INTERFACE_HOLD_W::new(self)
    }
    #[doc = "Bits 2:3 - HS DP/DM pulldown resistance select."]
    #[inline(always)]
    #[must_use]
    pub fn hstpulldown(&mut self) -> HSTPULLDOWN_W<2> {
        HSTPULLDOWN_W::new(self)
    }
    #[doc = "Bits 4:5 - Enable Host pulldown"]
    #[inline(always)]
    #[must_use]
    pub fn enhstpulldown(&mut self) -> ENHSTPULLDOWN_W<4> {
        ENHSTPULLDOWN_W::new(self)
    }
    #[doc = "Bits 8:11 - TX2RXCOUNT"]
    #[inline(always)]
    #[must_use]
    pub fn tx2rxcount(&mut self) -> TX2RXCOUNT_W<8> {
        TX2RXCOUNT_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    #[must_use]
    pub fn entx2rxcount(&mut self) -> ENTX2RXCOUNT_W<12> {
        ENTX2RXCOUNT_W::new(self)
    }
    #[doc = "Bits 16:20 - Squelch reset count"]
    #[inline(always)]
    #[must_use]
    pub fn squelchresetcount(&mut self) -> SQUELCHRESETCOUNT_W<16> {
        SQUELCHRESETCOUNT_W::new(self)
    }
    #[doc = "Bit 24 - Enable squelch reset"]
    #[inline(always)]
    #[must_use]
    pub fn ensquelchreset(&mut self) -> ENSQUELCHRESET_W<24> {
        ENSQUELCHRESET_W::new(self)
    }
    #[doc = "Bits 25:28 - Squelch reset length"]
    #[inline(always)]
    #[must_use]
    pub fn squelchresetlength(&mut self) -> SQUELCHRESETLENGTH_W<25> {
        SQUELCHRESETLENGTH_W::new(self)
    }
    #[doc = "Bit 29 - Host resume"]
    #[inline(always)]
    #[must_use]
    pub fn host_resume_debug(&mut self) -> HOST_RESUME_DEBUG_W<29> {
        HOST_RESUME_DEBUG_W::new(self)
    }
    #[doc = "Bit 30 - Test clock gate"]
    #[inline(always)]
    #[must_use]
    pub fn clkgate(&mut self) -> CLKGATE_W<30> {
        CLKGATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug0](index.html) module"]
pub struct DEBUG0_SPEC;
impl crate::RegisterSpec for DEBUG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug0::R](R) reader structure"]
impl crate::Readable for DEBUG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug0::W](W) writer structure"]
impl crate::Writable for DEBUG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBUG0 to value 0x7f18_0000"]
impl crate::Resettable for DEBUG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f18_0000;
}
