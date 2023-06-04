#[doc = "Register `DMAC0_OTRIG_SEL[%s]` reader"]
pub struct R(crate::R<DMAC0_OTRIG_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC0_OTRIG_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC0_OTRIG_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC0_OTRIG_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC0_OTRIG_SEL[%s]` writer"]
pub struct W(crate::W<DMAC0_OTRIG_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC0_OTRIG_SEL_SPEC>;
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
impl From<crate::W<DMAC0_OTRIG_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC0_OTRIG_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAC0_OTRIG_SEL` reader - DMAC0 Output Triggers Select for A, B, C, D IE"]
pub type DMAC0_OTRIG_SEL_R = crate::FieldReader<u8, DMAC0_OTRIG_SEL_A>;
#[doc = "DMAC0 Output Triggers Select for A, B, C, D IE\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMAC0_OTRIG_SEL_A {
    #[doc = "0: DMAC0_OTRIG_CH0"]
    DMAC0_OTRIG_CH0 = 0,
    #[doc = "1: DMAC0_OTRIG_CH1"]
    DMAC0_OTRIG_CH1 = 1,
    #[doc = "2: DMAC0_OTRIG_CH2"]
    DMAC0_OTRIG_CH2 = 2,
    #[doc = "3: DMAC0_OTRIG_CH3"]
    DMAC0_OTRIG_CH3 = 3,
    #[doc = "4: DMAC0_OTRIG_CH4"]
    DMAC0_OTRIG_CH4 = 4,
    #[doc = "5: DMAC0_OTRIG_CH5"]
    DMAC0_OTRIG_CH5 = 5,
    #[doc = "6: DMAC0_OTRIG_CH6"]
    DMAC0_OTRIG_CH6 = 6,
    #[doc = "7: DMAC0_OTRIG_CH7"]
    DMAC0_OTRIG_CH7 = 7,
    #[doc = "8: DMAC0_OTRIG_CH8"]
    DMAC0_OTRIG_CH8 = 8,
    #[doc = "9: DMAC0_OTRIG_CH9"]
    DMAC0_OTRIG_CH9 = 9,
    #[doc = "10: DMAC0_OTRIG_CH10"]
    DMAC0_OTRIG_CH10 = 10,
    #[doc = "11: DMAC0_OTRIG_CH11"]
    DMAC0_OTRIG_CH11 = 11,
    #[doc = "12: DMAC0_OTRIG_CH12"]
    DMAC0_OTRIG_CH12 = 12,
    #[doc = "13: DMAC0_OTRIG_CH13"]
    DMAC0_OTRIG_CH13 = 13,
    #[doc = "14: DMAC0_OTRIG_CH14"]
    DMAC0_OTRIG_CH14 = 14,
    #[doc = "15: DMAC0_OTRIG_CH15"]
    DMAC0_OTRIG_CH15 = 15,
    #[doc = "16: DMAC0_OTRIG_CH16"]
    DMAC0_OTRIG_CH16 = 16,
    #[doc = "17: DMAC0_OTRIG_CH17"]
    DMAC0_OTRIG_CH17 = 17,
    #[doc = "18: DMAC0_OTRIG_CH18"]
    DMAC0_OTRIG_CH18 = 18,
    #[doc = "19: DMAC0_OTRIG_CH19"]
    DMAC0_OTRIG_CH19 = 19,
    #[doc = "20: DMAC0_OTRIG_CH20"]
    DMAC0_OTRIG_CH20 = 20,
    #[doc = "21: DMAC0_OTRIG_CH21"]
    DMAC0_OTRIG_CH21 = 21,
    #[doc = "22: DMAC0_OTRIG_CH22"]
    DMAC0_OTRIG_CH22 = 22,
    #[doc = "23: DMAC0_OTRIG_CH23"]
    DMAC0_OTRIG_CH23 = 23,
    #[doc = "24: DMAC0_OTRIG_CH24"]
    DMAC0_OTRIG_CH24 = 24,
    #[doc = "25: DMAC0_OTRIG_CH25"]
    DMAC0_OTRIG_CH25 = 25,
    #[doc = "26: DMAC0_OTRIG_CH26"]
    DMAC0_OTRIG_CH26 = 26,
    #[doc = "27: DMAC0_OTRIG_CH27"]
    DMAC0_OTRIG_CH27 = 27,
    #[doc = "28: DMAC0_OTRIG_CH28"]
    DMAC0_OTRIG_CH28 = 28,
    #[doc = "29: DMAC0_OTRIG_CH29"]
    DMAC0_OTRIG_CH29 = 29,
    #[doc = "30: DMAC0_OTRIG_CH30"]
    DMAC0_OTRIG_CH30 = 30,
    #[doc = "31: DMAC0_OTRIG_CH31"]
    DMAC0_OTRIG_CH31 = 31,
    #[doc = "32: DMAC0_OTRIG_CH32"]
    DMAC0_OTRIG_CH32 = 32,
    #[doc = "33: DMAC0_OTRIG_CH33"]
    DMAC0_OTRIG_CH33 = 33,
    #[doc = "34: DMAC0_OTRIG_CH34"]
    DMAC0_OTRIG_CH34 = 34,
    #[doc = "35: DMAC0_OTRIG_CH35"]
    DMAC0_OTRIG_CH35 = 35,
    #[doc = "36: DMAC0_OTRIG_CH36"]
    DMAC0_OTRIG_CH36 = 36,
}
impl From<DMAC0_OTRIG_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAC0_OTRIG_SEL_A) -> Self {
        variant as _
    }
}
impl DMAC0_OTRIG_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMAC0_OTRIG_SEL_A> {
        match self.bits {
            0 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH0),
            1 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH1),
            2 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH2),
            3 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH3),
            4 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH4),
            5 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH5),
            6 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH6),
            7 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH7),
            8 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH8),
            9 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH9),
            10 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH10),
            11 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH11),
            12 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH12),
            13 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH13),
            14 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH14),
            15 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH15),
            16 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH16),
            17 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH17),
            18 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH18),
            19 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH19),
            20 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH20),
            21 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH21),
            22 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH22),
            23 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH23),
            24 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH24),
            25 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH25),
            26 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH26),
            27 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH27),
            28 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH28),
            29 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH29),
            30 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH30),
            31 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH31),
            32 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH32),
            33 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH33),
            34 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH34),
            35 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH35),
            36 => Some(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH36),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH0`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch0(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH0
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH1`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch1(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH1
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH2`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch2(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH2
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH3`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch3(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH3
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH4`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch4(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH4
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH5`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch5(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH5
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH6`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch6(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH6
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH7`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch7(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH7
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH8`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch8(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH8
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH9`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch9(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH9
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH10`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch10(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH10
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH11`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch11(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH11
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH12`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch12(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH12
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH13`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch13(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH13
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH14`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch14(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH14
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH15`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch15(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH15
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH16`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch16(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH16
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH17`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch17(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH17
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH18`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch18(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH18
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH19`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch19(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH19
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH20`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch20(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH20
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH21`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch21(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH21
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH22`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch22(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH22
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH23`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch23(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH23
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH24`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch24(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH24
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH25`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch25(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH25
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH26`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch26(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH26
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH27`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch27(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH27
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH28`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch28(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH28
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH29`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch29(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH29
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH30`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch30(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH30
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH31`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch31(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH31
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH32`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch32(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH32
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH33`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch33(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH33
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH34`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch34(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH34
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH35`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch35(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH35
    }
    #[doc = "Checks if the value of the field is `DMAC0_OTRIG_CH36`"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch36(&self) -> bool {
        *self == DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH36
    }
}
#[doc = "Field `DMAC0_OTRIG_SEL` writer - DMAC0 Output Triggers Select for A, B, C, D IE"]
pub type DMAC0_OTRIG_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMAC0_OTRIG_SEL_SPEC, u8, DMAC0_OTRIG_SEL_A, 6, O>;
impl<'a, const O: u8> DMAC0_OTRIG_SEL_W<'a, O> {
    #[doc = "DMAC0_OTRIG_CH0"]
    #[inline(always)]
    pub fn dmac0_otrig_ch0(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH0)
    }
    #[doc = "DMAC0_OTRIG_CH1"]
    #[inline(always)]
    pub fn dmac0_otrig_ch1(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH1)
    }
    #[doc = "DMAC0_OTRIG_CH2"]
    #[inline(always)]
    pub fn dmac0_otrig_ch2(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH2)
    }
    #[doc = "DMAC0_OTRIG_CH3"]
    #[inline(always)]
    pub fn dmac0_otrig_ch3(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH3)
    }
    #[doc = "DMAC0_OTRIG_CH4"]
    #[inline(always)]
    pub fn dmac0_otrig_ch4(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH4)
    }
    #[doc = "DMAC0_OTRIG_CH5"]
    #[inline(always)]
    pub fn dmac0_otrig_ch5(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH5)
    }
    #[doc = "DMAC0_OTRIG_CH6"]
    #[inline(always)]
    pub fn dmac0_otrig_ch6(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH6)
    }
    #[doc = "DMAC0_OTRIG_CH7"]
    #[inline(always)]
    pub fn dmac0_otrig_ch7(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH7)
    }
    #[doc = "DMAC0_OTRIG_CH8"]
    #[inline(always)]
    pub fn dmac0_otrig_ch8(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH8)
    }
    #[doc = "DMAC0_OTRIG_CH9"]
    #[inline(always)]
    pub fn dmac0_otrig_ch9(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH9)
    }
    #[doc = "DMAC0_OTRIG_CH10"]
    #[inline(always)]
    pub fn dmac0_otrig_ch10(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH10)
    }
    #[doc = "DMAC0_OTRIG_CH11"]
    #[inline(always)]
    pub fn dmac0_otrig_ch11(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH11)
    }
    #[doc = "DMAC0_OTRIG_CH12"]
    #[inline(always)]
    pub fn dmac0_otrig_ch12(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH12)
    }
    #[doc = "DMAC0_OTRIG_CH13"]
    #[inline(always)]
    pub fn dmac0_otrig_ch13(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH13)
    }
    #[doc = "DMAC0_OTRIG_CH14"]
    #[inline(always)]
    pub fn dmac0_otrig_ch14(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH14)
    }
    #[doc = "DMAC0_OTRIG_CH15"]
    #[inline(always)]
    pub fn dmac0_otrig_ch15(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH15)
    }
    #[doc = "DMAC0_OTRIG_CH16"]
    #[inline(always)]
    pub fn dmac0_otrig_ch16(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH16)
    }
    #[doc = "DMAC0_OTRIG_CH17"]
    #[inline(always)]
    pub fn dmac0_otrig_ch17(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH17)
    }
    #[doc = "DMAC0_OTRIG_CH18"]
    #[inline(always)]
    pub fn dmac0_otrig_ch18(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH18)
    }
    #[doc = "DMAC0_OTRIG_CH19"]
    #[inline(always)]
    pub fn dmac0_otrig_ch19(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH19)
    }
    #[doc = "DMAC0_OTRIG_CH20"]
    #[inline(always)]
    pub fn dmac0_otrig_ch20(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH20)
    }
    #[doc = "DMAC0_OTRIG_CH21"]
    #[inline(always)]
    pub fn dmac0_otrig_ch21(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH21)
    }
    #[doc = "DMAC0_OTRIG_CH22"]
    #[inline(always)]
    pub fn dmac0_otrig_ch22(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH22)
    }
    #[doc = "DMAC0_OTRIG_CH23"]
    #[inline(always)]
    pub fn dmac0_otrig_ch23(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH23)
    }
    #[doc = "DMAC0_OTRIG_CH24"]
    #[inline(always)]
    pub fn dmac0_otrig_ch24(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH24)
    }
    #[doc = "DMAC0_OTRIG_CH25"]
    #[inline(always)]
    pub fn dmac0_otrig_ch25(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH25)
    }
    #[doc = "DMAC0_OTRIG_CH26"]
    #[inline(always)]
    pub fn dmac0_otrig_ch26(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH26)
    }
    #[doc = "DMAC0_OTRIG_CH27"]
    #[inline(always)]
    pub fn dmac0_otrig_ch27(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH27)
    }
    #[doc = "DMAC0_OTRIG_CH28"]
    #[inline(always)]
    pub fn dmac0_otrig_ch28(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH28)
    }
    #[doc = "DMAC0_OTRIG_CH29"]
    #[inline(always)]
    pub fn dmac0_otrig_ch29(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH29)
    }
    #[doc = "DMAC0_OTRIG_CH30"]
    #[inline(always)]
    pub fn dmac0_otrig_ch30(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH30)
    }
    #[doc = "DMAC0_OTRIG_CH31"]
    #[inline(always)]
    pub fn dmac0_otrig_ch31(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH31)
    }
    #[doc = "DMAC0_OTRIG_CH32"]
    #[inline(always)]
    pub fn dmac0_otrig_ch32(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH32)
    }
    #[doc = "DMAC0_OTRIG_CH33"]
    #[inline(always)]
    pub fn dmac0_otrig_ch33(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH33)
    }
    #[doc = "DMAC0_OTRIG_CH34"]
    #[inline(always)]
    pub fn dmac0_otrig_ch34(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH34)
    }
    #[doc = "DMAC0_OTRIG_CH35"]
    #[inline(always)]
    pub fn dmac0_otrig_ch35(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH35)
    }
    #[doc = "DMAC0_OTRIG_CH36"]
    #[inline(always)]
    pub fn dmac0_otrig_ch36(self) -> &'a mut W {
        self.variant(DMAC0_OTRIG_SEL_A::DMAC0_OTRIG_CH36)
    }
}
impl R {
    #[doc = "Bits 0:5 - DMAC0 Output Triggers Select for A, B, C, D IE"]
    #[inline(always)]
    pub fn dmac0_otrig_sel(&self) -> DMAC0_OTRIG_SEL_R {
        DMAC0_OTRIG_SEL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - DMAC0 Output Triggers Select for A, B, C, D IE"]
    #[inline(always)]
    #[must_use]
    pub fn dmac0_otrig_sel(&mut self) -> DMAC0_OTRIG_SEL_W<0> {
        DMAC0_OTRIG_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC0 Output Trigger Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac0_otrig_sel](index.html) module"]
pub struct DMAC0_OTRIG_SEL_SPEC;
impl crate::RegisterSpec for DMAC0_OTRIG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac0_otrig_sel::R](R) reader structure"]
impl crate::Readable for DMAC0_OTRIG_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac0_otrig_sel::W](W) writer structure"]
impl crate::Writable for DMAC0_OTRIG_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAC0_OTRIG_SEL[%s]
to value 0x1f"]
impl crate::Resettable for DMAC0_OTRIG_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
