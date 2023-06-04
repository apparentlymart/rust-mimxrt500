#[doc = "Register `FIFORDNOPOP` reader"]
pub struct R(crate::R<FIFORDNOPOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFORDNOPOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFORDNOPOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFORDNOPOP_SPEC>) -> Self {
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
    #[doc = "0: Not selected"]
    NOT_SELECTED = 0,
    #[doc = "1: Selected"]
    RXSSEL0_N_SELECTED = 1,
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
            false => RXSSEL0_N_A::NOT_SELECTED,
            true => RXSSEL0_N_A::RXSSEL0_N_SELECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_SELECTED`"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == RXSSEL0_N_A::NOT_SELECTED
    }
    #[doc = "Checks if the value of the field is `RXSSEL0_N_SELECTED`"]
    #[inline(always)]
    pub fn is_rxssel0_n_selected(&self) -> bool {
        *self == RXSSEL0_N_A::RXSSEL0_N_SELECTED
    }
}
#[doc = "Field `RXSSEL1_N` reader - Slave Select 1 for Receive"]
pub type RXSSEL1_N_R = crate::BitReader<RXSSEL1_N_A>;
#[doc = "Slave Select 1 for Receive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXSSEL1_N_A {
    #[doc = "0: Not selected"]
    NOT_SELECTED = 0,
    #[doc = "1: Selected"]
    RXSSEL1_N_SELECTED = 1,
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
            false => RXSSEL1_N_A::NOT_SELECTED,
            true => RXSSEL1_N_A::RXSSEL1_N_SELECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_SELECTED`"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == RXSSEL1_N_A::NOT_SELECTED
    }
    #[doc = "Checks if the value of the field is `RXSSEL1_N_SELECTED`"]
    #[inline(always)]
    pub fn is_rxssel1_n_selected(&self) -> bool {
        *self == RXSSEL1_N_A::RXSSEL1_N_SELECTED
    }
}
#[doc = "Field `RXSSEL2_N` reader - Slave Select 2 for Receive"]
pub type RXSSEL2_N_R = crate::BitReader<RXSSEL2_N_A>;
#[doc = "Slave Select 2 for Receive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXSSEL2_N_A {
    #[doc = "0: Not selected"]
    NOT_SELECTED = 0,
    #[doc = "1: Selected"]
    RXSSEL2_N_SELECTED = 1,
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
            false => RXSSEL2_N_A::NOT_SELECTED,
            true => RXSSEL2_N_A::RXSSEL2_N_SELECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_SELECTED`"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == RXSSEL2_N_A::NOT_SELECTED
    }
    #[doc = "Checks if the value of the field is `RXSSEL2_N_SELECTED`"]
    #[inline(always)]
    pub fn is_rxssel2_n_selected(&self) -> bool {
        *self == RXSSEL2_N_A::RXSSEL2_N_SELECTED
    }
}
#[doc = "Field `RXSSEL3_N` reader - Slave Select 3 for Receive"]
pub type RXSSEL3_N_R = crate::BitReader<RXSSEL3_N_A>;
#[doc = "Slave Select 3 for Receive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXSSEL3_N_A {
    #[doc = "0: Not selected"]
    NOT_SELECTED = 0,
    #[doc = "1: Selected"]
    RXSSEL3_N_SELECTED = 1,
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
            false => RXSSEL3_N_A::NOT_SELECTED,
            true => RXSSEL3_N_A::RXSSEL3_N_SELECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_SELECTED`"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == RXSSEL3_N_A::NOT_SELECTED
    }
    #[doc = "Checks if the value of the field is `RXSSEL3_N_SELECTED`"]
    #[inline(always)]
    pub fn is_rxssel3_n_selected(&self) -> bool {
        *self == RXSSEL3_N_A::RXSSEL3_N_SELECTED
    }
}
#[doc = "Field `SOT` reader - Start of Transfer Flag"]
pub type SOT_R = crate::BitReader<SOT_A>;
#[doc = "Start of Transfer Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOT_A {
    #[doc = "0: Not active"]
    SOT_NOT_ACTIVE = 0,
    #[doc = "1: Active"]
    SOT_ACTIVE = 1,
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
            false => SOT_A::SOT_NOT_ACTIVE,
            true => SOT_A::SOT_ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `SOT_NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_sot_not_active(&self) -> bool {
        *self == SOT_A::SOT_NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `SOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_sot_active(&self) -> bool {
        *self == SOT_A::SOT_ACTIVE
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
#[doc = "FIFO Data Read with no FIFO Pop Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifordnopop](index.html) module"]
pub struct FIFORDNOPOP_SPEC;
impl crate::RegisterSpec for FIFORDNOPOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifordnopop::R](R) reader structure"]
impl crate::Readable for FIFORDNOPOP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFORDNOPOP to value 0"]
impl crate::Resettable for FIFORDNOPOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
