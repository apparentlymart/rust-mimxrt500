#[doc = "Register `OUTFORMAT` reader"]
pub struct R(crate::R<OUTFORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTFORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTFORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTFORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTFORMAT` writer"]
pub struct W(crate::W<OUTFORMAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTFORMAT_SPEC>;
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
impl From<crate::W<OUTFORMAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTFORMAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_FORMATINT` reader - Output internal format"]
pub type OUT_FORMATINT_R = crate::FieldReader<u8, OUT_FORMATINT_A>;
#[doc = "Output internal format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUT_FORMATINT_A {
    #[doc = "0: q15"]
    Q15 = 0,
    #[doc = "1: q31"]
    Q31 = 1,
    #[doc = "2: float"]
    FLOAT = 2,
}
impl From<OUT_FORMATINT_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT_FORMATINT_A) -> Self {
        variant as _
    }
}
impl OUT_FORMATINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUT_FORMATINT_A> {
        match self.bits {
            0 => Some(OUT_FORMATINT_A::Q15),
            1 => Some(OUT_FORMATINT_A::Q31),
            2 => Some(OUT_FORMATINT_A::FLOAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Q15`"]
    #[inline(always)]
    pub fn is_q15(&self) -> bool {
        *self == OUT_FORMATINT_A::Q15
    }
    #[doc = "Checks if the value of the field is `Q31`"]
    #[inline(always)]
    pub fn is_q31(&self) -> bool {
        *self == OUT_FORMATINT_A::Q31
    }
    #[doc = "Checks if the value of the field is `FLOAT`"]
    #[inline(always)]
    pub fn is_float(&self) -> bool {
        *self == OUT_FORMATINT_A::FLOAT
    }
}
#[doc = "Field `OUT_FORMATINT` writer - Output internal format"]
pub type OUT_FORMATINT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTFORMAT_SPEC, u8, OUT_FORMATINT_A, 2, O>;
impl<'a, const O: u8> OUT_FORMATINT_W<'a, O> {
    #[doc = "q15"]
    #[inline(always)]
    pub fn q15(self) -> &'a mut W {
        self.variant(OUT_FORMATINT_A::Q15)
    }
    #[doc = "q31"]
    #[inline(always)]
    pub fn q31(self) -> &'a mut W {
        self.variant(OUT_FORMATINT_A::Q31)
    }
    #[doc = "float"]
    #[inline(always)]
    pub fn float(self) -> &'a mut W {
        self.variant(OUT_FORMATINT_A::FLOAT)
    }
}
#[doc = "Field `OUT_FORMATEXT` reader - Output external format"]
pub type OUT_FORMATEXT_R = crate::FieldReader<u8, OUT_FORMATEXT_A>;
#[doc = "Output external format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUT_FORMATEXT_A {
    #[doc = "0: q15"]
    Q15 = 0,
    #[doc = "1: q31"]
    Q31 = 1,
    #[doc = "2: float"]
    FLOAT = 2,
}
impl From<OUT_FORMATEXT_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT_FORMATEXT_A) -> Self {
        variant as _
    }
}
impl OUT_FORMATEXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUT_FORMATEXT_A> {
        match self.bits {
            0 => Some(OUT_FORMATEXT_A::Q15),
            1 => Some(OUT_FORMATEXT_A::Q31),
            2 => Some(OUT_FORMATEXT_A::FLOAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Q15`"]
    #[inline(always)]
    pub fn is_q15(&self) -> bool {
        *self == OUT_FORMATEXT_A::Q15
    }
    #[doc = "Checks if the value of the field is `Q31`"]
    #[inline(always)]
    pub fn is_q31(&self) -> bool {
        *self == OUT_FORMATEXT_A::Q31
    }
    #[doc = "Checks if the value of the field is `FLOAT`"]
    #[inline(always)]
    pub fn is_float(&self) -> bool {
        *self == OUT_FORMATEXT_A::FLOAT
    }
}
#[doc = "Field `OUT_FORMATEXT` writer - Output external format"]
pub type OUT_FORMATEXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTFORMAT_SPEC, u8, OUT_FORMATEXT_A, 2, O>;
impl<'a, const O: u8> OUT_FORMATEXT_W<'a, O> {
    #[doc = "q15"]
    #[inline(always)]
    pub fn q15(self) -> &'a mut W {
        self.variant(OUT_FORMATEXT_A::Q15)
    }
    #[doc = "q31"]
    #[inline(always)]
    pub fn q31(self) -> &'a mut W {
        self.variant(OUT_FORMATEXT_A::Q31)
    }
    #[doc = "float"]
    #[inline(always)]
    pub fn float(self) -> &'a mut W {
        self.variant(OUT_FORMATEXT_A::FLOAT)
    }
}
#[doc = "Field `OUT_SCALER` reader - Output scaler value"]
pub type OUT_SCALER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT_SCALER` writer - Output scaler value"]
pub type OUT_SCALER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTFORMAT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:1 - Output internal format"]
    #[inline(always)]
    pub fn out_formatint(&self) -> OUT_FORMATINT_R {
        OUT_FORMATINT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Output external format"]
    #[inline(always)]
    pub fn out_formatext(&self) -> OUT_FORMATEXT_R {
        OUT_FORMATEXT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Output scaler value"]
    #[inline(always)]
    pub fn out_scaler(&self) -> OUT_SCALER_R {
        OUT_SCALER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Output internal format"]
    #[inline(always)]
    #[must_use]
    pub fn out_formatint(&mut self) -> OUT_FORMATINT_W<0> {
        OUT_FORMATINT_W::new(self)
    }
    #[doc = "Bits 4:5 - Output external format"]
    #[inline(always)]
    #[must_use]
    pub fn out_formatext(&mut self) -> OUT_FORMATEXT_W<4> {
        OUT_FORMATEXT_W::new(self)
    }
    #[doc = "Bits 8:15 - Output scaler value"]
    #[inline(always)]
    #[must_use]
    pub fn out_scaler(&mut self) -> OUT_SCALER_W<8> {
        OUT_SCALER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outformat](index.html) module"]
pub struct OUTFORMAT_SPEC;
impl crate::RegisterSpec for OUTFORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outformat::R](R) reader structure"]
impl crate::Readable for OUTFORMAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outformat::W](W) writer structure"]
impl crate::Writable for OUTFORMAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTFORMAT to value 0"]
impl crate::Resettable for OUTFORMAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
