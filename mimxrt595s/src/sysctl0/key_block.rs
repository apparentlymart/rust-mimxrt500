#[doc = "Register `KEY_BLOCK` reader"]
pub struct R(crate::R<KEY_BLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY_BLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY_BLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY_BLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY_BLOCK` writer"]
pub struct W(crate::W<KEY_BLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY_BLOCK_SPEC>;
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
impl From<crate::W<KEY_BLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY_BLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY_BLOCK` reader - PUF key and data output"]
pub type KEY_BLOCK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEY_BLOCK` writer - PUF key and data output"]
pub type KEY_BLOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, KEY_BLOCK_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - PUF key and data output"]
    #[inline(always)]
    pub fn key_block(&self) -> KEY_BLOCK_R {
        KEY_BLOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PUF key and data output"]
    #[inline(always)]
    #[must_use]
    pub fn key_block(&mut self) -> KEY_BLOCK_W<0> {
        KEY_BLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key block\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key_block](index.html) module"]
pub struct KEY_BLOCK_SPEC;
impl crate::RegisterSpec for KEY_BLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key_block::R](R) reader structure"]
impl crate::Readable for KEY_BLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key_block::W](W) writer structure"]
impl crate::Writable for KEY_BLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY_BLOCK to value 0x3cc3_5aa5"]
impl crate::Resettable for KEY_BLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0x3cc3_5aa5;
}
