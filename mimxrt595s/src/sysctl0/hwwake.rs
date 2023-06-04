#[doc = "Register `HWWAKE` reader"]
pub struct R(crate::R<HWWAKE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWWAKE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWWAKE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWWAKE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWWAKE` writer"]
pub struct W(crate::W<HWWAKE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWWAKE_SPEC>;
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
impl From<crate::W<HWWAKE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWWAKE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCEWAKE` reader - Force peripheral clocking to stay on during deep-sleep mode."]
pub type FORCEWAKE_R = crate::BitReader<bool>;
#[doc = "Field `FORCEWAKE` writer - Force peripheral clocking to stay on during deep-sleep mode."]
pub type FORCEWAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HWWAKE_SPEC, bool, O>;
#[doc = "Field `FCWAKE` reader - Wake for FlexComm Interfaces."]
pub type FCWAKE_R = crate::BitReader<bool>;
#[doc = "Field `FCWAKE` writer - Wake for FlexComm Interfaces."]
pub type FCWAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HWWAKE_SPEC, bool, O>;
#[doc = "Field `DMICWAKE` reader - Wake for Digital Microphone."]
pub type DMICWAKE_R = crate::BitReader<bool>;
#[doc = "Field `DMICWAKE` writer - Wake for Digital Microphone."]
pub type DMICWAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HWWAKE_SPEC, bool, O>;
#[doc = "Field `DMAC0WAKE` reader - Wake for DMAC0."]
pub type DMAC0WAKE_R = crate::BitReader<bool>;
#[doc = "Field `DMAC0WAKE` writer - Wake for DMAC0."]
pub type DMAC0WAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HWWAKE_SPEC, bool, O>;
#[doc = "Field `DMAC1WAKE` reader - Wake for DMAC1."]
pub type DMAC1WAKE_R = crate::BitReader<bool>;
#[doc = "Field `DMAC1WAKE` writer - Wake for DMAC1."]
pub type DMAC1WAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HWWAKE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Force peripheral clocking to stay on during deep-sleep mode."]
    #[inline(always)]
    pub fn forcewake(&self) -> FORCEWAKE_R {
        FORCEWAKE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake for FlexComm Interfaces."]
    #[inline(always)]
    pub fn fcwake(&self) -> FCWAKE_R {
        FCWAKE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake for Digital Microphone."]
    #[inline(always)]
    pub fn dmicwake(&self) -> DMICWAKE_R {
        DMICWAKE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake for DMAC0."]
    #[inline(always)]
    pub fn dmac0wake(&self) -> DMAC0WAKE_R {
        DMAC0WAKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake for DMAC1."]
    #[inline(always)]
    pub fn dmac1wake(&self) -> DMAC1WAKE_R {
        DMAC1WAKE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force peripheral clocking to stay on during deep-sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn forcewake(&mut self) -> FORCEWAKE_W<0> {
        FORCEWAKE_W::new(self)
    }
    #[doc = "Bit 1 - Wake for FlexComm Interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn fcwake(&mut self) -> FCWAKE_W<1> {
        FCWAKE_W::new(self)
    }
    #[doc = "Bit 2 - Wake for Digital Microphone."]
    #[inline(always)]
    #[must_use]
    pub fn dmicwake(&mut self) -> DMICWAKE_W<2> {
        DMICWAKE_W::new(self)
    }
    #[doc = "Bit 3 - Wake for DMAC0."]
    #[inline(always)]
    #[must_use]
    pub fn dmac0wake(&mut self) -> DMAC0WAKE_W<3> {
        DMAC0WAKE_W::new(self)
    }
    #[doc = "Bit 4 - Wake for DMAC1."]
    #[inline(always)]
    #[must_use]
    pub fn dmac1wake(&mut self) -> DMAC1WAKE_W<4> {
        DMAC1WAKE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware Wake\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwwake](index.html) module"]
pub struct HWWAKE_SPEC;
impl crate::RegisterSpec for HWWAKE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwwake::R](R) reader structure"]
impl crate::Readable for HWWAKE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwwake::W](W) writer structure"]
impl crate::Writable for HWWAKE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWWAKE to value 0"]
impl crate::Resettable for HWWAKE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
