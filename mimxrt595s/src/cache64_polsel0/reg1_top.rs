#[doc = "Register `REG1_TOP` reader"]
pub struct R(crate::R<REG1_TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG1_TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG1_TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG1_TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG1_TOP` writer"]
pub struct W(crate::W<REG1_TOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG1_TOP_SPEC>;
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
impl From<crate::W<REG1_TOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG1_TOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG1_TOP` reader - Upper Limit Of Region 1"]
pub type REG1_TOP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REG1_TOP` writer - Upper Limit Of Region 1"]
pub type REG1_TOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG1_TOP_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 10:26 - Upper Limit Of Region 1"]
    #[inline(always)]
    pub fn reg1_top(&self) -> REG1_TOP_R {
        REG1_TOP_R::new((self.bits >> 10) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 10:26 - Upper Limit Of Region 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg1_top(&mut self) -> REG1_TOP_W<10> {
        REG1_TOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Region 1 Top Boundary\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg1_top](index.html) module"]
pub struct REG1_TOP_SPEC;
impl crate::RegisterSpec for REG1_TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg1_top::R](R) reader structure"]
impl crate::Readable for REG1_TOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg1_top::W](W) writer structure"]
impl crate::Writable for REG1_TOP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG1_TOP to value 0x0555_5400"]
impl crate::Resettable for REG1_TOP_SPEC {
    const RESET_VALUE: Self::Ux = 0x0555_5400;
}
