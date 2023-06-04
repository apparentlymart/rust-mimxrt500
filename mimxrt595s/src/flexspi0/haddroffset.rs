#[doc = "Register `HADDROFFSET` reader"]
pub struct R(crate::R<HADDROFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HADDROFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HADDROFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HADDROFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HADDROFFSET` writer"]
pub struct W(crate::W<HADDROFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HADDROFFSET_SPEC>;
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
impl From<crate::W<HADDROFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HADDROFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDROFFSET` reader - HADDR offset field, remapped address will be ADDR\\[31:12\\]=ADDR_original\\[31:12\\]+ADDROFFSET"]
pub type ADDROFFSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDROFFSET` writer - HADDR offset field, remapped address will be ADDR\\[31:12\\]=ADDR_original\\[31:12\\]+ADDROFFSET"]
pub type ADDROFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HADDROFFSET_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 12:31 - HADDR offset field, remapped address will be ADDR\\[31:12\\]=ADDR_original\\[31:12\\]+ADDROFFSET"]
    #[inline(always)]
    pub fn addroffset(&self) -> ADDROFFSET_R {
        ADDROFFSET_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - HADDR offset field, remapped address will be ADDR\\[31:12\\]=ADDR_original\\[31:12\\]+ADDROFFSET"]
    #[inline(always)]
    #[must_use]
    pub fn addroffset(&mut self) -> ADDROFFSET_W<12> {
        ADDROFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HADDR REMAP OFFSET\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [haddroffset](index.html) module"]
pub struct HADDROFFSET_SPEC;
impl crate::RegisterSpec for HADDROFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [haddroffset::R](R) reader structure"]
impl crate::Readable for HADDROFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [haddroffset::W](W) writer structure"]
impl crate::Writable for HADDROFFSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HADDROFFSET to value 0"]
impl crate::Resettable for HADDROFFSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
