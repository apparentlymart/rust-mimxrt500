#[doc = "Register `DbiConfig0` reader"]
pub struct R(crate::R<DBI_CONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBI_CONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBI_CONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBI_CONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DbiConfig0` writer"]
pub struct W(crate::W<DBI_CONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBI_CONFIG0_SPEC>;
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
impl From<crate::W<DBI_CONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBI_CONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBI_DATA_FORMAT` reader - DBI Interface Data Format."]
pub type DBI_DATA_FORMAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBI_DATA_FORMAT` writer - DBI Interface Data Format."]
pub type DBI_DATA_FORMAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBI_CONFIG0_SPEC, u8, u8, 4, O>;
#[doc = "Field `BUS_OUTPUT_SEL` reader - Output Bus Select"]
pub type BUS_OUTPUT_SEL_R = crate::BitReader<BUS_OUTPUT_SEL_A>;
#[doc = "Output Bus Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUS_OUTPUT_SEL_A {
    #[doc = "0: DPI"]
    DISABLE = 0,
    #[doc = "1: DBI"]
    ENABLE = 1,
}
impl From<BUS_OUTPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: BUS_OUTPUT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl BUS_OUTPUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUS_OUTPUT_SEL_A {
        match self.bits {
            false => BUS_OUTPUT_SEL_A::DISABLE,
            true => BUS_OUTPUT_SEL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BUS_OUTPUT_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BUS_OUTPUT_SEL_A::ENABLE
    }
}
#[doc = "Field `BUS_OUTPUT_SEL` writer - Output Bus Select"]
pub type BUS_OUTPUT_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBI_CONFIG0_SPEC, BUS_OUTPUT_SEL_A, O>;
impl<'a, const O: u8> BUS_OUTPUT_SEL_W<'a, O> {
    #[doc = "DPI"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BUS_OUTPUT_SEL_A::DISABLE)
    }
    #[doc = "DBI"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BUS_OUTPUT_SEL_A::ENABLE)
    }
}
#[doc = "Field `DBIX_POLARITY` reader - D/CX Pin polarity."]
pub type DBIX_POLARITY_R = crate::BitReader<DBIX_POLARITY_A>;
#[doc = "D/CX Pin polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBIX_POLARITY_A {
    #[doc = "0: Default"]
    DISABLE = 0,
    #[doc = "1: Reverse"]
    ENABLE = 1,
}
impl From<DBIX_POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: DBIX_POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl DBIX_POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBIX_POLARITY_A {
        match self.bits {
            false => DBIX_POLARITY_A::DISABLE,
            true => DBIX_POLARITY_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DBIX_POLARITY_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DBIX_POLARITY_A::ENABLE
    }
}
#[doc = "Field `DBIX_POLARITY` writer - D/CX Pin polarity."]
pub type DBIX_POLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBI_CONFIG0_SPEC, DBIX_POLARITY_A, O>;
impl<'a, const O: u8> DBIX_POLARITY_W<'a, O> {
    #[doc = "Default"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DBIX_POLARITY_A::DISABLE)
    }
    #[doc = "Reverse"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DBIX_POLARITY_A::ENABLE)
    }
}
#[doc = "Field `DBI_AC_TIME_UNIT` reader - Time Unit for AC Characteristics"]
pub type DBI_AC_TIME_UNIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBI_AC_TIME_UNIT` writer - Time Unit for AC Characteristics"]
pub type DBI_AC_TIME_UNIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBI_CONFIG0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 2:5 - DBI Interface Data Format."]
    #[inline(always)]
    pub fn dbi_data_format(&self) -> DBI_DATA_FORMAT_R {
        DBI_DATA_FORMAT_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Output Bus Select"]
    #[inline(always)]
    pub fn bus_output_sel(&self) -> BUS_OUTPUT_SEL_R {
        BUS_OUTPUT_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - D/CX Pin polarity."]
    #[inline(always)]
    pub fn dbix_polarity(&self) -> DBIX_POLARITY_R {
        DBIX_POLARITY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Time Unit for AC Characteristics"]
    #[inline(always)]
    pub fn dbi_ac_time_unit(&self) -> DBI_AC_TIME_UNIT_R {
        DBI_AC_TIME_UNIT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:5 - DBI Interface Data Format."]
    #[inline(always)]
    #[must_use]
    pub fn dbi_data_format(&mut self) -> DBI_DATA_FORMAT_W<2> {
        DBI_DATA_FORMAT_W::new(self)
    }
    #[doc = "Bit 6 - Output Bus Select"]
    #[inline(always)]
    #[must_use]
    pub fn bus_output_sel(&mut self) -> BUS_OUTPUT_SEL_W<6> {
        BUS_OUTPUT_SEL_W::new(self)
    }
    #[doc = "Bit 7 - D/CX Pin polarity."]
    #[inline(always)]
    #[must_use]
    pub fn dbix_polarity(&mut self) -> DBIX_POLARITY_W<7> {
        DBIX_POLARITY_W::new(self)
    }
    #[doc = "Bits 8:11 - Time Unit for AC Characteristics"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_ac_time_unit(&mut self) -> DBI_AC_TIME_UNIT_W<8> {
        DBI_AC_TIME_UNIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBI Configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbi_config0](index.html) module"]
pub struct DBI_CONFIG0_SPEC;
impl crate::RegisterSpec for DBI_CONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbi_config0::R](R) reader structure"]
impl crate::Readable for DBI_CONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbi_config0::W](W) writer structure"]
impl crate::Writable for DBI_CONFIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DbiConfig0 to value 0x40"]
impl crate::Resettable for DBI_CONFIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
