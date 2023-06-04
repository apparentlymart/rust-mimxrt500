#[doc = "Register `CSW` reader"]
pub struct R(crate::R<CSW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSW` writer"]
pub struct W(crate::W<CSW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSW_SPEC>;
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
impl From<crate::W<CSW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESYNCH_REQ` reader - Re-synchronization Request"]
pub type RESYNCH_REQ_R = crate::BitReader<RESYNCH_REQ_A>;
#[doc = "Re-synchronization Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESYNCH_REQ_A {
    #[doc = "0: No Request"]
    NO_REQUEST = 0,
    #[doc = "1: Request for re-synchronization"]
    REQUEST = 1,
}
impl From<RESYNCH_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: RESYNCH_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl RESYNCH_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESYNCH_REQ_A {
        match self.bits {
            false => RESYNCH_REQ_A::NO_REQUEST,
            true => RESYNCH_REQ_A::REQUEST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_REQUEST`"]
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == RESYNCH_REQ_A::NO_REQUEST
    }
    #[doc = "Checks if the value of the field is `REQUEST`"]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == RESYNCH_REQ_A::REQUEST
    }
}
#[doc = "Field `RESYNCH_REQ` writer - Re-synchronization Request"]
pub type RESYNCH_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSW_SPEC, RESYNCH_REQ_A, O>;
impl<'a, const O: u8> RESYNCH_REQ_W<'a, O> {
    #[doc = "No Request"]
    #[inline(always)]
    pub fn no_request(self) -> &'a mut W {
        self.variant(RESYNCH_REQ_A::NO_REQUEST)
    }
    #[doc = "Request for re-synchronization"]
    #[inline(always)]
    pub fn request(self) -> &'a mut W {
        self.variant(RESYNCH_REQ_A::REQUEST)
    }
}
#[doc = "Field `REQ_PENDING` reader - Request Pending"]
pub type REQ_PENDING_R = crate::BitReader<REQ_PENDING_A>;
#[doc = "Request Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REQ_PENDING_A {
    #[doc = "0: No Request Pending"]
    NO_REQUEST_PENDING = 0,
    #[doc = "1: Request for Re-synchronization Pending"]
    REQUEST_PENDING = 1,
}
impl From<REQ_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: REQ_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl REQ_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQ_PENDING_A {
        match self.bits {
            false => REQ_PENDING_A::NO_REQUEST_PENDING,
            true => REQ_PENDING_A::REQUEST_PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_REQUEST_PENDING`"]
    #[inline(always)]
    pub fn is_no_request_pending(&self) -> bool {
        *self == REQ_PENDING_A::NO_REQUEST_PENDING
    }
    #[doc = "Checks if the value of the field is `REQUEST_PENDING`"]
    #[inline(always)]
    pub fn is_request_pending(&self) -> bool {
        *self == REQ_PENDING_A::REQUEST_PENDING
    }
}
#[doc = "Field `REQ_PENDING` writer - Request Pending"]
pub type REQ_PENDING_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSW_SPEC, REQ_PENDING_A, O>;
impl<'a, const O: u8> REQ_PENDING_W<'a, O> {
    #[doc = "No Request Pending"]
    #[inline(always)]
    pub fn no_request_pending(self) -> &'a mut W {
        self.variant(REQ_PENDING_A::NO_REQUEST_PENDING)
    }
    #[doc = "Request for Re-synchronization Pending"]
    #[inline(always)]
    pub fn request_pending(self) -> &'a mut W {
        self.variant(REQ_PENDING_A::REQUEST_PENDING)
    }
}
#[doc = "Field `DBG_OR_ERR` reader - Debug Overrun Error"]
pub type DBG_OR_ERR_R = crate::BitReader<DBG_OR_ERR_A>;
#[doc = "Debug Overrun Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_OR_ERR_A {
    #[doc = "0: No Debug Overrun error"]
    NO_OVERRUN_ERR = 0,
    #[doc = "1: Debug Overrun Error. A debug overrun occurred."]
    OVERRUN_ERR = 1,
}
impl From<DBG_OR_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_OR_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_OR_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_OR_ERR_A {
        match self.bits {
            false => DBG_OR_ERR_A::NO_OVERRUN_ERR,
            true => DBG_OR_ERR_A::OVERRUN_ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OVERRUN_ERR`"]
    #[inline(always)]
    pub fn is_no_overrun_err(&self) -> bool {
        *self == DBG_OR_ERR_A::NO_OVERRUN_ERR
    }
    #[doc = "Checks if the value of the field is `OVERRUN_ERR`"]
    #[inline(always)]
    pub fn is_overrun_err(&self) -> bool {
        *self == DBG_OR_ERR_A::OVERRUN_ERR
    }
}
#[doc = "Field `DBG_OR_ERR` writer - Debug Overrun Error"]
pub type DBG_OR_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSW_SPEC, DBG_OR_ERR_A, O>;
impl<'a, const O: u8> DBG_OR_ERR_W<'a, O> {
    #[doc = "No Debug Overrun error"]
    #[inline(always)]
    pub fn no_overrun_err(self) -> &'a mut W {
        self.variant(DBG_OR_ERR_A::NO_OVERRUN_ERR)
    }
    #[doc = "Debug Overrun Error. A debug overrun occurred."]
    #[inline(always)]
    pub fn overrun_err(self) -> &'a mut W {
        self.variant(DBG_OR_ERR_A::OVERRUN_ERR)
    }
}
#[doc = "Field `AHB_OR_ERR` reader - AHB Overrun Error"]
pub type AHB_OR_ERR_R = crate::BitReader<AHB_OR_ERR_A>;
#[doc = "AHB Overrun Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB_OR_ERR_A {
    #[doc = "0: No AHB Overrun Error"]
    NO_AHB_OVERRUN_ERR = 0,
    #[doc = "1: AHB Overrun Error. An AHB overrun occurred."]
    AHB_OVERRUN_ERR = 1,
}
impl From<AHB_OR_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_OR_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl AHB_OR_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_OR_ERR_A {
        match self.bits {
            false => AHB_OR_ERR_A::NO_AHB_OVERRUN_ERR,
            true => AHB_OR_ERR_A::AHB_OVERRUN_ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_AHB_OVERRUN_ERR`"]
    #[inline(always)]
    pub fn is_no_ahb_overrun_err(&self) -> bool {
        *self == AHB_OR_ERR_A::NO_AHB_OVERRUN_ERR
    }
    #[doc = "Checks if the value of the field is `AHB_OVERRUN_ERR`"]
    #[inline(always)]
    pub fn is_ahb_overrun_err(&self) -> bool {
        *self == AHB_OR_ERR_A::AHB_OVERRUN_ERR
    }
}
#[doc = "Field `AHB_OR_ERR` writer - AHB Overrun Error"]
pub type AHB_OR_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSW_SPEC, AHB_OR_ERR_A, O>;
impl<'a, const O: u8> AHB_OR_ERR_W<'a, O> {
    #[doc = "No AHB Overrun Error"]
    #[inline(always)]
    pub fn no_ahb_overrun_err(self) -> &'a mut W {
        self.variant(AHB_OR_ERR_A::NO_AHB_OVERRUN_ERR)
    }
    #[doc = "AHB Overrun Error. An AHB overrun occurred."]
    #[inline(always)]
    pub fn ahb_overrun_err(self) -> &'a mut W {
        self.variant(AHB_OR_ERR_A::AHB_OVERRUN_ERR)
    }
}
#[doc = "Field `SOFT_RESET` reader - Soft Reset"]
pub type SOFT_RESET_R = crate::BitReader<bool>;
#[doc = "Field `SOFT_RESET` writer - Soft Reset"]
pub type SOFT_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSW_SPEC, bool, O>;
#[doc = "Field `CHIP_RESET_REQ` reader - Chip Reset Request"]
pub type CHIP_RESET_REQ_R = crate::BitReader<bool>;
#[doc = "Field `CHIP_RESET_REQ` writer - Chip Reset Request"]
pub type CHIP_RESET_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSW_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Re-synchronization Request"]
    #[inline(always)]
    pub fn resynch_req(&self) -> RESYNCH_REQ_R {
        RESYNCH_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Request Pending"]
    #[inline(always)]
    pub fn req_pending(&self) -> REQ_PENDING_R {
        REQ_PENDING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Overrun Error"]
    #[inline(always)]
    pub fn dbg_or_err(&self) -> DBG_OR_ERR_R {
        DBG_OR_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AHB Overrun Error"]
    #[inline(always)]
    pub fn ahb_or_err(&self) -> AHB_OR_ERR_R {
        AHB_OR_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Soft Reset"]
    #[inline(always)]
    pub fn soft_reset(&self) -> SOFT_RESET_R {
        SOFT_RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Chip Reset Request"]
    #[inline(always)]
    pub fn chip_reset_req(&self) -> CHIP_RESET_REQ_R {
        CHIP_RESET_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Re-synchronization Request"]
    #[inline(always)]
    #[must_use]
    pub fn resynch_req(&mut self) -> RESYNCH_REQ_W<0> {
        RESYNCH_REQ_W::new(self)
    }
    #[doc = "Bit 1 - Request Pending"]
    #[inline(always)]
    #[must_use]
    pub fn req_pending(&mut self) -> REQ_PENDING_W<1> {
        REQ_PENDING_W::new(self)
    }
    #[doc = "Bit 2 - Debug Overrun Error"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_or_err(&mut self) -> DBG_OR_ERR_W<2> {
        DBG_OR_ERR_W::new(self)
    }
    #[doc = "Bit 3 - AHB Overrun Error"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_or_err(&mut self) -> AHB_OR_ERR_W<3> {
        AHB_OR_ERR_W::new(self)
    }
    #[doc = "Bit 4 - Soft Reset"]
    #[inline(always)]
    #[must_use]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W<4> {
        SOFT_RESET_W::new(self)
    }
    #[doc = "Bit 5 - Chip Reset Request"]
    #[inline(always)]
    #[must_use]
    pub fn chip_reset_req(&mut self) -> CHIP_RESET_REQ_W<5> {
        CHIP_RESET_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command and status word\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csw](index.html) module"]
pub struct CSW_SPEC;
impl crate::RegisterSpec for CSW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csw::R](R) reader structure"]
impl crate::Readable for CSW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csw::W](W) writer structure"]
impl crate::Writable for CSW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSW to value 0"]
impl crate::Resettable for CSW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
