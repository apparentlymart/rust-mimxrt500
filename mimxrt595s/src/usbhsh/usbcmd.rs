#[doc = "Register `USBCMD` reader"]
pub struct R(crate::R<USBCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCMD` writer"]
pub struct W(crate::W<USBCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCMD_SPEC>;
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
impl From<crate::W<USBCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RS` reader - Run/Stop"]
pub type RS_R = crate::BitReader<RS_A>;
#[doc = "Run/Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RS_A {
    #[doc = "0: Stop"]
    DISABLE = 0,
    #[doc = "1: Run"]
    ENABLE = 1,
}
impl From<RS_A> for bool {
    #[inline(always)]
    fn from(variant: RS_A) -> Self {
        variant as u8 != 0
    }
}
impl RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RS_A {
        match self.bits {
            false => RS_A::DISABLE,
            true => RS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RS_A::ENABLE
    }
}
#[doc = "Field `RS` writer - Run/Stop"]
pub type RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, RS_A, O>;
impl<'a, const O: u8> RS_W<'a, O> {
    #[doc = "Stop"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RS_A::DISABLE)
    }
    #[doc = "Run"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RS_A::ENABLE)
    }
}
#[doc = "Field `HCRESET` reader - Host Controller Reset"]
pub type HCRESET_R = crate::BitReader<bool>;
#[doc = "Field `HCRESET` writer - Host Controller Reset"]
pub type HCRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
#[doc = "Field `FLS` reader - Frame List Size"]
pub type FLS_R = crate::FieldReader<u8, FLS_A>;
#[doc = "Frame List Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLS_A {
    #[doc = "0: 1024 elements"]
    FLS_0 = 0,
    #[doc = "1: 512 elements"]
    FLS_01 = 1,
    #[doc = "2: 256 elements"]
    FLS_10 = 2,
}
impl From<FLS_A> for u8 {
    #[inline(always)]
    fn from(variant: FLS_A) -> Self {
        variant as _
    }
}
impl FLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLS_A> {
        match self.bits {
            0 => Some(FLS_A::FLS_0),
            1 => Some(FLS_A::FLS_01),
            2 => Some(FLS_A::FLS_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLS_0`"]
    #[inline(always)]
    pub fn is_fls_0(&self) -> bool {
        *self == FLS_A::FLS_0
    }
    #[doc = "Checks if the value of the field is `FLS_01`"]
    #[inline(always)]
    pub fn is_fls_01(&self) -> bool {
        *self == FLS_A::FLS_01
    }
    #[doc = "Checks if the value of the field is `FLS_10`"]
    #[inline(always)]
    pub fn is_fls_10(&self) -> bool {
        *self == FLS_A::FLS_10
    }
}
#[doc = "Field `FLS` writer - Frame List Size"]
pub type FLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBCMD_SPEC, u8, FLS_A, 2, O>;
impl<'a, const O: u8> FLS_W<'a, O> {
    #[doc = "1024 elements"]
    #[inline(always)]
    pub fn fls_0(self) -> &'a mut W {
        self.variant(FLS_A::FLS_0)
    }
    #[doc = "512 elements"]
    #[inline(always)]
    pub fn fls_01(self) -> &'a mut W {
        self.variant(FLS_A::FLS_01)
    }
    #[doc = "256 elements"]
    #[inline(always)]
    pub fn fls_10(self) -> &'a mut W {
        self.variant(FLS_A::FLS_10)
    }
}
#[doc = "Field `LHCR` reader - Light Host Controller Reset"]
pub type LHCR_R = crate::BitReader<bool>;
#[doc = "Field `LHCR` writer - Light Host Controller Reset"]
pub type LHCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
#[doc = "Field `ATL_EN` reader - ATL List enabled"]
pub type ATL_EN_R = crate::BitReader<bool>;
#[doc = "Field `ATL_EN` writer - ATL List enabled"]
pub type ATL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
#[doc = "Field `ISO_EN` reader - ISO List enabled"]
pub type ISO_EN_R = crate::BitReader<bool>;
#[doc = "Field `ISO_EN` writer - ISO List enabled"]
pub type ISO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
#[doc = "Field `INT_EN` reader - INT List enabled"]
pub type INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `INT_EN` writer - INT List enabled"]
pub type INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
#[doc = "Field `HIRD` reader - Host-Initiated Resume Duration"]
pub type HIRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIRD` writer - Host-Initiated Resume Duration"]
pub type HIRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBCMD_SPEC, u8, u8, 4, O>;
#[doc = "Field `LPM_RWU` reader - Remote wake up."]
pub type LPM_RWU_R = crate::BitReader<bool>;
#[doc = "Field `LPM_RWU` writer - Remote wake up."]
pub type LPM_RWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Run/Stop"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host Controller Reset"]
    #[inline(always)]
    pub fn hcreset(&self) -> HCRESET_R {
        HCRESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Frame List Size"]
    #[inline(always)]
    pub fn fls(&self) -> FLS_R {
        FLS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 7 - Light Host Controller Reset"]
    #[inline(always)]
    pub fn lhcr(&self) -> LHCR_R {
        LHCR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ATL List enabled"]
    #[inline(always)]
    pub fn atl_en(&self) -> ATL_EN_R {
        ATL_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ISO List enabled"]
    #[inline(always)]
    pub fn iso_en(&self) -> ISO_EN_R {
        ISO_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - INT List enabled"]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Host-Initiated Resume Duration"]
    #[inline(always)]
    pub fn hird(&self) -> HIRD_R {
        HIRD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Remote wake up."]
    #[inline(always)]
    pub fn lpm_rwu(&self) -> LPM_RWU_R {
        LPM_RWU_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Run/Stop"]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RS_W<0> {
        RS_W::new(self)
    }
    #[doc = "Bit 1 - Host Controller Reset"]
    #[inline(always)]
    #[must_use]
    pub fn hcreset(&mut self) -> HCRESET_W<1> {
        HCRESET_W::new(self)
    }
    #[doc = "Bits 2:3 - Frame List Size"]
    #[inline(always)]
    #[must_use]
    pub fn fls(&mut self) -> FLS_W<2> {
        FLS_W::new(self)
    }
    #[doc = "Bit 7 - Light Host Controller Reset"]
    #[inline(always)]
    #[must_use]
    pub fn lhcr(&mut self) -> LHCR_W<7> {
        LHCR_W::new(self)
    }
    #[doc = "Bit 8 - ATL List enabled"]
    #[inline(always)]
    #[must_use]
    pub fn atl_en(&mut self) -> ATL_EN_W<8> {
        ATL_EN_W::new(self)
    }
    #[doc = "Bit 9 - ISO List enabled"]
    #[inline(always)]
    #[must_use]
    pub fn iso_en(&mut self) -> ISO_EN_W<9> {
        ISO_EN_W::new(self)
    }
    #[doc = "Bit 10 - INT List enabled"]
    #[inline(always)]
    #[must_use]
    pub fn int_en(&mut self) -> INT_EN_W<10> {
        INT_EN_W::new(self)
    }
    #[doc = "Bits 24:27 - Host-Initiated Resume Duration"]
    #[inline(always)]
    #[must_use]
    pub fn hird(&mut self) -> HIRD_W<24> {
        HIRD_W::new(self)
    }
    #[doc = "Bit 28 - Remote wake up."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_rwu(&mut self) -> LPM_RWU_W<28> {
        LPM_RWU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbcmd](index.html) module"]
pub struct USBCMD_SPEC;
impl crate::RegisterSpec for USBCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbcmd::R](R) reader structure"]
impl crate::Readable for USBCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbcmd::W](W) writer structure"]
impl crate::Writable for USBCMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBCMD to value 0"]
impl crate::Resettable for USBCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
