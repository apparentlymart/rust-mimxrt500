#[doc = "Register `CursorAddress` reader"]
pub struct R(crate::R<CURSOR_ADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CURSOR_ADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CURSOR_ADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CURSOR_ADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CursorAddress` writer"]
pub struct W(crate::W<CURSOR_ADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CURSOR_ADDRESS_SPEC>;
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
impl From<crate::W<CURSOR_ADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CURSOR_ADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS` reader - ADDRESS"]
pub type ADDRESS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDRESS` writer - ADDRESS"]
pub type ADDRESS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CURSOR_ADDRESS_SPEC, u32, u32, 31, O>;
#[doc = "Field `TYPE` reader - System Type"]
pub type TYPE_R = crate::BitReader<TYPE_A>;
#[doc = "System Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TYPE_A {
    #[doc = "0: System."]
    DISABLE = 0,
    #[doc = "1: Virtual system."]
    ENABLE = 1,
}
impl From<TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE_A {
        match self.bits {
            false => TYPE_A::DISABLE,
            true => TYPE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TYPE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TYPE_A::ENABLE
    }
}
#[doc = "Field `TYPE` writer - System Type"]
pub type TYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CURSOR_ADDRESS_SPEC, TYPE_A, O>;
impl<'a, const O: u8> TYPE_W<'a, O> {
    #[doc = "System."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TYPE_A::DISABLE)
    }
    #[doc = "Virtual system."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TYPE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:30 - ADDRESS"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - System Type"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - ADDRESS"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<0> {
        ADDRESS_W::new(self)
    }
    #[doc = "Bit 31 - System Type"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TYPE_W<31> {
        TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Address of the Cursor Shape\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cursor_address](index.html) module"]
pub struct CURSOR_ADDRESS_SPEC;
impl crate::RegisterSpec for CURSOR_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cursor_address::R](R) reader structure"]
impl crate::Readable for CURSOR_ADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cursor_address::W](W) writer structure"]
impl crate::Writable for CURSOR_ADDRESS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CursorAddress to value 0"]
impl crate::Resettable for CURSOR_ADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
