#[doc = "Register `HADDRSTART` reader"]
pub struct R(crate::R<HADDRSTART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HADDRSTART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HADDRSTART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HADDRSTART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HADDRSTART` writer"]
pub struct W(crate::W<HADDRSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HADDRSTART_SPEC>;
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
impl From<crate::W<HADDRSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HADDRSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REMAPEN` reader - AHB Bus address remap function enable"]
pub type REMAPEN_R = crate::BitReader<REMAPEN_A>;
#[doc = "AHB Bus address remap function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REMAPEN_A {
    #[doc = "0: HADDR REMAP Disabled"]
    VAL0 = 0,
    #[doc = "1: HADDR REMAP Enabled"]
    VAL1 = 1,
}
impl From<REMAPEN_A> for bool {
    #[inline(always)]
    fn from(variant: REMAPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl REMAPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REMAPEN_A {
        match self.bits {
            false => REMAPEN_A::VAL0,
            true => REMAPEN_A::VAL1,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == REMAPEN_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == REMAPEN_A::VAL1
    }
}
#[doc = "Field `REMAPEN` writer - AHB Bus address remap function enable"]
pub type REMAPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HADDRSTART_SPEC, REMAPEN_A, O>;
impl<'a, const O: u8> REMAPEN_W<'a, O> {
    #[doc = "HADDR REMAP Disabled"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(REMAPEN_A::VAL0)
    }
    #[doc = "HADDR REMAP Enabled"]
    #[inline(always)]
    pub fn val1(self) -> &'a mut W {
        self.variant(REMAPEN_A::VAL1)
    }
}
#[doc = "Field `ADDRSTART` reader - HADDR start address"]
pub type ADDRSTART_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDRSTART` writer - HADDR start address"]
pub type ADDRSTART_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HADDRSTART_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bit 0 - AHB Bus address remap function enable"]
    #[inline(always)]
    pub fn remapen(&self) -> REMAPEN_R {
        REMAPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 12:31 - HADDR start address"]
    #[inline(always)]
    pub fn addrstart(&self) -> ADDRSTART_R {
        ADDRSTART_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - AHB Bus address remap function enable"]
    #[inline(always)]
    #[must_use]
    pub fn remapen(&mut self) -> REMAPEN_W<0> {
        REMAPEN_W::new(self)
    }
    #[doc = "Bits 12:31 - HADDR start address"]
    #[inline(always)]
    #[must_use]
    pub fn addrstart(&mut self) -> ADDRSTART_W<12> {
        ADDRSTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HADDR REMAP START ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [haddrstart](index.html) module"]
pub struct HADDRSTART_SPEC;
impl crate::RegisterSpec for HADDRSTART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [haddrstart::R](R) reader structure"]
impl crate::Readable for HADDRSTART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [haddrstart::W](W) writer structure"]
impl crate::Writable for HADDRSTART_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HADDRSTART to value 0"]
impl crate::Resettable for HADDRSTART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
