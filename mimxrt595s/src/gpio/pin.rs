#[doc = "Register `PIN[%s]` reader"]
pub struct R(crate::R<PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN[%s]` writer"]
pub struct W(crate::W<PIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_SPEC>;
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
impl From<crate::W<PIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORT0` reader - Port pins"]
pub type PORT0_R = crate::BitReader<PORT0_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT0_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT0_A> for bool {
    #[inline(always)]
    fn from(variant: PORT0_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT0_A {
        match self.bits {
            false => PORT0_A::PORT_0,
            true => PORT0_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT0_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT0_A::PORT_1
    }
}
#[doc = "Field `PORT0` writer - Port pins"]
pub type PORT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT0_A, O>;
impl<'a, const O: u8> PORT0_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT0_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT0_A::PORT_1)
    }
}
#[doc = "Field `PORT1` reader - Port pins"]
pub type PORT1_R = crate::BitReader<PORT1_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT1_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT1_A> for bool {
    #[inline(always)]
    fn from(variant: PORT1_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT1_A {
        match self.bits {
            false => PORT1_A::PORT_0,
            true => PORT1_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT1_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT1_A::PORT_1
    }
}
#[doc = "Field `PORT1` writer - Port pins"]
pub type PORT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT1_A, O>;
impl<'a, const O: u8> PORT1_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT1_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT1_A::PORT_1)
    }
}
#[doc = "Field `PORT2` reader - Port pins"]
pub type PORT2_R = crate::BitReader<PORT2_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT2_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT2_A> for bool {
    #[inline(always)]
    fn from(variant: PORT2_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT2_A {
        match self.bits {
            false => PORT2_A::PORT_0,
            true => PORT2_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT2_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT2_A::PORT_1
    }
}
#[doc = "Field `PORT2` writer - Port pins"]
pub type PORT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT2_A, O>;
impl<'a, const O: u8> PORT2_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT2_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT2_A::PORT_1)
    }
}
#[doc = "Field `PORT3` reader - Port pins"]
pub type PORT3_R = crate::BitReader<PORT3_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT3_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT3_A> for bool {
    #[inline(always)]
    fn from(variant: PORT3_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT3_A {
        match self.bits {
            false => PORT3_A::PORT_0,
            true => PORT3_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT3_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT3_A::PORT_1
    }
}
#[doc = "Field `PORT3` writer - Port pins"]
pub type PORT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT3_A, O>;
impl<'a, const O: u8> PORT3_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT3_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT3_A::PORT_1)
    }
}
#[doc = "Field `PORT4` reader - Port pins"]
pub type PORT4_R = crate::BitReader<PORT4_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT4_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT4_A> for bool {
    #[inline(always)]
    fn from(variant: PORT4_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT4_A {
        match self.bits {
            false => PORT4_A::PORT_0,
            true => PORT4_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT4_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT4_A::PORT_1
    }
}
#[doc = "Field `PORT4` writer - Port pins"]
pub type PORT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT4_A, O>;
impl<'a, const O: u8> PORT4_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT4_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT4_A::PORT_1)
    }
}
#[doc = "Field `PORT5` reader - Port pins"]
pub type PORT5_R = crate::BitReader<PORT5_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT5_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT5_A> for bool {
    #[inline(always)]
    fn from(variant: PORT5_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT5_A {
        match self.bits {
            false => PORT5_A::PORT_0,
            true => PORT5_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT5_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT5_A::PORT_1
    }
}
#[doc = "Field `PORT5` writer - Port pins"]
pub type PORT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT5_A, O>;
impl<'a, const O: u8> PORT5_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT5_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT5_A::PORT_1)
    }
}
#[doc = "Field `PORT6` reader - Port pins"]
pub type PORT6_R = crate::BitReader<PORT6_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT6_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT6_A> for bool {
    #[inline(always)]
    fn from(variant: PORT6_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT6_A {
        match self.bits {
            false => PORT6_A::PORT_0,
            true => PORT6_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT6_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT6_A::PORT_1
    }
}
#[doc = "Field `PORT6` writer - Port pins"]
pub type PORT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT6_A, O>;
impl<'a, const O: u8> PORT6_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT6_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT6_A::PORT_1)
    }
}
#[doc = "Field `PORT7` reader - Port pins"]
pub type PORT7_R = crate::BitReader<PORT7_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT7_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT7_A> for bool {
    #[inline(always)]
    fn from(variant: PORT7_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT7_A {
        match self.bits {
            false => PORT7_A::PORT_0,
            true => PORT7_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT7_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT7_A::PORT_1
    }
}
#[doc = "Field `PORT7` writer - Port pins"]
pub type PORT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT7_A, O>;
impl<'a, const O: u8> PORT7_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT7_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT7_A::PORT_1)
    }
}
#[doc = "Field `PORT8` reader - Port pins"]
pub type PORT8_R = crate::BitReader<PORT8_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT8_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT8_A> for bool {
    #[inline(always)]
    fn from(variant: PORT8_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT8_A {
        match self.bits {
            false => PORT8_A::PORT_0,
            true => PORT8_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT8_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT8_A::PORT_1
    }
}
#[doc = "Field `PORT8` writer - Port pins"]
pub type PORT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT8_A, O>;
impl<'a, const O: u8> PORT8_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT8_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT8_A::PORT_1)
    }
}
#[doc = "Field `PORT9` reader - Port pins"]
pub type PORT9_R = crate::BitReader<PORT9_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT9_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT9_A> for bool {
    #[inline(always)]
    fn from(variant: PORT9_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT9_A {
        match self.bits {
            false => PORT9_A::PORT_0,
            true => PORT9_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT9_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT9_A::PORT_1
    }
}
#[doc = "Field `PORT9` writer - Port pins"]
pub type PORT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT9_A, O>;
impl<'a, const O: u8> PORT9_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT9_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT9_A::PORT_1)
    }
}
#[doc = "Field `PORT10` reader - Port pins"]
pub type PORT10_R = crate::BitReader<PORT10_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT10_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT10_A> for bool {
    #[inline(always)]
    fn from(variant: PORT10_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT10_A {
        match self.bits {
            false => PORT10_A::PORT_0,
            true => PORT10_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT10_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT10_A::PORT_1
    }
}
#[doc = "Field `PORT10` writer - Port pins"]
pub type PORT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT10_A, O>;
impl<'a, const O: u8> PORT10_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT10_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT10_A::PORT_1)
    }
}
#[doc = "Field `PORT11` reader - Port pins"]
pub type PORT11_R = crate::BitReader<PORT11_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT11_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT11_A> for bool {
    #[inline(always)]
    fn from(variant: PORT11_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT11_A {
        match self.bits {
            false => PORT11_A::PORT_0,
            true => PORT11_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT11_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT11_A::PORT_1
    }
}
#[doc = "Field `PORT11` writer - Port pins"]
pub type PORT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT11_A, O>;
impl<'a, const O: u8> PORT11_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT11_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT11_A::PORT_1)
    }
}
#[doc = "Field `PORT12` reader - Port pins"]
pub type PORT12_R = crate::BitReader<PORT12_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT12_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT12_A> for bool {
    #[inline(always)]
    fn from(variant: PORT12_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT12_A {
        match self.bits {
            false => PORT12_A::PORT_0,
            true => PORT12_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT12_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT12_A::PORT_1
    }
}
#[doc = "Field `PORT12` writer - Port pins"]
pub type PORT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT12_A, O>;
impl<'a, const O: u8> PORT12_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT12_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT12_A::PORT_1)
    }
}
#[doc = "Field `PORT13` reader - Port pins"]
pub type PORT13_R = crate::BitReader<PORT13_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT13_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT13_A> for bool {
    #[inline(always)]
    fn from(variant: PORT13_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT13_A {
        match self.bits {
            false => PORT13_A::PORT_0,
            true => PORT13_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT13_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT13_A::PORT_1
    }
}
#[doc = "Field `PORT13` writer - Port pins"]
pub type PORT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT13_A, O>;
impl<'a, const O: u8> PORT13_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT13_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT13_A::PORT_1)
    }
}
#[doc = "Field `PORT14` reader - Port pins"]
pub type PORT14_R = crate::BitReader<PORT14_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT14_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT14_A> for bool {
    #[inline(always)]
    fn from(variant: PORT14_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT14_A {
        match self.bits {
            false => PORT14_A::PORT_0,
            true => PORT14_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT14_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT14_A::PORT_1
    }
}
#[doc = "Field `PORT14` writer - Port pins"]
pub type PORT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT14_A, O>;
impl<'a, const O: u8> PORT14_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT14_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT14_A::PORT_1)
    }
}
#[doc = "Field `PORT15` reader - Port pins"]
pub type PORT15_R = crate::BitReader<PORT15_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT15_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT15_A> for bool {
    #[inline(always)]
    fn from(variant: PORT15_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT15_A {
        match self.bits {
            false => PORT15_A::PORT_0,
            true => PORT15_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT15_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT15_A::PORT_1
    }
}
#[doc = "Field `PORT15` writer - Port pins"]
pub type PORT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT15_A, O>;
impl<'a, const O: u8> PORT15_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT15_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT15_A::PORT_1)
    }
}
#[doc = "Field `PORT16` reader - Port pins"]
pub type PORT16_R = crate::BitReader<PORT16_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT16_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT16_A> for bool {
    #[inline(always)]
    fn from(variant: PORT16_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT16_A {
        match self.bits {
            false => PORT16_A::PORT_0,
            true => PORT16_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT16_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT16_A::PORT_1
    }
}
#[doc = "Field `PORT16` writer - Port pins"]
pub type PORT16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT16_A, O>;
impl<'a, const O: u8> PORT16_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT16_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT16_A::PORT_1)
    }
}
#[doc = "Field `PORT17` reader - Port pins"]
pub type PORT17_R = crate::BitReader<PORT17_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT17_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT17_A> for bool {
    #[inline(always)]
    fn from(variant: PORT17_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT17_A {
        match self.bits {
            false => PORT17_A::PORT_0,
            true => PORT17_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT17_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT17_A::PORT_1
    }
}
#[doc = "Field `PORT17` writer - Port pins"]
pub type PORT17_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT17_A, O>;
impl<'a, const O: u8> PORT17_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT17_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT17_A::PORT_1)
    }
}
#[doc = "Field `PORT18` reader - Port pins"]
pub type PORT18_R = crate::BitReader<PORT18_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT18_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT18_A> for bool {
    #[inline(always)]
    fn from(variant: PORT18_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT18_A {
        match self.bits {
            false => PORT18_A::PORT_0,
            true => PORT18_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT18_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT18_A::PORT_1
    }
}
#[doc = "Field `PORT18` writer - Port pins"]
pub type PORT18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT18_A, O>;
impl<'a, const O: u8> PORT18_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT18_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT18_A::PORT_1)
    }
}
#[doc = "Field `PORT19` reader - Port pins"]
pub type PORT19_R = crate::BitReader<PORT19_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT19_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT19_A> for bool {
    #[inline(always)]
    fn from(variant: PORT19_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT19_A {
        match self.bits {
            false => PORT19_A::PORT_0,
            true => PORT19_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT19_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT19_A::PORT_1
    }
}
#[doc = "Field `PORT19` writer - Port pins"]
pub type PORT19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT19_A, O>;
impl<'a, const O: u8> PORT19_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT19_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT19_A::PORT_1)
    }
}
#[doc = "Field `PORT20` reader - Port pins"]
pub type PORT20_R = crate::BitReader<PORT20_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT20_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT20_A> for bool {
    #[inline(always)]
    fn from(variant: PORT20_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT20_A {
        match self.bits {
            false => PORT20_A::PORT_0,
            true => PORT20_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT20_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT20_A::PORT_1
    }
}
#[doc = "Field `PORT20` writer - Port pins"]
pub type PORT20_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT20_A, O>;
impl<'a, const O: u8> PORT20_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT20_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT20_A::PORT_1)
    }
}
#[doc = "Field `PORT21` reader - Port pins"]
pub type PORT21_R = crate::BitReader<PORT21_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT21_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT21_A> for bool {
    #[inline(always)]
    fn from(variant: PORT21_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT21_A {
        match self.bits {
            false => PORT21_A::PORT_0,
            true => PORT21_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT21_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT21_A::PORT_1
    }
}
#[doc = "Field `PORT21` writer - Port pins"]
pub type PORT21_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT21_A, O>;
impl<'a, const O: u8> PORT21_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT21_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT21_A::PORT_1)
    }
}
#[doc = "Field `PORT22` reader - Port pins"]
pub type PORT22_R = crate::BitReader<PORT22_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT22_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT22_A> for bool {
    #[inline(always)]
    fn from(variant: PORT22_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT22_A {
        match self.bits {
            false => PORT22_A::PORT_0,
            true => PORT22_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT22_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT22_A::PORT_1
    }
}
#[doc = "Field `PORT22` writer - Port pins"]
pub type PORT22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT22_A, O>;
impl<'a, const O: u8> PORT22_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT22_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT22_A::PORT_1)
    }
}
#[doc = "Field `PORT23` reader - Port pins"]
pub type PORT23_R = crate::BitReader<PORT23_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT23_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT23_A> for bool {
    #[inline(always)]
    fn from(variant: PORT23_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT23_A {
        match self.bits {
            false => PORT23_A::PORT_0,
            true => PORT23_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT23_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT23_A::PORT_1
    }
}
#[doc = "Field `PORT23` writer - Port pins"]
pub type PORT23_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT23_A, O>;
impl<'a, const O: u8> PORT23_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT23_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT23_A::PORT_1)
    }
}
#[doc = "Field `PORT24` reader - Port pins"]
pub type PORT24_R = crate::BitReader<PORT24_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT24_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT24_A> for bool {
    #[inline(always)]
    fn from(variant: PORT24_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT24_A {
        match self.bits {
            false => PORT24_A::PORT_0,
            true => PORT24_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT24_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT24_A::PORT_1
    }
}
#[doc = "Field `PORT24` writer - Port pins"]
pub type PORT24_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT24_A, O>;
impl<'a, const O: u8> PORT24_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT24_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT24_A::PORT_1)
    }
}
#[doc = "Field `PORT25` reader - Port pins"]
pub type PORT25_R = crate::BitReader<PORT25_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT25_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT25_A> for bool {
    #[inline(always)]
    fn from(variant: PORT25_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT25_A {
        match self.bits {
            false => PORT25_A::PORT_0,
            true => PORT25_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT25_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT25_A::PORT_1
    }
}
#[doc = "Field `PORT25` writer - Port pins"]
pub type PORT25_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT25_A, O>;
impl<'a, const O: u8> PORT25_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT25_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT25_A::PORT_1)
    }
}
#[doc = "Field `PORT26` reader - Port pins"]
pub type PORT26_R = crate::BitReader<PORT26_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT26_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT26_A> for bool {
    #[inline(always)]
    fn from(variant: PORT26_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT26_A {
        match self.bits {
            false => PORT26_A::PORT_0,
            true => PORT26_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT26_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT26_A::PORT_1
    }
}
#[doc = "Field `PORT26` writer - Port pins"]
pub type PORT26_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT26_A, O>;
impl<'a, const O: u8> PORT26_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT26_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT26_A::PORT_1)
    }
}
#[doc = "Field `PORT27` reader - Port pins"]
pub type PORT27_R = crate::BitReader<PORT27_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT27_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT27_A> for bool {
    #[inline(always)]
    fn from(variant: PORT27_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT27_A {
        match self.bits {
            false => PORT27_A::PORT_0,
            true => PORT27_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT27_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT27_A::PORT_1
    }
}
#[doc = "Field `PORT27` writer - Port pins"]
pub type PORT27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT27_A, O>;
impl<'a, const O: u8> PORT27_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT27_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT27_A::PORT_1)
    }
}
#[doc = "Field `PORT28` reader - Port pins"]
pub type PORT28_R = crate::BitReader<PORT28_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT28_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT28_A> for bool {
    #[inline(always)]
    fn from(variant: PORT28_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT28_A {
        match self.bits {
            false => PORT28_A::PORT_0,
            true => PORT28_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT28_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT28_A::PORT_1
    }
}
#[doc = "Field `PORT28` writer - Port pins"]
pub type PORT28_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT28_A, O>;
impl<'a, const O: u8> PORT28_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT28_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT28_A::PORT_1)
    }
}
#[doc = "Field `PORT29` reader - Port pins"]
pub type PORT29_R = crate::BitReader<PORT29_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT29_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT29_A> for bool {
    #[inline(always)]
    fn from(variant: PORT29_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT29_A {
        match self.bits {
            false => PORT29_A::PORT_0,
            true => PORT29_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT29_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT29_A::PORT_1
    }
}
#[doc = "Field `PORT29` writer - Port pins"]
pub type PORT29_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT29_A, O>;
impl<'a, const O: u8> PORT29_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT29_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT29_A::PORT_1)
    }
}
#[doc = "Field `PORT30` reader - Port pins"]
pub type PORT30_R = crate::BitReader<PORT30_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT30_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT30_A> for bool {
    #[inline(always)]
    fn from(variant: PORT30_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT30_A {
        match self.bits {
            false => PORT30_A::PORT_0,
            true => PORT30_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT30_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT30_A::PORT_1
    }
}
#[doc = "Field `PORT30` writer - Port pins"]
pub type PORT30_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT30_A, O>;
impl<'a, const O: u8> PORT30_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT30_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT30_A::PORT_1)
    }
}
#[doc = "Field `PORT31` reader - Port pins"]
pub type PORT31_R = crate::BitReader<PORT31_A>;
#[doc = "Port pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORT31_A {
    #[doc = "0: Read- pin is low; Write- clear output bit"]
    PORT_0 = 0,
    #[doc = "1: Read- pin is high; Write- set output bit"]
    PORT_1 = 1,
}
impl From<PORT31_A> for bool {
    #[inline(always)]
    fn from(variant: PORT31_A) -> Self {
        variant as u8 != 0
    }
}
impl PORT31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT31_A {
        match self.bits {
            false => PORT31_A::PORT_0,
            true => PORT31_A::PORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_0`"]
    #[inline(always)]
    pub fn is_port_0(&self) -> bool {
        *self == PORT31_A::PORT_0
    }
    #[doc = "Checks if the value of the field is `PORT_1`"]
    #[inline(always)]
    pub fn is_port_1(&self) -> bool {
        *self == PORT31_A::PORT_1
    }
}
#[doc = "Field `PORT31` writer - Port pins"]
pub type PORT31_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, PORT31_A, O>;
impl<'a, const O: u8> PORT31_W<'a, O> {
    #[doc = "Read- pin is low; Write- clear output bit"]
    #[inline(always)]
    pub fn port_0(self) -> &'a mut W {
        self.variant(PORT31_A::PORT_0)
    }
    #[doc = "Read- pin is high; Write- set output bit"]
    #[inline(always)]
    pub fn port_1(self) -> &'a mut W {
        self.variant(PORT31_A::PORT_1)
    }
}
impl R {
    #[doc = "Bit 0 - Port pins"]
    #[inline(always)]
    pub fn port0(&self) -> PORT0_R {
        PORT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port pins"]
    #[inline(always)]
    pub fn port1(&self) -> PORT1_R {
        PORT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port pins"]
    #[inline(always)]
    pub fn port2(&self) -> PORT2_R {
        PORT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port pins"]
    #[inline(always)]
    pub fn port3(&self) -> PORT3_R {
        PORT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port pins"]
    #[inline(always)]
    pub fn port4(&self) -> PORT4_R {
        PORT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port pins"]
    #[inline(always)]
    pub fn port5(&self) -> PORT5_R {
        PORT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port pins"]
    #[inline(always)]
    pub fn port6(&self) -> PORT6_R {
        PORT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port pins"]
    #[inline(always)]
    pub fn port7(&self) -> PORT7_R {
        PORT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port pins"]
    #[inline(always)]
    pub fn port8(&self) -> PORT8_R {
        PORT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port pins"]
    #[inline(always)]
    pub fn port9(&self) -> PORT9_R {
        PORT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port pins"]
    #[inline(always)]
    pub fn port10(&self) -> PORT10_R {
        PORT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port pins"]
    #[inline(always)]
    pub fn port11(&self) -> PORT11_R {
        PORT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port pins"]
    #[inline(always)]
    pub fn port12(&self) -> PORT12_R {
        PORT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port pins"]
    #[inline(always)]
    pub fn port13(&self) -> PORT13_R {
        PORT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port pins"]
    #[inline(always)]
    pub fn port14(&self) -> PORT14_R {
        PORT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port pins"]
    #[inline(always)]
    pub fn port15(&self) -> PORT15_R {
        PORT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port pins"]
    #[inline(always)]
    pub fn port16(&self) -> PORT16_R {
        PORT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port pins"]
    #[inline(always)]
    pub fn port17(&self) -> PORT17_R {
        PORT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port pins"]
    #[inline(always)]
    pub fn port18(&self) -> PORT18_R {
        PORT18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port pins"]
    #[inline(always)]
    pub fn port19(&self) -> PORT19_R {
        PORT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port pins"]
    #[inline(always)]
    pub fn port20(&self) -> PORT20_R {
        PORT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port pins"]
    #[inline(always)]
    pub fn port21(&self) -> PORT21_R {
        PORT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port pins"]
    #[inline(always)]
    pub fn port22(&self) -> PORT22_R {
        PORT22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port pins"]
    #[inline(always)]
    pub fn port23(&self) -> PORT23_R {
        PORT23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port pins"]
    #[inline(always)]
    pub fn port24(&self) -> PORT24_R {
        PORT24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port pins"]
    #[inline(always)]
    pub fn port25(&self) -> PORT25_R {
        PORT25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port pins"]
    #[inline(always)]
    pub fn port26(&self) -> PORT26_R {
        PORT26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port pins"]
    #[inline(always)]
    pub fn port27(&self) -> PORT27_R {
        PORT27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port pins"]
    #[inline(always)]
    pub fn port28(&self) -> PORT28_R {
        PORT28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port pins"]
    #[inline(always)]
    pub fn port29(&self) -> PORT29_R {
        PORT29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port pins"]
    #[inline(always)]
    pub fn port30(&self) -> PORT30_R {
        PORT30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port pins"]
    #[inline(always)]
    pub fn port31(&self) -> PORT31_R {
        PORT31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port0(&mut self) -> PORT0_W<0> {
        PORT0_W::new(self)
    }
    #[doc = "Bit 1 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port1(&mut self) -> PORT1_W<1> {
        PORT1_W::new(self)
    }
    #[doc = "Bit 2 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port2(&mut self) -> PORT2_W<2> {
        PORT2_W::new(self)
    }
    #[doc = "Bit 3 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port3(&mut self) -> PORT3_W<3> {
        PORT3_W::new(self)
    }
    #[doc = "Bit 4 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port4(&mut self) -> PORT4_W<4> {
        PORT4_W::new(self)
    }
    #[doc = "Bit 5 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port5(&mut self) -> PORT5_W<5> {
        PORT5_W::new(self)
    }
    #[doc = "Bit 6 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port6(&mut self) -> PORT6_W<6> {
        PORT6_W::new(self)
    }
    #[doc = "Bit 7 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port7(&mut self) -> PORT7_W<7> {
        PORT7_W::new(self)
    }
    #[doc = "Bit 8 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port8(&mut self) -> PORT8_W<8> {
        PORT8_W::new(self)
    }
    #[doc = "Bit 9 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port9(&mut self) -> PORT9_W<9> {
        PORT9_W::new(self)
    }
    #[doc = "Bit 10 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port10(&mut self) -> PORT10_W<10> {
        PORT10_W::new(self)
    }
    #[doc = "Bit 11 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port11(&mut self) -> PORT11_W<11> {
        PORT11_W::new(self)
    }
    #[doc = "Bit 12 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port12(&mut self) -> PORT12_W<12> {
        PORT12_W::new(self)
    }
    #[doc = "Bit 13 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port13(&mut self) -> PORT13_W<13> {
        PORT13_W::new(self)
    }
    #[doc = "Bit 14 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port14(&mut self) -> PORT14_W<14> {
        PORT14_W::new(self)
    }
    #[doc = "Bit 15 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port15(&mut self) -> PORT15_W<15> {
        PORT15_W::new(self)
    }
    #[doc = "Bit 16 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port16(&mut self) -> PORT16_W<16> {
        PORT16_W::new(self)
    }
    #[doc = "Bit 17 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port17(&mut self) -> PORT17_W<17> {
        PORT17_W::new(self)
    }
    #[doc = "Bit 18 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port18(&mut self) -> PORT18_W<18> {
        PORT18_W::new(self)
    }
    #[doc = "Bit 19 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port19(&mut self) -> PORT19_W<19> {
        PORT19_W::new(self)
    }
    #[doc = "Bit 20 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port20(&mut self) -> PORT20_W<20> {
        PORT20_W::new(self)
    }
    #[doc = "Bit 21 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port21(&mut self) -> PORT21_W<21> {
        PORT21_W::new(self)
    }
    #[doc = "Bit 22 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port22(&mut self) -> PORT22_W<22> {
        PORT22_W::new(self)
    }
    #[doc = "Bit 23 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port23(&mut self) -> PORT23_W<23> {
        PORT23_W::new(self)
    }
    #[doc = "Bit 24 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port24(&mut self) -> PORT24_W<24> {
        PORT24_W::new(self)
    }
    #[doc = "Bit 25 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port25(&mut self) -> PORT25_W<25> {
        PORT25_W::new(self)
    }
    #[doc = "Bit 26 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port26(&mut self) -> PORT26_W<26> {
        PORT26_W::new(self)
    }
    #[doc = "Bit 27 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port27(&mut self) -> PORT27_W<27> {
        PORT27_W::new(self)
    }
    #[doc = "Bit 28 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port28(&mut self) -> PORT28_W<28> {
        PORT28_W::new(self)
    }
    #[doc = "Bit 29 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port29(&mut self) -> PORT29_W<29> {
        PORT29_W::new(self)
    }
    #[doc = "Bit 30 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port30(&mut self) -> PORT30_W<30> {
        PORT30_W::new(self)
    }
    #[doc = "Bit 31 - Port pins"]
    #[inline(always)]
    #[must_use]
    pub fn port31(&mut self) -> PORT31_W<31> {
        PORT31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port pin\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](index.html) module"]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin::R](R) reader structure"]
impl crate::Readable for PIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin::W](W) writer structure"]
impl crate::Writable for PIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIN[%s]
to value 0"]
impl crate::Resettable for PIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
