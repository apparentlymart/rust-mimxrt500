#[doc = "Register `INTENA0` reader"]
pub struct R(crate::R<INTENA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENA0` writer"]
pub struct W(crate::W<INTENA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENA0_SPEC>;
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
impl From<crate::W<INTENA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_EN0` reader - Interrupt A enable bits."]
pub type INT_EN0_R = crate::BitReader<INT_EN0_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN0_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN0_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN0_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN0_A {
        match self.bits {
            false => INT_EN0_A::INT_EN_0,
            true => INT_EN0_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN0_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN0_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN0` writer - Interrupt A enable bits."]
pub type INT_EN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN0_A, O>;
impl<'a, const O: u8> INT_EN0_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN0_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN0_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN1` reader - Interrupt A enable bits."]
pub type INT_EN1_R = crate::BitReader<INT_EN1_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN1_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN1_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN1_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN1_A {
        match self.bits {
            false => INT_EN1_A::INT_EN_0,
            true => INT_EN1_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN1_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN1_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN1` writer - Interrupt A enable bits."]
pub type INT_EN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN1_A, O>;
impl<'a, const O: u8> INT_EN1_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN1_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN1_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN2` reader - Interrupt A enable bits."]
pub type INT_EN2_R = crate::BitReader<INT_EN2_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN2_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN2_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN2_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN2_A {
        match self.bits {
            false => INT_EN2_A::INT_EN_0,
            true => INT_EN2_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN2_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN2_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN2` writer - Interrupt A enable bits."]
pub type INT_EN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN2_A, O>;
impl<'a, const O: u8> INT_EN2_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN2_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN2_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN3` reader - Interrupt A enable bits."]
pub type INT_EN3_R = crate::BitReader<INT_EN3_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN3_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN3_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN3_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN3_A {
        match self.bits {
            false => INT_EN3_A::INT_EN_0,
            true => INT_EN3_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN3_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN3_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN3` writer - Interrupt A enable bits."]
pub type INT_EN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN3_A, O>;
impl<'a, const O: u8> INT_EN3_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN3_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN3_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN4` reader - Interrupt A enable bits."]
pub type INT_EN4_R = crate::BitReader<INT_EN4_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN4_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN4_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN4_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN4_A {
        match self.bits {
            false => INT_EN4_A::INT_EN_0,
            true => INT_EN4_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN4_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN4_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN4` writer - Interrupt A enable bits."]
pub type INT_EN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN4_A, O>;
impl<'a, const O: u8> INT_EN4_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN4_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN4_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN5` reader - Interrupt A enable bits."]
pub type INT_EN5_R = crate::BitReader<INT_EN5_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN5_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN5_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN5_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN5_A {
        match self.bits {
            false => INT_EN5_A::INT_EN_0,
            true => INT_EN5_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN5_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN5_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN5` writer - Interrupt A enable bits."]
pub type INT_EN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN5_A, O>;
impl<'a, const O: u8> INT_EN5_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN5_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN5_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN6` reader - Interrupt A enable bits."]
pub type INT_EN6_R = crate::BitReader<INT_EN6_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN6_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN6_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN6_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN6_A {
        match self.bits {
            false => INT_EN6_A::INT_EN_0,
            true => INT_EN6_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN6_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN6_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN6` writer - Interrupt A enable bits."]
pub type INT_EN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN6_A, O>;
impl<'a, const O: u8> INT_EN6_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN6_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN6_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN7` reader - Interrupt A enable bits."]
pub type INT_EN7_R = crate::BitReader<INT_EN7_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN7_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN7_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN7_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN7_A {
        match self.bits {
            false => INT_EN7_A::INT_EN_0,
            true => INT_EN7_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN7_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN7_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN7` writer - Interrupt A enable bits."]
pub type INT_EN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN7_A, O>;
impl<'a, const O: u8> INT_EN7_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN7_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN7_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN8` reader - Interrupt A enable bits."]
pub type INT_EN8_R = crate::BitReader<INT_EN8_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN8_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN8_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN8_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN8_A {
        match self.bits {
            false => INT_EN8_A::INT_EN_0,
            true => INT_EN8_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN8_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN8_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN8` writer - Interrupt A enable bits."]
pub type INT_EN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN8_A, O>;
impl<'a, const O: u8> INT_EN8_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN8_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN8_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN9` reader - Interrupt A enable bits."]
pub type INT_EN9_R = crate::BitReader<INT_EN9_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN9_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN9_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN9_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN9_A {
        match self.bits {
            false => INT_EN9_A::INT_EN_0,
            true => INT_EN9_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN9_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN9_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN9` writer - Interrupt A enable bits."]
pub type INT_EN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN9_A, O>;
impl<'a, const O: u8> INT_EN9_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN9_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN9_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN10` reader - Interrupt A enable bits."]
pub type INT_EN10_R = crate::BitReader<INT_EN10_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN10_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN10_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN10_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN10_A {
        match self.bits {
            false => INT_EN10_A::INT_EN_0,
            true => INT_EN10_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN10_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN10_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN10` writer - Interrupt A enable bits."]
pub type INT_EN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN10_A, O>;
impl<'a, const O: u8> INT_EN10_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN10_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN10_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN11` reader - Interrupt A enable bits."]
pub type INT_EN11_R = crate::BitReader<INT_EN11_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN11_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN11_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN11_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN11_A {
        match self.bits {
            false => INT_EN11_A::INT_EN_0,
            true => INT_EN11_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN11_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN11_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN11` writer - Interrupt A enable bits."]
pub type INT_EN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN11_A, O>;
impl<'a, const O: u8> INT_EN11_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN11_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN11_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN12` reader - Interrupt A enable bits."]
pub type INT_EN12_R = crate::BitReader<INT_EN12_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN12_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN12_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN12_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN12_A {
        match self.bits {
            false => INT_EN12_A::INT_EN_0,
            true => INT_EN12_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN12_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN12_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN12` writer - Interrupt A enable bits."]
pub type INT_EN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN12_A, O>;
impl<'a, const O: u8> INT_EN12_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN12_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN12_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN13` reader - Interrupt A enable bits."]
pub type INT_EN13_R = crate::BitReader<INT_EN13_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN13_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN13_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN13_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN13_A {
        match self.bits {
            false => INT_EN13_A::INT_EN_0,
            true => INT_EN13_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN13_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN13_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN13` writer - Interrupt A enable bits."]
pub type INT_EN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN13_A, O>;
impl<'a, const O: u8> INT_EN13_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN13_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN13_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN14` reader - Interrupt A enable bits."]
pub type INT_EN14_R = crate::BitReader<INT_EN14_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN14_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN14_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN14_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN14_A {
        match self.bits {
            false => INT_EN14_A::INT_EN_0,
            true => INT_EN14_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN14_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN14_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN14` writer - Interrupt A enable bits."]
pub type INT_EN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN14_A, O>;
impl<'a, const O: u8> INT_EN14_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN14_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN14_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN15` reader - Interrupt A enable bits."]
pub type INT_EN15_R = crate::BitReader<INT_EN15_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN15_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN15_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN15_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN15_A {
        match self.bits {
            false => INT_EN15_A::INT_EN_0,
            true => INT_EN15_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN15_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN15_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN15` writer - Interrupt A enable bits."]
pub type INT_EN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN15_A, O>;
impl<'a, const O: u8> INT_EN15_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN15_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN15_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN16` reader - Interrupt A enable bits."]
pub type INT_EN16_R = crate::BitReader<INT_EN16_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN16_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN16_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN16_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN16_A {
        match self.bits {
            false => INT_EN16_A::INT_EN_0,
            true => INT_EN16_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN16_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN16_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN16` writer - Interrupt A enable bits."]
pub type INT_EN16_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN16_A, O>;
impl<'a, const O: u8> INT_EN16_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN16_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN16_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN17` reader - Interrupt A enable bits."]
pub type INT_EN17_R = crate::BitReader<INT_EN17_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN17_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN17_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN17_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN17_A {
        match self.bits {
            false => INT_EN17_A::INT_EN_0,
            true => INT_EN17_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN17_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN17_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN17` writer - Interrupt A enable bits."]
pub type INT_EN17_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN17_A, O>;
impl<'a, const O: u8> INT_EN17_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN17_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN17_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN18` reader - Interrupt A enable bits."]
pub type INT_EN18_R = crate::BitReader<INT_EN18_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN18_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN18_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN18_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN18_A {
        match self.bits {
            false => INT_EN18_A::INT_EN_0,
            true => INT_EN18_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN18_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN18_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN18` writer - Interrupt A enable bits."]
pub type INT_EN18_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN18_A, O>;
impl<'a, const O: u8> INT_EN18_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN18_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN18_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN19` reader - Interrupt A enable bits."]
pub type INT_EN19_R = crate::BitReader<INT_EN19_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN19_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN19_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN19_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN19_A {
        match self.bits {
            false => INT_EN19_A::INT_EN_0,
            true => INT_EN19_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN19_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN19_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN19` writer - Interrupt A enable bits."]
pub type INT_EN19_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN19_A, O>;
impl<'a, const O: u8> INT_EN19_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN19_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN19_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN20` reader - Interrupt A enable bits."]
pub type INT_EN20_R = crate::BitReader<INT_EN20_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN20_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN20_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN20_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN20_A {
        match self.bits {
            false => INT_EN20_A::INT_EN_0,
            true => INT_EN20_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN20_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN20_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN20` writer - Interrupt A enable bits."]
pub type INT_EN20_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN20_A, O>;
impl<'a, const O: u8> INT_EN20_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN20_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN20_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN21` reader - Interrupt A enable bits."]
pub type INT_EN21_R = crate::BitReader<INT_EN21_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN21_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN21_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN21_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN21_A {
        match self.bits {
            false => INT_EN21_A::INT_EN_0,
            true => INT_EN21_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN21_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN21_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN21` writer - Interrupt A enable bits."]
pub type INT_EN21_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN21_A, O>;
impl<'a, const O: u8> INT_EN21_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN21_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN21_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN22` reader - Interrupt A enable bits."]
pub type INT_EN22_R = crate::BitReader<INT_EN22_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN22_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN22_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN22_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN22_A {
        match self.bits {
            false => INT_EN22_A::INT_EN_0,
            true => INT_EN22_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN22_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN22_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN22` writer - Interrupt A enable bits."]
pub type INT_EN22_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN22_A, O>;
impl<'a, const O: u8> INT_EN22_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN22_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN22_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN23` reader - Interrupt A enable bits."]
pub type INT_EN23_R = crate::BitReader<INT_EN23_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN23_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN23_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN23_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN23_A {
        match self.bits {
            false => INT_EN23_A::INT_EN_0,
            true => INT_EN23_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN23_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN23_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN23` writer - Interrupt A enable bits."]
pub type INT_EN23_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN23_A, O>;
impl<'a, const O: u8> INT_EN23_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN23_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN23_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN24` reader - Interrupt A enable bits."]
pub type INT_EN24_R = crate::BitReader<INT_EN24_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN24_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN24_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN24_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN24_A {
        match self.bits {
            false => INT_EN24_A::INT_EN_0,
            true => INT_EN24_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN24_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN24_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN24` writer - Interrupt A enable bits."]
pub type INT_EN24_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN24_A, O>;
impl<'a, const O: u8> INT_EN24_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN24_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN24_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN25` reader - Interrupt A enable bits."]
pub type INT_EN25_R = crate::BitReader<INT_EN25_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN25_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN25_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN25_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN25_A {
        match self.bits {
            false => INT_EN25_A::INT_EN_0,
            true => INT_EN25_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN25_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN25_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN25` writer - Interrupt A enable bits."]
pub type INT_EN25_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN25_A, O>;
impl<'a, const O: u8> INT_EN25_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN25_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN25_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN26` reader - Interrupt A enable bits."]
pub type INT_EN26_R = crate::BitReader<INT_EN26_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN26_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN26_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN26_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN26_A {
        match self.bits {
            false => INT_EN26_A::INT_EN_0,
            true => INT_EN26_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN26_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN26_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN26` writer - Interrupt A enable bits."]
pub type INT_EN26_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN26_A, O>;
impl<'a, const O: u8> INT_EN26_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN26_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN26_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN27` reader - Interrupt A enable bits."]
pub type INT_EN27_R = crate::BitReader<INT_EN27_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN27_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN27_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN27_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN27_A {
        match self.bits {
            false => INT_EN27_A::INT_EN_0,
            true => INT_EN27_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN27_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN27_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN27` writer - Interrupt A enable bits."]
pub type INT_EN27_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN27_A, O>;
impl<'a, const O: u8> INT_EN27_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN27_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN27_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN28` reader - Interrupt A enable bits."]
pub type INT_EN28_R = crate::BitReader<INT_EN28_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN28_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN28_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN28_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN28_A {
        match self.bits {
            false => INT_EN28_A::INT_EN_0,
            true => INT_EN28_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN28_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN28_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN28` writer - Interrupt A enable bits."]
pub type INT_EN28_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN28_A, O>;
impl<'a, const O: u8> INT_EN28_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN28_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN28_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN29` reader - Interrupt A enable bits."]
pub type INT_EN29_R = crate::BitReader<INT_EN29_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN29_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN29_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN29_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN29_A {
        match self.bits {
            false => INT_EN29_A::INT_EN_0,
            true => INT_EN29_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN29_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN29_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN29` writer - Interrupt A enable bits."]
pub type INT_EN29_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN29_A, O>;
impl<'a, const O: u8> INT_EN29_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN29_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN29_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN30` reader - Interrupt A enable bits."]
pub type INT_EN30_R = crate::BitReader<INT_EN30_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN30_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN30_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN30_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN30_A {
        match self.bits {
            false => INT_EN30_A::INT_EN_0,
            true => INT_EN30_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN30_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN30_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN30` writer - Interrupt A enable bits."]
pub type INT_EN30_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN30_A, O>;
impl<'a, const O: u8> INT_EN30_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN30_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN30_A::INT_EN_1)
    }
}
#[doc = "Field `INT_EN31` reader - Interrupt A enable bits."]
pub type INT_EN31_R = crate::BitReader<INT_EN31_A>;
#[doc = "Interrupt A enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EN31_A {
    #[doc = "0: Pin does not contribute to GPIO interrupt A"]
    INT_EN_0 = 0,
    #[doc = "1: Pin contributes to GPIO interrupt A"]
    INT_EN_1 = 1,
}
impl From<INT_EN31_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EN31_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EN31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_EN31_A {
        match self.bits {
            false => INT_EN31_A::INT_EN_0,
            true => INT_EN31_A::INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_EN_0`"]
    #[inline(always)]
    pub fn is_int_en_0(&self) -> bool {
        *self == INT_EN31_A::INT_EN_0
    }
    #[doc = "Checks if the value of the field is `INT_EN_1`"]
    #[inline(always)]
    pub fn is_int_en_1(&self) -> bool {
        *self == INT_EN31_A::INT_EN_1
    }
}
#[doc = "Field `INT_EN31` writer - Interrupt A enable bits."]
pub type INT_EN31_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENA0_SPEC, INT_EN31_A, O>;
impl<'a, const O: u8> INT_EN31_W<'a, O> {
    #[doc = "Pin does not contribute to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_0(self) -> &'a mut W {
        self.variant(INT_EN31_A::INT_EN_0)
    }
    #[doc = "Pin contributes to GPIO interrupt A"]
    #[inline(always)]
    pub fn int_en_1(self) -> &'a mut W {
        self.variant(INT_EN31_A::INT_EN_1)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en0(&self) -> INT_EN0_R {
        INT_EN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en1(&self) -> INT_EN1_R {
        INT_EN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en2(&self) -> INT_EN2_R {
        INT_EN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en3(&self) -> INT_EN3_R {
        INT_EN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en4(&self) -> INT_EN4_R {
        INT_EN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en5(&self) -> INT_EN5_R {
        INT_EN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en6(&self) -> INT_EN6_R {
        INT_EN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en7(&self) -> INT_EN7_R {
        INT_EN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en8(&self) -> INT_EN8_R {
        INT_EN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en9(&self) -> INT_EN9_R {
        INT_EN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en10(&self) -> INT_EN10_R {
        INT_EN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en11(&self) -> INT_EN11_R {
        INT_EN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en12(&self) -> INT_EN12_R {
        INT_EN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en13(&self) -> INT_EN13_R {
        INT_EN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en14(&self) -> INT_EN14_R {
        INT_EN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en15(&self) -> INT_EN15_R {
        INT_EN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en16(&self) -> INT_EN16_R {
        INT_EN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en17(&self) -> INT_EN17_R {
        INT_EN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en18(&self) -> INT_EN18_R {
        INT_EN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en19(&self) -> INT_EN19_R {
        INT_EN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en20(&self) -> INT_EN20_R {
        INT_EN20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en21(&self) -> INT_EN21_R {
        INT_EN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en22(&self) -> INT_EN22_R {
        INT_EN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en23(&self) -> INT_EN23_R {
        INT_EN23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en24(&self) -> INT_EN24_R {
        INT_EN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en25(&self) -> INT_EN25_R {
        INT_EN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en26(&self) -> INT_EN26_R {
        INT_EN26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en27(&self) -> INT_EN27_R {
        INT_EN27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en28(&self) -> INT_EN28_R {
        INT_EN28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en29(&self) -> INT_EN29_R {
        INT_EN29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en30(&self) -> INT_EN30_R {
        INT_EN30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt A enable bits."]
    #[inline(always)]
    pub fn int_en31(&self) -> INT_EN31_R {
        INT_EN31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en0(&mut self) -> INT_EN0_W<0> {
        INT_EN0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en1(&mut self) -> INT_EN1_W<1> {
        INT_EN1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en2(&mut self) -> INT_EN2_W<2> {
        INT_EN2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en3(&mut self) -> INT_EN3_W<3> {
        INT_EN3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en4(&mut self) -> INT_EN4_W<4> {
        INT_EN4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en5(&mut self) -> INT_EN5_W<5> {
        INT_EN5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en6(&mut self) -> INT_EN6_W<6> {
        INT_EN6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en7(&mut self) -> INT_EN7_W<7> {
        INT_EN7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en8(&mut self) -> INT_EN8_W<8> {
        INT_EN8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en9(&mut self) -> INT_EN9_W<9> {
        INT_EN9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en10(&mut self) -> INT_EN10_W<10> {
        INT_EN10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en11(&mut self) -> INT_EN11_W<11> {
        INT_EN11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en12(&mut self) -> INT_EN12_W<12> {
        INT_EN12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en13(&mut self) -> INT_EN13_W<13> {
        INT_EN13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en14(&mut self) -> INT_EN14_W<14> {
        INT_EN14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en15(&mut self) -> INT_EN15_W<15> {
        INT_EN15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en16(&mut self) -> INT_EN16_W<16> {
        INT_EN16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en17(&mut self) -> INT_EN17_W<17> {
        INT_EN17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en18(&mut self) -> INT_EN18_W<18> {
        INT_EN18_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en19(&mut self) -> INT_EN19_W<19> {
        INT_EN19_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en20(&mut self) -> INT_EN20_W<20> {
        INT_EN20_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en21(&mut self) -> INT_EN21_W<21> {
        INT_EN21_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en22(&mut self) -> INT_EN22_W<22> {
        INT_EN22_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en23(&mut self) -> INT_EN23_W<23> {
        INT_EN23_W::new(self)
    }
    #[doc = "Bit 24 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en24(&mut self) -> INT_EN24_W<24> {
        INT_EN24_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en25(&mut self) -> INT_EN25_W<25> {
        INT_EN25_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en26(&mut self) -> INT_EN26_W<26> {
        INT_EN26_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en27(&mut self) -> INT_EN27_W<27> {
        INT_EN27_W::new(self)
    }
    #[doc = "Bit 28 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en28(&mut self) -> INT_EN28_W<28> {
        INT_EN28_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en29(&mut self) -> INT_EN29_W<29> {
        INT_EN29_W::new(self)
    }
    #[doc = "Bit 30 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en30(&mut self) -> INT_EN30_W<30> {
        INT_EN30_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt A enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn int_en31(&mut self) -> INT_EN31_W<31> {
        INT_EN31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt A enable control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intena0](index.html) module"]
pub struct INTENA0_SPEC;
impl crate::RegisterSpec for INTENA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intena0::R](R) reader structure"]
impl crate::Readable for INTENA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intena0::W](W) writer structure"]
impl crate::Writable for INTENA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENA0 to value 0"]
impl crate::Resettable for INTENA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
