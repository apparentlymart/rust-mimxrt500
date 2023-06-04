#[doc = "Register `SCTRL` reader"]
pub struct R(crate::R<SCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTRL` writer"]
pub struct W(crate::W<SCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTRL_SPEC>;
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
impl From<crate::W<SCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENT` reader - EVENT"]
pub type EVENT_R = crate::FieldReader<u8, EVENT_A>;
#[doc = "EVENT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EVENT_A {
    #[doc = "0: NORMAL_MODE"]
    NORMAL_MODE = 0,
    #[doc = "1: IBI"]
    IBI = 1,
    #[doc = "2: MASTER_REQUEST"]
    MASTER_REQUEST = 2,
    #[doc = "3: HOT_JOIN_REQUEST"]
    HOT_JOIN_REQUEST = 3,
}
impl From<EVENT_A> for u8 {
    #[inline(always)]
    fn from(variant: EVENT_A) -> Self {
        variant as _
    }
}
impl EVENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENT_A {
        match self.bits {
            0 => EVENT_A::NORMAL_MODE,
            1 => EVENT_A::IBI,
            2 => EVENT_A::MASTER_REQUEST,
            3 => EVENT_A::HOT_JOIN_REQUEST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == EVENT_A::NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `IBI`"]
    #[inline(always)]
    pub fn is_ibi(&self) -> bool {
        *self == EVENT_A::IBI
    }
    #[doc = "Checks if the value of the field is `MASTER_REQUEST`"]
    #[inline(always)]
    pub fn is_master_request(&self) -> bool {
        *self == EVENT_A::MASTER_REQUEST
    }
    #[doc = "Checks if the value of the field is `HOT_JOIN_REQUEST`"]
    #[inline(always)]
    pub fn is_hot_join_request(&self) -> bool {
        *self == EVENT_A::HOT_JOIN_REQUEST
    }
}
#[doc = "Field `EVENT` writer - EVENT"]
pub type EVENT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SCTRL_SPEC, u8, EVENT_A, 2, O>;
impl<'a, const O: u8> EVENT_W<'a, O> {
    #[doc = "NORMAL_MODE"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(EVENT_A::NORMAL_MODE)
    }
    #[doc = "IBI"]
    #[inline(always)]
    pub fn ibi(self) -> &'a mut W {
        self.variant(EVENT_A::IBI)
    }
    #[doc = "MASTER_REQUEST"]
    #[inline(always)]
    pub fn master_request(self) -> &'a mut W {
        self.variant(EVENT_A::MASTER_REQUEST)
    }
    #[doc = "HOT_JOIN_REQUEST"]
    #[inline(always)]
    pub fn hot_join_request(self) -> &'a mut W {
        self.variant(EVENT_A::HOT_JOIN_REQUEST)
    }
}
#[doc = "Field `IBIDATA` reader - In-Band Interrupt Data"]
pub type IBIDATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIDATA` writer - In-Band Interrupt Data"]
pub type IBIDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `PENDINT` reader - Pending interrupt"]
pub type PENDINT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PENDINT` writer - Pending interrupt"]
pub type PENDINT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `ACTSTATE` reader - Activity state (of slave)"]
pub type ACTSTATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACTSTATE` writer - Activity state (of slave)"]
pub type ACTSTATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `VENDINFO` reader - Vendor information"]
pub type VENDINFO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VENDINFO` writer - Vendor information"]
pub type VENDINFO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:1 - EVENT"]
    #[inline(always)]
    pub fn event(&self) -> EVENT_R {
        EVENT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:15 - In-Band Interrupt Data"]
    #[inline(always)]
    pub fn ibidata(&self) -> IBIDATA_R {
        IBIDATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Pending interrupt"]
    #[inline(always)]
    pub fn pendint(&self) -> PENDINT_R {
        PENDINT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Activity state (of slave)"]
    #[inline(always)]
    pub fn actstate(&self) -> ACTSTATE_R {
        ACTSTATE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Vendor information"]
    #[inline(always)]
    pub fn vendinfo(&self) -> VENDINFO_R {
        VENDINFO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - EVENT"]
    #[inline(always)]
    #[must_use]
    pub fn event(&mut self) -> EVENT_W<0> {
        EVENT_W::new(self)
    }
    #[doc = "Bits 8:15 - In-Band Interrupt Data"]
    #[inline(always)]
    #[must_use]
    pub fn ibidata(&mut self) -> IBIDATA_W<8> {
        IBIDATA_W::new(self)
    }
    #[doc = "Bits 16:19 - Pending interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pendint(&mut self) -> PENDINT_W<16> {
        PENDINT_W::new(self)
    }
    #[doc = "Bits 20:21 - Activity state (of slave)"]
    #[inline(always)]
    #[must_use]
    pub fn actstate(&mut self) -> ACTSTATE_W<20> {
        ACTSTATE_W::new(self)
    }
    #[doc = "Bits 24:31 - Vendor information"]
    #[inline(always)]
    #[must_use]
    pub fn vendinfo(&mut self) -> VENDINFO_W<24> {
        VENDINFO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctrl](index.html) module"]
pub struct SCTRL_SPEC;
impl crate::RegisterSpec for SCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sctrl::R](R) reader structure"]
impl crate::Readable for SCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctrl::W](W) writer structure"]
impl crate::Writable for SCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTRL to value 0"]
impl crate::Resettable for SCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
