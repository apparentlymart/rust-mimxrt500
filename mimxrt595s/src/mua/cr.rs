#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Fn` reader - Fn"]
pub type FN_R = crate::FieldReader<u8, FN_A>;
#[doc = "Fn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FN_A {
    #[doc = "0: Clears the Fn bit in the SR register."]
    CLEAR = 0,
    #[doc = "1: Sets the Fn bit in the SR register."]
    SET = 1,
}
impl From<FN_A> for u8 {
    #[inline(always)]
    fn from(variant: FN_A) -> Self {
        variant as _
    }
}
impl FN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FN_A> {
        match self.bits {
            0 => Some(FN_A::CLEAR),
            1 => Some(FN_A::SET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == FN_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == FN_A::SET
    }
}
#[doc = "Field `Fn` writer - Fn"]
pub type FN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, FN_A, 3, O>;
impl<'a, const O: u8> FN_W<'a, O> {
    #[doc = "Clears the Fn bit in the SR register."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FN_A::CLEAR)
    }
    #[doc = "Sets the Fn bit in the SR register."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(FN_A::SET)
    }
}
#[doc = "Field `MUR` reader - MUR"]
pub type MUR_R = crate::BitReader<MUR_A>;
#[doc = "MUR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUR_A {
    #[doc = "0: N/A. Self clearing bit (default)."]
    SELF_CLEARING = 0,
    #[doc = "1: Asserts the MU reset."]
    MUR = 1,
}
impl From<MUR_A> for bool {
    #[inline(always)]
    fn from(variant: MUR_A) -> Self {
        variant as u8 != 0
    }
}
impl MUR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUR_A {
        match self.bits {
            false => MUR_A::SELF_CLEARING,
            true => MUR_A::MUR,
        }
    }
    #[doc = "Checks if the value of the field is `SELF_CLEARING`"]
    #[inline(always)]
    pub fn is_self_clearing(&self) -> bool {
        *self == MUR_A::SELF_CLEARING
    }
    #[doc = "Checks if the value of the field is `MUR`"]
    #[inline(always)]
    pub fn is_mur(&self) -> bool {
        *self == MUR_A::MUR
    }
}
#[doc = "Field `MUR` writer - MUR"]
pub type MUR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, MUR_A, O>;
impl<'a, const O: u8> MUR_W<'a, O> {
    #[doc = "N/A. Self clearing bit (default)."]
    #[inline(always)]
    pub fn self_clearing(self) -> &'a mut W {
        self.variant(MUR_A::SELF_CLEARING)
    }
    #[doc = "Asserts the MU reset."]
    #[inline(always)]
    pub fn mur(self) -> &'a mut W {
        self.variant(MUR_A::MUR)
    }
}
#[doc = "Field `RDIE` reader - RDIE"]
pub type RDIE_R = crate::BitReader<RDIE_A>;
#[doc = "RDIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDIE_A {
    #[doc = "0: Disables Processor A General Purpose Interrupt 3 request due to Processor B reset de-assertion."]
    DISABLE = 0,
    #[doc = "1: Enables Processor A General Purpose Interrupt 3 request due to Processor B reset de-assertion."]
    ENABLE = 1,
}
impl From<RDIE_A> for bool {
    #[inline(always)]
    fn from(variant: RDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDIE_A {
        match self.bits {
            false => RDIE_A::DISABLE,
            true => RDIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RDIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RDIE_A::ENABLE
    }
}
#[doc = "Field `RDIE` writer - RDIE"]
pub type RDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, RDIE_A, O>;
impl<'a, const O: u8> RDIE_W<'a, O> {
    #[doc = "Disables Processor A General Purpose Interrupt 3 request due to Processor B reset de-assertion."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RDIE_A::DISABLE)
    }
    #[doc = "Enables Processor A General Purpose Interrupt 3 request due to Processor B reset de-assertion."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RDIE_A::ENABLE)
    }
}
#[doc = "Field `RAIE` reader - RAIE"]
pub type RAIE_R = crate::BitReader<RAIE_A>;
#[doc = "RAIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAIE_A {
    #[doc = "0: Disables Processor A General Purpose Interrupt 3 request due to Processor B reset assertion."]
    DISABLE = 0,
    #[doc = "1: Enables Processor A General Purpose Interrupt 3 request due to Processor B reset assertion."]
    ENABLE = 1,
}
impl From<RAIE_A> for bool {
    #[inline(always)]
    fn from(variant: RAIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RAIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAIE_A {
        match self.bits {
            false => RAIE_A::DISABLE,
            true => RAIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RAIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RAIE_A::ENABLE
    }
}
#[doc = "Field `RAIE` writer - RAIE"]
pub type RAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, RAIE_A, O>;
impl<'a, const O: u8> RAIE_W<'a, O> {
    #[doc = "Disables Processor A General Purpose Interrupt 3 request due to Processor B reset assertion."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAIE_A::DISABLE)
    }
    #[doc = "Enables Processor A General Purpose Interrupt 3 request due to Processor B reset assertion."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAIE_A::ENABLE)
    }
}
#[doc = "Field `GIRn` reader - GIRn"]
pub type GIRN_R = crate::FieldReader<u8, GIRN_A>;
#[doc = "GIRn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GIRN_A {
    #[doc = "0: MUA General Interrupt n is not requested to the MUB (default)."]
    NOT_REQ = 0,
    #[doc = "1: MUA General Interrupt n is requested to the MUB."]
    REQ = 1,
}
impl From<GIRN_A> for u8 {
    #[inline(always)]
    fn from(variant: GIRN_A) -> Self {
        variant as _
    }
}
impl GIRN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GIRN_A> {
        match self.bits {
            0 => Some(GIRN_A::NOT_REQ),
            1 => Some(GIRN_A::REQ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_REQ`"]
    #[inline(always)]
    pub fn is_not_req(&self) -> bool {
        *self == GIRN_A::NOT_REQ
    }
    #[doc = "Checks if the value of the field is `REQ`"]
    #[inline(always)]
    pub fn is_req(&self) -> bool {
        *self == GIRN_A::REQ
    }
}
#[doc = "Field `GIRn` writer - GIRn"]
pub type GIRN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, GIRN_A, 4, O>;
impl<'a, const O: u8> GIRN_W<'a, O> {
    #[doc = "MUA General Interrupt n is not requested to the MUB (default)."]
    #[inline(always)]
    pub fn not_req(self) -> &'a mut W {
        self.variant(GIRN_A::NOT_REQ)
    }
    #[doc = "MUA General Interrupt n is requested to the MUB."]
    #[inline(always)]
    pub fn req(self) -> &'a mut W {
        self.variant(GIRN_A::REQ)
    }
}
#[doc = "Field `TIEn` reader - TIEn"]
pub type TIEN_R = crate::FieldReader<u8, TIEN_A>;
#[doc = "TIEn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIEN_A {
    #[doc = "0: Disables MUA Transmit Interrupt n. (default)"]
    DISABLE = 0,
    #[doc = "1: Enables MUA Transmit Interrupt n."]
    ENABLE = 1,
}
impl From<TIEN_A> for u8 {
    #[inline(always)]
    fn from(variant: TIEN_A) -> Self {
        variant as _
    }
}
impl TIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIEN_A> {
        match self.bits {
            0 => Some(TIEN_A::DISABLE),
            1 => Some(TIEN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TIEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TIEN_A::ENABLE
    }
}
#[doc = "Field `TIEn` writer - TIEn"]
pub type TIEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, TIEN_A, 4, O>;
impl<'a, const O: u8> TIEN_W<'a, O> {
    #[doc = "Disables MUA Transmit Interrupt n. (default)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIEN_A::DISABLE)
    }
    #[doc = "Enables MUA Transmit Interrupt n."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIEN_A::ENABLE)
    }
}
#[doc = "Field `RIEn` reader - RIEn"]
pub type RIEN_R = crate::FieldReader<u8, RIEN_A>;
#[doc = "RIEn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RIEN_A {
    #[doc = "0: Disables MUA Receive Interrupt n. (default)"]
    DISABLE = 0,
    #[doc = "1: Enables MUA Receive Interrupt n."]
    ENABLE = 1,
}
impl From<RIEN_A> for u8 {
    #[inline(always)]
    fn from(variant: RIEN_A) -> Self {
        variant as _
    }
}
impl RIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RIEN_A> {
        match self.bits {
            0 => Some(RIEN_A::DISABLE),
            1 => Some(RIEN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RIEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RIEN_A::ENABLE
    }
}
#[doc = "Field `RIEn` writer - RIEn"]
pub type RIEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, RIEN_A, 4, O>;
impl<'a, const O: u8> RIEN_W<'a, O> {
    #[doc = "Disables MUA Receive Interrupt n. (default)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RIEN_A::DISABLE)
    }
    #[doc = "Enables MUA Receive Interrupt n."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RIEN_A::ENABLE)
    }
}
#[doc = "Field `GIEn` reader - GIEn"]
pub type GIEN_R = crate::FieldReader<u8, GIEN_A>;
#[doc = "GIEn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GIEN_A {
    #[doc = "0: Disables MUA General Interrupt n. (default)"]
    DISABLE = 0,
    #[doc = "1: Enables MUA General Interrupt n."]
    ENABLE = 1,
}
impl From<GIEN_A> for u8 {
    #[inline(always)]
    fn from(variant: GIEN_A) -> Self {
        variant as _
    }
}
impl GIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GIEN_A> {
        match self.bits {
            0 => Some(GIEN_A::DISABLE),
            1 => Some(GIEN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GIEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GIEN_A::ENABLE
    }
}
#[doc = "Field `GIEn` writer - GIEn"]
pub type GIEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, GIEN_A, 4, O>;
impl<'a, const O: u8> GIEN_W<'a, O> {
    #[doc = "Disables MUA General Interrupt n. (default)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GIEN_A::DISABLE)
    }
    #[doc = "Enables MUA General Interrupt n."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GIEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Fn"]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - MUR"]
    #[inline(always)]
    pub fn mur(&self) -> MUR_R {
        MUR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RDIE"]
    #[inline(always)]
    pub fn rdie(&self) -> RDIE_R {
        RDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - RAIE"]
    #[inline(always)]
    pub fn raie(&self) -> RAIE_R {
        RAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - GIRn"]
    #[inline(always)]
    pub fn girn(&self) -> GIRN_R {
        GIRN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - TIEn"]
    #[inline(always)]
    pub fn tien(&self) -> TIEN_R {
        TIEN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - RIEn"]
    #[inline(always)]
    pub fn rien(&self) -> RIEN_R {
        RIEN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - GIEn"]
    #[inline(always)]
    pub fn gien(&self) -> GIEN_R {
        GIEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Fn"]
    #[inline(always)]
    #[must_use]
    pub fn fn_(&mut self) -> FN_W<0> {
        FN_W::new(self)
    }
    #[doc = "Bit 5 - MUR"]
    #[inline(always)]
    #[must_use]
    pub fn mur(&mut self) -> MUR_W<5> {
        MUR_W::new(self)
    }
    #[doc = "Bit 6 - RDIE"]
    #[inline(always)]
    #[must_use]
    pub fn rdie(&mut self) -> RDIE_W<6> {
        RDIE_W::new(self)
    }
    #[doc = "Bit 12 - RAIE"]
    #[inline(always)]
    #[must_use]
    pub fn raie(&mut self) -> RAIE_W<12> {
        RAIE_W::new(self)
    }
    #[doc = "Bits 16:19 - GIRn"]
    #[inline(always)]
    #[must_use]
    pub fn girn(&mut self) -> GIRN_W<16> {
        GIRN_W::new(self)
    }
    #[doc = "Bits 20:23 - TIEn"]
    #[inline(always)]
    #[must_use]
    pub fn tien(&mut self) -> TIEN_W<20> {
        TIEN_W::new(self)
    }
    #[doc = "Bits 24:27 - RIEn"]
    #[inline(always)]
    #[must_use]
    pub fn rien(&mut self) -> RIEN_W<24> {
        RIEN_W::new(self)
    }
    #[doc = "Bits 28:31 - GIEn"]
    #[inline(always)]
    #[must_use]
    pub fn gien(&mut self) -> GIEN_W<28> {
        GIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
