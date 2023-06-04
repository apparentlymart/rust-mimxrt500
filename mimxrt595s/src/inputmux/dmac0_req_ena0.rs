#[doc = "Register `DMAC0_REQ_ENA0` reader"]
pub struct R(crate::R<DMAC0_REQ_ENA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC0_REQ_ENA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC0_REQ_ENA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC0_REQ_ENA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC0_REQ_ENA0` writer"]
pub struct W(crate::W<DMAC0_REQ_ENA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC0_REQ_ENA0_SPEC>;
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
impl From<crate::W<DMAC0_REQ_ENA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC0_REQ_ENA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLEXCOMM0_RX` reader - FLEXCOMM0 RX enable"]
pub type FLEXCOMM0_RX_R = crate::BitReader<FLEXCOMM0_RX_A>;
#[doc = "FLEXCOMM0 RX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM0_RX_A {
    #[doc = "0: Disable"]
    FLEXCOMM0_RX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM0_RX_1 = 1,
}
impl From<FLEXCOMM0_RX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM0_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM0_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM0_RX_A {
        match self.bits {
            false => FLEXCOMM0_RX_A::FLEXCOMM0_RX_0,
            true => FLEXCOMM0_RX_A::FLEXCOMM0_RX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM0_RX_0`"]
    #[inline(always)]
    pub fn is_flexcomm0_rx_0(&self) -> bool {
        *self == FLEXCOMM0_RX_A::FLEXCOMM0_RX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM0_RX_1`"]
    #[inline(always)]
    pub fn is_flexcomm0_rx_1(&self) -> bool {
        *self == FLEXCOMM0_RX_A::FLEXCOMM0_RX_1
    }
}
#[doc = "Field `FLEXCOMM0_RX` writer - FLEXCOMM0 RX enable"]
pub type FLEXCOMM0_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM0_RX_A, O>;
impl<'a, const O: u8> FLEXCOMM0_RX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm0_rx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM0_RX_A::FLEXCOMM0_RX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm0_rx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM0_RX_A::FLEXCOMM0_RX_1)
    }
}
#[doc = "Field `FLEXCOMM0_TX` reader - FLEXCOMM0 TX enable"]
pub type FLEXCOMM0_TX_R = crate::BitReader<FLEXCOMM0_TX_A>;
#[doc = "FLEXCOMM0 TX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM0_TX_A {
    #[doc = "0: Disable"]
    FLEXCOMM0_TX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM0_TX_1 = 1,
}
impl From<FLEXCOMM0_TX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM0_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM0_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM0_TX_A {
        match self.bits {
            false => FLEXCOMM0_TX_A::FLEXCOMM0_TX_0,
            true => FLEXCOMM0_TX_A::FLEXCOMM0_TX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM0_TX_0`"]
    #[inline(always)]
    pub fn is_flexcomm0_tx_0(&self) -> bool {
        *self == FLEXCOMM0_TX_A::FLEXCOMM0_TX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM0_TX_1`"]
    #[inline(always)]
    pub fn is_flexcomm0_tx_1(&self) -> bool {
        *self == FLEXCOMM0_TX_A::FLEXCOMM0_TX_1
    }
}
#[doc = "Field `FLEXCOMM0_TX` writer - FLEXCOMM0 TX enable"]
pub type FLEXCOMM0_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM0_TX_A, O>;
impl<'a, const O: u8> FLEXCOMM0_TX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm0_tx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM0_TX_A::FLEXCOMM0_TX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm0_tx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM0_TX_A::FLEXCOMM0_TX_1)
    }
}
#[doc = "Field `FLEXCOMM1_RX` reader - FLEXCOMM1 RX enable"]
pub type FLEXCOMM1_RX_R = crate::BitReader<FLEXCOMM1_RX_A>;
#[doc = "FLEXCOMM1 RX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM1_RX_A {
    #[doc = "0: Disable"]
    FLEXCOMM1_RX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM1_RX_1 = 1,
}
impl From<FLEXCOMM1_RX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM1_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM1_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM1_RX_A {
        match self.bits {
            false => FLEXCOMM1_RX_A::FLEXCOMM1_RX_0,
            true => FLEXCOMM1_RX_A::FLEXCOMM1_RX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM1_RX_0`"]
    #[inline(always)]
    pub fn is_flexcomm1_rx_0(&self) -> bool {
        *self == FLEXCOMM1_RX_A::FLEXCOMM1_RX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM1_RX_1`"]
    #[inline(always)]
    pub fn is_flexcomm1_rx_1(&self) -> bool {
        *self == FLEXCOMM1_RX_A::FLEXCOMM1_RX_1
    }
}
#[doc = "Field `FLEXCOMM1_RX` writer - FLEXCOMM1 RX enable"]
pub type FLEXCOMM1_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM1_RX_A, O>;
impl<'a, const O: u8> FLEXCOMM1_RX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm1_rx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM1_RX_A::FLEXCOMM1_RX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm1_rx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM1_RX_A::FLEXCOMM1_RX_1)
    }
}
#[doc = "Field `FLEXCOMM1_TX` reader - FLEXCOMM1 TX enable"]
pub type FLEXCOMM1_TX_R = crate::BitReader<FLEXCOMM1_TX_A>;
#[doc = "FLEXCOMM1 TX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM1_TX_A {
    #[doc = "0: Disable"]
    FLEXCOMM1_TX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM1_TX_1 = 1,
}
impl From<FLEXCOMM1_TX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM1_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM1_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM1_TX_A {
        match self.bits {
            false => FLEXCOMM1_TX_A::FLEXCOMM1_TX_0,
            true => FLEXCOMM1_TX_A::FLEXCOMM1_TX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM1_TX_0`"]
    #[inline(always)]
    pub fn is_flexcomm1_tx_0(&self) -> bool {
        *self == FLEXCOMM1_TX_A::FLEXCOMM1_TX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM1_TX_1`"]
    #[inline(always)]
    pub fn is_flexcomm1_tx_1(&self) -> bool {
        *self == FLEXCOMM1_TX_A::FLEXCOMM1_TX_1
    }
}
#[doc = "Field `FLEXCOMM1_TX` writer - FLEXCOMM1 TX enable"]
pub type FLEXCOMM1_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM1_TX_A, O>;
impl<'a, const O: u8> FLEXCOMM1_TX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm1_tx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM1_TX_A::FLEXCOMM1_TX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm1_tx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM1_TX_A::FLEXCOMM1_TX_1)
    }
}
#[doc = "Field `FLEXCOMM2_RX` reader - FLEXCOMM2 RX enable"]
pub type FLEXCOMM2_RX_R = crate::BitReader<FLEXCOMM2_RX_A>;
#[doc = "FLEXCOMM2 RX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM2_RX_A {
    #[doc = "0: Disable"]
    FLEXCOMM2_RX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM2_RX_1 = 1,
}
impl From<FLEXCOMM2_RX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM2_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM2_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM2_RX_A {
        match self.bits {
            false => FLEXCOMM2_RX_A::FLEXCOMM2_RX_0,
            true => FLEXCOMM2_RX_A::FLEXCOMM2_RX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM2_RX_0`"]
    #[inline(always)]
    pub fn is_flexcomm2_rx_0(&self) -> bool {
        *self == FLEXCOMM2_RX_A::FLEXCOMM2_RX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM2_RX_1`"]
    #[inline(always)]
    pub fn is_flexcomm2_rx_1(&self) -> bool {
        *self == FLEXCOMM2_RX_A::FLEXCOMM2_RX_1
    }
}
#[doc = "Field `FLEXCOMM2_RX` writer - FLEXCOMM2 RX enable"]
pub type FLEXCOMM2_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM2_RX_A, O>;
impl<'a, const O: u8> FLEXCOMM2_RX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm2_rx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM2_RX_A::FLEXCOMM2_RX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm2_rx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM2_RX_A::FLEXCOMM2_RX_1)
    }
}
#[doc = "Field `FLEXCOMM2_TX` reader - FLEXCOMM2 TX enable"]
pub type FLEXCOMM2_TX_R = crate::BitReader<FLEXCOMM2_TX_A>;
#[doc = "FLEXCOMM2 TX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM2_TX_A {
    #[doc = "0: Disable"]
    FLEXCOMM2_TX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM2_TX_1 = 1,
}
impl From<FLEXCOMM2_TX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM2_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM2_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM2_TX_A {
        match self.bits {
            false => FLEXCOMM2_TX_A::FLEXCOMM2_TX_0,
            true => FLEXCOMM2_TX_A::FLEXCOMM2_TX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM2_TX_0`"]
    #[inline(always)]
    pub fn is_flexcomm2_tx_0(&self) -> bool {
        *self == FLEXCOMM2_TX_A::FLEXCOMM2_TX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM2_TX_1`"]
    #[inline(always)]
    pub fn is_flexcomm2_tx_1(&self) -> bool {
        *self == FLEXCOMM2_TX_A::FLEXCOMM2_TX_1
    }
}
#[doc = "Field `FLEXCOMM2_TX` writer - FLEXCOMM2 TX enable"]
pub type FLEXCOMM2_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM2_TX_A, O>;
impl<'a, const O: u8> FLEXCOMM2_TX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm2_tx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM2_TX_A::FLEXCOMM2_TX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm2_tx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM2_TX_A::FLEXCOMM2_TX_1)
    }
}
#[doc = "Field `FLEXCOMM3_RX` reader - FLEXCOMM3 RX enable"]
pub type FLEXCOMM3_RX_R = crate::BitReader<FLEXCOMM3_RX_A>;
#[doc = "FLEXCOMM3 RX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM3_RX_A {
    #[doc = "0: Disable"]
    FLEXCOMM3_RX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM3_RX_1 = 1,
}
impl From<FLEXCOMM3_RX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM3_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM3_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM3_RX_A {
        match self.bits {
            false => FLEXCOMM3_RX_A::FLEXCOMM3_RX_0,
            true => FLEXCOMM3_RX_A::FLEXCOMM3_RX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM3_RX_0`"]
    #[inline(always)]
    pub fn is_flexcomm3_rx_0(&self) -> bool {
        *self == FLEXCOMM3_RX_A::FLEXCOMM3_RX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM3_RX_1`"]
    #[inline(always)]
    pub fn is_flexcomm3_rx_1(&self) -> bool {
        *self == FLEXCOMM3_RX_A::FLEXCOMM3_RX_1
    }
}
#[doc = "Field `FLEXCOMM3_RX` writer - FLEXCOMM3 RX enable"]
pub type FLEXCOMM3_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM3_RX_A, O>;
impl<'a, const O: u8> FLEXCOMM3_RX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm3_rx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM3_RX_A::FLEXCOMM3_RX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm3_rx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM3_RX_A::FLEXCOMM3_RX_1)
    }
}
#[doc = "Field `FLEXCOMM3_TX` reader - FLEXCOMM3 TX enable"]
pub type FLEXCOMM3_TX_R = crate::BitReader<FLEXCOMM3_TX_A>;
#[doc = "FLEXCOMM3 TX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM3_TX_A {
    #[doc = "0: Disable"]
    FLEXCOMM3_TX_1 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM3_TX_0 = 1,
}
impl From<FLEXCOMM3_TX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM3_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM3_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM3_TX_A {
        match self.bits {
            false => FLEXCOMM3_TX_A::FLEXCOMM3_TX_1,
            true => FLEXCOMM3_TX_A::FLEXCOMM3_TX_0,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM3_TX_1`"]
    #[inline(always)]
    pub fn is_flexcomm3_tx_1(&self) -> bool {
        *self == FLEXCOMM3_TX_A::FLEXCOMM3_TX_1
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM3_TX_0`"]
    #[inline(always)]
    pub fn is_flexcomm3_tx_0(&self) -> bool {
        *self == FLEXCOMM3_TX_A::FLEXCOMM3_TX_0
    }
}
#[doc = "Field `FLEXCOMM3_TX` writer - FLEXCOMM3 TX enable"]
pub type FLEXCOMM3_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM3_TX_A, O>;
impl<'a, const O: u8> FLEXCOMM3_TX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm3_tx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM3_TX_A::FLEXCOMM3_TX_1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm3_tx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM3_TX_A::FLEXCOMM3_TX_0)
    }
}
#[doc = "Field `FLEXCOMM4_RX` reader - FLEXCOMM4 RX enable"]
pub type FLEXCOMM4_RX_R = crate::BitReader<FLEXCOMM4_RX_A>;
#[doc = "FLEXCOMM4 RX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM4_RX_A {
    #[doc = "0: Disable"]
    FLEXCOMM4_RX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM4_RX_1 = 1,
}
impl From<FLEXCOMM4_RX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM4_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM4_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM4_RX_A {
        match self.bits {
            false => FLEXCOMM4_RX_A::FLEXCOMM4_RX_0,
            true => FLEXCOMM4_RX_A::FLEXCOMM4_RX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM4_RX_0`"]
    #[inline(always)]
    pub fn is_flexcomm4_rx_0(&self) -> bool {
        *self == FLEXCOMM4_RX_A::FLEXCOMM4_RX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM4_RX_1`"]
    #[inline(always)]
    pub fn is_flexcomm4_rx_1(&self) -> bool {
        *self == FLEXCOMM4_RX_A::FLEXCOMM4_RX_1
    }
}
#[doc = "Field `FLEXCOMM4_RX` writer - FLEXCOMM4 RX enable"]
pub type FLEXCOMM4_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM4_RX_A, O>;
impl<'a, const O: u8> FLEXCOMM4_RX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm4_rx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM4_RX_A::FLEXCOMM4_RX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm4_rx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM4_RX_A::FLEXCOMM4_RX_1)
    }
}
#[doc = "Field `FLEXCOMM4_TX` reader - FLEXCOMM4 TX enable"]
pub type FLEXCOMM4_TX_R = crate::BitReader<FLEXCOMM4_TX_A>;
#[doc = "FLEXCOMM4 TX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM4_TX_A {
    #[doc = "0: Disable"]
    FLEXCOMM4_TX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM4_TX_1 = 1,
}
impl From<FLEXCOMM4_TX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM4_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM4_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM4_TX_A {
        match self.bits {
            false => FLEXCOMM4_TX_A::FLEXCOMM4_TX_0,
            true => FLEXCOMM4_TX_A::FLEXCOMM4_TX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM4_TX_0`"]
    #[inline(always)]
    pub fn is_flexcomm4_tx_0(&self) -> bool {
        *self == FLEXCOMM4_TX_A::FLEXCOMM4_TX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM4_TX_1`"]
    #[inline(always)]
    pub fn is_flexcomm4_tx_1(&self) -> bool {
        *self == FLEXCOMM4_TX_A::FLEXCOMM4_TX_1
    }
}
#[doc = "Field `FLEXCOMM4_TX` writer - FLEXCOMM4 TX enable"]
pub type FLEXCOMM4_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM4_TX_A, O>;
impl<'a, const O: u8> FLEXCOMM4_TX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm4_tx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM4_TX_A::FLEXCOMM4_TX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm4_tx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM4_TX_A::FLEXCOMM4_TX_1)
    }
}
#[doc = "Field `FLEXCOMM5_RX` reader - FLEXCOMM5 RX enable"]
pub type FLEXCOMM5_RX_R = crate::BitReader<FLEXCOMM5_RX_A>;
#[doc = "FLEXCOMM5 RX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM5_RX_A {
    #[doc = "0: Disable"]
    FLEXCOMM5_RX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM5_RX_1 = 1,
}
impl From<FLEXCOMM5_RX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM5_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM5_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM5_RX_A {
        match self.bits {
            false => FLEXCOMM5_RX_A::FLEXCOMM5_RX_0,
            true => FLEXCOMM5_RX_A::FLEXCOMM5_RX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM5_RX_0`"]
    #[inline(always)]
    pub fn is_flexcomm5_rx_0(&self) -> bool {
        *self == FLEXCOMM5_RX_A::FLEXCOMM5_RX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM5_RX_1`"]
    #[inline(always)]
    pub fn is_flexcomm5_rx_1(&self) -> bool {
        *self == FLEXCOMM5_RX_A::FLEXCOMM5_RX_1
    }
}
#[doc = "Field `FLEXCOMM5_RX` writer - FLEXCOMM5 RX enable"]
pub type FLEXCOMM5_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM5_RX_A, O>;
impl<'a, const O: u8> FLEXCOMM5_RX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm5_rx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM5_RX_A::FLEXCOMM5_RX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm5_rx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM5_RX_A::FLEXCOMM5_RX_1)
    }
}
#[doc = "Field `FLEXCOMM5_TX` reader - FLEXCOMM5 TX enable"]
pub type FLEXCOMM5_TX_R = crate::BitReader<FLEXCOMM5_TX_A>;
#[doc = "FLEXCOMM5 TX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM5_TX_A {
    #[doc = "0: Disable"]
    FLEXCOMM5_TX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM5_TX_1 = 1,
}
impl From<FLEXCOMM5_TX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM5_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM5_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM5_TX_A {
        match self.bits {
            false => FLEXCOMM5_TX_A::FLEXCOMM5_TX_0,
            true => FLEXCOMM5_TX_A::FLEXCOMM5_TX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM5_TX_0`"]
    #[inline(always)]
    pub fn is_flexcomm5_tx_0(&self) -> bool {
        *self == FLEXCOMM5_TX_A::FLEXCOMM5_TX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM5_TX_1`"]
    #[inline(always)]
    pub fn is_flexcomm5_tx_1(&self) -> bool {
        *self == FLEXCOMM5_TX_A::FLEXCOMM5_TX_1
    }
}
#[doc = "Field `FLEXCOMM5_TX` writer - FLEXCOMM5 TX enable"]
pub type FLEXCOMM5_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM5_TX_A, O>;
impl<'a, const O: u8> FLEXCOMM5_TX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm5_tx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM5_TX_A::FLEXCOMM5_TX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm5_tx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM5_TX_A::FLEXCOMM5_TX_1)
    }
}
#[doc = "Field `FLEXCOMM6_RX` reader - FLEXCOMM6 RX enable"]
pub type FLEXCOMM6_RX_R = crate::BitReader<FLEXCOMM6_RX_A>;
#[doc = "FLEXCOMM6 RX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM6_RX_A {
    #[doc = "0: Disable"]
    FLEXCOMM6_RX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM6_RX_1 = 1,
}
impl From<FLEXCOMM6_RX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM6_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM6_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM6_RX_A {
        match self.bits {
            false => FLEXCOMM6_RX_A::FLEXCOMM6_RX_0,
            true => FLEXCOMM6_RX_A::FLEXCOMM6_RX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM6_RX_0`"]
    #[inline(always)]
    pub fn is_flexcomm6_rx_0(&self) -> bool {
        *self == FLEXCOMM6_RX_A::FLEXCOMM6_RX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM6_RX_1`"]
    #[inline(always)]
    pub fn is_flexcomm6_rx_1(&self) -> bool {
        *self == FLEXCOMM6_RX_A::FLEXCOMM6_RX_1
    }
}
#[doc = "Field `FLEXCOMM6_RX` writer - FLEXCOMM6 RX enable"]
pub type FLEXCOMM6_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM6_RX_A, O>;
impl<'a, const O: u8> FLEXCOMM6_RX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm6_rx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM6_RX_A::FLEXCOMM6_RX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm6_rx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM6_RX_A::FLEXCOMM6_RX_1)
    }
}
#[doc = "Field `FLEXCOMM6_TX` reader - FLEXCOMM6 TX enable"]
pub type FLEXCOMM6_TX_R = crate::BitReader<FLEXCOMM6_TX_A>;
#[doc = "FLEXCOMM6 TX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM6_TX_A {
    #[doc = "0: Disable"]
    FLEXCOMM6_TX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM6_TX_1 = 1,
}
impl From<FLEXCOMM6_TX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM6_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM6_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM6_TX_A {
        match self.bits {
            false => FLEXCOMM6_TX_A::FLEXCOMM6_TX_0,
            true => FLEXCOMM6_TX_A::FLEXCOMM6_TX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM6_TX_0`"]
    #[inline(always)]
    pub fn is_flexcomm6_tx_0(&self) -> bool {
        *self == FLEXCOMM6_TX_A::FLEXCOMM6_TX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM6_TX_1`"]
    #[inline(always)]
    pub fn is_flexcomm6_tx_1(&self) -> bool {
        *self == FLEXCOMM6_TX_A::FLEXCOMM6_TX_1
    }
}
#[doc = "Field `FLEXCOMM6_TX` writer - FLEXCOMM6 TX enable"]
pub type FLEXCOMM6_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM6_TX_A, O>;
impl<'a, const O: u8> FLEXCOMM6_TX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm6_tx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM6_TX_A::FLEXCOMM6_TX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm6_tx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM6_TX_A::FLEXCOMM6_TX_1)
    }
}
#[doc = "Field `FLEXCOMM7_RX` reader - FLEXCOMM7 RX enable"]
pub type FLEXCOMM7_RX_R = crate::BitReader<FLEXCOMM7_RX_A>;
#[doc = "FLEXCOMM7 RX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM7_RX_A {
    #[doc = "0: Disable"]
    FLEXCOMM5_RX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM5_RX_1 = 1,
}
impl From<FLEXCOMM7_RX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM7_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM7_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM7_RX_A {
        match self.bits {
            false => FLEXCOMM7_RX_A::FLEXCOMM5_RX_0,
            true => FLEXCOMM7_RX_A::FLEXCOMM5_RX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM5_RX_0`"]
    #[inline(always)]
    pub fn is_flexcomm5_rx_0(&self) -> bool {
        *self == FLEXCOMM7_RX_A::FLEXCOMM5_RX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM5_RX_1`"]
    #[inline(always)]
    pub fn is_flexcomm5_rx_1(&self) -> bool {
        *self == FLEXCOMM7_RX_A::FLEXCOMM5_RX_1
    }
}
#[doc = "Field `FLEXCOMM7_RX` writer - FLEXCOMM7 RX enable"]
pub type FLEXCOMM7_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM7_RX_A, O>;
impl<'a, const O: u8> FLEXCOMM7_RX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm5_rx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM7_RX_A::FLEXCOMM5_RX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm5_rx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM7_RX_A::FLEXCOMM5_RX_1)
    }
}
#[doc = "Field `FLEXCOMM7_TX` reader - FLEXCOMM7 TX enable"]
pub type FLEXCOMM7_TX_R = crate::BitReader<FLEXCOMM7_TX_A>;
#[doc = "FLEXCOMM7 TX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM7_TX_A {
    #[doc = "0: Disable"]
    FLEXCOMM7_TX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM7_TX_1 = 1,
}
impl From<FLEXCOMM7_TX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM7_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM7_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM7_TX_A {
        match self.bits {
            false => FLEXCOMM7_TX_A::FLEXCOMM7_TX_0,
            true => FLEXCOMM7_TX_A::FLEXCOMM7_TX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM7_TX_0`"]
    #[inline(always)]
    pub fn is_flexcomm7_tx_0(&self) -> bool {
        *self == FLEXCOMM7_TX_A::FLEXCOMM7_TX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM7_TX_1`"]
    #[inline(always)]
    pub fn is_flexcomm7_tx_1(&self) -> bool {
        *self == FLEXCOMM7_TX_A::FLEXCOMM7_TX_1
    }
}
#[doc = "Field `FLEXCOMM7_TX` writer - FLEXCOMM7 TX enable"]
pub type FLEXCOMM7_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM7_TX_A, O>;
impl<'a, const O: u8> FLEXCOMM7_TX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm7_tx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM7_TX_A::FLEXCOMM7_TX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm7_tx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM7_TX_A::FLEXCOMM7_TX_1)
    }
}
#[doc = "Field `DMIC0_CH0_FLEXCOMM8_RX_DMA` reader - DMIC0 channel 0 / FLEXCOMM8 RX DMA enable"]
pub type DMIC0_CH0_FLEXCOMM8_RX_DMA_R = crate::BitReader<DMIC0_CH0_FLEXCOMM8_RX_DMA_A>;
#[doc = "DMIC0 channel 0 / FLEXCOMM8 RX DMA enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMIC0_CH0_FLEXCOMM8_RX_DMA_A {
    #[doc = "0: Disable"]
    DMIC0_CH0_FLEXCOMM8_RX_DMA_0 = 0,
    #[doc = "1: Enable"]
    DMIC0_CH0_FLEXCOMM8_RX_DMA_1 = 1,
}
impl From<DMIC0_CH0_FLEXCOMM8_RX_DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_CH0_FLEXCOMM8_RX_DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMIC0_CH0_FLEXCOMM8_RX_DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_CH0_FLEXCOMM8_RX_DMA_A {
        match self.bits {
            false => DMIC0_CH0_FLEXCOMM8_RX_DMA_A::DMIC0_CH0_FLEXCOMM8_RX_DMA_0,
            true => DMIC0_CH0_FLEXCOMM8_RX_DMA_A::DMIC0_CH0_FLEXCOMM8_RX_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_CH0_FLEXCOMM8_RX_DMA_0`"]
    #[inline(always)]
    pub fn is_dmic0_ch0_flexcomm8_rx_dma_0(&self) -> bool {
        *self == DMIC0_CH0_FLEXCOMM8_RX_DMA_A::DMIC0_CH0_FLEXCOMM8_RX_DMA_0
    }
    #[doc = "Checks if the value of the field is `DMIC0_CH0_FLEXCOMM8_RX_DMA_1`"]
    #[inline(always)]
    pub fn is_dmic0_ch0_flexcomm8_rx_dma_1(&self) -> bool {
        *self == DMIC0_CH0_FLEXCOMM8_RX_DMA_A::DMIC0_CH0_FLEXCOMM8_RX_DMA_1
    }
}
#[doc = "Field `DMIC0_CH0_FLEXCOMM8_RX_DMA` writer - DMIC0 channel 0 / FLEXCOMM8 RX DMA enable"]
pub type DMIC0_CH0_FLEXCOMM8_RX_DMA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, DMIC0_CH0_FLEXCOMM8_RX_DMA_A, O>;
impl<'a, const O: u8> DMIC0_CH0_FLEXCOMM8_RX_DMA_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dmic0_ch0_flexcomm8_rx_dma_0(self) -> &'a mut W {
        self.variant(DMIC0_CH0_FLEXCOMM8_RX_DMA_A::DMIC0_CH0_FLEXCOMM8_RX_DMA_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn dmic0_ch0_flexcomm8_rx_dma_1(self) -> &'a mut W {
        self.variant(DMIC0_CH0_FLEXCOMM8_RX_DMA_A::DMIC0_CH0_FLEXCOMM8_RX_DMA_1)
    }
}
#[doc = "Field `DMIC0_CH1_FLEXCOMM8_TX_DMA` reader - DMIC0 channel 1 / FLEXCOMM8 TX DMA enable"]
pub type DMIC0_CH1_FLEXCOMM8_TX_DMA_R = crate::BitReader<DMIC0_CH1_FLEXCOMM8_TX_DMA_A>;
#[doc = "DMIC0 channel 1 / FLEXCOMM8 TX DMA enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMIC0_CH1_FLEXCOMM8_TX_DMA_A {
    #[doc = "0: Disable"]
    DMIC0_CH1_FLEXCOMM8_TX_DMA_0 = 0,
    #[doc = "1: Enable"]
    DMIC0_CH1_FLEXCOMM8_TX_DMA_1 = 1,
}
impl From<DMIC0_CH1_FLEXCOMM8_TX_DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_CH1_FLEXCOMM8_TX_DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMIC0_CH1_FLEXCOMM8_TX_DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_CH1_FLEXCOMM8_TX_DMA_A {
        match self.bits {
            false => DMIC0_CH1_FLEXCOMM8_TX_DMA_A::DMIC0_CH1_FLEXCOMM8_TX_DMA_0,
            true => DMIC0_CH1_FLEXCOMM8_TX_DMA_A::DMIC0_CH1_FLEXCOMM8_TX_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_CH1_FLEXCOMM8_TX_DMA_0`"]
    #[inline(always)]
    pub fn is_dmic0_ch1_flexcomm8_tx_dma_0(&self) -> bool {
        *self == DMIC0_CH1_FLEXCOMM8_TX_DMA_A::DMIC0_CH1_FLEXCOMM8_TX_DMA_0
    }
    #[doc = "Checks if the value of the field is `DMIC0_CH1_FLEXCOMM8_TX_DMA_1`"]
    #[inline(always)]
    pub fn is_dmic0_ch1_flexcomm8_tx_dma_1(&self) -> bool {
        *self == DMIC0_CH1_FLEXCOMM8_TX_DMA_A::DMIC0_CH1_FLEXCOMM8_TX_DMA_1
    }
}
#[doc = "Field `DMIC0_CH1_FLEXCOMM8_TX_DMA` writer - DMIC0 channel 1 / FLEXCOMM8 TX DMA enable"]
pub type DMIC0_CH1_FLEXCOMM8_TX_DMA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, DMIC0_CH1_FLEXCOMM8_TX_DMA_A, O>;
impl<'a, const O: u8> DMIC0_CH1_FLEXCOMM8_TX_DMA_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dmic0_ch1_flexcomm8_tx_dma_0(self) -> &'a mut W {
        self.variant(DMIC0_CH1_FLEXCOMM8_TX_DMA_A::DMIC0_CH1_FLEXCOMM8_TX_DMA_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn dmic0_ch1_flexcomm8_tx_dma_1(self) -> &'a mut W {
        self.variant(DMIC0_CH1_FLEXCOMM8_TX_DMA_A::DMIC0_CH1_FLEXCOMM8_TX_DMA_1)
    }
}
#[doc = "Field `DMIC0_CH2_FLEXCOMM9_RX_DMA` reader - DMIC0 channel 2 / FLEXCOMM9 RX DMA enable"]
pub type DMIC0_CH2_FLEXCOMM9_RX_DMA_R = crate::BitReader<DMIC0_CH2_FLEXCOMM9_RX_DMA_A>;
#[doc = "DMIC0 channel 2 / FLEXCOMM9 RX DMA enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMIC0_CH2_FLEXCOMM9_RX_DMA_A {
    #[doc = "0: Disable"]
    DMIC0_CH2_FLEXCOMM9_RX_DMA_0 = 0,
    #[doc = "1: Enable"]
    DMIC0_CH2_FLEXCOMM9_RX_DMA_1 = 1,
}
impl From<DMIC0_CH2_FLEXCOMM9_RX_DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_CH2_FLEXCOMM9_RX_DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMIC0_CH2_FLEXCOMM9_RX_DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_CH2_FLEXCOMM9_RX_DMA_A {
        match self.bits {
            false => DMIC0_CH2_FLEXCOMM9_RX_DMA_A::DMIC0_CH2_FLEXCOMM9_RX_DMA_0,
            true => DMIC0_CH2_FLEXCOMM9_RX_DMA_A::DMIC0_CH2_FLEXCOMM9_RX_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_CH2_FLEXCOMM9_RX_DMA_0`"]
    #[inline(always)]
    pub fn is_dmic0_ch2_flexcomm9_rx_dma_0(&self) -> bool {
        *self == DMIC0_CH2_FLEXCOMM9_RX_DMA_A::DMIC0_CH2_FLEXCOMM9_RX_DMA_0
    }
    #[doc = "Checks if the value of the field is `DMIC0_CH2_FLEXCOMM9_RX_DMA_1`"]
    #[inline(always)]
    pub fn is_dmic0_ch2_flexcomm9_rx_dma_1(&self) -> bool {
        *self == DMIC0_CH2_FLEXCOMM9_RX_DMA_A::DMIC0_CH2_FLEXCOMM9_RX_DMA_1
    }
}
#[doc = "Field `DMIC0_CH2_FLEXCOMM9_RX_DMA` writer - DMIC0 channel 2 / FLEXCOMM9 RX DMA enable"]
pub type DMIC0_CH2_FLEXCOMM9_RX_DMA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, DMIC0_CH2_FLEXCOMM9_RX_DMA_A, O>;
impl<'a, const O: u8> DMIC0_CH2_FLEXCOMM9_RX_DMA_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dmic0_ch2_flexcomm9_rx_dma_0(self) -> &'a mut W {
        self.variant(DMIC0_CH2_FLEXCOMM9_RX_DMA_A::DMIC0_CH2_FLEXCOMM9_RX_DMA_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn dmic0_ch2_flexcomm9_rx_dma_1(self) -> &'a mut W {
        self.variant(DMIC0_CH2_FLEXCOMM9_RX_DMA_A::DMIC0_CH2_FLEXCOMM9_RX_DMA_1)
    }
}
#[doc = "Field `DMIC0_CH3_FLEXCOMM9_TX_DMA` reader - DMIC0 channel 3 / FLEXCOMM9 TX DMA enable"]
pub type DMIC0_CH3_FLEXCOMM9_TX_DMA_R = crate::BitReader<DMIC0_CH3_FLEXCOMM9_TX_DMA_A>;
#[doc = "DMIC0 channel 3 / FLEXCOMM9 TX DMA enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMIC0_CH3_FLEXCOMM9_TX_DMA_A {
    #[doc = "0: Disable"]
    DMIC0_CH3_FLEXCOMM9_TX_DMA_0 = 0,
    #[doc = "1: Enable"]
    DMIC0_CH3_FLEXCOMM9_TX_DMA_1 = 1,
}
impl From<DMIC0_CH3_FLEXCOMM9_TX_DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_CH3_FLEXCOMM9_TX_DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMIC0_CH3_FLEXCOMM9_TX_DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_CH3_FLEXCOMM9_TX_DMA_A {
        match self.bits {
            false => DMIC0_CH3_FLEXCOMM9_TX_DMA_A::DMIC0_CH3_FLEXCOMM9_TX_DMA_0,
            true => DMIC0_CH3_FLEXCOMM9_TX_DMA_A::DMIC0_CH3_FLEXCOMM9_TX_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_CH3_FLEXCOMM9_TX_DMA_0`"]
    #[inline(always)]
    pub fn is_dmic0_ch3_flexcomm9_tx_dma_0(&self) -> bool {
        *self == DMIC0_CH3_FLEXCOMM9_TX_DMA_A::DMIC0_CH3_FLEXCOMM9_TX_DMA_0
    }
    #[doc = "Checks if the value of the field is `DMIC0_CH3_FLEXCOMM9_TX_DMA_1`"]
    #[inline(always)]
    pub fn is_dmic0_ch3_flexcomm9_tx_dma_1(&self) -> bool {
        *self == DMIC0_CH3_FLEXCOMM9_TX_DMA_A::DMIC0_CH3_FLEXCOMM9_TX_DMA_1
    }
}
#[doc = "Field `DMIC0_CH3_FLEXCOMM9_TX_DMA` writer - DMIC0 channel 3 / FLEXCOMM9 TX DMA enable"]
pub type DMIC0_CH3_FLEXCOMM9_TX_DMA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, DMIC0_CH3_FLEXCOMM9_TX_DMA_A, O>;
impl<'a, const O: u8> DMIC0_CH3_FLEXCOMM9_TX_DMA_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dmic0_ch3_flexcomm9_tx_dma_0(self) -> &'a mut W {
        self.variant(DMIC0_CH3_FLEXCOMM9_TX_DMA_A::DMIC0_CH3_FLEXCOMM9_TX_DMA_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn dmic0_ch3_flexcomm9_tx_dma_1(self) -> &'a mut W {
        self.variant(DMIC0_CH3_FLEXCOMM9_TX_DMA_A::DMIC0_CH3_FLEXCOMM9_TX_DMA_1)
    }
}
#[doc = "Field `DMIC0_CH4_FLEXCOMM10_RX_DMA` reader - DMIC0 channel 4 / FLEXCOMM10 RX DMA enable"]
pub type DMIC0_CH4_FLEXCOMM10_RX_DMA_R = crate::BitReader<DMIC0_CH4_FLEXCOMM10_RX_DMA_A>;
#[doc = "DMIC0 channel 4 / FLEXCOMM10 RX DMA enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMIC0_CH4_FLEXCOMM10_RX_DMA_A {
    #[doc = "0: Disable"]
    DMIC0_CH4_FLEXCOMM10_RX_DMA_0 = 0,
    #[doc = "1: Enable"]
    DMIC0_CH4_FLEXCOMM10_RX_DMA_1 = 1,
}
impl From<DMIC0_CH4_FLEXCOMM10_RX_DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_CH4_FLEXCOMM10_RX_DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMIC0_CH4_FLEXCOMM10_RX_DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_CH4_FLEXCOMM10_RX_DMA_A {
        match self.bits {
            false => DMIC0_CH4_FLEXCOMM10_RX_DMA_A::DMIC0_CH4_FLEXCOMM10_RX_DMA_0,
            true => DMIC0_CH4_FLEXCOMM10_RX_DMA_A::DMIC0_CH4_FLEXCOMM10_RX_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_CH4_FLEXCOMM10_RX_DMA_0`"]
    #[inline(always)]
    pub fn is_dmic0_ch4_flexcomm10_rx_dma_0(&self) -> bool {
        *self == DMIC0_CH4_FLEXCOMM10_RX_DMA_A::DMIC0_CH4_FLEXCOMM10_RX_DMA_0
    }
    #[doc = "Checks if the value of the field is `DMIC0_CH4_FLEXCOMM10_RX_DMA_1`"]
    #[inline(always)]
    pub fn is_dmic0_ch4_flexcomm10_rx_dma_1(&self) -> bool {
        *self == DMIC0_CH4_FLEXCOMM10_RX_DMA_A::DMIC0_CH4_FLEXCOMM10_RX_DMA_1
    }
}
#[doc = "Field `DMIC0_CH4_FLEXCOMM10_RX_DMA` writer - DMIC0 channel 4 / FLEXCOMM10 RX DMA enable"]
pub type DMIC0_CH4_FLEXCOMM10_RX_DMA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, DMIC0_CH4_FLEXCOMM10_RX_DMA_A, O>;
impl<'a, const O: u8> DMIC0_CH4_FLEXCOMM10_RX_DMA_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dmic0_ch4_flexcomm10_rx_dma_0(self) -> &'a mut W {
        self.variant(DMIC0_CH4_FLEXCOMM10_RX_DMA_A::DMIC0_CH4_FLEXCOMM10_RX_DMA_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn dmic0_ch4_flexcomm10_rx_dma_1(self) -> &'a mut W {
        self.variant(DMIC0_CH4_FLEXCOMM10_RX_DMA_A::DMIC0_CH4_FLEXCOMM10_RX_DMA_1)
    }
}
#[doc = "Field `DMIC0_CH5_FLEXCOMM10_TX_DMA` reader - DMIC0 channel 5 / FLEXCOMM10 TX DMA enable"]
pub type DMIC0_CH5_FLEXCOMM10_TX_DMA_R = crate::BitReader<DMIC0_CH5_FLEXCOMM10_TX_DMA_A>;
#[doc = "DMIC0 channel 5 / FLEXCOMM10 TX DMA enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMIC0_CH5_FLEXCOMM10_TX_DMA_A {
    #[doc = "0: Disable"]
    DMIC0_CH5_FLEXCOMM10_TX_DMA_0 = 0,
    #[doc = "1: Enable"]
    DMIC0_CH5_FLEXCOMM10_TX_DMA_1 = 1,
}
impl From<DMIC0_CH5_FLEXCOMM10_TX_DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_CH5_FLEXCOMM10_TX_DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMIC0_CH5_FLEXCOMM10_TX_DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_CH5_FLEXCOMM10_TX_DMA_A {
        match self.bits {
            false => DMIC0_CH5_FLEXCOMM10_TX_DMA_A::DMIC0_CH5_FLEXCOMM10_TX_DMA_0,
            true => DMIC0_CH5_FLEXCOMM10_TX_DMA_A::DMIC0_CH5_FLEXCOMM10_TX_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_CH5_FLEXCOMM10_TX_DMA_0`"]
    #[inline(always)]
    pub fn is_dmic0_ch5_flexcomm10_tx_dma_0(&self) -> bool {
        *self == DMIC0_CH5_FLEXCOMM10_TX_DMA_A::DMIC0_CH5_FLEXCOMM10_TX_DMA_0
    }
    #[doc = "Checks if the value of the field is `DMIC0_CH5_FLEXCOMM10_TX_DMA_1`"]
    #[inline(always)]
    pub fn is_dmic0_ch5_flexcomm10_tx_dma_1(&self) -> bool {
        *self == DMIC0_CH5_FLEXCOMM10_TX_DMA_A::DMIC0_CH5_FLEXCOMM10_TX_DMA_1
    }
}
#[doc = "Field `DMIC0_CH5_FLEXCOMM10_TX_DMA` writer - DMIC0 channel 5 / FLEXCOMM10 TX DMA enable"]
pub type DMIC0_CH5_FLEXCOMM10_TX_DMA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, DMIC0_CH5_FLEXCOMM10_TX_DMA_A, O>;
impl<'a, const O: u8> DMIC0_CH5_FLEXCOMM10_TX_DMA_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dmic0_ch5_flexcomm10_tx_dma_0(self) -> &'a mut W {
        self.variant(DMIC0_CH5_FLEXCOMM10_TX_DMA_A::DMIC0_CH5_FLEXCOMM10_TX_DMA_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn dmic0_ch5_flexcomm10_tx_dma_1(self) -> &'a mut W {
        self.variant(DMIC0_CH5_FLEXCOMM10_TX_DMA_A::DMIC0_CH5_FLEXCOMM10_TX_DMA_1)
    }
}
#[doc = "Field `DMIC0_CH6_FLEXCOMM13_RX_DMA` reader - DMIC0 channel 6 / FLEXCOMM13 RX DMA enable"]
pub type DMIC0_CH6_FLEXCOMM13_RX_DMA_R = crate::BitReader<DMIC0_CH6_FLEXCOMM13_RX_DMA_A>;
#[doc = "DMIC0 channel 6 / FLEXCOMM13 RX DMA enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMIC0_CH6_FLEXCOMM13_RX_DMA_A {
    #[doc = "0: Disable"]
    DMIC0_CH6_FLEXCOMM13_RX_DMA_0 = 0,
    #[doc = "1: Enable"]
    DMIC0_CH6_FLEXCOMM13_RX_DMA_1 = 1,
}
impl From<DMIC0_CH6_FLEXCOMM13_RX_DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_CH6_FLEXCOMM13_RX_DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMIC0_CH6_FLEXCOMM13_RX_DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_CH6_FLEXCOMM13_RX_DMA_A {
        match self.bits {
            false => DMIC0_CH6_FLEXCOMM13_RX_DMA_A::DMIC0_CH6_FLEXCOMM13_RX_DMA_0,
            true => DMIC0_CH6_FLEXCOMM13_RX_DMA_A::DMIC0_CH6_FLEXCOMM13_RX_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_CH6_FLEXCOMM13_RX_DMA_0`"]
    #[inline(always)]
    pub fn is_dmic0_ch6_flexcomm13_rx_dma_0(&self) -> bool {
        *self == DMIC0_CH6_FLEXCOMM13_RX_DMA_A::DMIC0_CH6_FLEXCOMM13_RX_DMA_0
    }
    #[doc = "Checks if the value of the field is `DMIC0_CH6_FLEXCOMM13_RX_DMA_1`"]
    #[inline(always)]
    pub fn is_dmic0_ch6_flexcomm13_rx_dma_1(&self) -> bool {
        *self == DMIC0_CH6_FLEXCOMM13_RX_DMA_A::DMIC0_CH6_FLEXCOMM13_RX_DMA_1
    }
}
#[doc = "Field `DMIC0_CH6_FLEXCOMM13_RX_DMA` writer - DMIC0 channel 6 / FLEXCOMM13 RX DMA enable"]
pub type DMIC0_CH6_FLEXCOMM13_RX_DMA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, DMIC0_CH6_FLEXCOMM13_RX_DMA_A, O>;
impl<'a, const O: u8> DMIC0_CH6_FLEXCOMM13_RX_DMA_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dmic0_ch6_flexcomm13_rx_dma_0(self) -> &'a mut W {
        self.variant(DMIC0_CH6_FLEXCOMM13_RX_DMA_A::DMIC0_CH6_FLEXCOMM13_RX_DMA_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn dmic0_ch6_flexcomm13_rx_dma_1(self) -> &'a mut W {
        self.variant(DMIC0_CH6_FLEXCOMM13_RX_DMA_A::DMIC0_CH6_FLEXCOMM13_RX_DMA_1)
    }
}
#[doc = "Field `DMIC0_CH7_FLEXCOMM13_TX_DMA` reader - DMIC0 channel 7 / FLEXCOMM13 TX DMA enable"]
pub type DMIC0_CH7_FLEXCOMM13_TX_DMA_R = crate::BitReader<DMIC0_CH7_FLEXCOMM13_TX_DMA_A>;
#[doc = "DMIC0 channel 7 / FLEXCOMM13 TX DMA enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMIC0_CH7_FLEXCOMM13_TX_DMA_A {
    #[doc = "0: Disable"]
    DMIC0_CH7_FLEXCOMM13_TX_DMA_0 = 0,
    #[doc = "1: Enable"]
    DMIC0_CH7_FLEXCOMM13_TX_DMA_1 = 1,
}
impl From<DMIC0_CH7_FLEXCOMM13_TX_DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMIC0_CH7_FLEXCOMM13_TX_DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMIC0_CH7_FLEXCOMM13_TX_DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIC0_CH7_FLEXCOMM13_TX_DMA_A {
        match self.bits {
            false => DMIC0_CH7_FLEXCOMM13_TX_DMA_A::DMIC0_CH7_FLEXCOMM13_TX_DMA_0,
            true => DMIC0_CH7_FLEXCOMM13_TX_DMA_A::DMIC0_CH7_FLEXCOMM13_TX_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMIC0_CH7_FLEXCOMM13_TX_DMA_0`"]
    #[inline(always)]
    pub fn is_dmic0_ch7_flexcomm13_tx_dma_0(&self) -> bool {
        *self == DMIC0_CH7_FLEXCOMM13_TX_DMA_A::DMIC0_CH7_FLEXCOMM13_TX_DMA_0
    }
    #[doc = "Checks if the value of the field is `DMIC0_CH7_FLEXCOMM13_TX_DMA_1`"]
    #[inline(always)]
    pub fn is_dmic0_ch7_flexcomm13_tx_dma_1(&self) -> bool {
        *self == DMIC0_CH7_FLEXCOMM13_TX_DMA_A::DMIC0_CH7_FLEXCOMM13_TX_DMA_1
    }
}
#[doc = "Field `DMIC0_CH7_FLEXCOMM13_TX_DMA` writer - DMIC0 channel 7 / FLEXCOMM13 TX DMA enable"]
pub type DMIC0_CH7_FLEXCOMM13_TX_DMA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, DMIC0_CH7_FLEXCOMM13_TX_DMA_A, O>;
impl<'a, const O: u8> DMIC0_CH7_FLEXCOMM13_TX_DMA_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dmic0_ch7_flexcomm13_tx_dma_0(self) -> &'a mut W {
        self.variant(DMIC0_CH7_FLEXCOMM13_TX_DMA_A::DMIC0_CH7_FLEXCOMM13_TX_DMA_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn dmic0_ch7_flexcomm13_tx_dma_1(self) -> &'a mut W {
        self.variant(DMIC0_CH7_FLEXCOMM13_TX_DMA_A::DMIC0_CH7_FLEXCOMM13_TX_DMA_1)
    }
}
#[doc = "Field `I3C0_RX` reader - I3C RX enable"]
pub type I3C0_RX_R = crate::BitReader<I3C0_RX_A>;
#[doc = "I3C RX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C0_RX_A {
    #[doc = "0: Disable"]
    I3C0_RX_0 = 0,
    #[doc = "1: Enable"]
    I3C0_RX_1 = 1,
}
impl From<I3C0_RX_A> for bool {
    #[inline(always)]
    fn from(variant: I3C0_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl I3C0_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I3C0_RX_A {
        match self.bits {
            false => I3C0_RX_A::I3C0_RX_0,
            true => I3C0_RX_A::I3C0_RX_1,
        }
    }
    #[doc = "Checks if the value of the field is `I3C0_RX_0`"]
    #[inline(always)]
    pub fn is_i3c0_rx_0(&self) -> bool {
        *self == I3C0_RX_A::I3C0_RX_0
    }
    #[doc = "Checks if the value of the field is `I3C0_RX_1`"]
    #[inline(always)]
    pub fn is_i3c0_rx_1(&self) -> bool {
        *self == I3C0_RX_A::I3C0_RX_1
    }
}
#[doc = "Field `I3C0_RX` writer - I3C RX enable"]
pub type I3C0_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, I3C0_RX_A, O>;
impl<'a, const O: u8> I3C0_RX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn i3c0_rx_0(self) -> &'a mut W {
        self.variant(I3C0_RX_A::I3C0_RX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn i3c0_rx_1(self) -> &'a mut W {
        self.variant(I3C0_RX_A::I3C0_RX_1)
    }
}
#[doc = "Field `I3C0_TX` reader - I3C TX enable"]
pub type I3C0_TX_R = crate::BitReader<I3C0_TX_A>;
#[doc = "I3C TX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C0_TX_A {
    #[doc = "0: Disable"]
    I3C0_TX_0 = 0,
    #[doc = "1: Enable"]
    I3C0_TX_1 = 1,
}
impl From<I3C0_TX_A> for bool {
    #[inline(always)]
    fn from(variant: I3C0_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl I3C0_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I3C0_TX_A {
        match self.bits {
            false => I3C0_TX_A::I3C0_TX_0,
            true => I3C0_TX_A::I3C0_TX_1,
        }
    }
    #[doc = "Checks if the value of the field is `I3C0_TX_0`"]
    #[inline(always)]
    pub fn is_i3c0_tx_0(&self) -> bool {
        *self == I3C0_TX_A::I3C0_TX_0
    }
    #[doc = "Checks if the value of the field is `I3C0_TX_1`"]
    #[inline(always)]
    pub fn is_i3c0_tx_1(&self) -> bool {
        *self == I3C0_TX_A::I3C0_TX_1
    }
}
#[doc = "Field `I3C0_TX` writer - I3C TX enable"]
pub type I3C0_TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, I3C0_TX_A, O>;
impl<'a, const O: u8> I3C0_TX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn i3c0_tx_0(self) -> &'a mut W {
        self.variant(I3C0_TX_A::I3C0_TX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn i3c0_tx_1(self) -> &'a mut W {
        self.variant(I3C0_TX_A::I3C0_TX_1)
    }
}
#[doc = "Field `FLEXCOMM14_RX` reader - FLEXCOMM14 RX enable"]
pub type FLEXCOMM14_RX_R = crate::BitReader<FLEXCOMM14_RX_A>;
#[doc = "FLEXCOMM14 RX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM14_RX_A {
    #[doc = "0: Disable"]
    FLEXCOMM14_RX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM14_RX_1 = 1,
}
impl From<FLEXCOMM14_RX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM14_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM14_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM14_RX_A {
        match self.bits {
            false => FLEXCOMM14_RX_A::FLEXCOMM14_RX_0,
            true => FLEXCOMM14_RX_A::FLEXCOMM14_RX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM14_RX_0`"]
    #[inline(always)]
    pub fn is_flexcomm14_rx_0(&self) -> bool {
        *self == FLEXCOMM14_RX_A::FLEXCOMM14_RX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM14_RX_1`"]
    #[inline(always)]
    pub fn is_flexcomm14_rx_1(&self) -> bool {
        *self == FLEXCOMM14_RX_A::FLEXCOMM14_RX_1
    }
}
#[doc = "Field `FLEXCOMM14_RX` writer - FLEXCOMM14 RX enable"]
pub type FLEXCOMM14_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM14_RX_A, O>;
impl<'a, const O: u8> FLEXCOMM14_RX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm14_rx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM14_RX_A::FLEXCOMM14_RX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm14_rx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM14_RX_A::FLEXCOMM14_RX_1)
    }
}
#[doc = "Field `FLEXCOMM14_TX` reader - FLEXCOMM14 TX enable"]
pub type FLEXCOMM14_TX_R = crate::BitReader<FLEXCOMM14_TX_A>;
#[doc = "FLEXCOMM14 TX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM14_TX_A {
    #[doc = "0: Disable"]
    FLEXCOMM14_TX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM14_TX_1 = 1,
}
impl From<FLEXCOMM14_TX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM14_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM14_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM14_TX_A {
        match self.bits {
            false => FLEXCOMM14_TX_A::FLEXCOMM14_TX_0,
            true => FLEXCOMM14_TX_A::FLEXCOMM14_TX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM14_TX_0`"]
    #[inline(always)]
    pub fn is_flexcomm14_tx_0(&self) -> bool {
        *self == FLEXCOMM14_TX_A::FLEXCOMM14_TX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM14_TX_1`"]
    #[inline(always)]
    pub fn is_flexcomm14_tx_1(&self) -> bool {
        *self == FLEXCOMM14_TX_A::FLEXCOMM14_TX_1
    }
}
#[doc = "Field `FLEXCOMM14_TX` writer - FLEXCOMM14 TX enable"]
pub type FLEXCOMM14_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM14_TX_A, O>;
impl<'a, const O: u8> FLEXCOMM14_TX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm14_tx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM14_TX_A::FLEXCOMM14_TX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm14_tx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM14_TX_A::FLEXCOMM14_TX_1)
    }
}
#[doc = "Field `FLEXCOMM16_RX` reader - FLEXCOMM16 RX enable"]
pub type FLEXCOMM16_RX_R = crate::BitReader<FLEXCOMM16_RX_A>;
#[doc = "FLEXCOMM16 RX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM16_RX_A {
    #[doc = "0: Disable"]
    FLEXCOMM16_RX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM16_RX_1 = 1,
}
impl From<FLEXCOMM16_RX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM16_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM16_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM16_RX_A {
        match self.bits {
            false => FLEXCOMM16_RX_A::FLEXCOMM16_RX_0,
            true => FLEXCOMM16_RX_A::FLEXCOMM16_RX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM16_RX_0`"]
    #[inline(always)]
    pub fn is_flexcomm16_rx_0(&self) -> bool {
        *self == FLEXCOMM16_RX_A::FLEXCOMM16_RX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM16_RX_1`"]
    #[inline(always)]
    pub fn is_flexcomm16_rx_1(&self) -> bool {
        *self == FLEXCOMM16_RX_A::FLEXCOMM16_RX_1
    }
}
#[doc = "Field `FLEXCOMM16_RX` writer - FLEXCOMM16 RX enable"]
pub type FLEXCOMM16_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM16_RX_A, O>;
impl<'a, const O: u8> FLEXCOMM16_RX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm16_rx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM16_RX_A::FLEXCOMM16_RX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm16_rx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM16_RX_A::FLEXCOMM16_RX_1)
    }
}
#[doc = "Field `FLEXCOMM16_TX` reader - FLEXCOMM16 TX enable"]
pub type FLEXCOMM16_TX_R = crate::BitReader<FLEXCOMM16_TX_A>;
#[doc = "FLEXCOMM16 TX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCOMM16_TX_A {
    #[doc = "0: Disable"]
    FLEXCOMM16_TX_0 = 0,
    #[doc = "1: Enable"]
    FLEXCOMM16_TX_1 = 1,
}
impl From<FLEXCOMM16_TX_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM16_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCOMM16_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM16_TX_A {
        match self.bits {
            false => FLEXCOMM16_TX_A::FLEXCOMM16_TX_0,
            true => FLEXCOMM16_TX_A::FLEXCOMM16_TX_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM16_TX_0`"]
    #[inline(always)]
    pub fn is_flexcomm16_tx_0(&self) -> bool {
        *self == FLEXCOMM16_TX_A::FLEXCOMM16_TX_0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM16_TX_1`"]
    #[inline(always)]
    pub fn is_flexcomm16_tx_1(&self) -> bool {
        *self == FLEXCOMM16_TX_A::FLEXCOMM16_TX_1
    }
}
#[doc = "Field `FLEXCOMM16_TX` writer - FLEXCOMM16 TX enable"]
pub type FLEXCOMM16_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, FLEXCOMM16_TX_A, O>;
impl<'a, const O: u8> FLEXCOMM16_TX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn flexcomm16_tx_0(self) -> &'a mut W {
        self.variant(FLEXCOMM16_TX_A::FLEXCOMM16_TX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn flexcomm16_tx_1(self) -> &'a mut W {
        self.variant(FLEXCOMM16_TX_A::FLEXCOMM16_TX_1)
    }
}
#[doc = "Field `I3C1_RX` reader - I3C1_RX enable"]
pub type I3C1_RX_R = crate::BitReader<I3C1_RX_A>;
#[doc = "I3C1_RX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C1_RX_A {
    #[doc = "0: Disable"]
    I3C1_RX_0 = 0,
    #[doc = "1: Enable"]
    I3C1_RX_1 = 1,
}
impl From<I3C1_RX_A> for bool {
    #[inline(always)]
    fn from(variant: I3C1_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl I3C1_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I3C1_RX_A {
        match self.bits {
            false => I3C1_RX_A::I3C1_RX_0,
            true => I3C1_RX_A::I3C1_RX_1,
        }
    }
    #[doc = "Checks if the value of the field is `I3C1_RX_0`"]
    #[inline(always)]
    pub fn is_i3c1_rx_0(&self) -> bool {
        *self == I3C1_RX_A::I3C1_RX_0
    }
    #[doc = "Checks if the value of the field is `I3C1_RX_1`"]
    #[inline(always)]
    pub fn is_i3c1_rx_1(&self) -> bool {
        *self == I3C1_RX_A::I3C1_RX_1
    }
}
#[doc = "Field `I3C1_RX` writer - I3C1_RX enable"]
pub type I3C1_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, I3C1_RX_A, O>;
impl<'a, const O: u8> I3C1_RX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn i3c1_rx_0(self) -> &'a mut W {
        self.variant(I3C1_RX_A::I3C1_RX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn i3c1_rx_1(self) -> &'a mut W {
        self.variant(I3C1_RX_A::I3C1_RX_1)
    }
}
#[doc = "Field `I3C1_TX` reader - I3C1_TX enable"]
pub type I3C1_TX_R = crate::BitReader<I3C1_TX_A>;
#[doc = "I3C1_TX enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I3C1_TX_A {
    #[doc = "0: Disable"]
    I3C1_TX_0 = 0,
    #[doc = "1: Enable"]
    I3C1_TX_1 = 1,
}
impl From<I3C1_TX_A> for bool {
    #[inline(always)]
    fn from(variant: I3C1_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl I3C1_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I3C1_TX_A {
        match self.bits {
            false => I3C1_TX_A::I3C1_TX_0,
            true => I3C1_TX_A::I3C1_TX_1,
        }
    }
    #[doc = "Checks if the value of the field is `I3C1_TX_0`"]
    #[inline(always)]
    pub fn is_i3c1_tx_0(&self) -> bool {
        *self == I3C1_TX_A::I3C1_TX_0
    }
    #[doc = "Checks if the value of the field is `I3C1_TX_1`"]
    #[inline(always)]
    pub fn is_i3c1_tx_1(&self) -> bool {
        *self == I3C1_TX_A::I3C1_TX_1
    }
}
#[doc = "Field `I3C1_TX` writer - I3C1_TX enable"]
pub type I3C1_TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAC0_REQ_ENA0_SPEC, I3C1_TX_A, O>;
impl<'a, const O: u8> I3C1_TX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn i3c1_tx_0(self) -> &'a mut W {
        self.variant(I3C1_TX_A::I3C1_TX_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn i3c1_tx_1(self) -> &'a mut W {
        self.variant(I3C1_TX_A::I3C1_TX_1)
    }
}
impl R {
    #[doc = "Bit 0 - FLEXCOMM0 RX enable"]
    #[inline(always)]
    pub fn flexcomm0_rx(&self) -> FLEXCOMM0_RX_R {
        FLEXCOMM0_RX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLEXCOMM0 TX enable"]
    #[inline(always)]
    pub fn flexcomm0_tx(&self) -> FLEXCOMM0_TX_R {
        FLEXCOMM0_TX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLEXCOMM1 RX enable"]
    #[inline(always)]
    pub fn flexcomm1_rx(&self) -> FLEXCOMM1_RX_R {
        FLEXCOMM1_RX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FLEXCOMM1 TX enable"]
    #[inline(always)]
    pub fn flexcomm1_tx(&self) -> FLEXCOMM1_TX_R {
        FLEXCOMM1_TX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FLEXCOMM2 RX enable"]
    #[inline(always)]
    pub fn flexcomm2_rx(&self) -> FLEXCOMM2_RX_R {
        FLEXCOMM2_RX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FLEXCOMM2 TX enable"]
    #[inline(always)]
    pub fn flexcomm2_tx(&self) -> FLEXCOMM2_TX_R {
        FLEXCOMM2_TX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FLEXCOMM3 RX enable"]
    #[inline(always)]
    pub fn flexcomm3_rx(&self) -> FLEXCOMM3_RX_R {
        FLEXCOMM3_RX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FLEXCOMM3 TX enable"]
    #[inline(always)]
    pub fn flexcomm3_tx(&self) -> FLEXCOMM3_TX_R {
        FLEXCOMM3_TX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FLEXCOMM4 RX enable"]
    #[inline(always)]
    pub fn flexcomm4_rx(&self) -> FLEXCOMM4_RX_R {
        FLEXCOMM4_RX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FLEXCOMM4 TX enable"]
    #[inline(always)]
    pub fn flexcomm4_tx(&self) -> FLEXCOMM4_TX_R {
        FLEXCOMM4_TX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FLEXCOMM5 RX enable"]
    #[inline(always)]
    pub fn flexcomm5_rx(&self) -> FLEXCOMM5_RX_R {
        FLEXCOMM5_RX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FLEXCOMM5 TX enable"]
    #[inline(always)]
    pub fn flexcomm5_tx(&self) -> FLEXCOMM5_TX_R {
        FLEXCOMM5_TX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FLEXCOMM6 RX enable"]
    #[inline(always)]
    pub fn flexcomm6_rx(&self) -> FLEXCOMM6_RX_R {
        FLEXCOMM6_RX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FLEXCOMM6 TX enable"]
    #[inline(always)]
    pub fn flexcomm6_tx(&self) -> FLEXCOMM6_TX_R {
        FLEXCOMM6_TX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FLEXCOMM7 RX enable"]
    #[inline(always)]
    pub fn flexcomm7_rx(&self) -> FLEXCOMM7_RX_R {
        FLEXCOMM7_RX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FLEXCOMM7 TX enable"]
    #[inline(always)]
    pub fn flexcomm7_tx(&self) -> FLEXCOMM7_TX_R {
        FLEXCOMM7_TX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMIC0 channel 0 / FLEXCOMM8 RX DMA enable"]
    #[inline(always)]
    pub fn dmic0_ch0_flexcomm8_rx_dma(&self) -> DMIC0_CH0_FLEXCOMM8_RX_DMA_R {
        DMIC0_CH0_FLEXCOMM8_RX_DMA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMIC0 channel 1 / FLEXCOMM8 TX DMA enable"]
    #[inline(always)]
    pub fn dmic0_ch1_flexcomm8_tx_dma(&self) -> DMIC0_CH1_FLEXCOMM8_TX_DMA_R {
        DMIC0_CH1_FLEXCOMM8_TX_DMA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMIC0 channel 2 / FLEXCOMM9 RX DMA enable"]
    #[inline(always)]
    pub fn dmic0_ch2_flexcomm9_rx_dma(&self) -> DMIC0_CH2_FLEXCOMM9_RX_DMA_R {
        DMIC0_CH2_FLEXCOMM9_RX_DMA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DMIC0 channel 3 / FLEXCOMM9 TX DMA enable"]
    #[inline(always)]
    pub fn dmic0_ch3_flexcomm9_tx_dma(&self) -> DMIC0_CH3_FLEXCOMM9_TX_DMA_R {
        DMIC0_CH3_FLEXCOMM9_TX_DMA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DMIC0 channel 4 / FLEXCOMM10 RX DMA enable"]
    #[inline(always)]
    pub fn dmic0_ch4_flexcomm10_rx_dma(&self) -> DMIC0_CH4_FLEXCOMM10_RX_DMA_R {
        DMIC0_CH4_FLEXCOMM10_RX_DMA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMIC0 channel 5 / FLEXCOMM10 TX DMA enable"]
    #[inline(always)]
    pub fn dmic0_ch5_flexcomm10_tx_dma(&self) -> DMIC0_CH5_FLEXCOMM10_TX_DMA_R {
        DMIC0_CH5_FLEXCOMM10_TX_DMA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMIC0 channel 6 / FLEXCOMM13 RX DMA enable"]
    #[inline(always)]
    pub fn dmic0_ch6_flexcomm13_rx_dma(&self) -> DMIC0_CH6_FLEXCOMM13_RX_DMA_R {
        DMIC0_CH6_FLEXCOMM13_RX_DMA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DMIC0 channel 7 / FLEXCOMM13 TX DMA enable"]
    #[inline(always)]
    pub fn dmic0_ch7_flexcomm13_tx_dma(&self) -> DMIC0_CH7_FLEXCOMM13_TX_DMA_R {
        DMIC0_CH7_FLEXCOMM13_TX_DMA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I3C RX enable"]
    #[inline(always)]
    pub fn i3c0_rx(&self) -> I3C0_RX_R {
        I3C0_RX_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - I3C TX enable"]
    #[inline(always)]
    pub fn i3c0_tx(&self) -> I3C0_TX_R {
        I3C0_TX_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FLEXCOMM14 RX enable"]
    #[inline(always)]
    pub fn flexcomm14_rx(&self) -> FLEXCOMM14_RX_R {
        FLEXCOMM14_RX_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - FLEXCOMM14 TX enable"]
    #[inline(always)]
    pub fn flexcomm14_tx(&self) -> FLEXCOMM14_TX_R {
        FLEXCOMM14_TX_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - FLEXCOMM16 RX enable"]
    #[inline(always)]
    pub fn flexcomm16_rx(&self) -> FLEXCOMM16_RX_R {
        FLEXCOMM16_RX_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FLEXCOMM16 TX enable"]
    #[inline(always)]
    pub fn flexcomm16_tx(&self) -> FLEXCOMM16_TX_R {
        FLEXCOMM16_TX_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - I3C1_RX enable"]
    #[inline(always)]
    pub fn i3c1_rx(&self) -> I3C1_RX_R {
        I3C1_RX_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - I3C1_TX enable"]
    #[inline(always)]
    pub fn i3c1_tx(&self) -> I3C1_TX_R {
        I3C1_TX_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLEXCOMM0 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm0_rx(&mut self) -> FLEXCOMM0_RX_W<0> {
        FLEXCOMM0_RX_W::new(self)
    }
    #[doc = "Bit 1 - FLEXCOMM0 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm0_tx(&mut self) -> FLEXCOMM0_TX_W<1> {
        FLEXCOMM0_TX_W::new(self)
    }
    #[doc = "Bit 2 - FLEXCOMM1 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm1_rx(&mut self) -> FLEXCOMM1_RX_W<2> {
        FLEXCOMM1_RX_W::new(self)
    }
    #[doc = "Bit 3 - FLEXCOMM1 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm1_tx(&mut self) -> FLEXCOMM1_TX_W<3> {
        FLEXCOMM1_TX_W::new(self)
    }
    #[doc = "Bit 4 - FLEXCOMM2 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm2_rx(&mut self) -> FLEXCOMM2_RX_W<4> {
        FLEXCOMM2_RX_W::new(self)
    }
    #[doc = "Bit 5 - FLEXCOMM2 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm2_tx(&mut self) -> FLEXCOMM2_TX_W<5> {
        FLEXCOMM2_TX_W::new(self)
    }
    #[doc = "Bit 6 - FLEXCOMM3 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm3_rx(&mut self) -> FLEXCOMM3_RX_W<6> {
        FLEXCOMM3_RX_W::new(self)
    }
    #[doc = "Bit 7 - FLEXCOMM3 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm3_tx(&mut self) -> FLEXCOMM3_TX_W<7> {
        FLEXCOMM3_TX_W::new(self)
    }
    #[doc = "Bit 8 - FLEXCOMM4 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm4_rx(&mut self) -> FLEXCOMM4_RX_W<8> {
        FLEXCOMM4_RX_W::new(self)
    }
    #[doc = "Bit 9 - FLEXCOMM4 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm4_tx(&mut self) -> FLEXCOMM4_TX_W<9> {
        FLEXCOMM4_TX_W::new(self)
    }
    #[doc = "Bit 10 - FLEXCOMM5 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm5_rx(&mut self) -> FLEXCOMM5_RX_W<10> {
        FLEXCOMM5_RX_W::new(self)
    }
    #[doc = "Bit 11 - FLEXCOMM5 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm5_tx(&mut self) -> FLEXCOMM5_TX_W<11> {
        FLEXCOMM5_TX_W::new(self)
    }
    #[doc = "Bit 12 - FLEXCOMM6 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm6_rx(&mut self) -> FLEXCOMM6_RX_W<12> {
        FLEXCOMM6_RX_W::new(self)
    }
    #[doc = "Bit 13 - FLEXCOMM6 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm6_tx(&mut self) -> FLEXCOMM6_TX_W<13> {
        FLEXCOMM6_TX_W::new(self)
    }
    #[doc = "Bit 14 - FLEXCOMM7 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm7_rx(&mut self) -> FLEXCOMM7_RX_W<14> {
        FLEXCOMM7_RX_W::new(self)
    }
    #[doc = "Bit 15 - FLEXCOMM7 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm7_tx(&mut self) -> FLEXCOMM7_TX_W<15> {
        FLEXCOMM7_TX_W::new(self)
    }
    #[doc = "Bit 16 - DMIC0 channel 0 / FLEXCOMM8 RX DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0_ch0_flexcomm8_rx_dma(&mut self) -> DMIC0_CH0_FLEXCOMM8_RX_DMA_W<16> {
        DMIC0_CH0_FLEXCOMM8_RX_DMA_W::new(self)
    }
    #[doc = "Bit 17 - DMIC0 channel 1 / FLEXCOMM8 TX DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0_ch1_flexcomm8_tx_dma(&mut self) -> DMIC0_CH1_FLEXCOMM8_TX_DMA_W<17> {
        DMIC0_CH1_FLEXCOMM8_TX_DMA_W::new(self)
    }
    #[doc = "Bit 18 - DMIC0 channel 2 / FLEXCOMM9 RX DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0_ch2_flexcomm9_rx_dma(&mut self) -> DMIC0_CH2_FLEXCOMM9_RX_DMA_W<18> {
        DMIC0_CH2_FLEXCOMM9_RX_DMA_W::new(self)
    }
    #[doc = "Bit 19 - DMIC0 channel 3 / FLEXCOMM9 TX DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0_ch3_flexcomm9_tx_dma(&mut self) -> DMIC0_CH3_FLEXCOMM9_TX_DMA_W<19> {
        DMIC0_CH3_FLEXCOMM9_TX_DMA_W::new(self)
    }
    #[doc = "Bit 20 - DMIC0 channel 4 / FLEXCOMM10 RX DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0_ch4_flexcomm10_rx_dma(&mut self) -> DMIC0_CH4_FLEXCOMM10_RX_DMA_W<20> {
        DMIC0_CH4_FLEXCOMM10_RX_DMA_W::new(self)
    }
    #[doc = "Bit 21 - DMIC0 channel 5 / FLEXCOMM10 TX DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0_ch5_flexcomm10_tx_dma(&mut self) -> DMIC0_CH5_FLEXCOMM10_TX_DMA_W<21> {
        DMIC0_CH5_FLEXCOMM10_TX_DMA_W::new(self)
    }
    #[doc = "Bit 22 - DMIC0 channel 6 / FLEXCOMM13 RX DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0_ch6_flexcomm13_rx_dma(&mut self) -> DMIC0_CH6_FLEXCOMM13_RX_DMA_W<22> {
        DMIC0_CH6_FLEXCOMM13_RX_DMA_W::new(self)
    }
    #[doc = "Bit 23 - DMIC0 channel 7 / FLEXCOMM13 TX DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmic0_ch7_flexcomm13_tx_dma(&mut self) -> DMIC0_CH7_FLEXCOMM13_TX_DMA_W<23> {
        DMIC0_CH7_FLEXCOMM13_TX_DMA_W::new(self)
    }
    #[doc = "Bit 24 - I3C RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn i3c0_rx(&mut self) -> I3C0_RX_W<24> {
        I3C0_RX_W::new(self)
    }
    #[doc = "Bit 25 - I3C TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn i3c0_tx(&mut self) -> I3C0_TX_W<25> {
        I3C0_TX_W::new(self)
    }
    #[doc = "Bit 26 - FLEXCOMM14 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm14_rx(&mut self) -> FLEXCOMM14_RX_W<26> {
        FLEXCOMM14_RX_W::new(self)
    }
    #[doc = "Bit 27 - FLEXCOMM14 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm14_tx(&mut self) -> FLEXCOMM14_TX_W<27> {
        FLEXCOMM14_TX_W::new(self)
    }
    #[doc = "Bit 28 - FLEXCOMM16 RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm16_rx(&mut self) -> FLEXCOMM16_RX_W<28> {
        FLEXCOMM16_RX_W::new(self)
    }
    #[doc = "Bit 29 - FLEXCOMM16 TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm16_tx(&mut self) -> FLEXCOMM16_TX_W<29> {
        FLEXCOMM16_TX_W::new(self)
    }
    #[doc = "Bit 30 - I3C1_RX enable"]
    #[inline(always)]
    #[must_use]
    pub fn i3c1_rx(&mut self) -> I3C1_RX_W<30> {
        I3C1_RX_W::new(self)
    }
    #[doc = "Bit 31 - I3C1_TX enable"]
    #[inline(always)]
    #[must_use]
    pub fn i3c1_tx(&mut self) -> I3C1_TX_W<31> {
        I3C1_TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC0 request enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac0_req_ena0](index.html) module"]
pub struct DMAC0_REQ_ENA0_SPEC;
impl crate::RegisterSpec for DMAC0_REQ_ENA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac0_req_ena0::R](R) reader structure"]
impl crate::Readable for DMAC0_REQ_ENA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac0_req_ena0::W](W) writer structure"]
impl crate::Writable for DMAC0_REQ_ENA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAC0_REQ_ENA0 to value 0xffff_ffff"]
impl crate::Resettable for DMAC0_REQ_ENA0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
