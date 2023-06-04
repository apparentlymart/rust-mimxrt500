#[doc = "Register `DBG_LOCKEN` reader"]
pub struct R(crate::R<DBG_LOCKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_LOCKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_LOCKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_LOCKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG_LOCKEN` writer"]
pub struct W(crate::W<DBG_LOCKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_LOCKEN_SPEC>;
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
impl From<crate::W<DBG_LOCKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_LOCKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_LOCKEN` reader - Debug Write Lock the following registers"]
pub type DBG_LOCKEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBG_LOCKEN` writer - Debug Write Lock the following registers"]
pub type DBG_LOCKEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBG_LOCKEN_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Debug Write Lock the following registers"]
    #[inline(always)]
    pub fn dbg_locken(&self) -> DBG_LOCKEN_R {
        DBG_LOCKEN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Debug Write Lock the following registers"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_locken(&mut self) -> DBG_LOCKEN_W<0> {
        DBG_LOCKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Lock Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_locken](index.html) module"]
pub struct DBG_LOCKEN_SPEC;
impl crate::RegisterSpec for DBG_LOCKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_locken::R](R) reader structure"]
impl crate::Readable for DBG_LOCKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_locken::W](W) writer structure"]
impl crate::Writable for DBG_LOCKEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBG_LOCKEN to value 0x0a"]
impl crate::Resettable for DBG_LOCKEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
