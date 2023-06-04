#[doc = "Register `FCCTRLSEL[%s]` reader"]
pub struct R(crate::R<FCCTRLSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCCTRLSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCCTRLSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCCTRLSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCCTRLSEL[%s]` writer"]
pub struct W(crate::W<FCCTRLSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCCTRLSEL_SPEC>;
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
impl From<crate::W<FCCTRLSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCCTRLSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCKINSEL` reader - SCK IN Select"]
pub type SCKINSEL_R = crate::FieldReader<u8, SCKINSEL_A>;
#[doc = "SCK IN Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCKINSEL_A {
    #[doc = "0: Original FLEXCOMM I2S signals"]
    SCKINSEL_0 = 0,
    #[doc = "1: Shared Set0 I2S signals"]
    SCKINSEL_1 = 1,
    #[doc = "2: Shared Set1 I2S signals"]
    SCKINSEL_2 = 2,
}
impl From<SCKINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCKINSEL_A) -> Self {
        variant as _
    }
}
impl SCKINSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCKINSEL_A> {
        match self.bits {
            0 => Some(SCKINSEL_A::SCKINSEL_0),
            1 => Some(SCKINSEL_A::SCKINSEL_1),
            2 => Some(SCKINSEL_A::SCKINSEL_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCKINSEL_0`"]
    #[inline(always)]
    pub fn is_sckinsel_0(&self) -> bool {
        *self == SCKINSEL_A::SCKINSEL_0
    }
    #[doc = "Checks if the value of the field is `SCKINSEL_1`"]
    #[inline(always)]
    pub fn is_sckinsel_1(&self) -> bool {
        *self == SCKINSEL_A::SCKINSEL_1
    }
    #[doc = "Checks if the value of the field is `SCKINSEL_2`"]
    #[inline(always)]
    pub fn is_sckinsel_2(&self) -> bool {
        *self == SCKINSEL_A::SCKINSEL_2
    }
}
#[doc = "Field `SCKINSEL` writer - SCK IN Select"]
pub type SCKINSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCCTRLSEL_SPEC, u8, SCKINSEL_A, 2, O>;
impl<'a, const O: u8> SCKINSEL_W<'a, O> {
    #[doc = "Original FLEXCOMM I2S signals"]
    #[inline(always)]
    pub fn sckinsel_0(self) -> &'a mut W {
        self.variant(SCKINSEL_A::SCKINSEL_0)
    }
    #[doc = "Shared Set0 I2S signals"]
    #[inline(always)]
    pub fn sckinsel_1(self) -> &'a mut W {
        self.variant(SCKINSEL_A::SCKINSEL_1)
    }
    #[doc = "Shared Set1 I2S signals"]
    #[inline(always)]
    pub fn sckinsel_2(self) -> &'a mut W {
        self.variant(SCKINSEL_A::SCKINSEL_2)
    }
}
#[doc = "Field `WSINSEL` reader - SCK IN Select"]
pub type WSINSEL_R = crate::FieldReader<u8, WSINSEL_A>;
#[doc = "SCK IN Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WSINSEL_A {
    #[doc = "0: Original FLEXCOMM I2S signals"]
    SCKINSEL_0 = 0,
    #[doc = "1: Shared Set0 I2S signals"]
    SCKINSEL_1 = 1,
    #[doc = "2: Shared Set1 I2S signals"]
    SCKINSEL_2 = 2,
}
impl From<WSINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WSINSEL_A) -> Self {
        variant as _
    }
}
impl WSINSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WSINSEL_A> {
        match self.bits {
            0 => Some(WSINSEL_A::SCKINSEL_0),
            1 => Some(WSINSEL_A::SCKINSEL_1),
            2 => Some(WSINSEL_A::SCKINSEL_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCKINSEL_0`"]
    #[inline(always)]
    pub fn is_sckinsel_0(&self) -> bool {
        *self == WSINSEL_A::SCKINSEL_0
    }
    #[doc = "Checks if the value of the field is `SCKINSEL_1`"]
    #[inline(always)]
    pub fn is_sckinsel_1(&self) -> bool {
        *self == WSINSEL_A::SCKINSEL_1
    }
    #[doc = "Checks if the value of the field is `SCKINSEL_2`"]
    #[inline(always)]
    pub fn is_sckinsel_2(&self) -> bool {
        *self == WSINSEL_A::SCKINSEL_2
    }
}
#[doc = "Field `WSINSEL` writer - SCK IN Select"]
pub type WSINSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCCTRLSEL_SPEC, u8, WSINSEL_A, 2, O>;
impl<'a, const O: u8> WSINSEL_W<'a, O> {
    #[doc = "Original FLEXCOMM I2S signals"]
    #[inline(always)]
    pub fn sckinsel_0(self) -> &'a mut W {
        self.variant(WSINSEL_A::SCKINSEL_0)
    }
    #[doc = "Shared Set0 I2S signals"]
    #[inline(always)]
    pub fn sckinsel_1(self) -> &'a mut W {
        self.variant(WSINSEL_A::SCKINSEL_1)
    }
    #[doc = "Shared Set1 I2S signals"]
    #[inline(always)]
    pub fn sckinsel_2(self) -> &'a mut W {
        self.variant(WSINSEL_A::SCKINSEL_2)
    }
}
#[doc = "Field `DATAINSEL` reader - DATA IN Select"]
pub type DATAINSEL_R = crate::FieldReader<u8, DATAINSEL_A>;
#[doc = "DATA IN Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATAINSEL_A {
    #[doc = "0: Original FLEXCOMM I2S signals"]
    SCKINSEL_0 = 0,
    #[doc = "1: Shared Set0 I2S signals"]
    SCKINSEL_1 = 1,
    #[doc = "2: Shared Set1 I2S signals"]
    SCKINSEL_2 = 2,
}
impl From<DATAINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DATAINSEL_A) -> Self {
        variant as _
    }
}
impl DATAINSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATAINSEL_A> {
        match self.bits {
            0 => Some(DATAINSEL_A::SCKINSEL_0),
            1 => Some(DATAINSEL_A::SCKINSEL_1),
            2 => Some(DATAINSEL_A::SCKINSEL_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCKINSEL_0`"]
    #[inline(always)]
    pub fn is_sckinsel_0(&self) -> bool {
        *self == DATAINSEL_A::SCKINSEL_0
    }
    #[doc = "Checks if the value of the field is `SCKINSEL_1`"]
    #[inline(always)]
    pub fn is_sckinsel_1(&self) -> bool {
        *self == DATAINSEL_A::SCKINSEL_1
    }
    #[doc = "Checks if the value of the field is `SCKINSEL_2`"]
    #[inline(always)]
    pub fn is_sckinsel_2(&self) -> bool {
        *self == DATAINSEL_A::SCKINSEL_2
    }
}
#[doc = "Field `DATAINSEL` writer - DATA IN Select"]
pub type DATAINSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCCTRLSEL_SPEC, u8, DATAINSEL_A, 2, O>;
impl<'a, const O: u8> DATAINSEL_W<'a, O> {
    #[doc = "Original FLEXCOMM I2S signals"]
    #[inline(always)]
    pub fn sckinsel_0(self) -> &'a mut W {
        self.variant(DATAINSEL_A::SCKINSEL_0)
    }
    #[doc = "Shared Set0 I2S signals"]
    #[inline(always)]
    pub fn sckinsel_1(self) -> &'a mut W {
        self.variant(DATAINSEL_A::SCKINSEL_1)
    }
    #[doc = "Shared Set1 I2S signals"]
    #[inline(always)]
    pub fn sckinsel_2(self) -> &'a mut W {
        self.variant(DATAINSEL_A::SCKINSEL_2)
    }
}
#[doc = "Field `DATAOUTSEL` reader - DATA OUT Select"]
pub type DATAOUTSEL_R = crate::FieldReader<u8, DATAOUTSEL_A>;
#[doc = "DATA OUT Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATAOUTSEL_A {
    #[doc = "0: Original FLEXCOMM I2S signals"]
    SCKINSEL_0 = 0,
    #[doc = "1: Shared Set0 I2S signals"]
    SCKINSEL_1 = 1,
    #[doc = "2: Shared Set1 I2S signals"]
    SCKINSEL_2 = 2,
}
impl From<DATAOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DATAOUTSEL_A) -> Self {
        variant as _
    }
}
impl DATAOUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATAOUTSEL_A> {
        match self.bits {
            0 => Some(DATAOUTSEL_A::SCKINSEL_0),
            1 => Some(DATAOUTSEL_A::SCKINSEL_1),
            2 => Some(DATAOUTSEL_A::SCKINSEL_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCKINSEL_0`"]
    #[inline(always)]
    pub fn is_sckinsel_0(&self) -> bool {
        *self == DATAOUTSEL_A::SCKINSEL_0
    }
    #[doc = "Checks if the value of the field is `SCKINSEL_1`"]
    #[inline(always)]
    pub fn is_sckinsel_1(&self) -> bool {
        *self == DATAOUTSEL_A::SCKINSEL_1
    }
    #[doc = "Checks if the value of the field is `SCKINSEL_2`"]
    #[inline(always)]
    pub fn is_sckinsel_2(&self) -> bool {
        *self == DATAOUTSEL_A::SCKINSEL_2
    }
}
#[doc = "Field `DATAOUTSEL` writer - DATA OUT Select"]
pub type DATAOUTSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCCTRLSEL_SPEC, u8, DATAOUTSEL_A, 2, O>;
impl<'a, const O: u8> DATAOUTSEL_W<'a, O> {
    #[doc = "Original FLEXCOMM I2S signals"]
    #[inline(always)]
    pub fn sckinsel_0(self) -> &'a mut W {
        self.variant(DATAOUTSEL_A::SCKINSEL_0)
    }
    #[doc = "Shared Set0 I2S signals"]
    #[inline(always)]
    pub fn sckinsel_1(self) -> &'a mut W {
        self.variant(DATAOUTSEL_A::SCKINSEL_1)
    }
    #[doc = "Shared Set1 I2S signals"]
    #[inline(always)]
    pub fn sckinsel_2(self) -> &'a mut W {
        self.variant(DATAOUTSEL_A::SCKINSEL_2)
    }
}
impl R {
    #[doc = "Bits 0:1 - SCK IN Select"]
    #[inline(always)]
    pub fn sckinsel(&self) -> SCKINSEL_R {
        SCKINSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - SCK IN Select"]
    #[inline(always)]
    pub fn wsinsel(&self) -> WSINSEL_R {
        WSINSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - DATA IN Select"]
    #[inline(always)]
    pub fn datainsel(&self) -> DATAINSEL_R {
        DATAINSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - DATA OUT Select"]
    #[inline(always)]
    pub fn dataoutsel(&self) -> DATAOUTSEL_R {
        DATAOUTSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SCK IN Select"]
    #[inline(always)]
    #[must_use]
    pub fn sckinsel(&mut self) -> SCKINSEL_W<0> {
        SCKINSEL_W::new(self)
    }
    #[doc = "Bits 8:9 - SCK IN Select"]
    #[inline(always)]
    #[must_use]
    pub fn wsinsel(&mut self) -> WSINSEL_W<8> {
        WSINSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - DATA IN Select"]
    #[inline(always)]
    #[must_use]
    pub fn datainsel(&mut self) -> DATAINSEL_W<16> {
        DATAINSEL_W::new(self)
    }
    #[doc = "Bits 24:25 - DATA OUT Select"]
    #[inline(always)]
    #[must_use]
    pub fn dataoutsel(&mut self) -> DATAOUTSEL_W<24> {
        DATAOUTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flexcomm control selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcctrlsel](index.html) module"]
pub struct FCCTRLSEL_SPEC;
impl crate::RegisterSpec for FCCTRLSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcctrlsel::R](R) reader structure"]
impl crate::Readable for FCCTRLSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcctrlsel::W](W) writer structure"]
impl crate::Writable for FCCTRLSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCCTRLSEL[%s]
to value 0"]
impl crate::Resettable for FCCTRLSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
