#[doc = "Register `USB1_LOOPBACK_TOG` reader"]
pub struct R(crate::R<USB1_LOOPBACK_TOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1_LOOPBACK_TOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1_LOOPBACK_TOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1_LOOPBACK_TOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1_LOOPBACK_TOG` writer"]
pub struct W(crate::W<USB1_LOOPBACK_TOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1_LOOPBACK_TOG_SPEC>;
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
impl From<crate::W<USB1_LOOPBACK_TOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB1_LOOPBACK_TOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UTMI_TESTSTART` reader - USB loopback test."]
pub type UTMI_TESTSTART_R = crate::BitReader<UTMI_TESTSTART_A>;
#[doc = "USB loopback test.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UTMI_TESTSTART_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding bit"]
    ENABLE = 1,
}
impl From<UTMI_TESTSTART_A> for bool {
    #[inline(always)]
    fn from(variant: UTMI_TESTSTART_A) -> Self {
        variant as u8 != 0
    }
}
impl UTMI_TESTSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTMI_TESTSTART_A {
        match self.bits {
            false => UTMI_TESTSTART_A::DISABLE,
            true => UTMI_TESTSTART_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UTMI_TESTSTART_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UTMI_TESTSTART_A::ENABLE
    }
}
#[doc = "Field `UTMI_TESTSTART` writer - USB loopback test."]
pub type UTMI_TESTSTART_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_LOOPBACK_TOG_SPEC, UTMI_TESTSTART_A, O>;
impl<'a, const O: u8> UTMI_TESTSTART_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UTMI_TESTSTART_A::DISABLE)
    }
    #[doc = "Toggles the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UTMI_TESTSTART_A::ENABLE)
    }
}
#[doc = "Field `UTMI_DIG_TST0` reader - Mode control for USB loopback test."]
pub type UTMI_DIG_TST0_R = crate::BitReader<UTMI_DIG_TST0_A>;
#[doc = "Mode control for USB loopback test.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UTMI_DIG_TST0_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding bit"]
    ENABLE = 1,
}
impl From<UTMI_DIG_TST0_A> for bool {
    #[inline(always)]
    fn from(variant: UTMI_DIG_TST0_A) -> Self {
        variant as u8 != 0
    }
}
impl UTMI_DIG_TST0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTMI_DIG_TST0_A {
        match self.bits {
            false => UTMI_DIG_TST0_A::DISABLE,
            true => UTMI_DIG_TST0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UTMI_DIG_TST0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UTMI_DIG_TST0_A::ENABLE
    }
}
#[doc = "Field `UTMI_DIG_TST0` writer - Mode control for USB loopback test."]
pub type UTMI_DIG_TST0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_LOOPBACK_TOG_SPEC, UTMI_DIG_TST0_A, O>;
impl<'a, const O: u8> UTMI_DIG_TST0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UTMI_DIG_TST0_A::DISABLE)
    }
    #[doc = "Toggles the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UTMI_DIG_TST0_A::ENABLE)
    }
}
#[doc = "Field `UTMI_DIG_TST1` reader - Mode control for USB loopback test."]
pub type UTMI_DIG_TST1_R = crate::BitReader<UTMI_DIG_TST1_A>;
#[doc = "Mode control for USB loopback test.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UTMI_DIG_TST1_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding bit"]
    ENABLE = 1,
}
impl From<UTMI_DIG_TST1_A> for bool {
    #[inline(always)]
    fn from(variant: UTMI_DIG_TST1_A) -> Self {
        variant as u8 != 0
    }
}
impl UTMI_DIG_TST1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTMI_DIG_TST1_A {
        match self.bits {
            false => UTMI_DIG_TST1_A::DISABLE,
            true => UTMI_DIG_TST1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UTMI_DIG_TST1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UTMI_DIG_TST1_A::ENABLE
    }
}
#[doc = "Field `UTMI_DIG_TST1` writer - Mode control for USB loopback test."]
pub type UTMI_DIG_TST1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_LOOPBACK_TOG_SPEC, UTMI_DIG_TST1_A, O>;
impl<'a, const O: u8> UTMI_DIG_TST1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UTMI_DIG_TST1_A::DISABLE)
    }
    #[doc = "Toggles the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UTMI_DIG_TST1_A::ENABLE)
    }
}
#[doc = "Field `TSTI_TX_HS_MODE` reader - Select HS or FS mode for USB loopback testing."]
pub type TSTI_TX_HS_MODE_R = crate::BitReader<bool>;
#[doc = "Field `TSTI_TX_HS_MODE` writer - Select HS or FS mode for USB loopback testing."]
pub type TSTI_TX_HS_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_LOOPBACK_TOG_SPEC, bool, O>;
#[doc = "Field `TSTI_TX_LS_MODE` reader - Select HS or FS mode for USB loopback testing."]
pub type TSTI_TX_LS_MODE_R = crate::BitReader<TSTI_TX_LS_MODE_A>;
#[doc = "Select HS or FS mode for USB loopback testing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTI_TX_LS_MODE_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding bit"]
    ENABLE = 1,
}
impl From<TSTI_TX_LS_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: TSTI_TX_LS_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSTI_TX_LS_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTI_TX_LS_MODE_A {
        match self.bits {
            false => TSTI_TX_LS_MODE_A::DISABLE,
            true => TSTI_TX_LS_MODE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSTI_TX_LS_MODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TSTI_TX_LS_MODE_A::ENABLE
    }
}
#[doc = "Field `TSTI_TX_LS_MODE` writer - Select HS or FS mode for USB loopback testing."]
pub type TSTI_TX_LS_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_LOOPBACK_TOG_SPEC, TSTI_TX_LS_MODE_A, O>;
impl<'a, const O: u8> TSTI_TX_LS_MODE_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TSTI_TX_LS_MODE_A::DISABLE)
    }
    #[doc = "Toggles the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TSTI_TX_LS_MODE_A::ENABLE)
    }
}
#[doc = "Field `TSTI_TX_EN` reader - Enable TX for USB loopback test."]
pub type TSTI_TX_EN_R = crate::BitReader<TSTI_TX_EN_A>;
#[doc = "Enable TX for USB loopback test.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTI_TX_EN_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding bit"]
    ENABLE = 1,
}
impl From<TSTI_TX_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TSTI_TX_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TSTI_TX_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTI_TX_EN_A {
        match self.bits {
            false => TSTI_TX_EN_A::DISABLE,
            true => TSTI_TX_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSTI_TX_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TSTI_TX_EN_A::ENABLE
    }
}
#[doc = "Field `TSTI_TX_EN` writer - Enable TX for USB loopback test."]
pub type TSTI_TX_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_LOOPBACK_TOG_SPEC, TSTI_TX_EN_A, O>;
impl<'a, const O: u8> TSTI_TX_EN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TSTI_TX_EN_A::DISABLE)
    }
    #[doc = "Toggles the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TSTI_TX_EN_A::ENABLE)
    }
}
#[doc = "Field `TSTI_TX_HIZ` reader - Sets TX Hi-Z for USB loopback test."]
pub type TSTI_TX_HIZ_R = crate::BitReader<TSTI_TX_HIZ_A>;
#[doc = "Sets TX Hi-Z for USB loopback test.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTI_TX_HIZ_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding bit"]
    ENABLE = 1,
}
impl From<TSTI_TX_HIZ_A> for bool {
    #[inline(always)]
    fn from(variant: TSTI_TX_HIZ_A) -> Self {
        variant as u8 != 0
    }
}
impl TSTI_TX_HIZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTI_TX_HIZ_A {
        match self.bits {
            false => TSTI_TX_HIZ_A::DISABLE,
            true => TSTI_TX_HIZ_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSTI_TX_HIZ_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TSTI_TX_HIZ_A::ENABLE
    }
}
#[doc = "Field `TSTI_TX_HIZ` writer - Sets TX Hi-Z for USB loopback test."]
pub type TSTI_TX_HIZ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_LOOPBACK_TOG_SPEC, TSTI_TX_HIZ_A, O>;
impl<'a, const O: u8> TSTI_TX_HIZ_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TSTI_TX_HIZ_A::DISABLE)
    }
    #[doc = "Toggles the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TSTI_TX_HIZ_A::ENABLE)
    }
}
#[doc = "Field `UTMO_DIG_TST0` reader - Status bit for USB loopback test."]
pub type UTMO_DIG_TST0_R = crate::BitReader<UTMO_DIG_TST0_A>;
#[doc = "Status bit for USB loopback test.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UTMO_DIG_TST0_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding bit"]
    ENABLE = 1,
}
impl From<UTMO_DIG_TST0_A> for bool {
    #[inline(always)]
    fn from(variant: UTMO_DIG_TST0_A) -> Self {
        variant as u8 != 0
    }
}
impl UTMO_DIG_TST0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTMO_DIG_TST0_A {
        match self.bits {
            false => UTMO_DIG_TST0_A::DISABLE,
            true => UTMO_DIG_TST0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UTMO_DIG_TST0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UTMO_DIG_TST0_A::ENABLE
    }
}
#[doc = "Field `UTMO_DIG_TST1` reader - Status bit for USB loopback test."]
pub type UTMO_DIG_TST1_R = crate::BitReader<UTMO_DIG_TST1_A>;
#[doc = "Status bit for USB loopback test.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UTMO_DIG_TST1_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding bit"]
    ENABLE = 1,
}
impl From<UTMO_DIG_TST1_A> for bool {
    #[inline(always)]
    fn from(variant: UTMO_DIG_TST1_A) -> Self {
        variant as u8 != 0
    }
}
impl UTMO_DIG_TST1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTMO_DIG_TST1_A {
        match self.bits {
            false => UTMO_DIG_TST1_A::DISABLE,
            true => UTMO_DIG_TST1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UTMO_DIG_TST1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UTMO_DIG_TST1_A::ENABLE
    }
}
#[doc = "Field `TSTI_HSFS_MODE_EN` reader - TSTI_HSFS_MODE_EN"]
pub type TSTI_HSFS_MODE_EN_R = crate::BitReader<TSTI_HSFS_MODE_EN_A>;
#[doc = "TSTI_HSFS_MODE_EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTI_HSFS_MODE_EN_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding bit"]
    ENABLE = 1,
}
impl From<TSTI_HSFS_MODE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TSTI_HSFS_MODE_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TSTI_HSFS_MODE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSTI_HSFS_MODE_EN_A {
        match self.bits {
            false => TSTI_HSFS_MODE_EN_A::DISABLE,
            true => TSTI_HSFS_MODE_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSTI_HSFS_MODE_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TSTI_HSFS_MODE_EN_A::ENABLE
    }
}
#[doc = "Field `TSTI_HSFS_MODE_EN` writer - TSTI_HSFS_MODE_EN"]
pub type TSTI_HSFS_MODE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_LOOPBACK_TOG_SPEC, TSTI_HSFS_MODE_EN_A, O>;
impl<'a, const O: u8> TSTI_HSFS_MODE_EN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TSTI_HSFS_MODE_EN_A::DISABLE)
    }
    #[doc = "Toggles the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TSTI_HSFS_MODE_EN_A::ENABLE)
    }
}
#[doc = "Field `TSTPKT` reader - Test packet"]
pub type TSTPKT_R = crate::FieldReader<u8, TSTPKT_A>;
#[doc = "Test packet\n\nValue on reset: 85"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTPKT_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding bit"]
    ENABLE = 1,
}
impl From<TSTPKT_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTPKT_A) -> Self {
        variant as _
    }
}
impl TSTPKT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSTPKT_A> {
        match self.bits {
            0 => Some(TSTPKT_A::DISABLE),
            1 => Some(TSTPKT_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSTPKT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TSTPKT_A::ENABLE
    }
}
#[doc = "Field `TSTPKT` writer - Test packet"]
pub type TSTPKT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB1_LOOPBACK_TOG_SPEC, u8, TSTPKT_A, 8, O>;
impl<'a, const O: u8> TSTPKT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TSTPKT_A::DISABLE)
    }
    #[doc = "Toggles the corresponding bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TSTPKT_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - USB loopback test."]
    #[inline(always)]
    pub fn utmi_teststart(&self) -> UTMI_TESTSTART_R {
        UTMI_TESTSTART_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode control for USB loopback test."]
    #[inline(always)]
    pub fn utmi_dig_tst0(&self) -> UTMI_DIG_TST0_R {
        UTMI_DIG_TST0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode control for USB loopback test."]
    #[inline(always)]
    pub fn utmi_dig_tst1(&self) -> UTMI_DIG_TST1_R {
        UTMI_DIG_TST1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select HS or FS mode for USB loopback testing."]
    #[inline(always)]
    pub fn tsti_tx_hs_mode(&self) -> TSTI_TX_HS_MODE_R {
        TSTI_TX_HS_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Select HS or FS mode for USB loopback testing."]
    #[inline(always)]
    pub fn tsti_tx_ls_mode(&self) -> TSTI_TX_LS_MODE_R {
        TSTI_TX_LS_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable TX for USB loopback test."]
    #[inline(always)]
    pub fn tsti_tx_en(&self) -> TSTI_TX_EN_R {
        TSTI_TX_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sets TX Hi-Z for USB loopback test."]
    #[inline(always)]
    pub fn tsti_tx_hiz(&self) -> TSTI_TX_HIZ_R {
        TSTI_TX_HIZ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Status bit for USB loopback test."]
    #[inline(always)]
    pub fn utmo_dig_tst0(&self) -> UTMO_DIG_TST0_R {
        UTMO_DIG_TST0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Status bit for USB loopback test."]
    #[inline(always)]
    pub fn utmo_dig_tst1(&self) -> UTMO_DIG_TST1_R {
        UTMO_DIG_TST1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - TSTI_HSFS_MODE_EN"]
    #[inline(always)]
    pub fn tsti_hsfs_mode_en(&self) -> TSTI_HSFS_MODE_EN_R {
        TSTI_HSFS_MODE_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Test packet"]
    #[inline(always)]
    pub fn tstpkt(&self) -> TSTPKT_R {
        TSTPKT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB loopback test."]
    #[inline(always)]
    #[must_use]
    pub fn utmi_teststart(&mut self) -> UTMI_TESTSTART_W<0> {
        UTMI_TESTSTART_W::new(self)
    }
    #[doc = "Bit 1 - Mode control for USB loopback test."]
    #[inline(always)]
    #[must_use]
    pub fn utmi_dig_tst0(&mut self) -> UTMI_DIG_TST0_W<1> {
        UTMI_DIG_TST0_W::new(self)
    }
    #[doc = "Bit 2 - Mode control for USB loopback test."]
    #[inline(always)]
    #[must_use]
    pub fn utmi_dig_tst1(&mut self) -> UTMI_DIG_TST1_W<2> {
        UTMI_DIG_TST1_W::new(self)
    }
    #[doc = "Bit 3 - Select HS or FS mode for USB loopback testing."]
    #[inline(always)]
    #[must_use]
    pub fn tsti_tx_hs_mode(&mut self) -> TSTI_TX_HS_MODE_W<3> {
        TSTI_TX_HS_MODE_W::new(self)
    }
    #[doc = "Bit 4 - Select HS or FS mode for USB loopback testing."]
    #[inline(always)]
    #[must_use]
    pub fn tsti_tx_ls_mode(&mut self) -> TSTI_TX_LS_MODE_W<4> {
        TSTI_TX_LS_MODE_W::new(self)
    }
    #[doc = "Bit 5 - Enable TX for USB loopback test."]
    #[inline(always)]
    #[must_use]
    pub fn tsti_tx_en(&mut self) -> TSTI_TX_EN_W<5> {
        TSTI_TX_EN_W::new(self)
    }
    #[doc = "Bit 6 - Sets TX Hi-Z for USB loopback test."]
    #[inline(always)]
    #[must_use]
    pub fn tsti_tx_hiz(&mut self) -> TSTI_TX_HIZ_W<6> {
        TSTI_TX_HIZ_W::new(self)
    }
    #[doc = "Bit 15 - TSTI_HSFS_MODE_EN"]
    #[inline(always)]
    #[must_use]
    pub fn tsti_hsfs_mode_en(&mut self) -> TSTI_HSFS_MODE_EN_W<15> {
        TSTI_HSFS_MODE_EN_W::new(self)
    }
    #[doc = "Bits 16:23 - Test packet"]
    #[inline(always)]
    #[must_use]
    pub fn tstpkt(&mut self) -> TSTPKT_W<16> {
        TSTPKT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY Loopback Control/Status Toggle\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_loopback_tog](index.html) module"]
pub struct USB1_LOOPBACK_TOG_SPEC;
impl crate::RegisterSpec for USB1_LOOPBACK_TOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1_loopback_tog::R](R) reader structure"]
impl crate::Readable for USB1_LOOPBACK_TOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1_loopback_tog::W](W) writer structure"]
impl crate::Writable for USB1_LOOPBACK_TOG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB1_LOOPBACK_TOG to value 0x0055_0000"]
impl crate::Resettable for USB1_LOOPBACK_TOG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0055_0000;
}
