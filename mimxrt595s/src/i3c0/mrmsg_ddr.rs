#[doc = "Register `MRMSG_DDR` reader"]
pub struct R(crate::R<MRMSG_DDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRMSG_DDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRMSG_DDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRMSG_DDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MRMSG_DDR` writer"]
pub struct W(crate::W<MRMSG_DDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MRMSG_DDR_SPEC>;
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
impl From<crate::W<MRMSG_DDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MRMSG_DDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Data"]
pub type DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA` writer - Data"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRMSG_DDR_SPEC, u16, u16, 16, O>;
#[doc = "Field `CLEN` reader - Current length"]
pub type CLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLEN` writer - Current length"]
pub type CLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRMSG_DDR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:15 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - Current length"]
    #[inline(always)]
    pub fn clen(&self) -> CLEN_R {
        CLEN_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Bits 16:25 - Current length"]
    #[inline(always)]
    #[must_use]
    pub fn clen(&mut self) -> CLEN_W<16> {
        CLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Read Message in DDR mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrmsg_ddr](index.html) module"]
pub struct MRMSG_DDR_SPEC;
impl crate::RegisterSpec for MRMSG_DDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mrmsg_ddr::R](R) reader structure"]
impl crate::Readable for MRMSG_DDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mrmsg_ddr::W](W) writer structure"]
impl crate::Writable for MRMSG_DDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MRMSG_DDR to value 0"]
impl crate::Resettable for MRMSG_DDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
