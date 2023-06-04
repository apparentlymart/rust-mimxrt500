#[doc = "Register `TX_SET` reader"]
pub struct R(crate::R<TX_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_SET` writer"]
pub struct W(crate::W<TX_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_SET_SPEC>;
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
impl From<crate::W<TX_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_CAL` reader - Current Trim decode."]
pub type D_CAL_R = crate::FieldReader<u8, D_CAL_A>;
#[doc = "Current Trim decode.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum D_CAL_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding TX bit"]
    ENABLE = 1,
}
impl From<D_CAL_A> for u8 {
    #[inline(always)]
    fn from(variant: D_CAL_A) -> Self {
        variant as _
    }
}
impl D_CAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<D_CAL_A> {
        match self.bits {
            0 => Some(D_CAL_A::DISABLE),
            1 => Some(D_CAL_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == D_CAL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == D_CAL_A::ENABLE
    }
}
#[doc = "Field `D_CAL` writer - Current Trim decode."]
pub type D_CAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_SET_SPEC, u8, D_CAL_A, 4, O>;
impl<'a, const O: u8> D_CAL_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(D_CAL_A::DISABLE)
    }
    #[doc = "Sets the corresponding TX bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(D_CAL_A::ENABLE)
    }
}
#[doc = "Field `TXENCAL45DM` reader - DM series termination resistance trim."]
pub type TXENCAL45DM_R = crate::FieldReader<u8, TXENCAL45DM_A>;
#[doc = "DM series termination resistance trim.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXENCAL45DM_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding TX bit"]
    ENABLE = 1,
}
impl From<TXENCAL45DM_A> for u8 {
    #[inline(always)]
    fn from(variant: TXENCAL45DM_A) -> Self {
        variant as _
    }
}
impl TXENCAL45DM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXENCAL45DM_A> {
        match self.bits {
            0 => Some(TXENCAL45DM_A::DISABLE),
            1 => Some(TXENCAL45DM_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXENCAL45DM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXENCAL45DM_A::ENABLE
    }
}
#[doc = "Field `TXENCAL45DM` writer - DM series termination resistance trim."]
pub type TXENCAL45DM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_SET_SPEC, u8, TXENCAL45DM_A, 4, O>;
impl<'a, const O: u8> TXENCAL45DM_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXENCAL45DM_A::DISABLE)
    }
    #[doc = "Sets the corresponding TX bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXENCAL45DM_A::ENABLE)
    }
}
#[doc = "Field `TXENCAL45DN` reader - Enable resistance calibration on DN."]
pub type TXENCAL45DN_R = crate::BitReader<TXENCAL45DN_A>;
#[doc = "Enable resistance calibration on DN.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXENCAL45DN_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding TX bit"]
    ENABLE = 1,
}
impl From<TXENCAL45DN_A> for bool {
    #[inline(always)]
    fn from(variant: TXENCAL45DN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXENCAL45DN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXENCAL45DN_A {
        match self.bits {
            false => TXENCAL45DN_A::DISABLE,
            true => TXENCAL45DN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXENCAL45DN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXENCAL45DN_A::ENABLE
    }
}
#[doc = "Field `TXENCAL45DN` writer - Enable resistance calibration on DN."]
pub type TXENCAL45DN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_SET_SPEC, TXENCAL45DN_A, O>;
impl<'a, const O: u8> TXENCAL45DN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXENCAL45DN_A::DISABLE)
    }
    #[doc = "Sets the corresponding TX bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXENCAL45DN_A::ENABLE)
    }
}
#[doc = "Field `TXCAL45DP` reader - DP series termination resistance trim."]
pub type TXCAL45DP_R = crate::FieldReader<u8, TXCAL45DP_A>;
#[doc = "DP series termination resistance trim.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXCAL45DP_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding TX bit"]
    ENABLE = 1,
}
impl From<TXCAL45DP_A> for u8 {
    #[inline(always)]
    fn from(variant: TXCAL45DP_A) -> Self {
        variant as _
    }
}
impl TXCAL45DP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXCAL45DP_A> {
        match self.bits {
            0 => Some(TXCAL45DP_A::DISABLE),
            1 => Some(TXCAL45DP_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXCAL45DP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXCAL45DP_A::ENABLE
    }
}
#[doc = "Field `TXCAL45DP` writer - DP series termination resistance trim."]
pub type TXCAL45DP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_SET_SPEC, u8, TXCAL45DP_A, 4, O>;
impl<'a, const O: u8> TXCAL45DP_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXCAL45DP_A::DISABLE)
    }
    #[doc = "Sets the corresponding TX bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXCAL45DP_A::ENABLE)
    }
}
#[doc = "Field `TXENCAL45DP` reader - Enable resistance calibration on DP"]
pub type TXENCAL45DP_R = crate::BitReader<TXENCAL45DP_A>;
#[doc = "Enable resistance calibration on DP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXENCAL45DP_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Sets the corresponding TX bit"]
    ENABLE = 1,
}
impl From<TXENCAL45DP_A> for bool {
    #[inline(always)]
    fn from(variant: TXENCAL45DP_A) -> Self {
        variant as u8 != 0
    }
}
impl TXENCAL45DP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXENCAL45DP_A {
        match self.bits {
            false => TXENCAL45DP_A::DISABLE,
            true => TXENCAL45DP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXENCAL45DP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXENCAL45DP_A::ENABLE
    }
}
#[doc = "Field `TXENCAL45DP` writer - Enable resistance calibration on DP"]
pub type TXENCAL45DP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_SET_SPEC, TXENCAL45DP_A, O>;
impl<'a, const O: u8> TXENCAL45DP_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXENCAL45DP_A::DISABLE)
    }
    #[doc = "Sets the corresponding TX bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXENCAL45DP_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - Current Trim decode."]
    #[inline(always)]
    pub fn d_cal(&self) -> D_CAL_R {
        D_CAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DM series termination resistance trim."]
    #[inline(always)]
    pub fn txencal45dm(&self) -> TXENCAL45DM_R {
        TXENCAL45DM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Enable resistance calibration on DN."]
    #[inline(always)]
    pub fn txencal45dn(&self) -> TXENCAL45DN_R {
        TXENCAL45DN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - DP series termination resistance trim."]
    #[inline(always)]
    pub fn txcal45dp(&self) -> TXCAL45DP_R {
        TXCAL45DP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - Enable resistance calibration on DP"]
    #[inline(always)]
    pub fn txencal45dp(&self) -> TXENCAL45DP_R {
        TXENCAL45DP_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Current Trim decode."]
    #[inline(always)]
    #[must_use]
    pub fn d_cal(&mut self) -> D_CAL_W<0> {
        D_CAL_W::new(self)
    }
    #[doc = "Bits 8:11 - DM series termination resistance trim."]
    #[inline(always)]
    #[must_use]
    pub fn txencal45dm(&mut self) -> TXENCAL45DM_W<8> {
        TXENCAL45DM_W::new(self)
    }
    #[doc = "Bit 13 - Enable resistance calibration on DN."]
    #[inline(always)]
    #[must_use]
    pub fn txencal45dn(&mut self) -> TXENCAL45DN_W<13> {
        TXENCAL45DN_W::new(self)
    }
    #[doc = "Bits 16:19 - DP series termination resistance trim."]
    #[inline(always)]
    #[must_use]
    pub fn txcal45dp(&mut self) -> TXCAL45DP_W<16> {
        TXCAL45DP_W::new(self)
    }
    #[doc = "Bit 21 - Enable resistance calibration on DP"]
    #[inline(always)]
    #[must_use]
    pub fn txencal45dp(&mut self) -> TXENCAL45DP_W<21> {
        TXENCAL45DP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Control Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_set](index.html) module"]
pub struct TX_SET_SPEC;
impl crate::RegisterSpec for TX_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_set::R](R) reader structure"]
impl crate::Readable for TX_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_set::W](W) writer structure"]
impl crate::Writable for TX_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_SET to value 0x0007_0707"]
impl crate::Resettable for TX_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0707;
}
