#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENCACHE` reader - Cache Enable"]
pub type ENCACHE_R = crate::BitReader<ENCACHE_A>;
#[doc = "Cache Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENCACHE_A {
    #[doc = "0: Disables"]
    DISABLED = 0,
    #[doc = "1: Enables"]
    ENABLED = 1,
}
impl From<ENCACHE_A> for bool {
    #[inline(always)]
    fn from(variant: ENCACHE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENCACHE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENCACHE_A {
        match self.bits {
            false => ENCACHE_A::DISABLED,
            true => ENCACHE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENCACHE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENCACHE_A::ENABLED
    }
}
#[doc = "Field `ENCACHE` writer - Cache Enable"]
pub type ENCACHE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, ENCACHE_A, O>;
impl<'a, const O: u8> ENCACHE_W<'a, O> {
    #[doc = "Disables"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENCACHE_A::DISABLED)
    }
    #[doc = "Enables"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENCACHE_A::ENABLED)
    }
}
#[doc = "Field `ENWRBUF` reader - Enable Write Buffer"]
pub type ENWRBUF_R = crate::BitReader<ENWRBUF_A>;
#[doc = "Enable Write Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENWRBUF_A {
    #[doc = "0: Disables"]
    DISABLED = 0,
    #[doc = "1: Enables"]
    ENABLED = 1,
}
impl From<ENWRBUF_A> for bool {
    #[inline(always)]
    fn from(variant: ENWRBUF_A) -> Self {
        variant as u8 != 0
    }
}
impl ENWRBUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENWRBUF_A {
        match self.bits {
            false => ENWRBUF_A::DISABLED,
            true => ENWRBUF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENWRBUF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENWRBUF_A::ENABLED
    }
}
#[doc = "Field `ENWRBUF` writer - Enable Write Buffer"]
pub type ENWRBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, ENWRBUF_A, O>;
impl<'a, const O: u8> ENWRBUF_W<'a, O> {
    #[doc = "Disables"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENWRBUF_A::DISABLED)
    }
    #[doc = "Enables"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENWRBUF_A::ENABLED)
    }
}
#[doc = "Field `INVW0` reader - Invalidate Way 0"]
pub type INVW0_R = crate::BitReader<INVW0_A>;
#[doc = "Invalidate Way 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INVW0_A {
    #[doc = "0: No operation"]
    NO_OPERATION = 0,
    #[doc = "1: Invalidates all lines in way 0"]
    INVW0 = 1,
}
impl From<INVW0_A> for bool {
    #[inline(always)]
    fn from(variant: INVW0_A) -> Self {
        variant as u8 != 0
    }
}
impl INVW0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVW0_A {
        match self.bits {
            false => INVW0_A::NO_OPERATION,
            true => INVW0_A::INVW0,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OPERATION`"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == INVW0_A::NO_OPERATION
    }
    #[doc = "Checks if the value of the field is `INVW0`"]
    #[inline(always)]
    pub fn is_invw0(&self) -> bool {
        *self == INVW0_A::INVW0
    }
}
#[doc = "Field `INVW0` writer - Invalidate Way 0"]
pub type INVW0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, INVW0_A, O>;
impl<'a, const O: u8> INVW0_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(INVW0_A::NO_OPERATION)
    }
    #[doc = "Invalidates all lines in way 0"]
    #[inline(always)]
    pub fn invw0(self) -> &'a mut W {
        self.variant(INVW0_A::INVW0)
    }
}
#[doc = "Field `PUSHW0` reader - Push Way 0"]
pub type PUSHW0_R = crate::BitReader<PUSHW0_A>;
#[doc = "Push Way 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUSHW0_A {
    #[doc = "0: No operation"]
    NO_OPERATION = 0,
    #[doc = "1: Push all modified lines in way 0"]
    PUSHW0 = 1,
}
impl From<PUSHW0_A> for bool {
    #[inline(always)]
    fn from(variant: PUSHW0_A) -> Self {
        variant as u8 != 0
    }
}
impl PUSHW0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUSHW0_A {
        match self.bits {
            false => PUSHW0_A::NO_OPERATION,
            true => PUSHW0_A::PUSHW0,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OPERATION`"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == PUSHW0_A::NO_OPERATION
    }
    #[doc = "Checks if the value of the field is `PUSHW0`"]
    #[inline(always)]
    pub fn is_pushw0(&self) -> bool {
        *self == PUSHW0_A::PUSHW0
    }
}
#[doc = "Field `PUSHW0` writer - Push Way 0"]
pub type PUSHW0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, PUSHW0_A, O>;
impl<'a, const O: u8> PUSHW0_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(PUSHW0_A::NO_OPERATION)
    }
    #[doc = "Push all modified lines in way 0"]
    #[inline(always)]
    pub fn pushw0(self) -> &'a mut W {
        self.variant(PUSHW0_A::PUSHW0)
    }
}
#[doc = "Field `INVW1` reader - Invalidate Way 1"]
pub type INVW1_R = crate::BitReader<INVW1_A>;
#[doc = "Invalidate Way 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INVW1_A {
    #[doc = "0: No operation"]
    NO_OPERATION = 0,
    #[doc = "1: Invalidates all lines in way 1"]
    INVW1 = 1,
}
impl From<INVW1_A> for bool {
    #[inline(always)]
    fn from(variant: INVW1_A) -> Self {
        variant as u8 != 0
    }
}
impl INVW1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVW1_A {
        match self.bits {
            false => INVW1_A::NO_OPERATION,
            true => INVW1_A::INVW1,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OPERATION`"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == INVW1_A::NO_OPERATION
    }
    #[doc = "Checks if the value of the field is `INVW1`"]
    #[inline(always)]
    pub fn is_invw1(&self) -> bool {
        *self == INVW1_A::INVW1
    }
}
#[doc = "Field `INVW1` writer - Invalidate Way 1"]
pub type INVW1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, INVW1_A, O>;
impl<'a, const O: u8> INVW1_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(INVW1_A::NO_OPERATION)
    }
    #[doc = "Invalidates all lines in way 1"]
    #[inline(always)]
    pub fn invw1(self) -> &'a mut W {
        self.variant(INVW1_A::INVW1)
    }
}
#[doc = "Field `PUSHW1` reader - Push Way 1"]
pub type PUSHW1_R = crate::BitReader<PUSHW1_A>;
#[doc = "Push Way 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUSHW1_A {
    #[doc = "0: No operation"]
    NO_OPERATION = 0,
    #[doc = "1: Push all modified lines in way 1"]
    PUSHW1 = 1,
}
impl From<PUSHW1_A> for bool {
    #[inline(always)]
    fn from(variant: PUSHW1_A) -> Self {
        variant as u8 != 0
    }
}
impl PUSHW1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUSHW1_A {
        match self.bits {
            false => PUSHW1_A::NO_OPERATION,
            true => PUSHW1_A::PUSHW1,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OPERATION`"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == PUSHW1_A::NO_OPERATION
    }
    #[doc = "Checks if the value of the field is `PUSHW1`"]
    #[inline(always)]
    pub fn is_pushw1(&self) -> bool {
        *self == PUSHW1_A::PUSHW1
    }
}
#[doc = "Field `PUSHW1` writer - Push Way 1"]
pub type PUSHW1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, PUSHW1_A, O>;
impl<'a, const O: u8> PUSHW1_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(PUSHW1_A::NO_OPERATION)
    }
    #[doc = "Push all modified lines in way 1"]
    #[inline(always)]
    pub fn pushw1(self) -> &'a mut W {
        self.variant(PUSHW1_A::PUSHW1)
    }
}
#[doc = "Field `GO` reader - Initiate Cache Command"]
pub type GO_R = crate::BitReader<GO_A>;
#[doc = "Initiate Cache Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GO_A {
    #[doc = "0: Write: no effect; Read: no cache command active"]
    NO_EFFECT = 0,
    #[doc = "1: Write: initiates cache command; Read: cache command active"]
    INIT_CMD = 1,
}
impl From<GO_A> for bool {
    #[inline(always)]
    fn from(variant: GO_A) -> Self {
        variant as u8 != 0
    }
}
impl GO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GO_A {
        match self.bits {
            false => GO_A::NO_EFFECT,
            true => GO_A::INIT_CMD,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == GO_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `INIT_CMD`"]
    #[inline(always)]
    pub fn is_init_cmd(&self) -> bool {
        *self == GO_A::INIT_CMD
    }
}
#[doc = "Field `GO` writer - Initiate Cache Command"]
pub type GO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, GO_A, O>;
impl<'a, const O: u8> GO_W<'a, O> {
    #[doc = "Write: no effect; Read: no cache command active"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(GO_A::NO_EFFECT)
    }
    #[doc = "Write: initiates cache command; Read: cache command active"]
    #[inline(always)]
    pub fn init_cmd(self) -> &'a mut W {
        self.variant(GO_A::INIT_CMD)
    }
}
impl R {
    #[doc = "Bit 0 - Cache Enable"]
    #[inline(always)]
    pub fn encache(&self) -> ENCACHE_R {
        ENCACHE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Buffer"]
    #[inline(always)]
    pub fn enwrbuf(&self) -> ENWRBUF_R {
        ENWRBUF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 24 - Invalidate Way 0"]
    #[inline(always)]
    pub fn invw0(&self) -> INVW0_R {
        INVW0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Push Way 0"]
    #[inline(always)]
    pub fn pushw0(&self) -> PUSHW0_R {
        PUSHW0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Invalidate Way 1"]
    #[inline(always)]
    pub fn invw1(&self) -> INVW1_R {
        INVW1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Push Way 1"]
    #[inline(always)]
    pub fn pushw1(&self) -> PUSHW1_R {
        PUSHW1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - Initiate Cache Command"]
    #[inline(always)]
    pub fn go(&self) -> GO_R {
        GO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache Enable"]
    #[inline(always)]
    #[must_use]
    pub fn encache(&mut self) -> ENCACHE_W<0> {
        ENCACHE_W::new(self)
    }
    #[doc = "Bit 1 - Enable Write Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn enwrbuf(&mut self) -> ENWRBUF_W<1> {
        ENWRBUF_W::new(self)
    }
    #[doc = "Bit 24 - Invalidate Way 0"]
    #[inline(always)]
    #[must_use]
    pub fn invw0(&mut self) -> INVW0_W<24> {
        INVW0_W::new(self)
    }
    #[doc = "Bit 25 - Push Way 0"]
    #[inline(always)]
    #[must_use]
    pub fn pushw0(&mut self) -> PUSHW0_W<25> {
        PUSHW0_W::new(self)
    }
    #[doc = "Bit 26 - Invalidate Way 1"]
    #[inline(always)]
    #[must_use]
    pub fn invw1(&mut self) -> INVW1_W<26> {
        INVW1_W::new(self)
    }
    #[doc = "Bit 27 - Push Way 1"]
    #[inline(always)]
    #[must_use]
    pub fn pushw1(&mut self) -> PUSHW1_W<27> {
        PUSHW1_W::new(self)
    }
    #[doc = "Bit 31 - Initiate Cache Command"]
    #[inline(always)]
    #[must_use]
    pub fn go(&mut self) -> GO_W<31> {
        GO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
