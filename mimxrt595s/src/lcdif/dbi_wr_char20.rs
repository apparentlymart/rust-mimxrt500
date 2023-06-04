#[doc = "Register `DbiWrChar20` reader"]
pub struct R(crate::R<DBI_WR_CHAR20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBI_WR_CHAR20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBI_WR_CHAR20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBI_WR_CHAR20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DbiWrChar20` writer"]
pub struct W(crate::W<DBI_WR_CHAR20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBI_WR_CHAR20_SPEC>;
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
impl From<crate::W<DBI_WR_CHAR20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBI_WR_CHAR20_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBI_WR_EOR_WR_DE_ASRT` reader - Cycle number=Setting*(DbiAcTimeUnit+1)."]
pub type DBI_WR_EOR_WR_DE_ASRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBI_WR_EOR_WR_DE_ASRT` writer - Cycle number=Setting*(DbiAcTimeUnit+1)."]
pub type DBI_WR_EOR_WR_DE_ASRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBI_WR_CHAR20_SPEC, u8, u8, 8, O>;
#[doc = "Field `DBI_WR_CS_DE_ASRT` reader - Cycle number=Setting*(DbiAcTimeUnit+1)."]
pub type DBI_WR_CS_DE_ASRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBI_WR_CS_DE_ASRT` writer - Cycle number=Setting*(DbiAcTimeUnit+1)."]
pub type DBI_WR_CS_DE_ASRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBI_WR_CHAR20_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Cycle number=Setting*(DbiAcTimeUnit+1)."]
    #[inline(always)]
    pub fn dbi_wr_eor_wr_de_asrt(&self) -> DBI_WR_EOR_WR_DE_ASRT_R {
        DBI_WR_EOR_WR_DE_ASRT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Cycle number=Setting*(DbiAcTimeUnit+1)."]
    #[inline(always)]
    pub fn dbi_wr_cs_de_asrt(&self) -> DBI_WR_CS_DE_ASRT_R {
        DBI_WR_CS_DE_ASRT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Cycle number=Setting*(DbiAcTimeUnit+1)."]
    #[inline(always)]
    #[must_use]
    pub fn dbi_wr_eor_wr_de_asrt(&mut self) -> DBI_WR_EOR_WR_DE_ASRT_W<0> {
        DBI_WR_EOR_WR_DE_ASRT_W::new(self)
    }
    #[doc = "Bits 8:15 - Cycle number=Setting*(DbiAcTimeUnit+1)."]
    #[inline(always)]
    #[must_use]
    pub fn dbi_wr_cs_de_asrt(&mut self) -> DBI_WR_CS_DE_ASRT_W<8> {
        DBI_WR_CS_DE_ASRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBI Write Characteristics 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbi_wr_char20](index.html) module"]
pub struct DBI_WR_CHAR20_SPEC;
impl crate::RegisterSpec for DBI_WR_CHAR20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbi_wr_char20::R](R) reader structure"]
impl crate::Readable for DBI_WR_CHAR20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbi_wr_char20::W](W) writer structure"]
impl crate::Writable for DBI_WR_CHAR20_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DbiWrChar20 to value 0"]
impl crate::Resettable for DBI_WR_CHAR20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
