#[doc = "Register `PIO4_19` reader"]
pub struct R(crate::R<PIO4_19_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO4_19_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO4_19_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO4_19_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIO4_19` writer"]
pub struct W(crate::W<PIO4_19_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO4_19_SPEC>;
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
impl From<crate::W<PIO4_19_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO4_19_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSEL` reader - Function Selector (Digital Function)"]
pub type FSEL_R = crate::FieldReader<u8, FSEL_A>;
#[doc = "Function Selector (Digital Function)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSEL_A {
    #[doc = "0: Function 0"]
    FSEL_0 = 0,
    #[doc = "1: Function 1"]
    FSEL_1 = 1,
    #[doc = "2: Function 2"]
    FSEL_2 = 2,
    #[doc = "3: Function 3"]
    FSEL_3 = 3,
    #[doc = "4: Function 4"]
    FSEL_4 = 4,
    #[doc = "5: Function 5"]
    FSEL_5 = 5,
    #[doc = "6: Function 6"]
    FSEL_6 = 6,
    #[doc = "7: Function 7"]
    FSEL_7 = 7,
    #[doc = "8: Function 8"]
    FSEL_8 = 8,
    #[doc = "9: Function 9"]
    FSEL_9 = 9,
    #[doc = "10: Function 10"]
    FSEL_10 = 10,
    #[doc = "11: Function 11"]
    FSEL_11 = 11,
    #[doc = "12: Function 12"]
    FSEL_12 = 12,
    #[doc = "13: Function 13"]
    FSEL_13 = 13,
    #[doc = "14: Function 14"]
    FSEL_14 = 14,
    #[doc = "15: Function 15"]
    FSEL_15 = 15,
}
impl From<FSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL_A) -> Self {
        variant as _
    }
}
impl FSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL_A {
        match self.bits {
            0 => FSEL_A::FSEL_0,
            1 => FSEL_A::FSEL_1,
            2 => FSEL_A::FSEL_2,
            3 => FSEL_A::FSEL_3,
            4 => FSEL_A::FSEL_4,
            5 => FSEL_A::FSEL_5,
            6 => FSEL_A::FSEL_6,
            7 => FSEL_A::FSEL_7,
            8 => FSEL_A::FSEL_8,
            9 => FSEL_A::FSEL_9,
            10 => FSEL_A::FSEL_10,
            11 => FSEL_A::FSEL_11,
            12 => FSEL_A::FSEL_12,
            13 => FSEL_A::FSEL_13,
            14 => FSEL_A::FSEL_14,
            15 => FSEL_A::FSEL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FSEL_0`"]
    #[inline(always)]
    pub fn is_fsel_0(&self) -> bool {
        *self == FSEL_A::FSEL_0
    }
    #[doc = "Checks if the value of the field is `FSEL_1`"]
    #[inline(always)]
    pub fn is_fsel_1(&self) -> bool {
        *self == FSEL_A::FSEL_1
    }
    #[doc = "Checks if the value of the field is `FSEL_2`"]
    #[inline(always)]
    pub fn is_fsel_2(&self) -> bool {
        *self == FSEL_A::FSEL_2
    }
    #[doc = "Checks if the value of the field is `FSEL_3`"]
    #[inline(always)]
    pub fn is_fsel_3(&self) -> bool {
        *self == FSEL_A::FSEL_3
    }
    #[doc = "Checks if the value of the field is `FSEL_4`"]
    #[inline(always)]
    pub fn is_fsel_4(&self) -> bool {
        *self == FSEL_A::FSEL_4
    }
    #[doc = "Checks if the value of the field is `FSEL_5`"]
    #[inline(always)]
    pub fn is_fsel_5(&self) -> bool {
        *self == FSEL_A::FSEL_5
    }
    #[doc = "Checks if the value of the field is `FSEL_6`"]
    #[inline(always)]
    pub fn is_fsel_6(&self) -> bool {
        *self == FSEL_A::FSEL_6
    }
    #[doc = "Checks if the value of the field is `FSEL_7`"]
    #[inline(always)]
    pub fn is_fsel_7(&self) -> bool {
        *self == FSEL_A::FSEL_7
    }
    #[doc = "Checks if the value of the field is `FSEL_8`"]
    #[inline(always)]
    pub fn is_fsel_8(&self) -> bool {
        *self == FSEL_A::FSEL_8
    }
    #[doc = "Checks if the value of the field is `FSEL_9`"]
    #[inline(always)]
    pub fn is_fsel_9(&self) -> bool {
        *self == FSEL_A::FSEL_9
    }
    #[doc = "Checks if the value of the field is `FSEL_10`"]
    #[inline(always)]
    pub fn is_fsel_10(&self) -> bool {
        *self == FSEL_A::FSEL_10
    }
    #[doc = "Checks if the value of the field is `FSEL_11`"]
    #[inline(always)]
    pub fn is_fsel_11(&self) -> bool {
        *self == FSEL_A::FSEL_11
    }
    #[doc = "Checks if the value of the field is `FSEL_12`"]
    #[inline(always)]
    pub fn is_fsel_12(&self) -> bool {
        *self == FSEL_A::FSEL_12
    }
    #[doc = "Checks if the value of the field is `FSEL_13`"]
    #[inline(always)]
    pub fn is_fsel_13(&self) -> bool {
        *self == FSEL_A::FSEL_13
    }
    #[doc = "Checks if the value of the field is `FSEL_14`"]
    #[inline(always)]
    pub fn is_fsel_14(&self) -> bool {
        *self == FSEL_A::FSEL_14
    }
    #[doc = "Checks if the value of the field is `FSEL_15`"]
    #[inline(always)]
    pub fn is_fsel_15(&self) -> bool {
        *self == FSEL_A::FSEL_15
    }
}
#[doc = "Field `FSEL` writer - Function Selector (Digital Function)"]
pub type FSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PIO4_19_SPEC, u8, FSEL_A, 4, O>;
impl<'a, const O: u8> FSEL_W<'a, O> {
    #[doc = "Function 0"]
    #[inline(always)]
    pub fn fsel_0(self) -> &'a mut W {
        self.variant(FSEL_A::FSEL_0)
    }
    #[doc = "Function 1"]
    #[inline(always)]
    pub fn fsel_1(self) -> &'a mut W {
        self.variant(FSEL_A::FSEL_1)
    }
    #[doc = "Function 2"]
    #[inline(always)]
    pub fn fsel_2(self) -> &'a mut W {
        self.variant(FSEL_A::FSEL_2)
    }
    #[doc = "Function 3"]
    #[inline(always)]
    pub fn fsel_3(self) -> &'a mut W {
        self.variant(FSEL_A::FSEL_3)
    }
    #[doc = "Function 4"]
    #[inline(always)]
    pub fn fsel_4(self) -> &'a mut W {
        self.variant(FSEL_A::FSEL_4)
    }
    #[doc = "Function 5"]
    #[inline(always)]
    pub fn fsel_5(self) -> &'a mut W {
        self.variant(FSEL_A::FSEL_5)
    }
    #[doc = "Function 6"]
    #[inline(always)]
    pub fn fsel_6(self) -> &'a mut W {
        self.variant(FSEL_A::FSEL_6)
    }
    #[doc = "Function 7"]
    #[inline(always)]
    pub fn fsel_7(self) -> &'a mut W {
        self.variant(FSEL_A::FSEL_7)
    }
    #[doc = "Function 8"]
    #[inline(always)]
    pub fn fsel_8(self) -> &'a mut W {
        self.variant(FSEL_A::FSEL_8)
    }
    #[doc = "Function 9"]
    #[inline(always)]
    pub fn fsel_9(self) -> &'a mut W {
        self.variant(FSEL_A::FSEL_9)
    }
    #[doc = "Function 10"]
    #[inline(always)]
    pub fn fsel_10(self) -> &'a mut W {
        self.variant(FSEL_A::FSEL_10)
    }
    #[doc = "Function 11"]
    #[inline(always)]
    pub fn fsel_11(self) -> &'a mut W {
        self.variant(FSEL_A::FSEL_11)
    }
    #[doc = "Function 12"]
    #[inline(always)]
    pub fn fsel_12(self) -> &'a mut W {
        self.variant(FSEL_A::FSEL_12)
    }
    #[doc = "Function 13"]
    #[inline(always)]
    pub fn fsel_13(self) -> &'a mut W {
        self.variant(FSEL_A::FSEL_13)
    }
    #[doc = "Function 14"]
    #[inline(always)]
    pub fn fsel_14(self) -> &'a mut W {
        self.variant(FSEL_A::FSEL_14)
    }
    #[doc = "Function 15"]
    #[inline(always)]
    pub fn fsel_15(self) -> &'a mut W {
        self.variant(FSEL_A::FSEL_15)
    }
}
#[doc = "Field `PUPDENA` reader - Pullup / Pulldown Enable"]
pub type PUPDENA_R = crate::BitReader<PUPDENA_A>;
#[doc = "Pullup / Pulldown Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUPDENA_A {
    #[doc = "0: Disable"]
    PUPDENA_0 = 0,
    #[doc = "1: Enable"]
    PUPDENA_1 = 1,
}
impl From<PUPDENA_A> for bool {
    #[inline(always)]
    fn from(variant: PUPDENA_A) -> Self {
        variant as u8 != 0
    }
}
impl PUPDENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUPDENA_A {
        match self.bits {
            false => PUPDENA_A::PUPDENA_0,
            true => PUPDENA_A::PUPDENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `PUPDENA_0`"]
    #[inline(always)]
    pub fn is_pupdena_0(&self) -> bool {
        *self == PUPDENA_A::PUPDENA_0
    }
    #[doc = "Checks if the value of the field is `PUPDENA_1`"]
    #[inline(always)]
    pub fn is_pupdena_1(&self) -> bool {
        *self == PUPDENA_A::PUPDENA_1
    }
}
#[doc = "Field `PUPDENA` writer - Pullup / Pulldown Enable"]
pub type PUPDENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO4_19_SPEC, PUPDENA_A, O>;
impl<'a, const O: u8> PUPDENA_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn pupdena_0(self) -> &'a mut W {
        self.variant(PUPDENA_A::PUPDENA_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn pupdena_1(self) -> &'a mut W {
        self.variant(PUPDENA_A::PUPDENA_1)
    }
}
#[doc = "Field `PUPDSEL` reader - Pullup or Pulldown Selector"]
pub type PUPDSEL_R = crate::BitReader<PUPDSEL_A>;
#[doc = "Pullup or Pulldown Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUPDSEL_A {
    #[doc = "0: Pull-down"]
    PUPDSEL_0 = 0,
    #[doc = "1: Pull-up"]
    PUPDSEL_1 = 1,
}
impl From<PUPDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PUPDSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl PUPDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUPDSEL_A {
        match self.bits {
            false => PUPDSEL_A::PUPDSEL_0,
            true => PUPDSEL_A::PUPDSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PUPDSEL_0`"]
    #[inline(always)]
    pub fn is_pupdsel_0(&self) -> bool {
        *self == PUPDSEL_A::PUPDSEL_0
    }
    #[doc = "Checks if the value of the field is `PUPDSEL_1`"]
    #[inline(always)]
    pub fn is_pupdsel_1(&self) -> bool {
        *self == PUPDSEL_A::PUPDSEL_1
    }
}
#[doc = "Field `PUPDSEL` writer - Pullup or Pulldown Selector"]
pub type PUPDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO4_19_SPEC, PUPDSEL_A, O>;
impl<'a, const O: u8> PUPDSEL_W<'a, O> {
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pupdsel_0(self) -> &'a mut W {
        self.variant(PUPDSEL_A::PUPDSEL_0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pupdsel_1(self) -> &'a mut W {
        self.variant(PUPDSEL_A::PUPDSEL_1)
    }
}
#[doc = "Field `IBENA` reader - Input Buffer Enable"]
pub type IBENA_R = crate::BitReader<IBENA_A>;
#[doc = "Input Buffer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBENA_A {
    #[doc = "0: Input buffer disabled"]
    IBENA_0 = 0,
    #[doc = "1: Input buffer enabled"]
    IBENA_1 = 1,
}
impl From<IBENA_A> for bool {
    #[inline(always)]
    fn from(variant: IBENA_A) -> Self {
        variant as u8 != 0
    }
}
impl IBENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBENA_A {
        match self.bits {
            false => IBENA_A::IBENA_0,
            true => IBENA_A::IBENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `IBENA_0`"]
    #[inline(always)]
    pub fn is_ibena_0(&self) -> bool {
        *self == IBENA_A::IBENA_0
    }
    #[doc = "Checks if the value of the field is `IBENA_1`"]
    #[inline(always)]
    pub fn is_ibena_1(&self) -> bool {
        *self == IBENA_A::IBENA_1
    }
}
#[doc = "Field `IBENA` writer - Input Buffer Enable"]
pub type IBENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO4_19_SPEC, IBENA_A, O>;
impl<'a, const O: u8> IBENA_W<'a, O> {
    #[doc = "Input buffer disabled"]
    #[inline(always)]
    pub fn ibena_0(self) -> &'a mut W {
        self.variant(IBENA_A::IBENA_0)
    }
    #[doc = "Input buffer enabled"]
    #[inline(always)]
    pub fn ibena_1(self) -> &'a mut W {
        self.variant(IBENA_A::IBENA_1)
    }
}
#[doc = "Field `SLEWRATE` reader - Slew Rate Control"]
pub type SLEWRATE_R = crate::BitReader<SLEWRATE_A>;
#[doc = "Slew Rate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEWRATE_A {
    #[doc = "0: Standard mode, output slew rate is not controlled."]
    SLEWRATE_0 = 0,
    #[doc = "1: Slow mode, output slew rate control is enabled, limiting the output rate change and maximum toggle frequency. See device datasheet for details."]
    SLEWRATE_1 = 1,
}
impl From<SLEWRATE_A> for bool {
    #[inline(always)]
    fn from(variant: SLEWRATE_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEWRATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEWRATE_A {
        match self.bits {
            false => SLEWRATE_A::SLEWRATE_0,
            true => SLEWRATE_A::SLEWRATE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLEWRATE_0`"]
    #[inline(always)]
    pub fn is_slewrate_0(&self) -> bool {
        *self == SLEWRATE_A::SLEWRATE_0
    }
    #[doc = "Checks if the value of the field is `SLEWRATE_1`"]
    #[inline(always)]
    pub fn is_slewrate_1(&self) -> bool {
        *self == SLEWRATE_A::SLEWRATE_1
    }
}
#[doc = "Field `SLEWRATE` writer - Slew Rate Control"]
pub type SLEWRATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO4_19_SPEC, SLEWRATE_A, O>;
impl<'a, const O: u8> SLEWRATE_W<'a, O> {
    #[doc = "Standard mode, output slew rate is not controlled."]
    #[inline(always)]
    pub fn slewrate_0(self) -> &'a mut W {
        self.variant(SLEWRATE_A::SLEWRATE_0)
    }
    #[doc = "Slow mode, output slew rate control is enabled, limiting the output rate change and maximum toggle frequency. See device datasheet for details."]
    #[inline(always)]
    pub fn slewrate_1(self) -> &'a mut W {
        self.variant(SLEWRATE_A::SLEWRATE_1)
    }
}
#[doc = "Field `FULLDRIVE` reader - Drive Selector"]
pub type FULLDRIVE_R = crate::BitReader<FULLDRIVE_A>;
#[doc = "Drive Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FULLDRIVE_A {
    #[doc = "0: Normal output drive"]
    FULLDRIVE_0 = 0,
    #[doc = "1: Full output drive, twice the drive of normal mode."]
    FULLDRIVE_1 = 1,
}
impl From<FULLDRIVE_A> for bool {
    #[inline(always)]
    fn from(variant: FULLDRIVE_A) -> Self {
        variant as u8 != 0
    }
}
impl FULLDRIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FULLDRIVE_A {
        match self.bits {
            false => FULLDRIVE_A::FULLDRIVE_0,
            true => FULLDRIVE_A::FULLDRIVE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FULLDRIVE_0`"]
    #[inline(always)]
    pub fn is_fulldrive_0(&self) -> bool {
        *self == FULLDRIVE_A::FULLDRIVE_0
    }
    #[doc = "Checks if the value of the field is `FULLDRIVE_1`"]
    #[inline(always)]
    pub fn is_fulldrive_1(&self) -> bool {
        *self == FULLDRIVE_A::FULLDRIVE_1
    }
}
#[doc = "Field `FULLDRIVE` writer - Drive Selector"]
pub type FULLDRIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO4_19_SPEC, FULLDRIVE_A, O>;
impl<'a, const O: u8> FULLDRIVE_W<'a, O> {
    #[doc = "Normal output drive"]
    #[inline(always)]
    pub fn fulldrive_0(self) -> &'a mut W {
        self.variant(FULLDRIVE_A::FULLDRIVE_0)
    }
    #[doc = "Full output drive, twice the drive of normal mode."]
    #[inline(always)]
    pub fn fulldrive_1(self) -> &'a mut W {
        self.variant(FULLDRIVE_A::FULLDRIVE_1)
    }
}
#[doc = "Field `AMENA` reader - Analog Mux Enable"]
pub type AMENA_R = crate::BitReader<AMENA_A>;
#[doc = "Analog Mux Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMENA_A {
    #[doc = "0: Analog multiplexor disabled, required for digital pin function"]
    AMENA_0 = 0,
    #[doc = "1: Analog multiplexor enabled, required for analog pin function"]
    AMENA_1 = 1,
}
impl From<AMENA_A> for bool {
    #[inline(always)]
    fn from(variant: AMENA_A) -> Self {
        variant as u8 != 0
    }
}
impl AMENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMENA_A {
        match self.bits {
            false => AMENA_A::AMENA_0,
            true => AMENA_A::AMENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `AMENA_0`"]
    #[inline(always)]
    pub fn is_amena_0(&self) -> bool {
        *self == AMENA_A::AMENA_0
    }
    #[doc = "Checks if the value of the field is `AMENA_1`"]
    #[inline(always)]
    pub fn is_amena_1(&self) -> bool {
        *self == AMENA_A::AMENA_1
    }
}
#[doc = "Field `AMENA` writer - Analog Mux Enable"]
pub type AMENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO4_19_SPEC, AMENA_A, O>;
impl<'a, const O: u8> AMENA_W<'a, O> {
    #[doc = "Analog multiplexor disabled, required for digital pin function"]
    #[inline(always)]
    pub fn amena_0(self) -> &'a mut W {
        self.variant(AMENA_A::AMENA_0)
    }
    #[doc = "Analog multiplexor enabled, required for analog pin function"]
    #[inline(always)]
    pub fn amena_1(self) -> &'a mut W {
        self.variant(AMENA_A::AMENA_1)
    }
}
#[doc = "Field `ODENA` reader - Open-drain mode enable"]
pub type ODENA_R = crate::BitReader<ODENA_A>;
#[doc = "Open-drain mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODENA_A {
    #[doc = "0: Normal push-pull output"]
    ODENA_0 = 0,
    #[doc = "1: Pseudo open-drain output (high drive disabled)"]
    ODENA_1 = 1,
}
impl From<ODENA_A> for bool {
    #[inline(always)]
    fn from(variant: ODENA_A) -> Self {
        variant as u8 != 0
    }
}
impl ODENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODENA_A {
        match self.bits {
            false => ODENA_A::ODENA_0,
            true => ODENA_A::ODENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `ODENA_0`"]
    #[inline(always)]
    pub fn is_odena_0(&self) -> bool {
        *self == ODENA_A::ODENA_0
    }
    #[doc = "Checks if the value of the field is `ODENA_1`"]
    #[inline(always)]
    pub fn is_odena_1(&self) -> bool {
        *self == ODENA_A::ODENA_1
    }
}
#[doc = "Field `ODENA` writer - Open-drain mode enable"]
pub type ODENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO4_19_SPEC, ODENA_A, O>;
impl<'a, const O: u8> ODENA_W<'a, O> {
    #[doc = "Normal push-pull output"]
    #[inline(always)]
    pub fn odena_0(self) -> &'a mut W {
        self.variant(ODENA_A::ODENA_0)
    }
    #[doc = "Pseudo open-drain output (high drive disabled)"]
    #[inline(always)]
    pub fn odena_1(self) -> &'a mut W {
        self.variant(ODENA_A::ODENA_1)
    }
}
#[doc = "Field `IIENA` reader - Input Invert Enable"]
pub type IIENA_R = crate::BitReader<IIENA_A>;
#[doc = "Input Invert Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IIENA_A {
    #[doc = "0: Disabled, Input function is not inverted"]
    IIENA_0 = 0,
    #[doc = "1: Enabled, Input is function inverted"]
    IIENA_1 = 1,
}
impl From<IIENA_A> for bool {
    #[inline(always)]
    fn from(variant: IIENA_A) -> Self {
        variant as u8 != 0
    }
}
impl IIENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IIENA_A {
        match self.bits {
            false => IIENA_A::IIENA_0,
            true => IIENA_A::IIENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `IIENA_0`"]
    #[inline(always)]
    pub fn is_iiena_0(&self) -> bool {
        *self == IIENA_A::IIENA_0
    }
    #[doc = "Checks if the value of the field is `IIENA_1`"]
    #[inline(always)]
    pub fn is_iiena_1(&self) -> bool {
        *self == IIENA_A::IIENA_1
    }
}
#[doc = "Field `IIENA` writer - Input Invert Enable"]
pub type IIENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO4_19_SPEC, IIENA_A, O>;
impl<'a, const O: u8> IIENA_W<'a, O> {
    #[doc = "Disabled, Input function is not inverted"]
    #[inline(always)]
    pub fn iiena_0(self) -> &'a mut W {
        self.variant(IIENA_A::IIENA_0)
    }
    #[doc = "Enabled, Input is function inverted"]
    #[inline(always)]
    pub fn iiena_1(self) -> &'a mut W {
        self.variant(IIENA_A::IIENA_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function Selector (Digital Function)"]
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Pullup / Pulldown Enable"]
    #[inline(always)]
    pub fn pupdena(&self) -> PUPDENA_R {
        PUPDENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pullup or Pulldown Selector"]
    #[inline(always)]
    pub fn pupdsel(&self) -> PUPDSEL_R {
        PUPDSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input Buffer Enable"]
    #[inline(always)]
    pub fn ibena(&self) -> IBENA_R {
        IBENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Slew Rate Control"]
    #[inline(always)]
    pub fn slewrate(&self) -> SLEWRATE_R {
        SLEWRATE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Drive Selector"]
    #[inline(always)]
    pub fn fulldrive(&self) -> FULLDRIVE_R {
        FULLDRIVE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog Mux Enable"]
    #[inline(always)]
    pub fn amena(&self) -> AMENA_R {
        AMENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Open-drain mode enable"]
    #[inline(always)]
    pub fn odena(&self) -> ODENA_R {
        ODENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input Invert Enable"]
    #[inline(always)]
    pub fn iiena(&self) -> IIENA_R {
        IIENA_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function Selector (Digital Function)"]
    #[inline(always)]
    #[must_use]
    pub fn fsel(&mut self) -> FSEL_W<0> {
        FSEL_W::new(self)
    }
    #[doc = "Bit 4 - Pullup / Pulldown Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pupdena(&mut self) -> PUPDENA_W<4> {
        PUPDENA_W::new(self)
    }
    #[doc = "Bit 5 - Pullup or Pulldown Selector"]
    #[inline(always)]
    #[must_use]
    pub fn pupdsel(&mut self) -> PUPDSEL_W<5> {
        PUPDSEL_W::new(self)
    }
    #[doc = "Bit 6 - Input Buffer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ibena(&mut self) -> IBENA_W<6> {
        IBENA_W::new(self)
    }
    #[doc = "Bit 7 - Slew Rate Control"]
    #[inline(always)]
    #[must_use]
    pub fn slewrate(&mut self) -> SLEWRATE_W<7> {
        SLEWRATE_W::new(self)
    }
    #[doc = "Bit 8 - Drive Selector"]
    #[inline(always)]
    #[must_use]
    pub fn fulldrive(&mut self) -> FULLDRIVE_W<8> {
        FULLDRIVE_W::new(self)
    }
    #[doc = "Bit 9 - Analog Mux Enable"]
    #[inline(always)]
    #[must_use]
    pub fn amena(&mut self) -> AMENA_W<9> {
        AMENA_W::new(self)
    }
    #[doc = "Bit 10 - Open-drain mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn odena(&mut self) -> ODENA_W<10> {
        ODENA_W::new(self)
    }
    #[doc = "Bit 11 - Input Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iiena(&mut self) -> IIENA_W<11> {
        IIENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOPCTL configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio4_19](index.html) module"]
pub struct PIO4_19_SPEC;
impl crate::RegisterSpec for PIO4_19_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio4_19::R](R) reader structure"]
impl crate::Readable for PIO4_19_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio4_19::W](W) writer structure"]
impl crate::Writable for PIO4_19_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIO4_19 to value 0"]
impl crate::Resettable for PIO4_19_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
