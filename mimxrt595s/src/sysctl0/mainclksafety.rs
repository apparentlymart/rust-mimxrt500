#[doc = "Register `MAINCLKSAFETY` reader"]
pub struct R(crate::R<MAINCLKSAFETY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAINCLKSAFETY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAINCLKSAFETY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAINCLKSAFETY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAINCLKSAFETY` writer"]
pub struct W(crate::W<MAINCLKSAFETY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAINCLKSAFETY_SPEC>;
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
impl From<crate::W<MAINCLKSAFETY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAINCLKSAFETY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DELAY` reader - Main Clock turn on delay for Deep Sleep wake up"]
pub type DELAY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DELAY` writer - Main Clock turn on delay for Deep Sleep wake up"]
pub type DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAINCLKSAFETY_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Main Clock turn on delay for Deep Sleep wake up"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Main Clock turn on delay for Deep Sleep wake up"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<0> {
        DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Clock Safety\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainclksafety](index.html) module"]
pub struct MAINCLKSAFETY_SPEC;
impl crate::RegisterSpec for MAINCLKSAFETY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mainclksafety::R](R) reader structure"]
impl crate::Readable for MAINCLKSAFETY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mainclksafety::W](W) writer structure"]
impl crate::Writable for MAINCLKSAFETY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAINCLKSAFETY to value 0"]
impl crate::Resettable for MAINCLKSAFETY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
