#[doc = "Register `TMPFORMAT` reader"]
pub struct R(crate::R<TMPFORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMPFORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMPFORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMPFORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMPFORMAT` writer"]
pub struct W(crate::W<TMPFORMAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMPFORMAT_SPEC>;
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
impl From<crate::W<TMPFORMAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMPFORMAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMP_FORMATINT` reader - Temporary internal format"]
pub type TMP_FORMATINT_R = crate::FieldReader<u8, TMP_FORMATINT_A>;
#[doc = "Temporary internal format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMP_FORMATINT_A {
    #[doc = "0: q15"]
    Q15 = 0,
    #[doc = "1: q31"]
    Q31 = 1,
    #[doc = "2: float"]
    FLOAT = 2,
}
impl From<TMP_FORMATINT_A> for u8 {
    #[inline(always)]
    fn from(variant: TMP_FORMATINT_A) -> Self {
        variant as _
    }
}
impl TMP_FORMATINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMP_FORMATINT_A> {
        match self.bits {
            0 => Some(TMP_FORMATINT_A::Q15),
            1 => Some(TMP_FORMATINT_A::Q31),
            2 => Some(TMP_FORMATINT_A::FLOAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Q15`"]
    #[inline(always)]
    pub fn is_q15(&self) -> bool {
        *self == TMP_FORMATINT_A::Q15
    }
    #[doc = "Checks if the value of the field is `Q31`"]
    #[inline(always)]
    pub fn is_q31(&self) -> bool {
        *self == TMP_FORMATINT_A::Q31
    }
    #[doc = "Checks if the value of the field is `FLOAT`"]
    #[inline(always)]
    pub fn is_float(&self) -> bool {
        *self == TMP_FORMATINT_A::FLOAT
    }
}
#[doc = "Field `TMP_FORMATINT` writer - Temporary internal format"]
pub type TMP_FORMATINT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TMPFORMAT_SPEC, u8, TMP_FORMATINT_A, 2, O>;
impl<'a, const O: u8> TMP_FORMATINT_W<'a, O> {
    #[doc = "q15"]
    #[inline(always)]
    pub fn q15(self) -> &'a mut W {
        self.variant(TMP_FORMATINT_A::Q15)
    }
    #[doc = "q31"]
    #[inline(always)]
    pub fn q31(self) -> &'a mut W {
        self.variant(TMP_FORMATINT_A::Q31)
    }
    #[doc = "float"]
    #[inline(always)]
    pub fn float(self) -> &'a mut W {
        self.variant(TMP_FORMATINT_A::FLOAT)
    }
}
#[doc = "Field `TMP_FORMATEXT` reader - Temporary external format"]
pub type TMP_FORMATEXT_R = crate::FieldReader<u8, TMP_FORMATEXT_A>;
#[doc = "Temporary external format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMP_FORMATEXT_A {
    #[doc = "0: q15"]
    Q15 = 0,
    #[doc = "1: q31"]
    Q31 = 1,
    #[doc = "2: float"]
    FLOAT = 2,
}
impl From<TMP_FORMATEXT_A> for u8 {
    #[inline(always)]
    fn from(variant: TMP_FORMATEXT_A) -> Self {
        variant as _
    }
}
impl TMP_FORMATEXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMP_FORMATEXT_A> {
        match self.bits {
            0 => Some(TMP_FORMATEXT_A::Q15),
            1 => Some(TMP_FORMATEXT_A::Q31),
            2 => Some(TMP_FORMATEXT_A::FLOAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Q15`"]
    #[inline(always)]
    pub fn is_q15(&self) -> bool {
        *self == TMP_FORMATEXT_A::Q15
    }
    #[doc = "Checks if the value of the field is `Q31`"]
    #[inline(always)]
    pub fn is_q31(&self) -> bool {
        *self == TMP_FORMATEXT_A::Q31
    }
    #[doc = "Checks if the value of the field is `FLOAT`"]
    #[inline(always)]
    pub fn is_float(&self) -> bool {
        *self == TMP_FORMATEXT_A::FLOAT
    }
}
#[doc = "Field `TMP_FORMATEXT` writer - Temporary external format"]
pub type TMP_FORMATEXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TMPFORMAT_SPEC, u8, TMP_FORMATEXT_A, 2, O>;
impl<'a, const O: u8> TMP_FORMATEXT_W<'a, O> {
    #[doc = "q15"]
    #[inline(always)]
    pub fn q15(self) -> &'a mut W {
        self.variant(TMP_FORMATEXT_A::Q15)
    }
    #[doc = "q31"]
    #[inline(always)]
    pub fn q31(self) -> &'a mut W {
        self.variant(TMP_FORMATEXT_A::Q31)
    }
    #[doc = "float"]
    #[inline(always)]
    pub fn float(self) -> &'a mut W {
        self.variant(TMP_FORMATEXT_A::FLOAT)
    }
}
#[doc = "Field `TMP_SCALER` reader - Temporary scaler value"]
pub type TMP_SCALER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMP_SCALER` writer - Temporary scaler value"]
pub type TMP_SCALER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMPFORMAT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:1 - Temporary internal format"]
    #[inline(always)]
    pub fn tmp_formatint(&self) -> TMP_FORMATINT_R {
        TMP_FORMATINT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Temporary external format"]
    #[inline(always)]
    pub fn tmp_formatext(&self) -> TMP_FORMATEXT_R {
        TMP_FORMATEXT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Temporary scaler value"]
    #[inline(always)]
    pub fn tmp_scaler(&self) -> TMP_SCALER_R {
        TMP_SCALER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Temporary internal format"]
    #[inline(always)]
    #[must_use]
    pub fn tmp_formatint(&mut self) -> TMP_FORMATINT_W<0> {
        TMP_FORMATINT_W::new(self)
    }
    #[doc = "Bits 4:5 - Temporary external format"]
    #[inline(always)]
    #[must_use]
    pub fn tmp_formatext(&mut self) -> TMP_FORMATEXT_W<4> {
        TMP_FORMATEXT_W::new(self)
    }
    #[doc = "Bits 8:15 - Temporary scaler value"]
    #[inline(always)]
    #[must_use]
    pub fn tmp_scaler(&mut self) -> TMP_SCALER_W<8> {
        TMP_SCALER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Temporary Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmpformat](index.html) module"]
pub struct TMPFORMAT_SPEC;
impl crate::RegisterSpec for TMPFORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmpformat::R](R) reader structure"]
impl crate::Readable for TMPFORMAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmpformat::W](W) writer structure"]
impl crate::Writable for TMPFORMAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMPFORMAT to value 0"]
impl crate::Resettable for TMPFORMAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
