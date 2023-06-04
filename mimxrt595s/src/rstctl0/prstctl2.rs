#[doc = "Register `PRSTCTL2` reader"]
pub struct R(crate::R<PRSTCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSTCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSTCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSTCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRSTCTL2` writer"]
pub struct W(crate::W<PRSTCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSTCTL2_SPEC>;
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
impl From<crate::W<PRSTCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSTCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UTICK0` reader - Micro-tick timer reset control"]
pub type UTICK0_R = crate::BitReader<UTICK0_A>;
#[doc = "Micro-tick timer reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UTICK0_A {
    #[doc = "0: Clear Reset"]
    UTICK0_CLR = 0,
    #[doc = "1: Set Reset"]
    UTICK0_SET = 1,
}
impl From<UTICK0_A> for bool {
    #[inline(always)]
    fn from(variant: UTICK0_A) -> Self {
        variant as u8 != 0
    }
}
impl UTICK0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTICK0_A {
        match self.bits {
            false => UTICK0_A::UTICK0_CLR,
            true => UTICK0_A::UTICK0_SET,
        }
    }
    #[doc = "Checks if the value of the field is `UTICK0_CLR`"]
    #[inline(always)]
    pub fn is_utick0_clr(&self) -> bool {
        *self == UTICK0_A::UTICK0_CLR
    }
    #[doc = "Checks if the value of the field is `UTICK0_SET`"]
    #[inline(always)]
    pub fn is_utick0_set(&self) -> bool {
        *self == UTICK0_A::UTICK0_SET
    }
}
#[doc = "Field `UTICK0` writer - Micro-tick timer reset control"]
pub type UTICK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SPEC, UTICK0_A, O>;
impl<'a, const O: u8> UTICK0_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn utick0_clr(self) -> &'a mut W {
        self.variant(UTICK0_A::UTICK0_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn utick0_set(self) -> &'a mut W {
        self.variant(UTICK0_A::UTICK0_SET)
    }
}
#[doc = "Field `WWDT0` reader - Watchdog timer reset control"]
pub type WWDT0_R = crate::BitReader<WWDT0_A>;
#[doc = "Watchdog timer reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDT0_A {
    #[doc = "0: Clear Reset"]
    WWDT0_CLR = 0,
    #[doc = "1: Set Reset"]
    WWDT0_SET = 1,
}
impl From<WWDT0_A> for bool {
    #[inline(always)]
    fn from(variant: WWDT0_A) -> Self {
        variant as u8 != 0
    }
}
impl WWDT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT0_A {
        match self.bits {
            false => WWDT0_A::WWDT0_CLR,
            true => WWDT0_A::WWDT0_SET,
        }
    }
    #[doc = "Checks if the value of the field is `WWDT0_CLR`"]
    #[inline(always)]
    pub fn is_wwdt0_clr(&self) -> bool {
        *self == WWDT0_A::WWDT0_CLR
    }
    #[doc = "Checks if the value of the field is `WWDT0_SET`"]
    #[inline(always)]
    pub fn is_wwdt0_set(&self) -> bool {
        *self == WWDT0_A::WWDT0_SET
    }
}
#[doc = "Field `WWDT0` writer - Watchdog timer reset control"]
pub type WWDT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSTCTL2_SPEC, WWDT0_A, O>;
impl<'a, const O: u8> WWDT0_W<'a, O> {
    #[doc = "Clear Reset"]
    #[inline(always)]
    pub fn wwdt0_clr(self) -> &'a mut W {
        self.variant(WWDT0_A::WWDT0_CLR)
    }
    #[doc = "Set Reset"]
    #[inline(always)]
    pub fn wwdt0_set(self) -> &'a mut W {
        self.variant(WWDT0_A::WWDT0_SET)
    }
}
impl R {
    #[doc = "Bit 0 - Micro-tick timer reset control"]
    #[inline(always)]
    pub fn utick0(&self) -> UTICK0_R {
        UTICK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog timer reset control"]
    #[inline(always)]
    pub fn wwdt0(&self) -> WWDT0_R {
        WWDT0_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Micro-tick timer reset control"]
    #[inline(always)]
    #[must_use]
    pub fn utick0(&mut self) -> UTICK0_W<0> {
        UTICK0_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog timer reset control"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt0(&mut self) -> WWDT0_W<1> {
        WWDT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Reset Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstctl2](index.html) module"]
pub struct PRSTCTL2_SPEC;
impl crate::RegisterSpec for PRSTCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prstctl2::R](R) reader structure"]
impl crate::Readable for PRSTCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prstctl2::W](W) writer structure"]
impl crate::Writable for PRSTCTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSTCTL2 to value 0x1c00_0001"]
impl crate::Resettable for PRSTCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x1c00_0001;
}
