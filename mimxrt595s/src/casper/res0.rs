#[doc = "Register `RES0` reader"]
pub struct R(crate::R<RES0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RES0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RES0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RES0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RES0` writer"]
pub struct W(crate::W<RES0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RES0_SPEC>;
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
impl From<crate::W<RES0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RES0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_VALUE` reader - Register Value"]
pub type REG_VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REG_VALUE` writer - Register Value"]
pub type REG_VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RES0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Register Value"]
    #[inline(always)]
    pub fn reg_value(&self) -> REG_VALUE_R {
        REG_VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register Value"]
    #[inline(always)]
    #[must_use]
    pub fn reg_value(&mut self) -> REG_VALUE_W<0> {
        REG_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Result Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res0](index.html) module"]
pub struct RES0_SPEC;
impl crate::RegisterSpec for RES0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [res0::R](R) reader structure"]
impl crate::Readable for RES0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [res0::W](W) writer structure"]
impl crate::Writable for RES0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RES0 to value 0"]
impl crate::Resettable for RES0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
