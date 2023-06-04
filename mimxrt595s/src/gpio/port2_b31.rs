#[doc = "Register `Port2_B31` reader"]
pub struct R(crate::R<PORT2_B31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT2_B31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT2_B31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT2_B31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Port2_B31` writer"]
pub struct W(crate::W<PORT2_B31_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT2_B31_SPEC>;
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
impl From<crate::W<PORT2_B31_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT2_B31_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBYTE` reader - Port Byte"]
pub type PBYTE_R = crate::BitReader<bool>;
#[doc = "Field `PBYTE` writer - Port Byte"]
pub type PBYTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, PORT2_B31_SPEC, bool, O>;
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
#[doc = "Byte pin registers for all port GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port2_b31](index.html) module"]
pub struct PORT2_B31_SPEC;
impl crate::RegisterSpec for PORT2_B31_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [port2_b31::R](R) reader structure"]
impl crate::Readable for PORT2_B31_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port2_b31::W](W) writer structure"]
impl crate::Writable for PORT2_B31_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Port2_B31 to value 0"]
impl crate::Resettable for PORT2_B31_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
