#[doc = "Register `PINOUTD` reader"]
pub struct R(crate::R<PINOUTD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINOUTD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINOUTD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINOUTD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINOUTD` writer"]
pub struct W(crate::W<PINOUTD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINOUTD_SPEC>;
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
impl From<crate::W<PINOUTD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINOUTD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTD` reader - Output Data"]
pub type OUTD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OUTD` writer - Output Data"]
pub type OUTD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PINOUTD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Output Data"]
    #[inline(always)]
    pub fn outd(&self) -> OUTD_R {
        OUTD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn outd(&mut self) -> OUTD_W<0> {
        OUTD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Output Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinoutd](index.html) module"]
pub struct PINOUTD_SPEC;
impl crate::RegisterSpec for PINOUTD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinoutd::R](R) reader structure"]
impl crate::Readable for PINOUTD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinoutd::W](W) writer structure"]
impl crate::Writable for PINOUTD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINOUTD to value 0"]
impl crate::Resettable for PINOUTD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
