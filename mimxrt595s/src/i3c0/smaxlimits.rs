#[doc = "Register `SMAXLIMITS` reader"]
pub struct R(crate::R<SMAXLIMITS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMAXLIMITS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMAXLIMITS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMAXLIMITS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMAXLIMITS` writer"]
pub struct W(crate::W<SMAXLIMITS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMAXLIMITS_SPEC>;
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
impl From<crate::W<SMAXLIMITS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMAXLIMITS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXRD` reader - Maximum read length"]
pub type MAXRD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MAXRD` writer - Maximum read length"]
pub type MAXRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMAXLIMITS_SPEC, u16, u16, 12, O>;
#[doc = "Field `MAXWR` reader - Maximum write length"]
pub type MAXWR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MAXWR` writer - Maximum write length"]
pub type MAXWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMAXLIMITS_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Maximum read length"]
    #[inline(always)]
    pub fn maxrd(&self) -> MAXRD_R {
        MAXRD_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Maximum write length"]
    #[inline(always)]
    pub fn maxwr(&self) -> MAXWR_R {
        MAXWR_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Maximum read length"]
    #[inline(always)]
    #[must_use]
    pub fn maxrd(&mut self) -> MAXRD_W<0> {
        MAXRD_W::new(self)
    }
    #[doc = "Bits 16:27 - Maximum write length"]
    #[inline(always)]
    #[must_use]
    pub fn maxwr(&mut self) -> MAXWR_W<16> {
        MAXWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Maximum Limits Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smaxlimits](index.html) module"]
pub struct SMAXLIMITS_SPEC;
impl crate::RegisterSpec for SMAXLIMITS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smaxlimits::R](R) reader structure"]
impl crate::Readable for SMAXLIMITS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smaxlimits::W](W) writer structure"]
impl crate::Writable for SMAXLIMITS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMAXLIMITS to value 0"]
impl crate::Resettable for SMAXLIMITS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
