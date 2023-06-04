#[doc = "Register `REG0_TOP` reader"]
pub struct R(crate::R<REG0_TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG0_TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG0_TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG0_TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG0_TOP` writer"]
pub struct W(crate::W<REG0_TOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG0_TOP_SPEC>;
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
impl From<crate::W<REG0_TOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG0_TOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG0_TOP` reader - Upper Limit Of Region 0"]
pub type REG0_TOP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REG0_TOP` writer - Upper Limit Of Region 0"]
pub type REG0_TOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG0_TOP_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 10:26 - Upper Limit Of Region 0"]
    #[inline(always)]
    pub fn reg0_top(&self) -> REG0_TOP_R {
        REG0_TOP_R::new((self.bits >> 10) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 10:26 - Upper Limit Of Region 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg0_top(&mut self) -> REG0_TOP_W<10> {
        REG0_TOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Region 0 Top Boundary\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg0_top](index.html) module"]
pub struct REG0_TOP_SPEC;
impl crate::RegisterSpec for REG0_TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg0_top::R](R) reader structure"]
impl crate::Readable for REG0_TOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg0_top::W](W) writer structure"]
impl crate::Writable for REG0_TOP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG0_TOP to value 0x02aa_a800"]
impl crate::Resettable for REG0_TOP_SPEC {
    const RESET_VALUE: Self::Ux = 0x02aa_a800;
}
