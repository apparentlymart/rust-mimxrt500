#[doc = "Register `MWMSG_SDR_CONTROL` writer"]
pub struct W(crate::W<MWMSG_SDR_MWMSG_SDR_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MWMSG_SDR_MWMSG_SDR_CONTROL_SPEC>;
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
impl From<crate::W<MWMSG_SDR_MWMSG_SDR_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MWMSG_SDR_MWMSG_SDR_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_AW {
    #[doc = "0: Write"]
    WRITE = 0,
    #[doc = "1: Read"]
    READ = 1,
}
impl From<DIR_AW> for bool {
    #[inline(always)]
    fn from(variant: DIR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` writer - Direction"]
pub type DIR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MWMSG_SDR_MWMSG_SDR_CONTROL_SPEC, DIR_AW, O>;
impl<'a, const O: u8> DIR_W<'a, O> {
    #[doc = "Write"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(DIR_AW::WRITE)
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(DIR_AW::READ)
    }
}
#[doc = "Field `ADDR` writer - Address to be written to"]
pub type ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MWMSG_SDR_MWMSG_SDR_CONTROL_SPEC, u8, u8, 7, O>;
#[doc = "Field `END` writer - End of SDR message"]
pub type END_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MWMSG_SDR_MWMSG_SDR_CONTROL_SPEC, bool, O>;
#[doc = "I2C\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_AW {
    #[doc = "0: I3C message"]
    I3CMESSAGE = 0,
    #[doc = "1: I2C message"]
    I2CMESSAGE = 1,
}
impl From<I2C_AW> for bool {
    #[inline(always)]
    fn from(variant: I2C_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C` writer - I2C"]
pub type I2C_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MWMSG_SDR_MWMSG_SDR_CONTROL_SPEC, I2C_AW, O>;
impl<'a, const O: u8> I2C_W<'a, O> {
    #[doc = "I3C message"]
    #[inline(always)]
    pub fn i3cmessage(self) -> &'a mut W {
        self.variant(I2C_AW::I3CMESSAGE)
    }
    #[doc = "I2C message"]
    #[inline(always)]
    pub fn i2cmessage(self) -> &'a mut W {
        self.variant(I2C_AW::I2CMESSAGE)
    }
}
#[doc = "Field `LEN` writer - Length"]
pub type LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MWMSG_SDR_MWMSG_SDR_CONTROL_SPEC, u8, u8, 5, O>;
impl W {
    #[doc = "Bit 0 - Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<0> {
        DIR_W::new(self)
    }
    #[doc = "Bits 1:7 - Address to be written to"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<1> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 8 - End of SDR message"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> END_W<8> {
        END_W::new(self)
    }
    #[doc = "Bit 10 - I2C"]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2C_W<10> {
        I2C_W::new(self)
    }
    #[doc = "Bits 11:15 - Length"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<11> {
        LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Write Message in SDR mode\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mwmsg_sdr_mwmsg_sdr_control](index.html) module"]
pub struct MWMSG_SDR_MWMSG_SDR_CONTROL_SPEC;
impl crate::RegisterSpec for MWMSG_SDR_MWMSG_SDR_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mwmsg_sdr_mwmsg_sdr_control::W](W) writer structure"]
impl crate::Writable for MWMSG_SDR_MWMSG_SDR_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MWMSG_SDR_CONTROL to value 0"]
impl crate::Resettable for MWMSG_SDR_MWMSG_SDR_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
