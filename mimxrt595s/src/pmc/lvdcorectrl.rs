#[doc = "Register `LVDCORECTRL` reader"]
pub struct R(crate::R<LVDCORECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVDCORECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVDCORECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVDCORECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVDCORECTRL` writer"]
pub struct W(crate::W<LVDCORECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVDCORECTRL_SPEC>;
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
impl From<crate::W<LVDCORECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVDCORECTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVDCORELVL` reader - Vddcore LVD falling trip voltage, in steps of 15mV"]
pub type LVDCORELVL_R = crate::FieldReader<u8, LVDCORELVL_A>;
#[doc = "Vddcore LVD falling trip voltage, in steps of 15mV\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVDCORELVL_A {
    #[doc = "0: 0.720V"]
    VALUE_0B0000 = 0,
    #[doc = "7: 0.825V = 0.720V + 7 x 15mV"]
    VALUE_0B0111 = 7,
    #[doc = "15: 0.945V"]
    VALUE_0B1111 = 15,
}
impl From<LVDCORELVL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVDCORELVL_A) -> Self {
        variant as _
    }
}
impl LVDCORELVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LVDCORELVL_A> {
        match self.bits {
            0 => Some(LVDCORELVL_A::VALUE_0B0000),
            7 => Some(LVDCORELVL_A::VALUE_0B0111),
            15 => Some(LVDCORELVL_A::VALUE_0B1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0B0000`"]
    #[inline(always)]
    pub fn is_value_0b0000(&self) -> bool {
        *self == LVDCORELVL_A::VALUE_0B0000
    }
    #[doc = "Checks if the value of the field is `VALUE_0B0111`"]
    #[inline(always)]
    pub fn is_value_0b0111(&self) -> bool {
        *self == LVDCORELVL_A::VALUE_0B0111
    }
    #[doc = "Checks if the value of the field is `VALUE_0B1111`"]
    #[inline(always)]
    pub fn is_value_0b1111(&self) -> bool {
        *self == LVDCORELVL_A::VALUE_0B1111
    }
}
#[doc = "Field `LVDCORELVL` writer - Vddcore LVD falling trip voltage, in steps of 15mV"]
pub type LVDCORELVL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LVDCORECTRL_SPEC, u8, LVDCORELVL_A, 4, O>;
impl<'a, const O: u8> LVDCORELVL_W<'a, O> {
    #[doc = "0.720V"]
    #[inline(always)]
    pub fn value_0b0000(self) -> &'a mut W {
        self.variant(LVDCORELVL_A::VALUE_0B0000)
    }
    #[doc = "0.825V = 0.720V + 7 x 15mV"]
    #[inline(always)]
    pub fn value_0b0111(self) -> &'a mut W {
        self.variant(LVDCORELVL_A::VALUE_0B0111)
    }
    #[doc = "0.945V"]
    #[inline(always)]
    pub fn value_0b1111(self) -> &'a mut W {
        self.variant(LVDCORELVL_A::VALUE_0B1111)
    }
}
impl R {
    #[doc = "Bits 0:3 - Vddcore LVD falling trip voltage, in steps of 15mV"]
    #[inline(always)]
    pub fn lvdcorelvl(&self) -> LVDCORELVL_R {
        LVDCORELVL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Vddcore LVD falling trip voltage, in steps of 15mV"]
    #[inline(always)]
    #[must_use]
    pub fn lvdcorelvl(&mut self) -> LVDCORELVL_W<0> {
        LVDCORELVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMC Active vddcore LVD monitor trip adjust\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvdcorectrl](index.html) module"]
pub struct LVDCORECTRL_SPEC;
impl crate::RegisterSpec for LVDCORECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lvdcorectrl::R](R) reader structure"]
impl crate::Readable for LVDCORECTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvdcorectrl::W](W) writer structure"]
impl crate::Writable for LVDCORECTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVDCORECTRL to value 0x0a"]
impl crate::Resettable for LVDCORECTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
