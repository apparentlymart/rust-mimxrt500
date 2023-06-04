#[doc = "Register `SHAREDCTRLSET[%s]` reader"]
pub struct R(crate::R<SHAREDCTRLSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHAREDCTRLSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHAREDCTRLSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHAREDCTRLSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHAREDCTRLSET[%s]` writer"]
pub struct W(crate::W<SHAREDCTRLSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHAREDCTRLSET_SPEC>;
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
impl From<crate::W<SHAREDCTRLSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHAREDCTRLSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHAREDSCKSEL` reader - Shared SCK Select"]
pub type SHAREDSCKSEL_R = crate::FieldReader<u8, SHAREDSCKSEL_A>;
#[doc = "Shared SCK Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHAREDSCKSEL_A {
    #[doc = "0: FLEXCOMM0"]
    SHAREDSCKSEL_0 = 0,
    #[doc = "1: FLEXCOMM1"]
    SHAREDSCKSEL_1 = 1,
    #[doc = "2: FLEXCOMM2"]
    SHAREDSCKSEL_2 = 2,
    #[doc = "3: FLEXCOMM3"]
    SHAREDSCKSEL_3 = 3,
    #[doc = "4: FLEXCOMM4"]
    SHAREDSCKSEL_4 = 4,
    #[doc = "5: FLEXCOMM5"]
    SHAREDSCKSEL_5 = 5,
    #[doc = "6: FLEXCOMM6"]
    SHAREDSCKSEL_6 = 6,
    #[doc = "7: FLEXCOMM7"]
    SHAREDSCKSEL_7 = 7,
}
impl From<SHAREDSCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SHAREDSCKSEL_A) -> Self {
        variant as _
    }
}
impl SHAREDSCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHAREDSCKSEL_A {
        match self.bits {
            0 => SHAREDSCKSEL_A::SHAREDSCKSEL_0,
            1 => SHAREDSCKSEL_A::SHAREDSCKSEL_1,
            2 => SHAREDSCKSEL_A::SHAREDSCKSEL_2,
            3 => SHAREDSCKSEL_A::SHAREDSCKSEL_3,
            4 => SHAREDSCKSEL_A::SHAREDSCKSEL_4,
            5 => SHAREDSCKSEL_A::SHAREDSCKSEL_5,
            6 => SHAREDSCKSEL_A::SHAREDSCKSEL_6,
            7 => SHAREDSCKSEL_A::SHAREDSCKSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SHAREDSCKSEL_0`"]
    #[inline(always)]
    pub fn is_sharedscksel_0(&self) -> bool {
        *self == SHAREDSCKSEL_A::SHAREDSCKSEL_0
    }
    #[doc = "Checks if the value of the field is `SHAREDSCKSEL_1`"]
    #[inline(always)]
    pub fn is_sharedscksel_1(&self) -> bool {
        *self == SHAREDSCKSEL_A::SHAREDSCKSEL_1
    }
    #[doc = "Checks if the value of the field is `SHAREDSCKSEL_2`"]
    #[inline(always)]
    pub fn is_sharedscksel_2(&self) -> bool {
        *self == SHAREDSCKSEL_A::SHAREDSCKSEL_2
    }
    #[doc = "Checks if the value of the field is `SHAREDSCKSEL_3`"]
    #[inline(always)]
    pub fn is_sharedscksel_3(&self) -> bool {
        *self == SHAREDSCKSEL_A::SHAREDSCKSEL_3
    }
    #[doc = "Checks if the value of the field is `SHAREDSCKSEL_4`"]
    #[inline(always)]
    pub fn is_sharedscksel_4(&self) -> bool {
        *self == SHAREDSCKSEL_A::SHAREDSCKSEL_4
    }
    #[doc = "Checks if the value of the field is `SHAREDSCKSEL_5`"]
    #[inline(always)]
    pub fn is_sharedscksel_5(&self) -> bool {
        *self == SHAREDSCKSEL_A::SHAREDSCKSEL_5
    }
    #[doc = "Checks if the value of the field is `SHAREDSCKSEL_6`"]
    #[inline(always)]
    pub fn is_sharedscksel_6(&self) -> bool {
        *self == SHAREDSCKSEL_A::SHAREDSCKSEL_6
    }
    #[doc = "Checks if the value of the field is `SHAREDSCKSEL_7`"]
    #[inline(always)]
    pub fn is_sharedscksel_7(&self) -> bool {
        *self == SHAREDSCKSEL_A::SHAREDSCKSEL_7
    }
}
#[doc = "Field `SHAREDSCKSEL` writer - Shared SCK Select"]
pub type SHAREDSCKSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SHAREDCTRLSET_SPEC, u8, SHAREDSCKSEL_A, 3, O>;
impl<'a, const O: u8> SHAREDSCKSEL_W<'a, O> {
    #[doc = "FLEXCOMM0"]
    #[inline(always)]
    pub fn sharedscksel_0(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::SHAREDSCKSEL_0)
    }
    #[doc = "FLEXCOMM1"]
    #[inline(always)]
    pub fn sharedscksel_1(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::SHAREDSCKSEL_1)
    }
    #[doc = "FLEXCOMM2"]
    #[inline(always)]
    pub fn sharedscksel_2(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::SHAREDSCKSEL_2)
    }
    #[doc = "FLEXCOMM3"]
    #[inline(always)]
    pub fn sharedscksel_3(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::SHAREDSCKSEL_3)
    }
    #[doc = "FLEXCOMM4"]
    #[inline(always)]
    pub fn sharedscksel_4(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::SHAREDSCKSEL_4)
    }
    #[doc = "FLEXCOMM5"]
    #[inline(always)]
    pub fn sharedscksel_5(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::SHAREDSCKSEL_5)
    }
    #[doc = "FLEXCOMM6"]
    #[inline(always)]
    pub fn sharedscksel_6(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::SHAREDSCKSEL_6)
    }
    #[doc = "FLEXCOMM7"]
    #[inline(always)]
    pub fn sharedscksel_7(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::SHAREDSCKSEL_7)
    }
}
#[doc = "Field `SHAREDWSSEL` reader - Shared WS Select:"]
pub type SHAREDWSSEL_R = crate::FieldReader<u8, SHAREDWSSEL_A>;
#[doc = "Shared WS Select:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHAREDWSSEL_A {
    #[doc = "0: FLEXCOMM0"]
    SHAREDWSSEL_0 = 0,
    #[doc = "1: FLEXCOMM1"]
    SHAREDWSSEL_1 = 1,
    #[doc = "2: FLEXCOMM2"]
    SHAREDWSSEL_2 = 2,
    #[doc = "3: FLEXCOMM3"]
    SHAREDWSSEL_3 = 3,
    #[doc = "4: FLEXCOMM4"]
    SHAREDWSSEL_4 = 4,
    #[doc = "5: FLEXCOMM5"]
    SHAREDWSSEL_5 = 5,
    #[doc = "6: FLEXCOMM6"]
    SHAREDWSSEL_6 = 6,
    #[doc = "7: FLEXCOMM7"]
    SHAREDWSSEL_7 = 7,
}
impl From<SHAREDWSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SHAREDWSSEL_A) -> Self {
        variant as _
    }
}
impl SHAREDWSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHAREDWSSEL_A {
        match self.bits {
            0 => SHAREDWSSEL_A::SHAREDWSSEL_0,
            1 => SHAREDWSSEL_A::SHAREDWSSEL_1,
            2 => SHAREDWSSEL_A::SHAREDWSSEL_2,
            3 => SHAREDWSSEL_A::SHAREDWSSEL_3,
            4 => SHAREDWSSEL_A::SHAREDWSSEL_4,
            5 => SHAREDWSSEL_A::SHAREDWSSEL_5,
            6 => SHAREDWSSEL_A::SHAREDWSSEL_6,
            7 => SHAREDWSSEL_A::SHAREDWSSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SHAREDWSSEL_0`"]
    #[inline(always)]
    pub fn is_sharedwssel_0(&self) -> bool {
        *self == SHAREDWSSEL_A::SHAREDWSSEL_0
    }
    #[doc = "Checks if the value of the field is `SHAREDWSSEL_1`"]
    #[inline(always)]
    pub fn is_sharedwssel_1(&self) -> bool {
        *self == SHAREDWSSEL_A::SHAREDWSSEL_1
    }
    #[doc = "Checks if the value of the field is `SHAREDWSSEL_2`"]
    #[inline(always)]
    pub fn is_sharedwssel_2(&self) -> bool {
        *self == SHAREDWSSEL_A::SHAREDWSSEL_2
    }
    #[doc = "Checks if the value of the field is `SHAREDWSSEL_3`"]
    #[inline(always)]
    pub fn is_sharedwssel_3(&self) -> bool {
        *self == SHAREDWSSEL_A::SHAREDWSSEL_3
    }
    #[doc = "Checks if the value of the field is `SHAREDWSSEL_4`"]
    #[inline(always)]
    pub fn is_sharedwssel_4(&self) -> bool {
        *self == SHAREDWSSEL_A::SHAREDWSSEL_4
    }
    #[doc = "Checks if the value of the field is `SHAREDWSSEL_5`"]
    #[inline(always)]
    pub fn is_sharedwssel_5(&self) -> bool {
        *self == SHAREDWSSEL_A::SHAREDWSSEL_5
    }
    #[doc = "Checks if the value of the field is `SHAREDWSSEL_6`"]
    #[inline(always)]
    pub fn is_sharedwssel_6(&self) -> bool {
        *self == SHAREDWSSEL_A::SHAREDWSSEL_6
    }
    #[doc = "Checks if the value of the field is `SHAREDWSSEL_7`"]
    #[inline(always)]
    pub fn is_sharedwssel_7(&self) -> bool {
        *self == SHAREDWSSEL_A::SHAREDWSSEL_7
    }
}
#[doc = "Field `SHAREDWSSEL` writer - Shared WS Select:"]
pub type SHAREDWSSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SHAREDCTRLSET_SPEC, u8, SHAREDWSSEL_A, 3, O>;
impl<'a, const O: u8> SHAREDWSSEL_W<'a, O> {
    #[doc = "FLEXCOMM0"]
    #[inline(always)]
    pub fn sharedwssel_0(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::SHAREDWSSEL_0)
    }
    #[doc = "FLEXCOMM1"]
    #[inline(always)]
    pub fn sharedwssel_1(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::SHAREDWSSEL_1)
    }
    #[doc = "FLEXCOMM2"]
    #[inline(always)]
    pub fn sharedwssel_2(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::SHAREDWSSEL_2)
    }
    #[doc = "FLEXCOMM3"]
    #[inline(always)]
    pub fn sharedwssel_3(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::SHAREDWSSEL_3)
    }
    #[doc = "FLEXCOMM4"]
    #[inline(always)]
    pub fn sharedwssel_4(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::SHAREDWSSEL_4)
    }
    #[doc = "FLEXCOMM5"]
    #[inline(always)]
    pub fn sharedwssel_5(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::SHAREDWSSEL_5)
    }
    #[doc = "FLEXCOMM6"]
    #[inline(always)]
    pub fn sharedwssel_6(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::SHAREDWSSEL_6)
    }
    #[doc = "FLEXCOMM7"]
    #[inline(always)]
    pub fn sharedwssel_7(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::SHAREDWSSEL_7)
    }
}
#[doc = "Field `SHAREDDATASEL` reader - Shared DATA Select:"]
pub type SHAREDDATASEL_R = crate::FieldReader<u8, SHAREDDATASEL_A>;
#[doc = "Shared DATA Select:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHAREDDATASEL_A {
    #[doc = "0: FLEXCOMM0"]
    SHAREDWSSEL_0 = 0,
    #[doc = "1: FLEXCOMM1"]
    SHAREDDATASEL_1 = 1,
    #[doc = "2: FLEXCOMM2"]
    SHAREDDATASEL_2 = 2,
    #[doc = "3: FLEXCOMM3"]
    SHAREDDATASEL_3 = 3,
    #[doc = "4: FLEXCOMM4"]
    SHAREDDATASEL_4 = 4,
    #[doc = "5: FLEXCOMM5"]
    SHAREDDATASEL_5 = 5,
    #[doc = "6: FLEXCOMM6"]
    SHAREDDATASEL_6 = 6,
    #[doc = "7: FLEXCOMM7"]
    SHAREDDATASEL_7 = 7,
}
impl From<SHAREDDATASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SHAREDDATASEL_A) -> Self {
        variant as _
    }
}
impl SHAREDDATASEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHAREDDATASEL_A {
        match self.bits {
            0 => SHAREDDATASEL_A::SHAREDWSSEL_0,
            1 => SHAREDDATASEL_A::SHAREDDATASEL_1,
            2 => SHAREDDATASEL_A::SHAREDDATASEL_2,
            3 => SHAREDDATASEL_A::SHAREDDATASEL_3,
            4 => SHAREDDATASEL_A::SHAREDDATASEL_4,
            5 => SHAREDDATASEL_A::SHAREDDATASEL_5,
            6 => SHAREDDATASEL_A::SHAREDDATASEL_6,
            7 => SHAREDDATASEL_A::SHAREDDATASEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SHAREDWSSEL_0`"]
    #[inline(always)]
    pub fn is_sharedwssel_0(&self) -> bool {
        *self == SHAREDDATASEL_A::SHAREDWSSEL_0
    }
    #[doc = "Checks if the value of the field is `SHAREDDATASEL_1`"]
    #[inline(always)]
    pub fn is_shareddatasel_1(&self) -> bool {
        *self == SHAREDDATASEL_A::SHAREDDATASEL_1
    }
    #[doc = "Checks if the value of the field is `SHAREDDATASEL_2`"]
    #[inline(always)]
    pub fn is_shareddatasel_2(&self) -> bool {
        *self == SHAREDDATASEL_A::SHAREDDATASEL_2
    }
    #[doc = "Checks if the value of the field is `SHAREDDATASEL_3`"]
    #[inline(always)]
    pub fn is_shareddatasel_3(&self) -> bool {
        *self == SHAREDDATASEL_A::SHAREDDATASEL_3
    }
    #[doc = "Checks if the value of the field is `SHAREDDATASEL_4`"]
    #[inline(always)]
    pub fn is_shareddatasel_4(&self) -> bool {
        *self == SHAREDDATASEL_A::SHAREDDATASEL_4
    }
    #[doc = "Checks if the value of the field is `SHAREDDATASEL_5`"]
    #[inline(always)]
    pub fn is_shareddatasel_5(&self) -> bool {
        *self == SHAREDDATASEL_A::SHAREDDATASEL_5
    }
    #[doc = "Checks if the value of the field is `SHAREDDATASEL_6`"]
    #[inline(always)]
    pub fn is_shareddatasel_6(&self) -> bool {
        *self == SHAREDDATASEL_A::SHAREDDATASEL_6
    }
    #[doc = "Checks if the value of the field is `SHAREDDATASEL_7`"]
    #[inline(always)]
    pub fn is_shareddatasel_7(&self) -> bool {
        *self == SHAREDDATASEL_A::SHAREDDATASEL_7
    }
}
#[doc = "Field `SHAREDDATASEL` writer - Shared DATA Select:"]
pub type SHAREDDATASEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SHAREDCTRLSET_SPEC, u8, SHAREDDATASEL_A, 3, O>;
impl<'a, const O: u8> SHAREDDATASEL_W<'a, O> {
    #[doc = "FLEXCOMM0"]
    #[inline(always)]
    pub fn sharedwssel_0(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::SHAREDWSSEL_0)
    }
    #[doc = "FLEXCOMM1"]
    #[inline(always)]
    pub fn shareddatasel_1(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::SHAREDDATASEL_1)
    }
    #[doc = "FLEXCOMM2"]
    #[inline(always)]
    pub fn shareddatasel_2(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::SHAREDDATASEL_2)
    }
    #[doc = "FLEXCOMM3"]
    #[inline(always)]
    pub fn shareddatasel_3(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::SHAREDDATASEL_3)
    }
    #[doc = "FLEXCOMM4"]
    #[inline(always)]
    pub fn shareddatasel_4(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::SHAREDDATASEL_4)
    }
    #[doc = "FLEXCOMM5"]
    #[inline(always)]
    pub fn shareddatasel_5(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::SHAREDDATASEL_5)
    }
    #[doc = "FLEXCOMM6"]
    #[inline(always)]
    pub fn shareddatasel_6(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::SHAREDDATASEL_6)
    }
    #[doc = "FLEXCOMM7"]
    #[inline(always)]
    pub fn shareddatasel_7(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::SHAREDDATASEL_7)
    }
}
#[doc = "Field `FC0DATAOUTEN` reader - FLEXCOMM0 DATAOUT Output Enable:"]
pub type FC0DATAOUTEN_R = crate::BitReader<FC0DATAOUTEN_A>;
#[doc = "FLEXCOMM0 DATAOUT Output Enable:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC0DATAOUTEN_A {
    #[doc = "0: Input"]
    FC0DATAOUTEN0_0 = 0,
    #[doc = "1: Output"]
    FC0DATAOUTEN0_1 = 1,
}
impl From<FC0DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC0DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FC0DATAOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC0DATAOUTEN_A {
        match self.bits {
            false => FC0DATAOUTEN_A::FC0DATAOUTEN0_0,
            true => FC0DATAOUTEN_A::FC0DATAOUTEN0_1,
        }
    }
    #[doc = "Checks if the value of the field is `FC0DATAOUTEN0_0`"]
    #[inline(always)]
    pub fn is_fc0dataouten0_0(&self) -> bool {
        *self == FC0DATAOUTEN_A::FC0DATAOUTEN0_0
    }
    #[doc = "Checks if the value of the field is `FC0DATAOUTEN0_1`"]
    #[inline(always)]
    pub fn is_fc0dataouten0_1(&self) -> bool {
        *self == FC0DATAOUTEN_A::FC0DATAOUTEN0_1
    }
}
#[doc = "Field `FC0DATAOUTEN` writer - FLEXCOMM0 DATAOUT Output Enable:"]
pub type FC0DATAOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHAREDCTRLSET_SPEC, FC0DATAOUTEN_A, O>;
impl<'a, const O: u8> FC0DATAOUTEN_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn fc0dataouten0_0(self) -> &'a mut W {
        self.variant(FC0DATAOUTEN_A::FC0DATAOUTEN0_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn fc0dataouten0_1(self) -> &'a mut W {
        self.variant(FC0DATAOUTEN_A::FC0DATAOUTEN0_1)
    }
}
#[doc = "Field `FC1DATAOUTEN` reader - FLEXCOMM1 DATAOUT Output Enable:"]
pub type FC1DATAOUTEN_R = crate::BitReader<FC1DATAOUTEN_A>;
#[doc = "FLEXCOMM1 DATAOUT Output Enable:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC1DATAOUTEN_A {
    #[doc = "0: Input"]
    FC0DATAOUTEN1_0 = 0,
    #[doc = "1: Output"]
    FC0DATAOUTEN1_1 = 1,
}
impl From<FC1DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC1DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FC1DATAOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC1DATAOUTEN_A {
        match self.bits {
            false => FC1DATAOUTEN_A::FC0DATAOUTEN1_0,
            true => FC1DATAOUTEN_A::FC0DATAOUTEN1_1,
        }
    }
    #[doc = "Checks if the value of the field is `FC0DATAOUTEN1_0`"]
    #[inline(always)]
    pub fn is_fc0dataouten1_0(&self) -> bool {
        *self == FC1DATAOUTEN_A::FC0DATAOUTEN1_0
    }
    #[doc = "Checks if the value of the field is `FC0DATAOUTEN1_1`"]
    #[inline(always)]
    pub fn is_fc0dataouten1_1(&self) -> bool {
        *self == FC1DATAOUTEN_A::FC0DATAOUTEN1_1
    }
}
#[doc = "Field `FC1DATAOUTEN` writer - FLEXCOMM1 DATAOUT Output Enable:"]
pub type FC1DATAOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHAREDCTRLSET_SPEC, FC1DATAOUTEN_A, O>;
impl<'a, const O: u8> FC1DATAOUTEN_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn fc0dataouten1_0(self) -> &'a mut W {
        self.variant(FC1DATAOUTEN_A::FC0DATAOUTEN1_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn fc0dataouten1_1(self) -> &'a mut W {
        self.variant(FC1DATAOUTEN_A::FC0DATAOUTEN1_1)
    }
}
#[doc = "Field `FC2DATAOUTEN` reader - FLEXCOMM2 DATAOUT Output Enable:"]
pub type FC2DATAOUTEN_R = crate::BitReader<FC2DATAOUTEN_A>;
#[doc = "FLEXCOMM2 DATAOUT Output Enable:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC2DATAOUTEN_A {
    #[doc = "0: Input"]
    FC0DATAOUTEN2_0 = 0,
    #[doc = "1: Output"]
    FC0DATAOUTEN2_1 = 1,
}
impl From<FC2DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC2DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FC2DATAOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC2DATAOUTEN_A {
        match self.bits {
            false => FC2DATAOUTEN_A::FC0DATAOUTEN2_0,
            true => FC2DATAOUTEN_A::FC0DATAOUTEN2_1,
        }
    }
    #[doc = "Checks if the value of the field is `FC0DATAOUTEN2_0`"]
    #[inline(always)]
    pub fn is_fc0dataouten2_0(&self) -> bool {
        *self == FC2DATAOUTEN_A::FC0DATAOUTEN2_0
    }
    #[doc = "Checks if the value of the field is `FC0DATAOUTEN2_1`"]
    #[inline(always)]
    pub fn is_fc0dataouten2_1(&self) -> bool {
        *self == FC2DATAOUTEN_A::FC0DATAOUTEN2_1
    }
}
#[doc = "Field `FC2DATAOUTEN` writer - FLEXCOMM2 DATAOUT Output Enable:"]
pub type FC2DATAOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHAREDCTRLSET_SPEC, FC2DATAOUTEN_A, O>;
impl<'a, const O: u8> FC2DATAOUTEN_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn fc0dataouten2_0(self) -> &'a mut W {
        self.variant(FC2DATAOUTEN_A::FC0DATAOUTEN2_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn fc0dataouten2_1(self) -> &'a mut W {
        self.variant(FC2DATAOUTEN_A::FC0DATAOUTEN2_1)
    }
}
#[doc = "Field `FC3DATAOUTEN` reader - FLEXCOMM3 DATAOUT Output Enable:"]
pub type FC3DATAOUTEN_R = crate::BitReader<FC3DATAOUTEN_A>;
#[doc = "FLEXCOMM3 DATAOUT Output Enable:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC3DATAOUTEN_A {
    #[doc = "0: Input"]
    FC0DATAOUTEN3_0 = 0,
    #[doc = "1: Output"]
    FC0DATAOUTEN3_1 = 1,
}
impl From<FC3DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC3DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FC3DATAOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC3DATAOUTEN_A {
        match self.bits {
            false => FC3DATAOUTEN_A::FC0DATAOUTEN3_0,
            true => FC3DATAOUTEN_A::FC0DATAOUTEN3_1,
        }
    }
    #[doc = "Checks if the value of the field is `FC0DATAOUTEN3_0`"]
    #[inline(always)]
    pub fn is_fc0dataouten3_0(&self) -> bool {
        *self == FC3DATAOUTEN_A::FC0DATAOUTEN3_0
    }
    #[doc = "Checks if the value of the field is `FC0DATAOUTEN3_1`"]
    #[inline(always)]
    pub fn is_fc0dataouten3_1(&self) -> bool {
        *self == FC3DATAOUTEN_A::FC0DATAOUTEN3_1
    }
}
#[doc = "Field `FC3DATAOUTEN` writer - FLEXCOMM3 DATAOUT Output Enable:"]
pub type FC3DATAOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHAREDCTRLSET_SPEC, FC3DATAOUTEN_A, O>;
impl<'a, const O: u8> FC3DATAOUTEN_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn fc0dataouten3_0(self) -> &'a mut W {
        self.variant(FC3DATAOUTEN_A::FC0DATAOUTEN3_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn fc0dataouten3_1(self) -> &'a mut W {
        self.variant(FC3DATAOUTEN_A::FC0DATAOUTEN3_1)
    }
}
#[doc = "Field `FC4DATAOUTEN` reader - FLEXCOMM4 DATAOUT Output Enable:"]
pub type FC4DATAOUTEN_R = crate::BitReader<FC4DATAOUTEN_A>;
#[doc = "FLEXCOMM4 DATAOUT Output Enable:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC4DATAOUTEN_A {
    #[doc = "0: Input"]
    FC0DATAOUTEN4_0 = 0,
    #[doc = "1: Output"]
    FC0DATAOUTEN4_1 = 1,
}
impl From<FC4DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC4DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FC4DATAOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC4DATAOUTEN_A {
        match self.bits {
            false => FC4DATAOUTEN_A::FC0DATAOUTEN4_0,
            true => FC4DATAOUTEN_A::FC0DATAOUTEN4_1,
        }
    }
    #[doc = "Checks if the value of the field is `FC0DATAOUTEN4_0`"]
    #[inline(always)]
    pub fn is_fc0dataouten4_0(&self) -> bool {
        *self == FC4DATAOUTEN_A::FC0DATAOUTEN4_0
    }
    #[doc = "Checks if the value of the field is `FC0DATAOUTEN4_1`"]
    #[inline(always)]
    pub fn is_fc0dataouten4_1(&self) -> bool {
        *self == FC4DATAOUTEN_A::FC0DATAOUTEN4_1
    }
}
#[doc = "Field `FC4DATAOUTEN` writer - FLEXCOMM4 DATAOUT Output Enable:"]
pub type FC4DATAOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHAREDCTRLSET_SPEC, FC4DATAOUTEN_A, O>;
impl<'a, const O: u8> FC4DATAOUTEN_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn fc0dataouten4_0(self) -> &'a mut W {
        self.variant(FC4DATAOUTEN_A::FC0DATAOUTEN4_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn fc0dataouten4_1(self) -> &'a mut W {
        self.variant(FC4DATAOUTEN_A::FC0DATAOUTEN4_1)
    }
}
#[doc = "Field `FC5DATAOUTEN` reader - FLEXCOMM5 DATAOUT Output Enable:"]
pub type FC5DATAOUTEN_R = crate::BitReader<FC5DATAOUTEN_A>;
#[doc = "FLEXCOMM5 DATAOUT Output Enable:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC5DATAOUTEN_A {
    #[doc = "0: Input"]
    FC0DATAOUTEN5_0 = 0,
    #[doc = "1: Output"]
    FC0DATAOUTEN5_1 = 1,
}
impl From<FC5DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC5DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FC5DATAOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC5DATAOUTEN_A {
        match self.bits {
            false => FC5DATAOUTEN_A::FC0DATAOUTEN5_0,
            true => FC5DATAOUTEN_A::FC0DATAOUTEN5_1,
        }
    }
    #[doc = "Checks if the value of the field is `FC0DATAOUTEN5_0`"]
    #[inline(always)]
    pub fn is_fc0dataouten5_0(&self) -> bool {
        *self == FC5DATAOUTEN_A::FC0DATAOUTEN5_0
    }
    #[doc = "Checks if the value of the field is `FC0DATAOUTEN5_1`"]
    #[inline(always)]
    pub fn is_fc0dataouten5_1(&self) -> bool {
        *self == FC5DATAOUTEN_A::FC0DATAOUTEN5_1
    }
}
#[doc = "Field `FC5DATAOUTEN` writer - FLEXCOMM5 DATAOUT Output Enable:"]
pub type FC5DATAOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHAREDCTRLSET_SPEC, FC5DATAOUTEN_A, O>;
impl<'a, const O: u8> FC5DATAOUTEN_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn fc0dataouten5_0(self) -> &'a mut W {
        self.variant(FC5DATAOUTEN_A::FC0DATAOUTEN5_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn fc0dataouten5_1(self) -> &'a mut W {
        self.variant(FC5DATAOUTEN_A::FC0DATAOUTEN5_1)
    }
}
#[doc = "Field `FC6DATAOUTEN` reader - FLEXCOMM6 DATAOUT Output Enable:"]
pub type FC6DATAOUTEN_R = crate::BitReader<FC6DATAOUTEN_A>;
#[doc = "FLEXCOMM6 DATAOUT Output Enable:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC6DATAOUTEN_A {
    #[doc = "0: Input"]
    FC0DATAOUTEN6_0 = 0,
    #[doc = "1: Output"]
    FC0DATAOUTEN6_1 = 1,
}
impl From<FC6DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC6DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FC6DATAOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC6DATAOUTEN_A {
        match self.bits {
            false => FC6DATAOUTEN_A::FC0DATAOUTEN6_0,
            true => FC6DATAOUTEN_A::FC0DATAOUTEN6_1,
        }
    }
    #[doc = "Checks if the value of the field is `FC0DATAOUTEN6_0`"]
    #[inline(always)]
    pub fn is_fc0dataouten6_0(&self) -> bool {
        *self == FC6DATAOUTEN_A::FC0DATAOUTEN6_0
    }
    #[doc = "Checks if the value of the field is `FC0DATAOUTEN6_1`"]
    #[inline(always)]
    pub fn is_fc0dataouten6_1(&self) -> bool {
        *self == FC6DATAOUTEN_A::FC0DATAOUTEN6_1
    }
}
#[doc = "Field `FC6DATAOUTEN` writer - FLEXCOMM6 DATAOUT Output Enable:"]
pub type FC6DATAOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHAREDCTRLSET_SPEC, FC6DATAOUTEN_A, O>;
impl<'a, const O: u8> FC6DATAOUTEN_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn fc0dataouten6_0(self) -> &'a mut W {
        self.variant(FC6DATAOUTEN_A::FC0DATAOUTEN6_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn fc0dataouten6_1(self) -> &'a mut W {
        self.variant(FC6DATAOUTEN_A::FC0DATAOUTEN6_1)
    }
}
#[doc = "Field `FC7DATAOUTEN` reader - FLEXCOMM7 DATAOUT Output Enable:"]
pub type FC7DATAOUTEN_R = crate::BitReader<FC7DATAOUTEN_A>;
#[doc = "FLEXCOMM7 DATAOUT Output Enable:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC7DATAOUTEN_A {
    #[doc = "0: Input"]
    FC0DATAOUTEN7_0 = 0,
    #[doc = "1: Output"]
    FC0DATAOUTEN7_1 = 1,
}
impl From<FC7DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC7DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FC7DATAOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC7DATAOUTEN_A {
        match self.bits {
            false => FC7DATAOUTEN_A::FC0DATAOUTEN7_0,
            true => FC7DATAOUTEN_A::FC0DATAOUTEN7_1,
        }
    }
    #[doc = "Checks if the value of the field is `FC0DATAOUTEN7_0`"]
    #[inline(always)]
    pub fn is_fc0dataouten7_0(&self) -> bool {
        *self == FC7DATAOUTEN_A::FC0DATAOUTEN7_0
    }
    #[doc = "Checks if the value of the field is `FC0DATAOUTEN7_1`"]
    #[inline(always)]
    pub fn is_fc0dataouten7_1(&self) -> bool {
        *self == FC7DATAOUTEN_A::FC0DATAOUTEN7_1
    }
}
#[doc = "Field `FC7DATAOUTEN` writer - FLEXCOMM7 DATAOUT Output Enable:"]
pub type FC7DATAOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHAREDCTRLSET_SPEC, FC7DATAOUTEN_A, O>;
impl<'a, const O: u8> FC7DATAOUTEN_W<'a, O> {
    #[doc = "Input"]
    #[inline(always)]
    pub fn fc0dataouten7_0(self) -> &'a mut W {
        self.variant(FC7DATAOUTEN_A::FC0DATAOUTEN7_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn fc0dataouten7_1(self) -> &'a mut W {
        self.variant(FC7DATAOUTEN_A::FC0DATAOUTEN7_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Shared SCK Select"]
    #[inline(always)]
    pub fn sharedscksel(&self) -> SHAREDSCKSEL_R {
        SHAREDSCKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Shared WS Select:"]
    #[inline(always)]
    pub fn sharedwssel(&self) -> SHAREDWSSEL_R {
        SHAREDWSSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Shared DATA Select:"]
    #[inline(always)]
    pub fn shareddatasel(&self) -> SHAREDDATASEL_R {
        SHAREDDATASEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - FLEXCOMM0 DATAOUT Output Enable:"]
    #[inline(always)]
    pub fn fc0dataouten(&self) -> FC0DATAOUTEN_R {
        FC0DATAOUTEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FLEXCOMM1 DATAOUT Output Enable:"]
    #[inline(always)]
    pub fn fc1dataouten(&self) -> FC1DATAOUTEN_R {
        FC1DATAOUTEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FLEXCOMM2 DATAOUT Output Enable:"]
    #[inline(always)]
    pub fn fc2dataouten(&self) -> FC2DATAOUTEN_R {
        FC2DATAOUTEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - FLEXCOMM3 DATAOUT Output Enable:"]
    #[inline(always)]
    pub fn fc3dataouten(&self) -> FC3DATAOUTEN_R {
        FC3DATAOUTEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - FLEXCOMM4 DATAOUT Output Enable:"]
    #[inline(always)]
    pub fn fc4dataouten(&self) -> FC4DATAOUTEN_R {
        FC4DATAOUTEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - FLEXCOMM5 DATAOUT Output Enable:"]
    #[inline(always)]
    pub fn fc5dataouten(&self) -> FC5DATAOUTEN_R {
        FC5DATAOUTEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - FLEXCOMM6 DATAOUT Output Enable:"]
    #[inline(always)]
    pub fn fc6dataouten(&self) -> FC6DATAOUTEN_R {
        FC6DATAOUTEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - FLEXCOMM7 DATAOUT Output Enable:"]
    #[inline(always)]
    pub fn fc7dataouten(&self) -> FC7DATAOUTEN_R {
        FC7DATAOUTEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Shared SCK Select"]
    #[inline(always)]
    #[must_use]
    pub fn sharedscksel(&mut self) -> SHAREDSCKSEL_W<0> {
        SHAREDSCKSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - Shared WS Select:"]
    #[inline(always)]
    #[must_use]
    pub fn sharedwssel(&mut self) -> SHAREDWSSEL_W<4> {
        SHAREDWSSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - Shared DATA Select:"]
    #[inline(always)]
    #[must_use]
    pub fn shareddatasel(&mut self) -> SHAREDDATASEL_W<8> {
        SHAREDDATASEL_W::new(self)
    }
    #[doc = "Bit 16 - FLEXCOMM0 DATAOUT Output Enable:"]
    #[inline(always)]
    #[must_use]
    pub fn fc0dataouten(&mut self) -> FC0DATAOUTEN_W<16> {
        FC0DATAOUTEN_W::new(self)
    }
    #[doc = "Bit 17 - FLEXCOMM1 DATAOUT Output Enable:"]
    #[inline(always)]
    #[must_use]
    pub fn fc1dataouten(&mut self) -> FC1DATAOUTEN_W<17> {
        FC1DATAOUTEN_W::new(self)
    }
    #[doc = "Bit 18 - FLEXCOMM2 DATAOUT Output Enable:"]
    #[inline(always)]
    #[must_use]
    pub fn fc2dataouten(&mut self) -> FC2DATAOUTEN_W<18> {
        FC2DATAOUTEN_W::new(self)
    }
    #[doc = "Bit 19 - FLEXCOMM3 DATAOUT Output Enable:"]
    #[inline(always)]
    #[must_use]
    pub fn fc3dataouten(&mut self) -> FC3DATAOUTEN_W<19> {
        FC3DATAOUTEN_W::new(self)
    }
    #[doc = "Bit 20 - FLEXCOMM4 DATAOUT Output Enable:"]
    #[inline(always)]
    #[must_use]
    pub fn fc4dataouten(&mut self) -> FC4DATAOUTEN_W<20> {
        FC4DATAOUTEN_W::new(self)
    }
    #[doc = "Bit 21 - FLEXCOMM5 DATAOUT Output Enable:"]
    #[inline(always)]
    #[must_use]
    pub fn fc5dataouten(&mut self) -> FC5DATAOUTEN_W<21> {
        FC5DATAOUTEN_W::new(self)
    }
    #[doc = "Bit 22 - FLEXCOMM6 DATAOUT Output Enable:"]
    #[inline(always)]
    #[must_use]
    pub fn fc6dataouten(&mut self) -> FC6DATAOUTEN_W<22> {
        FC6DATAOUTEN_W::new(self)
    }
    #[doc = "Bit 23 - FLEXCOMM7 DATAOUT Output Enable:"]
    #[inline(always)]
    #[must_use]
    pub fn fc7dataouten(&mut self) -> FC7DATAOUTEN_W<23> {
        FC7DATAOUTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shared control set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sharedctrlset](index.html) module"]
pub struct SHAREDCTRLSET_SPEC;
impl crate::RegisterSpec for SHAREDCTRLSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sharedctrlset::R](R) reader structure"]
impl crate::Readable for SHAREDCTRLSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sharedctrlset::W](W) writer structure"]
impl crate::Writable for SHAREDCTRLSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHAREDCTRLSET[%s]
to value 0"]
impl crate::Resettable for SHAREDCTRLSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
