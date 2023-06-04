#[doc = "Register `FIFORD` reader"]
pub struct R(crate::R<FIFORD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFORD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFORD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFORD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATA` reader - Received Data from the FIFO"]
pub type RXDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXSSEL0_N` reader - Slave Select 0 for Receive"]
pub type RXSSEL0_N_R = crate::BitReader<RXSSEL0_N_A>;
#[doc = "Slave Select 0 for Receive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXSSEL0_N_A {
    #[doc = "0: Slave Select 0 is active"]
    RXSSEL0_ISACTIVE = 0,
    #[doc = "1: Slave Select 0 is not active"]
    RXSSEL0_ISNOTACTIVE = 1,
}
impl From<RXSSEL0_N_A> for bool {
    #[inline(always)]
    fn from(variant: RXSSEL0_N_A) -> Self {
        variant as u8 != 0
    }
}
impl RXSSEL0_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSSEL0_N_A {
        match self.bits {
            false => RXSSEL0_N_A::RXSSEL0_ISACTIVE,
            true => RXSSEL0_N_A::RXSSEL0_ISNOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `RXSSEL0_ISACTIVE`"]
    #[inline(always)]
    pub fn is_rxssel0_isactive(&self) -> bool {
        *self == RXSSEL0_N_A::RXSSEL0_ISACTIVE
    }
    #[doc = "Checks if the value of the field is `RXSSEL0_ISNOTACTIVE`"]
    #[inline(always)]
    pub fn is_rxssel0_isnotactive(&self) -> bool {
        *self == RXSSEL0_N_A::RXSSEL0_ISNOTACTIVE
    }
}
#[doc = "Field `RXSSEL1_N` reader - Slave Select 1 for Receive"]
pub type RXSSEL1_N_R = crate::BitReader<RXSSEL1_N_A>;
#[doc = "Slave Select 1 for Receive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXSSEL1_N_A {
    #[doc = "0: Slave Select 1 is active"]
    RXSSEL1_ISACTIVE = 0,
    #[doc = "1: Slave Select 1 is not active"]
    RXSSEL1_ISNOTACTIVE = 1,
}
impl From<RXSSEL1_N_A> for bool {
    #[inline(always)]
    fn from(variant: RXSSEL1_N_A) -> Self {
        variant as u8 != 0
    }
}
impl RXSSEL1_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSSEL1_N_A {
        match self.bits {
            false => RXSSEL1_N_A::RXSSEL1_ISACTIVE,
            true => RXSSEL1_N_A::RXSSEL1_ISNOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `RXSSEL1_ISACTIVE`"]
    #[inline(always)]
    pub fn is_rxssel1_isactive(&self) -> bool {
        *self == RXSSEL1_N_A::RXSSEL1_ISACTIVE
    }
    #[doc = "Checks if the value of the field is `RXSSEL1_ISNOTACTIVE`"]
    #[inline(always)]
    pub fn is_rxssel1_isnotactive(&self) -> bool {
        *self == RXSSEL1_N_A::RXSSEL1_ISNOTACTIVE
    }
}
#[doc = "Field `RXSSEL2_N` reader - Slave Select 2 for Receive"]
pub type RXSSEL2_N_R = crate::BitReader<RXSSEL2_N_A>;
#[doc = "Slave Select 2 for Receive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXSSEL2_N_A {
    #[doc = "0: Slave Select 2 is active"]
    RXSSEL2_ISACTIVE = 0,
    #[doc = "1: Slave Select 2 is not active"]
    RXSSEL2_ISNOTACTIVE = 1,
}
impl From<RXSSEL2_N_A> for bool {
    #[inline(always)]
    fn from(variant: RXSSEL2_N_A) -> Self {
        variant as u8 != 0
    }
}
impl RXSSEL2_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSSEL2_N_A {
        match self.bits {
            false => RXSSEL2_N_A::RXSSEL2_ISACTIVE,
            true => RXSSEL2_N_A::RXSSEL2_ISNOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `RXSSEL2_ISACTIVE`"]
    #[inline(always)]
    pub fn is_rxssel2_isactive(&self) -> bool {
        *self == RXSSEL2_N_A::RXSSEL2_ISACTIVE
    }
    #[doc = "Checks if the value of the field is `RXSSEL2_ISNOTACTIVE`"]
    #[inline(always)]
    pub fn is_rxssel2_isnotactive(&self) -> bool {
        *self == RXSSEL2_N_A::RXSSEL2_ISNOTACTIVE
    }
}
#[doc = "Field `RXSSEL3_N` reader - Slave Select 3 for Receive"]
pub type RXSSEL3_N_R = crate::BitReader<RXSSEL3_N_A>;
#[doc = "Slave Select 3 for Receive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXSSEL3_N_A {
    #[doc = "0: Slave Select 3 is active"]
    RXSSEL3_ISACTIVE = 0,
    #[doc = "1: Slave Select 3 is not active"]
    RXSSEL3_ISNOTACTIVE = 1,
}
impl From<RXSSEL3_N_A> for bool {
    #[inline(always)]
    fn from(variant: RXSSEL3_N_A) -> Self {
        variant as u8 != 0
    }
}
impl RXSSEL3_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSSEL3_N_A {
        match self.bits {
            false => RXSSEL3_N_A::RXSSEL3_ISACTIVE,
            true => RXSSEL3_N_A::RXSSEL3_ISNOTACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `RXSSEL3_ISACTIVE`"]
    #[inline(always)]
    pub fn is_rxssel3_isactive(&self) -> bool {
        *self == RXSSEL3_N_A::RXSSEL3_ISACTIVE
    }
    #[doc = "Checks if the value of the field is `RXSSEL3_ISNOTACTIVE`"]
    #[inline(always)]
    pub fn is_rxssel3_isnotactive(&self) -> bool {
        *self == RXSSEL3_N_A::RXSSEL3_ISNOTACTIVE
    }
}
#[doc = "Field `SOT` reader - Start of Transfer Flag"]
pub type SOT_R = crate::BitReader<SOT_A>;
#[doc = "Start of Transfer Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOT_A {
    #[doc = "0: This is not the 1st data after the SSELs went from deasserted to asserted"]
    SOT0 = 0,
    #[doc = "1: This is the 1st data after the SSELs went from deasserted to asserted (i.e., any previous transfer has ended). This information can be used to identify the 1st piece of data in cases where the transfer length is greater than 16 bits."]
    SOT1 = 1,
}
impl From<SOT_A> for bool {
    #[inline(always)]
    fn from(variant: SOT_A) -> Self {
        variant as u8 != 0
    }
}
impl SOT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOT_A {
        match self.bits {
            false => SOT_A::SOT0,
            true => SOT_A::SOT1,
        }
    }
    #[doc = "Checks if the value of the field is `SOT0`"]
    #[inline(always)]
    pub fn is_sot0(&self) -> bool {
        *self == SOT_A::SOT0
    }
    #[doc = "Checks if the value of the field is `SOT1`"]
    #[inline(always)]
    pub fn is_sot1(&self) -> bool {
        *self == SOT_A::SOT1
    }
}
impl R {
    #[doc = "Bits 0:15 - Received Data from the FIFO"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Slave Select 0 for Receive"]
    #[inline(always)]
    pub fn rxssel0_n(&self) -> RXSSEL0_N_R {
        RXSSEL0_N_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slave Select 1 for Receive"]
    #[inline(always)]
    pub fn rxssel1_n(&self) -> RXSSEL1_N_R {
        RXSSEL1_N_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slave Select 2 for Receive"]
    #[inline(always)]
    pub fn rxssel2_n(&self) -> RXSSEL2_N_R {
        RXSSEL2_N_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Slave Select 3 for Receive"]
    #[inline(always)]
    pub fn rxssel3_n(&self) -> RXSSEL3_N_R {
        RXSSEL3_N_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Start of Transfer Flag"]
    #[inline(always)]
    pub fn sot(&self) -> SOT_R {
        SOT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "FIFO Read Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fiford](index.html) module"]
pub struct FIFORD_SPEC;
impl crate::RegisterSpec for FIFORD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fiford::R](R) reader structure"]
impl crate::Readable for FIFORD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFORD to value 0"]
impl crate::Resettable for FIFORD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
