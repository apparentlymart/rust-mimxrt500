#[doc = "Register `ERRINT1` reader"]
pub struct R(crate::R<ERRINT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRINT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRINT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRINT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERRINT1` writer"]
pub struct W(crate::W<ERRINT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERRINT1_SPEC>;
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
impl From<crate::W<ERRINT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERRINT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERR32` reader - Error Interrupt flag for DMA channel."]
pub type ERR32_R = crate::BitReader<ERR32_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR32_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR32_A> for bool {
    #[inline(always)]
    fn from(variant: ERR32_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR32_A {
        match self.bits {
            false => ERR32_A::NOT_ACTIVE,
            true => ERR32_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR32_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR32_A::PENDING
    }
}
#[doc = "Field `ERR32` writer - Error Interrupt flag for DMA channel."]
pub type ERR32_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT1_SPEC, ERR32_A, O>;
impl<'a, const O: u8> ERR32_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR32_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR32_A::PENDING)
    }
}
#[doc = "Field `ERR33` reader - Error Interrupt flag for DMA channel."]
pub type ERR33_R = crate::BitReader<ERR33_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR33_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR33_A> for bool {
    #[inline(always)]
    fn from(variant: ERR33_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR33_A {
        match self.bits {
            false => ERR33_A::NOT_ACTIVE,
            true => ERR33_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR33_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR33_A::PENDING
    }
}
#[doc = "Field `ERR33` writer - Error Interrupt flag for DMA channel."]
pub type ERR33_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT1_SPEC, ERR33_A, O>;
impl<'a, const O: u8> ERR33_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR33_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR33_A::PENDING)
    }
}
#[doc = "Field `ERR34` reader - Error Interrupt flag for DMA channel."]
pub type ERR34_R = crate::BitReader<ERR34_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR34_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR34_A> for bool {
    #[inline(always)]
    fn from(variant: ERR34_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR34_A {
        match self.bits {
            false => ERR34_A::NOT_ACTIVE,
            true => ERR34_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR34_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR34_A::PENDING
    }
}
#[doc = "Field `ERR34` writer - Error Interrupt flag for DMA channel."]
pub type ERR34_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT1_SPEC, ERR34_A, O>;
impl<'a, const O: u8> ERR34_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR34_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR34_A::PENDING)
    }
}
#[doc = "Field `ERR35` reader - Error Interrupt flag for DMA channel."]
pub type ERR35_R = crate::BitReader<ERR35_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR35_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR35_A> for bool {
    #[inline(always)]
    fn from(variant: ERR35_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR35_A {
        match self.bits {
            false => ERR35_A::NOT_ACTIVE,
            true => ERR35_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR35_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR35_A::PENDING
    }
}
#[doc = "Field `ERR35` writer - Error Interrupt flag for DMA channel."]
pub type ERR35_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT1_SPEC, ERR35_A, O>;
impl<'a, const O: u8> ERR35_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR35_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR35_A::PENDING)
    }
}
#[doc = "Field `ERR36` reader - Error Interrupt flag for DMA channel."]
pub type ERR36_R = crate::BitReader<ERR36_A>;
#[doc = "Error Interrupt flag for DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR36_A {
    #[doc = "0: The Error Interrupt is not active for DMA channel."]
    NOT_ACTIVE = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel."]
    PENDING = 1,
}
impl From<ERR36_A> for bool {
    #[inline(always)]
    fn from(variant: ERR36_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR36_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR36_A {
        match self.bits {
            false => ERR36_A::NOT_ACTIVE,
            true => ERR36_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ERR36_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ERR36_A::PENDING
    }
}
#[doc = "Field `ERR36` writer - Error Interrupt flag for DMA channel."]
pub type ERR36_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERRINT1_SPEC, ERR36_A, O>;
impl<'a, const O: u8> ERR36_W<'a, O> {
    #[doc = "The Error Interrupt is not active for DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(ERR36_A::NOT_ACTIVE)
    }
    #[doc = "The Error Interrupt is pending for DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ERR36_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 0 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err32(&self) -> ERR32_R {
        ERR32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err33(&self) -> ERR33_R {
        ERR33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err34(&self) -> ERR34_R {
        ERR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err35(&self) -> ERR35_R {
        ERR35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    pub fn err36(&self) -> ERR36_R {
        ERR36_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err32(&mut self) -> ERR32_W<0> {
        ERR32_W::new(self)
    }
    #[doc = "Bit 1 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err33(&mut self) -> ERR33_W<1> {
        ERR33_W::new(self)
    }
    #[doc = "Bit 2 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err34(&mut self) -> ERR34_W<2> {
        ERR34_W::new(self)
    }
    #[doc = "Bit 3 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err35(&mut self) -> ERR35_W<3> {
        ERR35_W::new(self)
    }
    #[doc = "Bit 4 - Error Interrupt flag for DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn err36(&mut self) -> ERR36_W<4> {
        ERR36_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Interrupt status for all DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errint1](index.html) module"]
pub struct ERRINT1_SPEC;
impl crate::RegisterSpec for ERRINT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [errint1::R](R) reader structure"]
impl crate::Readable for ERRINT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [errint1::W](W) writer structure"]
impl crate::Writable for ERRINT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERRINT1 to value 0"]
impl crate::Resettable for ERRINT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
