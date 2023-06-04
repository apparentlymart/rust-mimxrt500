#[doc = "Register `Port0_B1` reader"]
pub struct R(crate::R<PORT0_B1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT0_B1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT0_B1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT0_B1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Port0_B1` writer"]
pub struct W(crate::W<PORT0_B1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT0_B1_SPEC>;
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
impl From<crate::W<PORT0_B1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT0_B1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBYTE` reader - Port Byte"]
pub type PBYTE_R = crate::BitReader<bool>;
#[doc = "Field `PBYTE` writer - Port Byte"]
pub type PBYTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORT0_B1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Port Byte"]
    #[inline(always)]
    pub fn pbyte(&self) -> PBYTE_R {
        PBYTE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Byte"]
    #[inline(always)]
    #[must_use]
    pub fn pbyte(&mut self) -> PBYTE_W<0> {
        PBYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Byte pin registers for all port GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port0_b1](index.html) module"]
pub struct PORT0_B1_SPEC;
impl crate::RegisterSpec for PORT0_B1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [port0_b1::R](R) reader structure"]
impl crate::Readable for PORT0_B1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port0_b1::W](W) writer structure"]
impl crate::Writable for PORT0_B1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Port0_B1 to value 0"]
impl crate::Resettable for PORT0_B1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
