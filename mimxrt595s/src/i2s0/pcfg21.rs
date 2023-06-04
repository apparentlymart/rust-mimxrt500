#[doc = "Register `PCFG21` reader"]
pub struct R(crate::R<PCFG21_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCFG21_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCFG21_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCFG21_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCFG21` writer"]
pub struct W(crate::W<PCFG21_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCFG21_SPEC>;
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
impl From<crate::W<PCFG21_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCFG21_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POSITION` reader - Data Position"]
pub type POSITION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `POSITION` writer - Data Position"]
pub type POSITION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCFG21_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 16:24 - Data Position"]
    #[inline(always)]
    pub fn position(&self) -> POSITION_R {
        POSITION_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:24 - Data Position"]
    #[inline(always)]
    #[must_use]
    pub fn position(&mut self) -> POSITION_W<16> {
        POSITION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register 2 for Channel Pair 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcfg21](index.html) module"]
pub struct PCFG21_SPEC;
impl crate::RegisterSpec for PCFG21_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcfg21::R](R) reader structure"]
impl crate::Readable for PCFG21_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcfg21::W](W) writer structure"]
impl crate::Writable for PCFG21_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCFG21 to value 0"]
impl crate::Resettable for PCFG21_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
