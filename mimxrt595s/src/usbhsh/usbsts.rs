#[doc = "Register `USBSTS` reader"]
pub struct R(crate::R<USBSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBSTS` writer"]
pub struct W(crate::W<USBSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBSTS_SPEC>;
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
impl From<crate::W<USBSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCD` reader - Port Change Detect Interrupt Request"]
pub type PCD_R = crate::BitReader<PCD_A>;
#[doc = "Port Change Detect Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCD_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<PCD_A> for bool {
    #[inline(always)]
    fn from(variant: PCD_A) -> Self {
        variant as u8 != 0
    }
}
impl PCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCD_A {
        match self.bits {
            false => PCD_A::DISABLE,
            true => PCD_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PCD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PCD_A::ENABLE
    }
}
#[doc = "Field `PCD` writer - Port Change Detect Interrupt Request"]
pub type PCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, PCD_A, O>;
impl<'a, const O: u8> PCD_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PCD_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PCD_A::ENABLE)
    }
}
#[doc = "Field `FLR` reader - Frame List Rollover Interrupt Request"]
pub type FLR_R = crate::BitReader<bool>;
#[doc = "Field `FLR` writer - Frame List Rollover Interrupt Request"]
pub type FLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `ATL_IRQ` reader - ATL Interrupt Request Interrupt Request"]
pub type ATL_IRQ_R = crate::BitReader<ATL_IRQ_A>;
#[doc = "ATL Interrupt Request Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATL_IRQ_A {
    #[doc = "0: No ATL PTD event occurred."]
    ENABLE = 0,
    #[doc = "1: ATL PTD event occurred."]
    DISABLE = 1,
}
impl From<ATL_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: ATL_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl ATL_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATL_IRQ_A {
        match self.bits {
            false => ATL_IRQ_A::ENABLE,
            true => ATL_IRQ_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ATL_IRQ_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ATL_IRQ_A::DISABLE
    }
}
#[doc = "Field `ATL_IRQ` writer - ATL Interrupt Request Interrupt Request"]
pub type ATL_IRQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, USBSTS_SPEC, ATL_IRQ_A, O>;
impl<'a, const O: u8> ATL_IRQ_W<'a, O> {
    #[doc = "No ATL PTD event occurred."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ATL_IRQ_A::ENABLE)
    }
    #[doc = "ATL PTD event occurred."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ATL_IRQ_A::DISABLE)
    }
}
#[doc = "Field `ISO_IRQ` reader - ISO Interrupt Request"]
pub type ISO_IRQ_R = crate::BitReader<ISO_IRQ_A>;
#[doc = "ISO Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISO_IRQ_A {
    #[doc = "0: No ISO PTD event occurred."]
    ENABLE = 0,
    #[doc = "1: ISO PTD event occurred."]
    DISABLE = 1,
}
impl From<ISO_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: ISO_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl ISO_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISO_IRQ_A {
        match self.bits {
            false => ISO_IRQ_A::ENABLE,
            true => ISO_IRQ_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ISO_IRQ_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ISO_IRQ_A::DISABLE
    }
}
#[doc = "Field `ISO_IRQ` writer - ISO Interrupt Request"]
pub type ISO_IRQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, USBSTS_SPEC, ISO_IRQ_A, O>;
impl<'a, const O: u8> ISO_IRQ_W<'a, O> {
    #[doc = "No ISO PTD event occurred."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ISO_IRQ_A::ENABLE)
    }
    #[doc = "ISO PTD event occurred."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ISO_IRQ_A::DISABLE)
    }
}
#[doc = "Field `INT_IRQ` reader - INT Interrupt Request"]
pub type INT_IRQ_R = crate::BitReader<INT_IRQ_A>;
#[doc = "INT Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_IRQ_A {
    #[doc = "0: No INT PTD event occurred."]
    ENABLE = 0,
    #[doc = "1: INT PTD event occurred."]
    DISABLE = 1,
}
impl From<INT_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: INT_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_IRQ_A {
        match self.bits {
            false => INT_IRQ_A::ENABLE,
            true => INT_IRQ_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INT_IRQ_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INT_IRQ_A::DISABLE
    }
}
#[doc = "Field `INT_IRQ` writer - INT Interrupt Request"]
pub type INT_IRQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, USBSTS_SPEC, INT_IRQ_A, O>;
impl<'a, const O: u8> INT_IRQ_W<'a, O> {
    #[doc = "No INT PTD event occurred."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INT_IRQ_A::ENABLE)
    }
    #[doc = "INT PTD event occurred."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INT_IRQ_A::DISABLE)
    }
}
#[doc = "Field `SOF_IRQ` reader - SOF Interrupt Request"]
pub type SOF_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `SOF_IRQ` writer - SOF Interrupt Request"]
pub type SOF_IRQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, USBSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Port Change Detect Interrupt Request"]
    #[inline(always)]
    pub fn pcd(&self) -> PCD_R {
        PCD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover Interrupt Request"]
    #[inline(always)]
    pub fn flr(&self) -> FLR_R {
        FLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - ATL Interrupt Request Interrupt Request"]
    #[inline(always)]
    pub fn atl_irq(&self) -> ATL_IRQ_R {
        ATL_IRQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ISO Interrupt Request"]
    #[inline(always)]
    pub fn iso_irq(&self) -> ISO_IRQ_R {
        ISO_IRQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - INT Interrupt Request"]
    #[inline(always)]
    pub fn int_irq(&self) -> INT_IRQ_R {
        INT_IRQ_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SOF Interrupt Request"]
    #[inline(always)]
    pub fn sof_irq(&self) -> SOF_IRQ_R {
        SOF_IRQ_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Port Change Detect Interrupt Request"]
    #[inline(always)]
    #[must_use]
    pub fn pcd(&mut self) -> PCD_W<2> {
        PCD_W::new(self)
    }
    #[doc = "Bit 3 - Frame List Rollover Interrupt Request"]
    #[inline(always)]
    #[must_use]
    pub fn flr(&mut self) -> FLR_W<3> {
        FLR_W::new(self)
    }
    #[doc = "Bit 16 - ATL Interrupt Request Interrupt Request"]
    #[inline(always)]
    #[must_use]
    pub fn atl_irq(&mut self) -> ATL_IRQ_W<16> {
        ATL_IRQ_W::new(self)
    }
    #[doc = "Bit 17 - ISO Interrupt Request"]
    #[inline(always)]
    #[must_use]
    pub fn iso_irq(&mut self) -> ISO_IRQ_W<17> {
        ISO_IRQ_W::new(self)
    }
    #[doc = "Bit 18 - INT Interrupt Request"]
    #[inline(always)]
    #[must_use]
    pub fn int_irq(&mut self) -> INT_IRQ_W<18> {
        INT_IRQ_W::new(self)
    }
    #[doc = "Bit 19 - SOF Interrupt Request"]
    #[inline(always)]
    #[must_use]
    pub fn sof_irq(&mut self) -> SOF_IRQ_W<19> {
        SOF_IRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbsts](index.html) module"]
pub struct USBSTS_SPEC;
impl crate::RegisterSpec for USBSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbsts::R](R) reader structure"]
impl crate::Readable for USBSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbsts::W](W) writer structure"]
impl crate::Writable for USBSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x000f_0000;
}
#[doc = "`reset()` method sets USBSTS to value 0"]
impl crate::Resettable for USBSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
