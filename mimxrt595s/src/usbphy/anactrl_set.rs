#[doc = "Register `ANACTRL_SET` reader"]
pub struct R(crate::R<ANACTRL_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANACTRL_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANACTRL_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANACTRL_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANACTRL_SET` writer"]
pub struct W(crate::W<ANACTRL_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANACTRL_SET_SPEC>;
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
impl From<crate::W<ANACTRL_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANACTRL_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEV_PULLDOWN` reader - Device Pull-down"]
pub type DEV_PULLDOWN_R = crate::BitReader<DEV_PULLDOWN_A>;
#[doc = "Device Pull-down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEV_PULLDOWN_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding bit"]
    ENABLE = 1,
}
impl From<DEV_PULLDOWN_A> for bool {
    #[inline(always)]
    fn from(variant: DEV_PULLDOWN_A) -> Self {
        variant as u8 != 0
    }
}
impl DEV_PULLDOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEV_PULLDOWN_A {
        match self.bits {
            false => DEV_PULLDOWN_A::DISABLE,
            true => DEV_PULLDOWN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DEV_PULLDOWN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DEV_PULLDOWN_A::ENABLE
    }
}
#[doc = "Field `DEV_PULLDOWN` writer - Device Pull-down"]
pub type DEV_PULLDOWN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ANACTRL_SET_SPEC, DEV_PULLDOWN_A, O>;
impl<'a, const O: u8> DEV_PULLDOWN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DEV_PULLDOWN_A::DISABLE)
    }
    #[doc = "Sets the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DEV_PULLDOWN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 10 - Device Pull-down"]
    #[inline(always)]
    pub fn dev_pulldown(&self) -> DEV_PULLDOWN_R {
        DEV_PULLDOWN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Device Pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn dev_pulldown(&mut self) -> DEV_PULLDOWN_W<10> {
        DEV_PULLDOWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Control Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anactrl_set](index.html) module"]
pub struct ANACTRL_SET_SPEC;
impl crate::RegisterSpec for ANACTRL_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [anactrl_set::R](R) reader structure"]
impl crate::Readable for ANACTRL_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [anactrl_set::W](W) writer structure"]
impl crate::Writable for ANACTRL_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANACTRL_SET to value 0x0400"]
impl crate::Resettable for ANACTRL_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
