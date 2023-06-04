#[doc = "Register `DMAC0_ITRIG_ENA0` reader"]
pub struct R(crate::R<DMAC0_ITRIG_ENA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC0_ITRIG_ENA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC0_ITRIG_ENA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC0_ITRIG_ENA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC0_ITRIG_ENA0` writer"]
pub struct W(crate::W<DMAC0_ITRIG_ENA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC0_ITRIG_ENA0_SPEC>;
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
impl From<crate::W<DMAC0_ITRIG_ENA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC0_ITRIG_ENA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_INT0` reader - GPIO_INT0"]
pub type GPIO_INT0_R = crate::BitReader<GPIO_INT0_A>;
#[doc = "GPIO_INT0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT0_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<GPIO_INT0_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_A {
        match self.bits {
            false => GPIO_INT0_A::DISABLE,
            true => GPIO_INT0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT0_A::ENABLE
    }
}
#[doc = "Field `GPIO_INT0` writer - GPIO_INT0"]
pub type GPIO_INT0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, GPIO_INT0_A, O>;
impl<'a, const O: u8> GPIO_INT0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT0_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT0_A::ENABLE)
    }
}
#[doc = "Field `GPIO_INT1` reader - GPIO_INT1"]
pub type GPIO_INT1_R = crate::BitReader<GPIO_INT1_A>;
#[doc = "GPIO_INT1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT1_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<GPIO_INT1_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT1_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT1_A {
        match self.bits {
            false => GPIO_INT1_A::DISABLE,
            true => GPIO_INT1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT1_A::ENABLE
    }
}
#[doc = "Field `GPIO_INT1` writer - GPIO_INT1"]
pub type GPIO_INT1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, GPIO_INT1_A, O>;
impl<'a, const O: u8> GPIO_INT1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT1_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT1_A::ENABLE)
    }
}
#[doc = "Field `GPIO_INT2` reader - GPIO_INT2"]
pub type GPIO_INT2_R = crate::BitReader<GPIO_INT2_A>;
#[doc = "GPIO_INT2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT2_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<GPIO_INT2_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT2_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT2_A {
        match self.bits {
            false => GPIO_INT2_A::DISABLE,
            true => GPIO_INT2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT2_A::ENABLE
    }
}
#[doc = "Field `GPIO_INT2` writer - GPIO_INT2"]
pub type GPIO_INT2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, GPIO_INT2_A, O>;
impl<'a, const O: u8> GPIO_INT2_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT2_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT2_A::ENABLE)
    }
}
#[doc = "Field `GPIO_INT3` reader - GPIO_INT3"]
pub type GPIO_INT3_R = crate::BitReader<GPIO_INT3_A>;
#[doc = "GPIO_INT3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_INT3_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<GPIO_INT3_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT3_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_INT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT3_A {
        match self.bits {
            false => GPIO_INT3_A::DISABLE,
            true => GPIO_INT3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT3_A::ENABLE
    }
}
#[doc = "Field `GPIO_INT3` writer - GPIO_INT3"]
pub type GPIO_INT3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, GPIO_INT3_A, O>;
impl<'a, const O: u8> GPIO_INT3_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT3_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT3_A::ENABLE)
    }
}
#[doc = "Field `T0_DMAREQ_M0` reader - T0_DMAREQ_M0"]
pub type T0_DMAREQ_M0_R = crate::BitReader<T0_DMAREQ_M0_A>;
#[doc = "T0_DMAREQ_M0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T0_DMAREQ_M0_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<T0_DMAREQ_M0_A> for bool {
    #[inline(always)]
    fn from(variant: T0_DMAREQ_M0_A) -> Self {
        variant as u8 != 0
    }
}
impl T0_DMAREQ_M0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T0_DMAREQ_M0_A {
        match self.bits {
            false => T0_DMAREQ_M0_A::DISABLE,
            true => T0_DMAREQ_M0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == T0_DMAREQ_M0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == T0_DMAREQ_M0_A::ENABLE
    }
}
#[doc = "Field `T0_DMAREQ_M0` writer - T0_DMAREQ_M0"]
pub type T0_DMAREQ_M0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, T0_DMAREQ_M0_A, O>;
impl<'a, const O: u8> T0_DMAREQ_M0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T0_DMAREQ_M0_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T0_DMAREQ_M0_A::ENABLE)
    }
}
#[doc = "Field `T0_DMAREQ_M1` reader - T0_DMAREQ_M1"]
pub type T0_DMAREQ_M1_R = crate::BitReader<T0_DMAREQ_M1_A>;
#[doc = "T0_DMAREQ_M1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T0_DMAREQ_M1_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<T0_DMAREQ_M1_A> for bool {
    #[inline(always)]
    fn from(variant: T0_DMAREQ_M1_A) -> Self {
        variant as u8 != 0
    }
}
impl T0_DMAREQ_M1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T0_DMAREQ_M1_A {
        match self.bits {
            false => T0_DMAREQ_M1_A::DISABLE,
            true => T0_DMAREQ_M1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == T0_DMAREQ_M1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == T0_DMAREQ_M1_A::ENABLE
    }
}
#[doc = "Field `T0_DMAREQ_M1` writer - T0_DMAREQ_M1"]
pub type T0_DMAREQ_M1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, T0_DMAREQ_M1_A, O>;
impl<'a, const O: u8> T0_DMAREQ_M1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T0_DMAREQ_M1_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T0_DMAREQ_M1_A::ENABLE)
    }
}
#[doc = "Field `T1_DMAREQ_M0` reader - T1_DMAREQ_M0"]
pub type T1_DMAREQ_M0_R = crate::BitReader<T1_DMAREQ_M0_A>;
#[doc = "T1_DMAREQ_M0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T1_DMAREQ_M0_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<T1_DMAREQ_M0_A> for bool {
    #[inline(always)]
    fn from(variant: T1_DMAREQ_M0_A) -> Self {
        variant as u8 != 0
    }
}
impl T1_DMAREQ_M0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T1_DMAREQ_M0_A {
        match self.bits {
            false => T1_DMAREQ_M0_A::DISABLE,
            true => T1_DMAREQ_M0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == T1_DMAREQ_M0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == T1_DMAREQ_M0_A::ENABLE
    }
}
#[doc = "Field `T1_DMAREQ_M0` writer - T1_DMAREQ_M0"]
pub type T1_DMAREQ_M0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, T1_DMAREQ_M0_A, O>;
impl<'a, const O: u8> T1_DMAREQ_M0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T1_DMAREQ_M0_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T1_DMAREQ_M0_A::ENABLE)
    }
}
#[doc = "Field `T1_DMAREQ_M1` reader - T1_DMAREQ_M1"]
pub type T1_DMAREQ_M1_R = crate::BitReader<T1_DMAREQ_M1_A>;
#[doc = "T1_DMAREQ_M1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T1_DMAREQ_M1_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<T1_DMAREQ_M1_A> for bool {
    #[inline(always)]
    fn from(variant: T1_DMAREQ_M1_A) -> Self {
        variant as u8 != 0
    }
}
impl T1_DMAREQ_M1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T1_DMAREQ_M1_A {
        match self.bits {
            false => T1_DMAREQ_M1_A::DISABLE,
            true => T1_DMAREQ_M1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == T1_DMAREQ_M1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == T1_DMAREQ_M1_A::ENABLE
    }
}
#[doc = "Field `T1_DMAREQ_M1` writer - T1_DMAREQ_M1"]
pub type T1_DMAREQ_M1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, T1_DMAREQ_M1_A, O>;
impl<'a, const O: u8> T1_DMAREQ_M1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T1_DMAREQ_M1_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T1_DMAREQ_M1_A::ENABLE)
    }
}
#[doc = "Field `T2_DMAREQ_M0` reader - T2_DMAREQ_M0"]
pub type T2_DMAREQ_M0_R = crate::BitReader<T2_DMAREQ_M0_A>;
#[doc = "T2_DMAREQ_M0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T2_DMAREQ_M0_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<T2_DMAREQ_M0_A> for bool {
    #[inline(always)]
    fn from(variant: T2_DMAREQ_M0_A) -> Self {
        variant as u8 != 0
    }
}
impl T2_DMAREQ_M0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T2_DMAREQ_M0_A {
        match self.bits {
            false => T2_DMAREQ_M0_A::DISABLE,
            true => T2_DMAREQ_M0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == T2_DMAREQ_M0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == T2_DMAREQ_M0_A::ENABLE
    }
}
#[doc = "Field `T2_DMAREQ_M0` writer - T2_DMAREQ_M0"]
pub type T2_DMAREQ_M0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, T2_DMAREQ_M0_A, O>;
impl<'a, const O: u8> T2_DMAREQ_M0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T2_DMAREQ_M0_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T2_DMAREQ_M0_A::ENABLE)
    }
}
#[doc = "Field `T2_DMAREQ_M1` reader - T2_DMAREQ_M1"]
pub type T2_DMAREQ_M1_R = crate::BitReader<T2_DMAREQ_M1_A>;
#[doc = "T2_DMAREQ_M1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T2_DMAREQ_M1_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<T2_DMAREQ_M1_A> for bool {
    #[inline(always)]
    fn from(variant: T2_DMAREQ_M1_A) -> Self {
        variant as u8 != 0
    }
}
impl T2_DMAREQ_M1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T2_DMAREQ_M1_A {
        match self.bits {
            false => T2_DMAREQ_M1_A::DISABLE,
            true => T2_DMAREQ_M1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == T2_DMAREQ_M1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == T2_DMAREQ_M1_A::ENABLE
    }
}
#[doc = "Field `T2_DMAREQ_M1` writer - T2_DMAREQ_M1"]
pub type T2_DMAREQ_M1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, T2_DMAREQ_M1_A, O>;
impl<'a, const O: u8> T2_DMAREQ_M1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T2_DMAREQ_M1_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T2_DMAREQ_M1_A::ENABLE)
    }
}
#[doc = "Field `T3_DMAREQ_M0` reader - T3_DMAREQ_M0"]
pub type T3_DMAREQ_M0_R = crate::BitReader<T3_DMAREQ_M0_A>;
#[doc = "T3_DMAREQ_M0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T3_DMAREQ_M0_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<T3_DMAREQ_M0_A> for bool {
    #[inline(always)]
    fn from(variant: T3_DMAREQ_M0_A) -> Self {
        variant as u8 != 0
    }
}
impl T3_DMAREQ_M0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T3_DMAREQ_M0_A {
        match self.bits {
            false => T3_DMAREQ_M0_A::DISABLE,
            true => T3_DMAREQ_M0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == T3_DMAREQ_M0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == T3_DMAREQ_M0_A::ENABLE
    }
}
#[doc = "Field `T3_DMAREQ_M0` writer - T3_DMAREQ_M0"]
pub type T3_DMAREQ_M0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, T3_DMAREQ_M0_A, O>;
impl<'a, const O: u8> T3_DMAREQ_M0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T3_DMAREQ_M0_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T3_DMAREQ_M0_A::ENABLE)
    }
}
#[doc = "Field `T3_DMAREQ_M1` reader - T3_DMAREQ_M1"]
pub type T3_DMAREQ_M1_R = crate::BitReader<T3_DMAREQ_M1_A>;
#[doc = "T3_DMAREQ_M1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T3_DMAREQ_M1_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<T3_DMAREQ_M1_A> for bool {
    #[inline(always)]
    fn from(variant: T3_DMAREQ_M1_A) -> Self {
        variant as u8 != 0
    }
}
impl T3_DMAREQ_M1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T3_DMAREQ_M1_A {
        match self.bits {
            false => T3_DMAREQ_M1_A::DISABLE,
            true => T3_DMAREQ_M1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == T3_DMAREQ_M1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == T3_DMAREQ_M1_A::ENABLE
    }
}
#[doc = "Field `T3_DMAREQ_M1` writer - T3_DMAREQ_M1"]
pub type T3_DMAREQ_M1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, T3_DMAREQ_M1_A, O>;
impl<'a, const O: u8> T3_DMAREQ_M1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T3_DMAREQ_M1_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T3_DMAREQ_M1_A::ENABLE)
    }
}
#[doc = "Field `T4_DMAREQ_M0` reader - T4_DMAREQ_M0"]
pub type T4_DMAREQ_M0_R = crate::BitReader<T4_DMAREQ_M0_A>;
#[doc = "T4_DMAREQ_M0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T4_DMAREQ_M0_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<T4_DMAREQ_M0_A> for bool {
    #[inline(always)]
    fn from(variant: T4_DMAREQ_M0_A) -> Self {
        variant as u8 != 0
    }
}
impl T4_DMAREQ_M0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T4_DMAREQ_M0_A {
        match self.bits {
            false => T4_DMAREQ_M0_A::DISABLE,
            true => T4_DMAREQ_M0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == T4_DMAREQ_M0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == T4_DMAREQ_M0_A::ENABLE
    }
}
#[doc = "Field `T4_DMAREQ_M0` writer - T4_DMAREQ_M0"]
pub type T4_DMAREQ_M0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, T4_DMAREQ_M0_A, O>;
impl<'a, const O: u8> T4_DMAREQ_M0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T4_DMAREQ_M0_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T4_DMAREQ_M0_A::ENABLE)
    }
}
#[doc = "Field `T4_DMAREQ_M1` reader - T4_DMAREQ_M1"]
pub type T4_DMAREQ_M1_R = crate::BitReader<T4_DMAREQ_M1_A>;
#[doc = "T4_DMAREQ_M1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T4_DMAREQ_M1_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<T4_DMAREQ_M1_A> for bool {
    #[inline(always)]
    fn from(variant: T4_DMAREQ_M1_A) -> Self {
        variant as u8 != 0
    }
}
impl T4_DMAREQ_M1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T4_DMAREQ_M1_A {
        match self.bits {
            false => T4_DMAREQ_M1_A::DISABLE,
            true => T4_DMAREQ_M1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == T4_DMAREQ_M1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == T4_DMAREQ_M1_A::ENABLE
    }
}
#[doc = "Field `T4_DMAREQ_M1` writer - T4_DMAREQ_M1"]
pub type T4_DMAREQ_M1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, T4_DMAREQ_M1_A, O>;
impl<'a, const O: u8> T4_DMAREQ_M1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(T4_DMAREQ_M1_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(T4_DMAREQ_M1_A::ENABLE)
    }
}
#[doc = "Field `DMA_TRIGOUT_A` reader - DMA_TRIGOUT_A"]
pub type DMA_TRIGOUT_A_R = crate::BitReader<DMA_TRIGOUT_A_A>;
#[doc = "DMA_TRIGOUT_A\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_TRIGOUT_A_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<DMA_TRIGOUT_A_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_TRIGOUT_A_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_TRIGOUT_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_TRIGOUT_A_A {
        match self.bits {
            false => DMA_TRIGOUT_A_A::DISABLE,
            true => DMA_TRIGOUT_A_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMA_TRIGOUT_A_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMA_TRIGOUT_A_A::ENABLE
    }
}
#[doc = "Field `DMA_TRIGOUT_A` writer - DMA_TRIGOUT_A"]
pub type DMA_TRIGOUT_A_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, DMA_TRIGOUT_A_A, O>;
impl<'a, const O: u8> DMA_TRIGOUT_A_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA_TRIGOUT_A_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA_TRIGOUT_A_A::ENABLE)
    }
}
#[doc = "Field `DMA_TRIGOUT_B` reader - DMA_TRIGOUT_B"]
pub type DMA_TRIGOUT_B_R = crate::BitReader<DMA_TRIGOUT_B_A>;
#[doc = "DMA_TRIGOUT_B\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_TRIGOUT_B_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<DMA_TRIGOUT_B_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_TRIGOUT_B_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_TRIGOUT_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_TRIGOUT_B_A {
        match self.bits {
            false => DMA_TRIGOUT_B_A::DISABLE,
            true => DMA_TRIGOUT_B_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMA_TRIGOUT_B_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMA_TRIGOUT_B_A::ENABLE
    }
}
#[doc = "Field `DMA_TRIGOUT_B` writer - DMA_TRIGOUT_B"]
pub type DMA_TRIGOUT_B_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, DMA_TRIGOUT_B_A, O>;
impl<'a, const O: u8> DMA_TRIGOUT_B_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA_TRIGOUT_B_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA_TRIGOUT_B_A::ENABLE)
    }
}
#[doc = "Field `DMA_TRIGOUT_C` reader - DMA_TRIGOUT_C"]
pub type DMA_TRIGOUT_C_R = crate::BitReader<DMA_TRIGOUT_C_A>;
#[doc = "DMA_TRIGOUT_C\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_TRIGOUT_C_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<DMA_TRIGOUT_C_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_TRIGOUT_C_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_TRIGOUT_C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_TRIGOUT_C_A {
        match self.bits {
            false => DMA_TRIGOUT_C_A::DISABLE,
            true => DMA_TRIGOUT_C_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMA_TRIGOUT_C_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMA_TRIGOUT_C_A::ENABLE
    }
}
#[doc = "Field `DMA_TRIGOUT_C` writer - DMA_TRIGOUT_C"]
pub type DMA_TRIGOUT_C_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, DMA_TRIGOUT_C_A, O>;
impl<'a, const O: u8> DMA_TRIGOUT_C_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA_TRIGOUT_C_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA_TRIGOUT_C_A::ENABLE)
    }
}
#[doc = "Field `DMA_TRIGOUT_D` reader - DMA_TRIGOUT_D"]
pub type DMA_TRIGOUT_D_R = crate::BitReader<DMA_TRIGOUT_D_A>;
#[doc = "DMA_TRIGOUT_D\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_TRIGOUT_D_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<DMA_TRIGOUT_D_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_TRIGOUT_D_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_TRIGOUT_D_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_TRIGOUT_D_A {
        match self.bits {
            false => DMA_TRIGOUT_D_A::DISABLE,
            true => DMA_TRIGOUT_D_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMA_TRIGOUT_D_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMA_TRIGOUT_D_A::ENABLE
    }
}
#[doc = "Field `DMA_TRIGOUT_D` writer - DMA_TRIGOUT_D"]
pub type DMA_TRIGOUT_D_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, DMA_TRIGOUT_D_A, O>;
impl<'a, const O: u8> DMA_TRIGOUT_D_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA_TRIGOUT_D_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA_TRIGOUT_D_A::ENABLE)
    }
}
#[doc = "Field `SCT_DMA_REQ0` reader - SCT_DMA_REQ0"]
pub type SCT_DMA_REQ0_R = crate::BitReader<SCT_DMA_REQ0_A>;
#[doc = "SCT_DMA_REQ0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCT_DMA_REQ0_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<SCT_DMA_REQ0_A> for bool {
    #[inline(always)]
    fn from(variant: SCT_DMA_REQ0_A) -> Self {
        variant as u8 != 0
    }
}
impl SCT_DMA_REQ0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_DMA_REQ0_A {
        match self.bits {
            false => SCT_DMA_REQ0_A::DISABLE,
            true => SCT_DMA_REQ0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCT_DMA_REQ0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SCT_DMA_REQ0_A::ENABLE
    }
}
#[doc = "Field `SCT_DMA_REQ0` writer - SCT_DMA_REQ0"]
pub type SCT_DMA_REQ0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, SCT_DMA_REQ0_A, O>;
impl<'a, const O: u8> SCT_DMA_REQ0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCT_DMA_REQ0_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCT_DMA_REQ0_A::ENABLE)
    }
}
#[doc = "Field `SCT_DMA_REQ1` reader - SCT_DMA_REQ1"]
pub type SCT_DMA_REQ1_R = crate::BitReader<SCT_DMA_REQ1_A>;
#[doc = "SCT_DMA_REQ1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCT_DMA_REQ1_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<SCT_DMA_REQ1_A> for bool {
    #[inline(always)]
    fn from(variant: SCT_DMA_REQ1_A) -> Self {
        variant as u8 != 0
    }
}
impl SCT_DMA_REQ1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_DMA_REQ1_A {
        match self.bits {
            false => SCT_DMA_REQ1_A::DISABLE,
            true => SCT_DMA_REQ1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCT_DMA_REQ1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SCT_DMA_REQ1_A::ENABLE
    }
}
#[doc = "Field `SCT_DMA_REQ1` writer - SCT_DMA_REQ1"]
pub type SCT_DMA_REQ1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, SCT_DMA_REQ1_A, O>;
impl<'a, const O: u8> SCT_DMA_REQ1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCT_DMA_REQ1_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCT_DMA_REQ1_A::ENABLE)
    }
}
#[doc = "Field `HASHCRYPT_OUT` reader - HASHCRYPT_OUT"]
pub type HASHCRYPT_OUT_R = crate::BitReader<HASHCRYPT_OUT_A>;
#[doc = "HASHCRYPT_OUT\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHCRYPT_OUT_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<HASHCRYPT_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: HASHCRYPT_OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl HASHCRYPT_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASHCRYPT_OUT_A {
        match self.bits {
            false => HASHCRYPT_OUT_A::DISABLE,
            true => HASHCRYPT_OUT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HASHCRYPT_OUT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HASHCRYPT_OUT_A::ENABLE
    }
}
#[doc = "Field `HASHCRYPT_OUT` writer - HASHCRYPT_OUT"]
pub type HASHCRYPT_OUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, HASHCRYPT_OUT_A, O>;
impl<'a, const O: u8> HASHCRYPT_OUT_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HASHCRYPT_OUT_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HASHCRYPT_OUT_A::ENABLE)
    }
}
#[doc = "Field `ACMP` reader - ACMP"]
pub type ACMP_R = crate::BitReader<ACMP_A>;
#[doc = "ACMP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ACMP_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_A {
        match self.bits {
            false => ACMP_A::DISABLE,
            true => ACMP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ACMP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ACMP_A::ENABLE
    }
}
#[doc = "Field `ACMP` writer - ACMP"]
pub type ACMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, ACMP_A, O>;
impl<'a, const O: u8> ACMP_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACMP_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ACMP_A::ENABLE)
    }
}
#[doc = "Field `FLEXSPI0_RX` reader - FlexSPI0_RX"]
pub type FLEXSPI0_RX_R = crate::BitReader<FLEXSPI0_RX_A>;
#[doc = "FlexSPI0_RX\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI0_RX_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FLEXSPI0_RX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI0_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXSPI0_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI0_RX_A {
        match self.bits {
            false => FLEXSPI0_RX_A::DISABLE,
            true => FLEXSPI0_RX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXSPI0_RX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXSPI0_RX_A::ENABLE
    }
}
#[doc = "Field `FLEXSPI0_RX` writer - FlexSPI0_RX"]
pub type FLEXSPI0_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, FLEXSPI0_RX_A, O>;
impl<'a, const O: u8> FLEXSPI0_RX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXSPI0_RX_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXSPI0_RX_A::ENABLE)
    }
}
#[doc = "Field `FLEXSPI0_TX` reader - FlexSPI0_TX"]
pub type FLEXSPI0_TX_R = crate::BitReader<FLEXSPI0_TX_A>;
#[doc = "FlexSPI0_TX\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI0_TX_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FLEXSPI0_TX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI0_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXSPI0_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI0_TX_A {
        match self.bits {
            false => FLEXSPI0_TX_A::DISABLE,
            true => FLEXSPI0_TX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXSPI0_TX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXSPI0_TX_A::ENABLE
    }
}
#[doc = "Field `FLEXSPI0_TX` writer - FlexSPI0_TX"]
pub type FLEXSPI0_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, FLEXSPI0_TX_A, O>;
impl<'a, const O: u8> FLEXSPI0_TX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXSPI0_TX_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXSPI0_TX_A::ENABLE)
    }
}
#[doc = "Field `ADC` reader - ADC"]
pub type ADC_R = crate::BitReader<ADC_A>;
#[doc = "ADC\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::DISABLE,
            true => ADC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC_A::ENABLE
    }
}
#[doc = "Field `ADC` writer - ADC"]
pub type ADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, ADC_A, O>;
impl<'a, const O: u8> ADC_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC_A::ENABLE)
    }
}
#[doc = "Field `FLEXSPI1_RX` reader - FlexSPI1_RX"]
pub type FLEXSPI1_RX_R = crate::BitReader<FLEXSPI1_RX_A>;
#[doc = "FlexSPI1_RX\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI1_RX_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FLEXSPI1_RX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI1_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXSPI1_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI1_RX_A {
        match self.bits {
            false => FLEXSPI1_RX_A::DISABLE,
            true => FLEXSPI1_RX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXSPI1_RX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXSPI1_RX_A::ENABLE
    }
}
#[doc = "Field `FLEXSPI1_RX` writer - FlexSPI1_RX"]
pub type FLEXSPI1_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, FLEXSPI1_RX_A, O>;
impl<'a, const O: u8> FLEXSPI1_RX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXSPI1_RX_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXSPI1_RX_A::ENABLE)
    }
}
#[doc = "Field `FLEXSPI1_TX` reader - FlexSPI1_TX"]
pub type FLEXSPI1_TX_R = crate::BitReader<FLEXSPI1_TX_A>;
#[doc = "FlexSPI1_TX\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI1_TX_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FLEXSPI1_TX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI1_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXSPI1_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI1_TX_A {
        match self.bits {
            false => FLEXSPI1_TX_A::DISABLE,
            true => FLEXSPI1_TX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXSPI1_TX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXSPI1_TX_A::ENABLE
    }
}
#[doc = "Field `FLEXSPI1_TX` writer - FlexSPI1_TX"]
pub type FLEXSPI1_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_ITRIG_ENA0_SPEC, FLEXSPI1_TX_A, O>;
impl<'a, const O: u8> FLEXSPI1_TX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXSPI1_TX_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXSPI1_TX_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - GPIO_INT0"]
    #[inline(always)]
    pub fn gpio_int0(&self) -> GPIO_INT0_R {
        GPIO_INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO_INT1"]
    #[inline(always)]
    pub fn gpio_int1(&self) -> GPIO_INT1_R {
        GPIO_INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO_INT2"]
    #[inline(always)]
    pub fn gpio_int2(&self) -> GPIO_INT2_R {
        GPIO_INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO_INT3"]
    #[inline(always)]
    pub fn gpio_int3(&self) -> GPIO_INT3_R {
        GPIO_INT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - T0_DMAREQ_M0"]
    #[inline(always)]
    pub fn t0_dmareq_m0(&self) -> T0_DMAREQ_M0_R {
        T0_DMAREQ_M0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - T0_DMAREQ_M1"]
    #[inline(always)]
    pub fn t0_dmareq_m1(&self) -> T0_DMAREQ_M1_R {
        T0_DMAREQ_M1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - T1_DMAREQ_M0"]
    #[inline(always)]
    pub fn t1_dmareq_m0(&self) -> T1_DMAREQ_M0_R {
        T1_DMAREQ_M0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - T1_DMAREQ_M1"]
    #[inline(always)]
    pub fn t1_dmareq_m1(&self) -> T1_DMAREQ_M1_R {
        T1_DMAREQ_M1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - T2_DMAREQ_M0"]
    #[inline(always)]
    pub fn t2_dmareq_m0(&self) -> T2_DMAREQ_M0_R {
        T2_DMAREQ_M0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - T2_DMAREQ_M1"]
    #[inline(always)]
    pub fn t2_dmareq_m1(&self) -> T2_DMAREQ_M1_R {
        T2_DMAREQ_M1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - T3_DMAREQ_M0"]
    #[inline(always)]
    pub fn t3_dmareq_m0(&self) -> T3_DMAREQ_M0_R {
        T3_DMAREQ_M0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - T3_DMAREQ_M1"]
    #[inline(always)]
    pub fn t3_dmareq_m1(&self) -> T3_DMAREQ_M1_R {
        T3_DMAREQ_M1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - T4_DMAREQ_M0"]
    #[inline(always)]
    pub fn t4_dmareq_m0(&self) -> T4_DMAREQ_M0_R {
        T4_DMAREQ_M0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - T4_DMAREQ_M1"]
    #[inline(always)]
    pub fn t4_dmareq_m1(&self) -> T4_DMAREQ_M1_R {
        T4_DMAREQ_M1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA_TRIGOUT_A"]
    #[inline(always)]
    pub fn dma_trigout_a(&self) -> DMA_TRIGOUT_A_R {
        DMA_TRIGOUT_A_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA_TRIGOUT_B"]
    #[inline(always)]
    pub fn dma_trigout_b(&self) -> DMA_TRIGOUT_B_R {
        DMA_TRIGOUT_B_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA_TRIGOUT_C"]
    #[inline(always)]
    pub fn dma_trigout_c(&self) -> DMA_TRIGOUT_C_R {
        DMA_TRIGOUT_C_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA_TRIGOUT_D"]
    #[inline(always)]
    pub fn dma_trigout_d(&self) -> DMA_TRIGOUT_D_R {
        DMA_TRIGOUT_D_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCT_DMA_REQ0"]
    #[inline(always)]
    pub fn sct_dma_req0(&self) -> SCT_DMA_REQ0_R {
        SCT_DMA_REQ0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCT_DMA_REQ1"]
    #[inline(always)]
    pub fn sct_dma_req1(&self) -> SCT_DMA_REQ1_R {
        SCT_DMA_REQ1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - HASHCRYPT_OUT"]
    #[inline(always)]
    pub fn hashcrypt_out(&self) -> HASHCRYPT_OUT_R {
        HASHCRYPT_OUT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ACMP"]
    #[inline(always)]
    pub fn acmp(&self) -> ACMP_R {
        ACMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - FlexSPI0_RX"]
    #[inline(always)]
    pub fn flexspi0_rx(&self) -> FLEXSPI0_RX_R {
        FLEXSPI0_RX_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - FlexSPI0_TX"]
    #[inline(always)]
    pub fn flexspi0_tx(&self) -> FLEXSPI0_TX_R {
        FLEXSPI0_TX_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FlexSPI1_RX"]
    #[inline(always)]
    pub fn flexspi1_rx(&self) -> FLEXSPI1_RX_R {
        FLEXSPI1_RX_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FlexSPI1_TX"]
    #[inline(always)]
    pub fn flexspi1_tx(&self) -> FLEXSPI1_TX_R {
        FLEXSPI1_TX_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO_INT0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int0(&mut self) -> GPIO_INT0_W<0> {
        GPIO_INT0_W::new(self)
    }
    #[doc = "Bit 1 - GPIO_INT1"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int1(&mut self) -> GPIO_INT1_W<1> {
        GPIO_INT1_W::new(self)
    }
    #[doc = "Bit 2 - GPIO_INT2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int2(&mut self) -> GPIO_INT2_W<2> {
        GPIO_INT2_W::new(self)
    }
    #[doc = "Bit 3 - GPIO_INT3"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_int3(&mut self) -> GPIO_INT3_W<3> {
        GPIO_INT3_W::new(self)
    }
    #[doc = "Bit 4 - T0_DMAREQ_M0"]
    #[inline(always)]
    #[must_use]
    pub fn t0_dmareq_m0(&mut self) -> T0_DMAREQ_M0_W<4> {
        T0_DMAREQ_M0_W::new(self)
    }
    #[doc = "Bit 5 - T0_DMAREQ_M1"]
    #[inline(always)]
    #[must_use]
    pub fn t0_dmareq_m1(&mut self) -> T0_DMAREQ_M1_W<5> {
        T0_DMAREQ_M1_W::new(self)
    }
    #[doc = "Bit 6 - T1_DMAREQ_M0"]
    #[inline(always)]
    #[must_use]
    pub fn t1_dmareq_m0(&mut self) -> T1_DMAREQ_M0_W<6> {
        T1_DMAREQ_M0_W::new(self)
    }
    #[doc = "Bit 7 - T1_DMAREQ_M1"]
    #[inline(always)]
    #[must_use]
    pub fn t1_dmareq_m1(&mut self) -> T1_DMAREQ_M1_W<7> {
        T1_DMAREQ_M1_W::new(self)
    }
    #[doc = "Bit 8 - T2_DMAREQ_M0"]
    #[inline(always)]
    #[must_use]
    pub fn t2_dmareq_m0(&mut self) -> T2_DMAREQ_M0_W<8> {
        T2_DMAREQ_M0_W::new(self)
    }
    #[doc = "Bit 9 - T2_DMAREQ_M1"]
    #[inline(always)]
    #[must_use]
    pub fn t2_dmareq_m1(&mut self) -> T2_DMAREQ_M1_W<9> {
        T2_DMAREQ_M1_W::new(self)
    }
    #[doc = "Bit 10 - T3_DMAREQ_M0"]
    #[inline(always)]
    #[must_use]
    pub fn t3_dmareq_m0(&mut self) -> T3_DMAREQ_M0_W<10> {
        T3_DMAREQ_M0_W::new(self)
    }
    #[doc = "Bit 11 - T3_DMAREQ_M1"]
    #[inline(always)]
    #[must_use]
    pub fn t3_dmareq_m1(&mut self) -> T3_DMAREQ_M1_W<11> {
        T3_DMAREQ_M1_W::new(self)
    }
    #[doc = "Bit 12 - T4_DMAREQ_M0"]
    #[inline(always)]
    #[must_use]
    pub fn t4_dmareq_m0(&mut self) -> T4_DMAREQ_M0_W<12> {
        T4_DMAREQ_M0_W::new(self)
    }
    #[doc = "Bit 13 - T4_DMAREQ_M1"]
    #[inline(always)]
    #[must_use]
    pub fn t4_dmareq_m1(&mut self) -> T4_DMAREQ_M1_W<13> {
        T4_DMAREQ_M1_W::new(self)
    }
    #[doc = "Bit 14 - DMA_TRIGOUT_A"]
    #[inline(always)]
    #[must_use]
    pub fn dma_trigout_a(&mut self) -> DMA_TRIGOUT_A_W<14> {
        DMA_TRIGOUT_A_W::new(self)
    }
    #[doc = "Bit 15 - DMA_TRIGOUT_B"]
    #[inline(always)]
    #[must_use]
    pub fn dma_trigout_b(&mut self) -> DMA_TRIGOUT_B_W<15> {
        DMA_TRIGOUT_B_W::new(self)
    }
    #[doc = "Bit 16 - DMA_TRIGOUT_C"]
    #[inline(always)]
    #[must_use]
    pub fn dma_trigout_c(&mut self) -> DMA_TRIGOUT_C_W<16> {
        DMA_TRIGOUT_C_W::new(self)
    }
    #[doc = "Bit 17 - DMA_TRIGOUT_D"]
    #[inline(always)]
    #[must_use]
    pub fn dma_trigout_d(&mut self) -> DMA_TRIGOUT_D_W<17> {
        DMA_TRIGOUT_D_W::new(self)
    }
    #[doc = "Bit 18 - SCT_DMA_REQ0"]
    #[inline(always)]
    #[must_use]
    pub fn sct_dma_req0(&mut self) -> SCT_DMA_REQ0_W<18> {
        SCT_DMA_REQ0_W::new(self)
    }
    #[doc = "Bit 19 - SCT_DMA_REQ1"]
    #[inline(always)]
    #[must_use]
    pub fn sct_dma_req1(&mut self) -> SCT_DMA_REQ1_W<19> {
        SCT_DMA_REQ1_W::new(self)
    }
    #[doc = "Bit 20 - HASHCRYPT_OUT"]
    #[inline(always)]
    #[must_use]
    pub fn hashcrypt_out(&mut self) -> HASHCRYPT_OUT_W<20> {
        HASHCRYPT_OUT_W::new(self)
    }
    #[doc = "Bit 21 - ACMP"]
    #[inline(always)]
    #[must_use]
    pub fn acmp(&mut self) -> ACMP_W<21> {
        ACMP_W::new(self)
    }
    #[doc = "Bit 22 - FlexSPI0_RX"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi0_rx(&mut self) -> FLEXSPI0_RX_W<22> {
        FLEXSPI0_RX_W::new(self)
    }
    #[doc = "Bit 23 - FlexSPI0_TX"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi0_tx(&mut self) -> FLEXSPI0_TX_W<23> {
        FLEXSPI0_TX_W::new(self)
    }
    #[doc = "Bit 24 - ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<24> {
        ADC_W::new(self)
    }
    #[doc = "Bit 25 - FlexSPI1_RX"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi1_rx(&mut self) -> FLEXSPI1_RX_W<25> {
        FLEXSPI1_RX_W::new(self)
    }
    #[doc = "Bit 26 - FlexSPI1_TX"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi1_tx(&mut self) -> FLEXSPI1_TX_W<26> {
        FLEXSPI1_TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC0 Input Trigger Enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac0_itrig_ena0](index.html) module"]
pub struct DMAC0_ITRIG_ENA0_SPEC;
impl crate::RegisterSpec for DMAC0_ITRIG_ENA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac0_itrig_ena0::R](R) reader structure"]
impl crate::Readable for DMAC0_ITRIG_ENA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac0_itrig_ena0::W](W) writer structure"]
impl crate::Writable for DMAC0_ITRIG_ENA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAC0_ITRIG_ENA0 to value 0xffff_ffff"]
impl crate::Resettable for DMAC0_ITRIG_ENA0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
