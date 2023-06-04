#[doc = "Register `EVEN` reader"]
pub struct R(crate::R<EVEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVEN` writer"]
pub struct W(crate::W<EVEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVEN_SPEC>;
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
impl From<crate::W<EVEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEN0` reader - Event Interrupt Enable n"]
pub type IEN0_R = crate::BitReader<IEN0_A>;
#[doc = "Event Interrupt Enable n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN0_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<IEN0_A> for bool {
    #[inline(always)]
    fn from(variant: IEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN0_A {
        match self.bits {
            false => IEN0_A::DISABLE,
            true => IEN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IEN0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IEN0_A::ENABLE
    }
}
#[doc = "Field `IEN0` writer - Event Interrupt Enable n"]
pub type IEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVEN_SPEC, IEN0_A, O>;
impl<'a, const O: u8> IEN0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IEN0_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IEN0_A::ENABLE)
    }
}
#[doc = "Field `IEN1` reader - Event Interrupt Enable n"]
pub type IEN1_R = crate::BitReader<IEN1_A>;
#[doc = "Event Interrupt Enable n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN1_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<IEN1_A> for bool {
    #[inline(always)]
    fn from(variant: IEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN1_A {
        match self.bits {
            false => IEN1_A::DISABLE,
            true => IEN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IEN1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IEN1_A::ENABLE
    }
}
#[doc = "Field `IEN1` writer - Event Interrupt Enable n"]
pub type IEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVEN_SPEC, IEN1_A, O>;
impl<'a, const O: u8> IEN1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IEN1_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IEN1_A::ENABLE)
    }
}
#[doc = "Field `IEN2` reader - Event Interrupt Enable n"]
pub type IEN2_R = crate::BitReader<IEN2_A>;
#[doc = "Event Interrupt Enable n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN2_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<IEN2_A> for bool {
    #[inline(always)]
    fn from(variant: IEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN2_A {
        match self.bits {
            false => IEN2_A::DISABLE,
            true => IEN2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IEN2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IEN2_A::ENABLE
    }
}
#[doc = "Field `IEN2` writer - Event Interrupt Enable n"]
pub type IEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVEN_SPEC, IEN2_A, O>;
impl<'a, const O: u8> IEN2_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IEN2_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IEN2_A::ENABLE)
    }
}
#[doc = "Field `IEN3` reader - Event Interrupt Enable n"]
pub type IEN3_R = crate::BitReader<IEN3_A>;
#[doc = "Event Interrupt Enable n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN3_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<IEN3_A> for bool {
    #[inline(always)]
    fn from(variant: IEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN3_A {
        match self.bits {
            false => IEN3_A::DISABLE,
            true => IEN3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IEN3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IEN3_A::ENABLE
    }
}
#[doc = "Field `IEN3` writer - Event Interrupt Enable n"]
pub type IEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVEN_SPEC, IEN3_A, O>;
impl<'a, const O: u8> IEN3_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IEN3_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IEN3_A::ENABLE)
    }
}
#[doc = "Field `IEN4` reader - Event Interrupt Enable n"]
pub type IEN4_R = crate::BitReader<IEN4_A>;
#[doc = "Event Interrupt Enable n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN4_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<IEN4_A> for bool {
    #[inline(always)]
    fn from(variant: IEN4_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN4_A {
        match self.bits {
            false => IEN4_A::DISABLE,
            true => IEN4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IEN4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IEN4_A::ENABLE
    }
}
#[doc = "Field `IEN4` writer - Event Interrupt Enable n"]
pub type IEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVEN_SPEC, IEN4_A, O>;
impl<'a, const O: u8> IEN4_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IEN4_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IEN4_A::ENABLE)
    }
}
#[doc = "Field `IEN5` reader - Event Interrupt Enable n"]
pub type IEN5_R = crate::BitReader<IEN5_A>;
#[doc = "Event Interrupt Enable n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN5_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<IEN5_A> for bool {
    #[inline(always)]
    fn from(variant: IEN5_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN5_A {
        match self.bits {
            false => IEN5_A::DISABLE,
            true => IEN5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IEN5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IEN5_A::ENABLE
    }
}
#[doc = "Field `IEN5` writer - Event Interrupt Enable n"]
pub type IEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVEN_SPEC, IEN5_A, O>;
impl<'a, const O: u8> IEN5_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IEN5_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IEN5_A::ENABLE)
    }
}
#[doc = "Field `IEN6` reader - Event Interrupt Enable n"]
pub type IEN6_R = crate::BitReader<IEN6_A>;
#[doc = "Event Interrupt Enable n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN6_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<IEN6_A> for bool {
    #[inline(always)]
    fn from(variant: IEN6_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN6_A {
        match self.bits {
            false => IEN6_A::DISABLE,
            true => IEN6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IEN6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IEN6_A::ENABLE
    }
}
#[doc = "Field `IEN6` writer - Event Interrupt Enable n"]
pub type IEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVEN_SPEC, IEN6_A, O>;
impl<'a, const O: u8> IEN6_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IEN6_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IEN6_A::ENABLE)
    }
}
#[doc = "Field `IEN7` reader - Event Interrupt Enable n"]
pub type IEN7_R = crate::BitReader<IEN7_A>;
#[doc = "Event Interrupt Enable n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN7_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<IEN7_A> for bool {
    #[inline(always)]
    fn from(variant: IEN7_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN7_A {
        match self.bits {
            false => IEN7_A::DISABLE,
            true => IEN7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IEN7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IEN7_A::ENABLE
    }
}
#[doc = "Field `IEN7` writer - Event Interrupt Enable n"]
pub type IEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVEN_SPEC, IEN7_A, O>;
impl<'a, const O: u8> IEN7_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IEN7_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IEN7_A::ENABLE)
    }
}
#[doc = "Field `IEN8` reader - Event Interrupt Enable n"]
pub type IEN8_R = crate::BitReader<IEN8_A>;
#[doc = "Event Interrupt Enable n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN8_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<IEN8_A> for bool {
    #[inline(always)]
    fn from(variant: IEN8_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN8_A {
        match self.bits {
            false => IEN8_A::DISABLE,
            true => IEN8_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IEN8_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IEN8_A::ENABLE
    }
}
#[doc = "Field `IEN8` writer - Event Interrupt Enable n"]
pub type IEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVEN_SPEC, IEN8_A, O>;
impl<'a, const O: u8> IEN8_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IEN8_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IEN8_A::ENABLE)
    }
}
#[doc = "Field `IEN9` reader - Event Interrupt Enable n"]
pub type IEN9_R = crate::BitReader<IEN9_A>;
#[doc = "Event Interrupt Enable n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN9_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<IEN9_A> for bool {
    #[inline(always)]
    fn from(variant: IEN9_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN9_A {
        match self.bits {
            false => IEN9_A::DISABLE,
            true => IEN9_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IEN9_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IEN9_A::ENABLE
    }
}
#[doc = "Field `IEN9` writer - Event Interrupt Enable n"]
pub type IEN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVEN_SPEC, IEN9_A, O>;
impl<'a, const O: u8> IEN9_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IEN9_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IEN9_A::ENABLE)
    }
}
#[doc = "Field `IEN10` reader - Event Interrupt Enable n"]
pub type IEN10_R = crate::BitReader<IEN10_A>;
#[doc = "Event Interrupt Enable n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN10_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<IEN10_A> for bool {
    #[inline(always)]
    fn from(variant: IEN10_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN10_A {
        match self.bits {
            false => IEN10_A::DISABLE,
            true => IEN10_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IEN10_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IEN10_A::ENABLE
    }
}
#[doc = "Field `IEN10` writer - Event Interrupt Enable n"]
pub type IEN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVEN_SPEC, IEN10_A, O>;
impl<'a, const O: u8> IEN10_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IEN10_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IEN10_A::ENABLE)
    }
}
#[doc = "Field `IEN11` reader - Event Interrupt Enable n"]
pub type IEN11_R = crate::BitReader<IEN11_A>;
#[doc = "Event Interrupt Enable n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN11_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<IEN11_A> for bool {
    #[inline(always)]
    fn from(variant: IEN11_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN11_A {
        match self.bits {
            false => IEN11_A::DISABLE,
            true => IEN11_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IEN11_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IEN11_A::ENABLE
    }
}
#[doc = "Field `IEN11` writer - Event Interrupt Enable n"]
pub type IEN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVEN_SPEC, IEN11_A, O>;
impl<'a, const O: u8> IEN11_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IEN11_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IEN11_A::ENABLE)
    }
}
#[doc = "Field `IEN12` reader - Event Interrupt Enable n"]
pub type IEN12_R = crate::BitReader<IEN12_A>;
#[doc = "Event Interrupt Enable n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN12_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<IEN12_A> for bool {
    #[inline(always)]
    fn from(variant: IEN12_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN12_A {
        match self.bits {
            false => IEN12_A::DISABLE,
            true => IEN12_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IEN12_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IEN12_A::ENABLE
    }
}
#[doc = "Field `IEN12` writer - Event Interrupt Enable n"]
pub type IEN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVEN_SPEC, IEN12_A, O>;
impl<'a, const O: u8> IEN12_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IEN12_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IEN12_A::ENABLE)
    }
}
#[doc = "Field `IEN13` reader - Event Interrupt Enable n"]
pub type IEN13_R = crate::BitReader<IEN13_A>;
#[doc = "Event Interrupt Enable n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN13_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<IEN13_A> for bool {
    #[inline(always)]
    fn from(variant: IEN13_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN13_A {
        match self.bits {
            false => IEN13_A::DISABLE,
            true => IEN13_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IEN13_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IEN13_A::ENABLE
    }
}
#[doc = "Field `IEN13` writer - Event Interrupt Enable n"]
pub type IEN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVEN_SPEC, IEN13_A, O>;
impl<'a, const O: u8> IEN13_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IEN13_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IEN13_A::ENABLE)
    }
}
#[doc = "Field `IEN14` reader - Event Interrupt Enable n"]
pub type IEN14_R = crate::BitReader<IEN14_A>;
#[doc = "Event Interrupt Enable n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN14_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<IEN14_A> for bool {
    #[inline(always)]
    fn from(variant: IEN14_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN14_A {
        match self.bits {
            false => IEN14_A::DISABLE,
            true => IEN14_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IEN14_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IEN14_A::ENABLE
    }
}
#[doc = "Field `IEN14` writer - Event Interrupt Enable n"]
pub type IEN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVEN_SPEC, IEN14_A, O>;
impl<'a, const O: u8> IEN14_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IEN14_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IEN14_A::ENABLE)
    }
}
#[doc = "Field `IEN15` reader - Event Interrupt Enable n"]
pub type IEN15_R = crate::BitReader<IEN15_A>;
#[doc = "Event Interrupt Enable n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN15_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<IEN15_A> for bool {
    #[inline(always)]
    fn from(variant: IEN15_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN15_A {
        match self.bits {
            false => IEN15_A::DISABLE,
            true => IEN15_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IEN15_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IEN15_A::ENABLE
    }
}
#[doc = "Field `IEN15` writer - Event Interrupt Enable n"]
pub type IEN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVEN_SPEC, IEN15_A, O>;
impl<'a, const O: u8> IEN15_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IEN15_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IEN15_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Event Interrupt Enable n"]
    #[inline(always)]
    pub fn ien0(&self) -> IEN0_R {
        IEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event Interrupt Enable n"]
    #[inline(always)]
    pub fn ien1(&self) -> IEN1_R {
        IEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event Interrupt Enable n"]
    #[inline(always)]
    pub fn ien2(&self) -> IEN2_R {
        IEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event Interrupt Enable n"]
    #[inline(always)]
    pub fn ien3(&self) -> IEN3_R {
        IEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event Interrupt Enable n"]
    #[inline(always)]
    pub fn ien4(&self) -> IEN4_R {
        IEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event Interrupt Enable n"]
    #[inline(always)]
    pub fn ien5(&self) -> IEN5_R {
        IEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event Interrupt Enable n"]
    #[inline(always)]
    pub fn ien6(&self) -> IEN6_R {
        IEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event Interrupt Enable n"]
    #[inline(always)]
    pub fn ien7(&self) -> IEN7_R {
        IEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event Interrupt Enable n"]
    #[inline(always)]
    pub fn ien8(&self) -> IEN8_R {
        IEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event Interrupt Enable n"]
    #[inline(always)]
    pub fn ien9(&self) -> IEN9_R {
        IEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Event Interrupt Enable n"]
    #[inline(always)]
    pub fn ien10(&self) -> IEN10_R {
        IEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Event Interrupt Enable n"]
    #[inline(always)]
    pub fn ien11(&self) -> IEN11_R {
        IEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Event Interrupt Enable n"]
    #[inline(always)]
    pub fn ien12(&self) -> IEN12_R {
        IEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Event Interrupt Enable n"]
    #[inline(always)]
    pub fn ien13(&self) -> IEN13_R {
        IEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event Interrupt Enable n"]
    #[inline(always)]
    pub fn ien14(&self) -> IEN14_R {
        IEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Event Interrupt Enable n"]
    #[inline(always)]
    pub fn ien15(&self) -> IEN15_R {
        IEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Interrupt Enable n"]
    #[inline(always)]
    #[must_use]
    pub fn ien0(&mut self) -> IEN0_W<0> {
        IEN0_W::new(self)
    }
    #[doc = "Bit 1 - Event Interrupt Enable n"]
    #[inline(always)]
    #[must_use]
    pub fn ien1(&mut self) -> IEN1_W<1> {
        IEN1_W::new(self)
    }
    #[doc = "Bit 2 - Event Interrupt Enable n"]
    #[inline(always)]
    #[must_use]
    pub fn ien2(&mut self) -> IEN2_W<2> {
        IEN2_W::new(self)
    }
    #[doc = "Bit 3 - Event Interrupt Enable n"]
    #[inline(always)]
    #[must_use]
    pub fn ien3(&mut self) -> IEN3_W<3> {
        IEN3_W::new(self)
    }
    #[doc = "Bit 4 - Event Interrupt Enable n"]
    #[inline(always)]
    #[must_use]
    pub fn ien4(&mut self) -> IEN4_W<4> {
        IEN4_W::new(self)
    }
    #[doc = "Bit 5 - Event Interrupt Enable n"]
    #[inline(always)]
    #[must_use]
    pub fn ien5(&mut self) -> IEN5_W<5> {
        IEN5_W::new(self)
    }
    #[doc = "Bit 6 - Event Interrupt Enable n"]
    #[inline(always)]
    #[must_use]
    pub fn ien6(&mut self) -> IEN6_W<6> {
        IEN6_W::new(self)
    }
    #[doc = "Bit 7 - Event Interrupt Enable n"]
    #[inline(always)]
    #[must_use]
    pub fn ien7(&mut self) -> IEN7_W<7> {
        IEN7_W::new(self)
    }
    #[doc = "Bit 8 - Event Interrupt Enable n"]
    #[inline(always)]
    #[must_use]
    pub fn ien8(&mut self) -> IEN8_W<8> {
        IEN8_W::new(self)
    }
    #[doc = "Bit 9 - Event Interrupt Enable n"]
    #[inline(always)]
    #[must_use]
    pub fn ien9(&mut self) -> IEN9_W<9> {
        IEN9_W::new(self)
    }
    #[doc = "Bit 10 - Event Interrupt Enable n"]
    #[inline(always)]
    #[must_use]
    pub fn ien10(&mut self) -> IEN10_W<10> {
        IEN10_W::new(self)
    }
    #[doc = "Bit 11 - Event Interrupt Enable n"]
    #[inline(always)]
    #[must_use]
    pub fn ien11(&mut self) -> IEN11_W<11> {
        IEN11_W::new(self)
    }
    #[doc = "Bit 12 - Event Interrupt Enable n"]
    #[inline(always)]
    #[must_use]
    pub fn ien12(&mut self) -> IEN12_W<12> {
        IEN12_W::new(self)
    }
    #[doc = "Bit 13 - Event Interrupt Enable n"]
    #[inline(always)]
    #[must_use]
    pub fn ien13(&mut self) -> IEN13_W<13> {
        IEN13_W::new(self)
    }
    #[doc = "Bit 14 - Event Interrupt Enable n"]
    #[inline(always)]
    #[must_use]
    pub fn ien14(&mut self) -> IEN14_W<14> {
        IEN14_W::new(self)
    }
    #[doc = "Bit 15 - Event Interrupt Enable n"]
    #[inline(always)]
    #[must_use]
    pub fn ien15(&mut self) -> IEN15_W<15> {
        IEN15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [even](index.html) module"]
pub struct EVEN_SPEC;
impl crate::RegisterSpec for EVEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [even::R](R) reader structure"]
impl crate::Readable for EVEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [even::W](W) writer structure"]
impl crate::Writable for EVEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVEN to value 0"]
impl crate::Resettable for EVEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
