#[doc = "Register `INAFORMAT` reader"]
pub struct R(crate::R<INAFORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INAFORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INAFORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INAFORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INAFORMAT` writer"]
pub struct W(crate::W<INAFORMAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INAFORMAT_SPEC>;
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
impl From<crate::W<INAFORMAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INAFORMAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INA_FORMATINT` reader - Input A internal format"]
pub type INA_FORMATINT_R = crate::FieldReader<u8, INA_FORMATINT_A>;
#[doc = "Input A internal format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INA_FORMATINT_A {
    #[doc = "0: q15"]
    Q15 = 0,
    #[doc = "1: q31"]
    Q31 = 1,
    #[doc = "2: float"]
    FLOAT = 2,
}
impl From<INA_FORMATINT_A> for u8 {
    #[inline(always)]
    fn from(variant: INA_FORMATINT_A) -> Self {
        variant as _
    }
}
impl INA_FORMATINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INA_FORMATINT_A> {
        match self.bits {
            0 => Some(INA_FORMATINT_A::Q15),
            1 => Some(INA_FORMATINT_A::Q31),
            2 => Some(INA_FORMATINT_A::FLOAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Q15`"]
    #[inline(always)]
    pub fn is_q15(&self) -> bool {
        *self == INA_FORMATINT_A::Q15
    }
    #[doc = "Checks if the value of the field is `Q31`"]
    #[inline(always)]
    pub fn is_q31(&self) -> bool {
        *self == INA_FORMATINT_A::Q31
    }
    #[doc = "Checks if the value of the field is `FLOAT`"]
    #[inline(always)]
    pub fn is_float(&self) -> bool {
        *self == INA_FORMATINT_A::FLOAT
    }
}
#[doc = "Field `INA_FORMATINT` writer - Input A internal format"]
pub type INA_FORMATINT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INAFORMAT_SPEC, u8, INA_FORMATINT_A, 2, O>;
impl<'a, const O: u8> INA_FORMATINT_W<'a, O> {
    #[doc = "q15"]
    #[inline(always)]
    pub fn q15(self) -> &'a mut W {
        self.variant(INA_FORMATINT_A::Q15)
    }
    #[doc = "q31"]
    #[inline(always)]
    pub fn q31(self) -> &'a mut W {
        self.variant(INA_FORMATINT_A::Q31)
    }
    #[doc = "float"]
    #[inline(always)]
    pub fn float(self) -> &'a mut W {
        self.variant(INA_FORMATINT_A::FLOAT)
    }
}
#[doc = "Field `INA_FORMATEXT` reader - Input A external format"]
pub type INA_FORMATEXT_R = crate::FieldReader<u8, INA_FORMATEXT_A>;
#[doc = "Input A external format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INA_FORMATEXT_A {
    #[doc = "0: q15"]
    Q15 = 0,
    #[doc = "1: q31"]
    Q31 = 1,
    #[doc = "2: float"]
    FLOAT = 2,
}
impl From<INA_FORMATEXT_A> for u8 {
    #[inline(always)]
    fn from(variant: INA_FORMATEXT_A) -> Self {
        variant as _
    }
}
impl INA_FORMATEXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INA_FORMATEXT_A> {
        match self.bits {
            0 => Some(INA_FORMATEXT_A::Q15),
            1 => Some(INA_FORMATEXT_A::Q31),
            2 => Some(INA_FORMATEXT_A::FLOAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Q15`"]
    #[inline(always)]
    pub fn is_q15(&self) -> bool {
        *self == INA_FORMATEXT_A::Q15
    }
    #[doc = "Checks if the value of the field is `Q31`"]
    #[inline(always)]
    pub fn is_q31(&self) -> bool {
        *self == INA_FORMATEXT_A::Q31
    }
    #[doc = "Checks if the value of the field is `FLOAT`"]
    #[inline(always)]
    pub fn is_float(&self) -> bool {
        *self == INA_FORMATEXT_A::FLOAT
    }
}
#[doc = "Field `INA_FORMATEXT` writer - Input A external format"]
pub type INA_FORMATEXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INAFORMAT_SPEC, u8, INA_FORMATEXT_A, 2, O>;
impl<'a, const O: u8> INA_FORMATEXT_W<'a, O> {
    #[doc = "q15"]
    #[inline(always)]
    pub fn q15(self) -> &'a mut W {
        self.variant(INA_FORMATEXT_A::Q15)
    }
    #[doc = "q31"]
    #[inline(always)]
    pub fn q31(self) -> &'a mut W {
        self.variant(INA_FORMATEXT_A::Q31)
    }
    #[doc = "float"]
    #[inline(always)]
    pub fn float(self) -> &'a mut W {
        self.variant(INA_FORMATEXT_A::FLOAT)
    }
}
#[doc = "Field `INA_SCALER` reader - Input A scaler value"]
pub type INA_SCALER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INA_SCALER` writer - Input A scaler value"]
pub type INA_SCALER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INAFORMAT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:1 - Input A internal format"]
    #[inline(always)]
    pub fn ina_formatint(&self) -> INA_FORMATINT_R {
        INA_FORMATINT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Input A external format"]
    #[inline(always)]
    pub fn ina_formatext(&self) -> INA_FORMATEXT_R {
        INA_FORMATEXT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Input A scaler value"]
    #[inline(always)]
    pub fn ina_scaler(&self) -> INA_SCALER_R {
        INA_SCALER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input A internal format"]
    #[inline(always)]
    #[must_use]
    pub fn ina_formatint(&mut self) -> INA_FORMATINT_W<0> {
        INA_FORMATINT_W::new(self)
    }
    #[doc = "Bits 4:5 - Input A external format"]
    #[inline(always)]
    #[must_use]
    pub fn ina_formatext(&mut self) -> INA_FORMATEXT_W<4> {
        INA_FORMATEXT_W::new(self)
    }
    #[doc = "Bits 8:15 - Input A scaler value"]
    #[inline(always)]
    #[must_use]
    pub fn ina_scaler(&mut self) -> INA_SCALER_W<8> {
        INA_SCALER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input A format\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inaformat](index.html) module"]
pub struct INAFORMAT_SPEC;
impl crate::RegisterSpec for INAFORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inaformat::R](R) reader structure"]
impl crate::Readable for INAFORMAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inaformat::W](W) writer structure"]
impl crate::Writable for INAFORMAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INAFORMAT to value 0"]
impl crate::Resettable for INAFORMAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
