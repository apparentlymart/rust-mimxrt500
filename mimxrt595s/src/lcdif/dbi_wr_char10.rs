#[doc = "Register `DbiWrChar10` reader"]
pub struct R(crate::R<DBI_WR_CHAR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBI_WR_CHAR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBI_WR_CHAR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBI_WR_CHAR10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DbiWrChar10` writer"]
pub struct W(crate::W<DBI_WR_CHAR10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBI_WR_CHAR10_SPEC>;
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
impl From<crate::W<DBI_WR_CHAR10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBI_WR_CHAR10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBI_WR_PERIOD` reader - Single Write Period Duration"]
pub type DBI_WR_PERIOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBI_WR_PERIOD` writer - Single Write Period Duration"]
pub type DBI_WR_PERIOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBI_WR_CHAR10_SPEC, u8, u8, 8, O>;
#[doc = "Field `DBI_WR_EOR_WR_ASSERT` reader - Cycle number=Setting*(DbiAcTimeUnit+1)."]
pub type DBI_WR_EOR_WR_ASSERT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBI_WR_EOR_WR_ASSERT` writer - Cycle number=Setting*(DbiAcTimeUnit+1)."]
pub type DBI_WR_EOR_WR_ASSERT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBI_WR_CHAR10_SPEC, u8, u8, 4, O>;
#[doc = "Field `DBI_WR_CS_ASSERT` reader - Cycle number=Setting*(DbiAcTimeUnit+1)."]
pub type DBI_WR_CS_ASSERT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBI_WR_CS_ASSERT` writer - Cycle number=Setting*(DbiAcTimeUnit+1)."]
pub type DBI_WR_CS_ASSERT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBI_WR_CHAR10_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - Single Write Period Duration"]
    #[inline(always)]
    pub fn dbi_wr_period(&self) -> DBI_WR_PERIOD_R {
        DBI_WR_PERIOD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Cycle number=Setting*(DbiAcTimeUnit+1)."]
    #[inline(always)]
    pub fn dbi_wr_eor_wr_assert(&self) -> DBI_WR_EOR_WR_ASSERT_R {
        DBI_WR_EOR_WR_ASSERT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Cycle number=Setting*(DbiAcTimeUnit+1)."]
    #[inline(always)]
    pub fn dbi_wr_cs_assert(&self) -> DBI_WR_CS_ASSERT_R {
        DBI_WR_CS_ASSERT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Single Write Period Duration"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_wr_period(&mut self) -> DBI_WR_PERIOD_W<0> {
        DBI_WR_PERIOD_W::new(self)
    }
    #[doc = "Bits 8:11 - Cycle number=Setting*(DbiAcTimeUnit+1)."]
    #[inline(always)]
    #[must_use]
    pub fn dbi_wr_eor_wr_assert(&mut self) -> DBI_WR_EOR_WR_ASSERT_W<8> {
        DBI_WR_EOR_WR_ASSERT_W::new(self)
    }
    #[doc = "Bits 12:15 - Cycle number=Setting*(DbiAcTimeUnit+1)."]
    #[inline(always)]
    #[must_use]
    pub fn dbi_wr_cs_assert(&mut self) -> DBI_WR_CS_ASSERT_W<12> {
        DBI_WR_CS_ASSERT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBI Write Characteristics 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbi_wr_char10](index.html) module"]
pub struct DBI_WR_CHAR10_SPEC;
impl crate::RegisterSpec for DBI_WR_CHAR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbi_wr_char10::R](R) reader structure"]
impl crate::Readable for DBI_WR_CHAR10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbi_wr_char10::W](W) writer structure"]
impl crate::Writable for DBI_WR_CHAR10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DbiWrChar10 to value 0"]
impl crate::Resettable for DBI_WR_CHAR10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
