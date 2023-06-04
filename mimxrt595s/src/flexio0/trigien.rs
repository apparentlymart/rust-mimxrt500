#[doc = "Register `TRIGIEN` reader"]
pub struct R(crate::R<TRIGIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIGIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIGIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIGIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIGIEN` writer"]
pub struct W(crate::W<TRIGIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIGIEN_SPEC>;
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
impl From<crate::W<TRIGIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIGIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIE` reader - External Trigger Interrupt Enable"]
pub type TRIE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TRIE` writer - External Trigger Interrupt Enable"]
pub type TRIE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIGIEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - External Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn trie(&self) -> TRIE_R {
        TRIE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - External Trigger Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trie(&mut self) -> TRIE_W<0> {
        TRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Trigger Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigien](index.html) module"]
pub struct TRIGIEN_SPEC;
impl crate::RegisterSpec for TRIGIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trigien::R](R) reader structure"]
impl crate::Readable for TRIGIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trigien::W](W) writer structure"]
impl crate::Writable for TRIGIEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIGIEN to value 0"]
impl crate::Resettable for TRIGIEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
