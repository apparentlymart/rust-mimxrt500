#[doc = "Register `STCCLOCK` reader"]
pub struct R(crate::R<STCCLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCCLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCCLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCCLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCCLOCK` writer"]
pub struct W(crate::W<STCCLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCCLOCK_SPEC>;
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
impl From<crate::W<STCCLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCCLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACCURACY` reader - Clock accuracy"]
pub type ACCURACY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACCURACY` writer - Clock accuracy"]
pub type ACCURACY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STCCLOCK_SPEC, u8, u8, 8, O>;
#[doc = "Field `FREQ` reader - Clock frequency"]
pub type FREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FREQ` writer - Clock frequency"]
pub type FREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STCCLOCK_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Clock accuracy"]
    #[inline(always)]
    pub fn accuracy(&self) -> ACCURACY_R {
        ACCURACY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock accuracy"]
    #[inline(always)]
    #[must_use]
    pub fn accuracy(&mut self) -> ACCURACY_W<0> {
        ACCURACY_W::new(self)
    }
    #[doc = "Bits 8:15 - Clock frequency"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<8> {
        FREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Time Control Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcclock](index.html) module"]
pub struct STCCLOCK_SPEC;
impl crate::RegisterSpec for STCCLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcclock::R](R) reader structure"]
impl crate::Readable for STCCLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stcclock::W](W) writer structure"]
impl crate::Writable for STCCLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STCCLOCK to value 0x0214"]
impl crate::Resettable for STCCLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0214;
}
