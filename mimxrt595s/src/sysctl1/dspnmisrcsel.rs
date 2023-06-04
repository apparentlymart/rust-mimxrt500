#[doc = "Register `DSPNMISRCSEL` reader"]
pub struct R(crate::R<DSPNMISRCSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSPNMISRCSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSPNMISRCSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSPNMISRCSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSPNMISRCSEL` writer"]
pub struct W(crate::W<DSPNMISRCSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSPNMISRCSEL_SPEC>;
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
impl From<crate::W<DSPNMISRCSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSPNMISRCSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NMISRCSEL` reader - DSP NMI source selection"]
pub type NMISRCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NMISRCSEL` writer - DSP NMI source selection"]
pub type NMISRCSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DSPNMISRCSEL_SPEC, u8, u8, 5, O>;
#[doc = "Field `NMIEN` reader - NMI Enable"]
pub type NMIEN_R = crate::BitReader<NMIEN_A>;
#[doc = "NMI Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMIEN_A {
    #[doc = "0: Disable NMI Interrupt"]
    NMIEN_0 = 0,
    #[doc = "1: Enable NMI Interrupt"]
    NMIEN_1 = 1,
}
impl From<NMIEN_A> for bool {
    #[inline(always)]
    fn from(variant: NMIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NMIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIEN_A {
        match self.bits {
            false => NMIEN_A::NMIEN_0,
            true => NMIEN_A::NMIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `NMIEN_0`"]
    #[inline(always)]
    pub fn is_nmien_0(&self) -> bool {
        *self == NMIEN_A::NMIEN_0
    }
    #[doc = "Checks if the value of the field is `NMIEN_1`"]
    #[inline(always)]
    pub fn is_nmien_1(&self) -> bool {
        *self == NMIEN_A::NMIEN_1
    }
}
#[doc = "Field `NMIEN` writer - NMI Enable"]
pub type NMIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSPNMISRCSEL_SPEC, NMIEN_A, O>;
impl<'a, const O: u8> NMIEN_W<'a, O> {
    #[doc = "Disable NMI Interrupt"]
    #[inline(always)]
    pub fn nmien_0(self) -> &'a mut W {
        self.variant(NMIEN_A::NMIEN_0)
    }
    #[doc = "Enable NMI Interrupt"]
    #[inline(always)]
    pub fn nmien_1(self) -> &'a mut W {
        self.variant(NMIEN_A::NMIEN_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - DSP NMI source selection"]
    #[inline(always)]
    pub fn nmisrcsel(&self) -> NMISRCSEL_R {
        NMISRCSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - NMI Enable"]
    #[inline(always)]
    pub fn nmien(&self) -> NMIEN_R {
        NMIEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - DSP NMI source selection"]
    #[inline(always)]
    #[must_use]
    pub fn nmisrcsel(&mut self) -> NMISRCSEL_W<0> {
        NMISRCSEL_W::new(self)
    }
    #[doc = "Bit 31 - NMI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nmien(&mut self) -> NMIEN_W<31> {
        NMIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSP NMI source selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dspnmisrcsel](index.html) module"]
pub struct DSPNMISRCSEL_SPEC;
impl crate::RegisterSpec for DSPNMISRCSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dspnmisrcsel::R](R) reader structure"]
impl crate::Readable for DSPNMISRCSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dspnmisrcsel::W](W) writer structure"]
impl crate::Writable for DSPNMISRCSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSPNMISRCSEL to value 0x05"]
impl crate::Resettable for DSPNMISRCSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
