#[doc = "Register `INBFORMAT` reader"]
pub struct R(crate::R<INBFORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INBFORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INBFORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INBFORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INBFORMAT` writer"]
pub struct W(crate::W<INBFORMAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INBFORMAT_SPEC>;
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
impl From<crate::W<INBFORMAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INBFORMAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INB_FORMATINT` reader - Input B internal format"]
pub type INB_FORMATINT_R = crate::FieldReader<u8, INB_FORMATINT_A>;
#[doc = "Input B internal format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INB_FORMATINT_A {
    #[doc = "0: q15"]
    Q15 = 0,
    #[doc = "1: q31"]
    Q31 = 1,
    #[doc = "2: float"]
    FLOAT = 2,
}
impl From<INB_FORMATINT_A> for u8 {
    #[inline(always)]
    fn from(variant: INB_FORMATINT_A) -> Self {
        variant as _
    }
}
impl INB_FORMATINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INB_FORMATINT_A> {
        match self.bits {
            0 => Some(INB_FORMATINT_A::Q15),
            1 => Some(INB_FORMATINT_A::Q31),
            2 => Some(INB_FORMATINT_A::FLOAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Q15`"]
    #[inline(always)]
    pub fn is_q15(&self) -> bool {
        *self == INB_FORMATINT_A::Q15
    }
    #[doc = "Checks if the value of the field is `Q31`"]
    #[inline(always)]
    pub fn is_q31(&self) -> bool {
        *self == INB_FORMATINT_A::Q31
    }
    #[doc = "Checks if the value of the field is `FLOAT`"]
    #[inline(always)]
    pub fn is_float(&self) -> bool {
        *self == INB_FORMATINT_A::FLOAT
    }
}
#[doc = "Field `INB_FORMATINT` writer - Input B internal format"]
pub type INB_FORMATINT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INBFORMAT_SPEC, u8, INB_FORMATINT_A, 2, O>;
impl<'a, const O: u8> INB_FORMATINT_W<'a, O> {
    #[doc = "q15"]
    #[inline(always)]
    pub fn q15(self) -> &'a mut W {
        self.variant(INB_FORMATINT_A::Q15)
    }
    #[doc = "q31"]
    #[inline(always)]
    pub fn q31(self) -> &'a mut W {
        self.variant(INB_FORMATINT_A::Q31)
    }
    #[doc = "float"]
    #[inline(always)]
    pub fn float(self) -> &'a mut W {
        self.variant(INB_FORMATINT_A::FLOAT)
    }
}
#[doc = "Field `INB_FORMATEXT` reader - Input B external format"]
pub type INB_FORMATEXT_R = crate::FieldReader<u8, INB_FORMATEXT_A>;
#[doc = "Input B external format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INB_FORMATEXT_A {
    #[doc = "0: q15"]
    Q15 = 0,
    #[doc = "1: q31"]
    Q31 = 1,
    #[doc = "2: float"]
    FLOAT = 2,
}
impl From<INB_FORMATEXT_A> for u8 {
    #[inline(always)]
    fn from(variant: INB_FORMATEXT_A) -> Self {
        variant as _
    }
}
impl INB_FORMATEXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INB_FORMATEXT_A> {
        match self.bits {
            0 => Some(INB_FORMATEXT_A::Q15),
            1 => Some(INB_FORMATEXT_A::Q31),
            2 => Some(INB_FORMATEXT_A::FLOAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Q15`"]
    #[inline(always)]
    pub fn is_q15(&self) -> bool {
        *self == INB_FORMATEXT_A::Q15
    }
    #[doc = "Checks if the value of the field is `Q31`"]
    #[inline(always)]
    pub fn is_q31(&self) -> bool {
        *self == INB_FORMATEXT_A::Q31
    }
    #[doc = "Checks if the value of the field is `FLOAT`"]
    #[inline(always)]
    pub fn is_float(&self) -> bool {
        *self == INB_FORMATEXT_A::FLOAT
    }
}
#[doc = "Field `INB_FORMATEXT` writer - Input B external format"]
pub type INB_FORMATEXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INBFORMAT_SPEC, u8, INB_FORMATEXT_A, 2, O>;
impl<'a, const O: u8> INB_FORMATEXT_W<'a, O> {
    #[doc = "q15"]
    #[inline(always)]
    pub fn q15(self) -> &'a mut W {
        self.variant(INB_FORMATEXT_A::Q15)
    }
    #[doc = "q31"]
    #[inline(always)]
    pub fn q31(self) -> &'a mut W {
        self.variant(INB_FORMATEXT_A::Q31)
    }
    #[doc = "float"]
    #[inline(always)]
    pub fn float(self) -> &'a mut W {
        self.variant(INB_FORMATEXT_A::FLOAT)
    }
}
#[doc = "Field `INB_SCALER` reader - Input B scaler value"]
pub type INB_SCALER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INB_SCALER` writer - Input B scaler value"]
pub type INB_SCALER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INBFORMAT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:1 - Input B internal format"]
    #[inline(always)]
    pub fn inb_formatint(&self) -> INB_FORMATINT_R {
        INB_FORMATINT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Input B external format"]
    #[inline(always)]
    pub fn inb_formatext(&self) -> INB_FORMATEXT_R {
        INB_FORMATEXT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Input B scaler value"]
    #[inline(always)]
    pub fn inb_scaler(&self) -> INB_SCALER_R {
        INB_SCALER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input B internal format"]
    #[inline(always)]
    #[must_use]
    pub fn inb_formatint(&mut self) -> INB_FORMATINT_W<0> {
        INB_FORMATINT_W::new(self)
    }
    #[doc = "Bits 4:5 - Input B external format"]
    #[inline(always)]
    #[must_use]
    pub fn inb_formatext(&mut self) -> INB_FORMATEXT_W<4> {
        INB_FORMATEXT_W::new(self)
    }
    #[doc = "Bits 8:15 - Input B scaler value"]
    #[inline(always)]
    #[must_use]
    pub fn inb_scaler(&mut self) -> INB_SCALER_W<8> {
        INB_SCALER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input B format\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inbformat](index.html) module"]
pub struct INBFORMAT_SPEC;
impl crate::RegisterSpec for INBFORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inbformat::R](R) reader structure"]
impl crate::Readable for INBFORMAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inbformat::W](W) writer structure"]
impl crate::Writable for INBFORMAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INBFORMAT to value 0"]
impl crate::Resettable for INBFORMAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
