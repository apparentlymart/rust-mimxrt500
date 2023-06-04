#[doc = "Register `DpiConfig0` reader"]
pub struct R(crate::R<DPI_CONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPI_CONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPI_CONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPI_CONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DpiConfig0` writer"]
pub struct W(crate::W<DPI_CONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPI_CONFIG0_SPEC>;
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
impl From<crate::W<DPI_CONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPI_CONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPI_DATA_FORMAT` reader - DPI Interface Data Format"]
pub type DPI_DATA_FORMAT_R = crate::FieldReader<u8, DPI_DATA_FORMAT_A>;
#[doc = "DPI Interface Data Format\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DPI_DATA_FORMAT_A {
    #[doc = "0: D16CFG1"]
    VAL0 = 0,
    #[doc = "1: D16CFG2"]
    VAL1 = 1,
    #[doc = "2: D16CFG3"]
    VAL2 = 2,
    #[doc = "3: D18CFG1"]
    VAL3 = 3,
    #[doc = "4: D18CFG2"]
    VAL4 = 4,
    #[doc = "5: D24"]
    VAL5 = 5,
    #[doc = "6: -"]
    VAL6_6 = 6,
    #[doc = "7: -"]
    VAL6_7 = 7,
}
impl From<DPI_DATA_FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: DPI_DATA_FORMAT_A) -> Self {
        variant as _
    }
}
impl DPI_DATA_FORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPI_DATA_FORMAT_A {
        match self.bits {
            0 => DPI_DATA_FORMAT_A::VAL0,
            1 => DPI_DATA_FORMAT_A::VAL1,
            2 => DPI_DATA_FORMAT_A::VAL2,
            3 => DPI_DATA_FORMAT_A::VAL3,
            4 => DPI_DATA_FORMAT_A::VAL4,
            5 => DPI_DATA_FORMAT_A::VAL5,
            6 => DPI_DATA_FORMAT_A::VAL6_6,
            7 => DPI_DATA_FORMAT_A::VAL6_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == DPI_DATA_FORMAT_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == DPI_DATA_FORMAT_A::VAL1
    }
    #[doc = "Checks if the value of the field is `VAL2`"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == DPI_DATA_FORMAT_A::VAL2
    }
    #[doc = "Checks if the value of the field is `VAL3`"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == DPI_DATA_FORMAT_A::VAL3
    }
    #[doc = "Checks if the value of the field is `VAL4`"]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        *self == DPI_DATA_FORMAT_A::VAL4
    }
    #[doc = "Checks if the value of the field is `VAL5`"]
    #[inline(always)]
    pub fn is_val5(&self) -> bool {
        *self == DPI_DATA_FORMAT_A::VAL5
    }
    #[doc = "Checks if the value of the field is `VAL6_6`"]
    #[inline(always)]
    pub fn is_val6_6(&self) -> bool {
        *self == DPI_DATA_FORMAT_A::VAL6_6
    }
    #[doc = "Checks if the value of the field is `VAL6_7`"]
    #[inline(always)]
    pub fn is_val6_7(&self) -> bool {
        *self == DPI_DATA_FORMAT_A::VAL6_7
    }
}
#[doc = "Field `DPI_DATA_FORMAT` writer - DPI Interface Data Format"]
pub type DPI_DATA_FORMAT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DPI_CONFIG0_SPEC, u8, DPI_DATA_FORMAT_A, 3, O>;
impl<'a, const O: u8> DPI_DATA_FORMAT_W<'a, O> {
    #[doc = "D16CFG1"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(DPI_DATA_FORMAT_A::VAL0)
    }
    #[doc = "D16CFG2"]
    #[inline(always)]
    pub fn val1(self) -> &'a mut W {
        self.variant(DPI_DATA_FORMAT_A::VAL1)
    }
    #[doc = "D16CFG3"]
    #[inline(always)]
    pub fn val2(self) -> &'a mut W {
        self.variant(DPI_DATA_FORMAT_A::VAL2)
    }
    #[doc = "D18CFG1"]
    #[inline(always)]
    pub fn val3(self) -> &'a mut W {
        self.variant(DPI_DATA_FORMAT_A::VAL3)
    }
    #[doc = "D18CFG2"]
    #[inline(always)]
    pub fn val4(self) -> &'a mut W {
        self.variant(DPI_DATA_FORMAT_A::VAL4)
    }
    #[doc = "D24"]
    #[inline(always)]
    pub fn val5(self) -> &'a mut W {
        self.variant(DPI_DATA_FORMAT_A::VAL5)
    }
    #[doc = "-"]
    #[inline(always)]
    pub fn val6_6(self) -> &'a mut W {
        self.variant(DPI_DATA_FORMAT_A::VAL6_6)
    }
    #[doc = "-"]
    #[inline(always)]
    pub fn val6_7(self) -> &'a mut W {
        self.variant(DPI_DATA_FORMAT_A::VAL6_7)
    }
}
impl R {
    #[doc = "Bits 0:2 - DPI Interface Data Format"]
    #[inline(always)]
    pub fn dpi_data_format(&self) -> DPI_DATA_FORMAT_R {
        DPI_DATA_FORMAT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DPI Interface Data Format"]
    #[inline(always)]
    #[must_use]
    pub fn dpi_data_format(&mut self) -> DPI_DATA_FORMAT_W<0> {
        DPI_DATA_FORMAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DPI Configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpi_config0](index.html) module"]
pub struct DPI_CONFIG0_SPEC;
impl crate::RegisterSpec for DPI_CONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpi_config0::R](R) reader structure"]
impl crate::Readable for DPI_CONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpi_config0::W](W) writer structure"]
impl crate::Writable for DPI_CONFIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DpiConfig0 to value 0x05"]
impl crate::Resettable for DPI_CONFIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
