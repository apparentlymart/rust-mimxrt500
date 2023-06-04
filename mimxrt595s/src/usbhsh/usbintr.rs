#[doc = "Register `USBINTR` reader"]
pub struct R(crate::R<USBINTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBINTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBINTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBINTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBINTR` writer"]
pub struct W(crate::W<USBINTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBINTR_SPEC>;
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
impl From<crate::W<USBINTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBINTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCDE` reader - Port Change Detect Interrupt Enable"]
pub type PCDE_R = crate::BitReader<PCDE_A>;
#[doc = "Port Change Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCDE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<PCDE_A> for bool {
    #[inline(always)]
    fn from(variant: PCDE_A) -> Self {
        variant as u8 != 0
    }
}
impl PCDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCDE_A {
        match self.bits {
            false => PCDE_A::DISABLE,
            true => PCDE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PCDE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PCDE_A::ENABLE
    }
}
#[doc = "Field `PCDE` writer - Port Change Detect Interrupt Enable"]
pub type PCDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBINTR_SPEC, PCDE_A, O>;
impl<'a, const O: u8> PCDE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PCDE_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PCDE_A::ENABLE)
    }
}
#[doc = "Field `FLRE` reader - Frame List Rollover Interrupt Enable"]
pub type FLRE_R = crate::BitReader<FLRE_A>;
#[doc = "Frame List Rollover Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLRE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FLRE_A> for bool {
    #[inline(always)]
    fn from(variant: FLRE_A) -> Self {
        variant as u8 != 0
    }
}
impl FLRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLRE_A {
        match self.bits {
            false => FLRE_A::DISABLE,
            true => FLRE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLRE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLRE_A::ENABLE
    }
}
#[doc = "Field `FLRE` writer - Frame List Rollover Interrupt Enable"]
pub type FLRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBINTR_SPEC, FLRE_A, O>;
impl<'a, const O: u8> FLRE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLRE_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLRE_A::ENABLE)
    }
}
#[doc = "Field `ATL_IRQ_E` reader - ATL Interrupt Enable"]
pub type ATL_IRQ_E_R = crate::BitReader<ATL_IRQ_E_A>;
#[doc = "ATL Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATL_IRQ_E_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ATL_IRQ_E_A> for bool {
    #[inline(always)]
    fn from(variant: ATL_IRQ_E_A) -> Self {
        variant as u8 != 0
    }
}
impl ATL_IRQ_E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATL_IRQ_E_A {
        match self.bits {
            false => ATL_IRQ_E_A::DISABLE,
            true => ATL_IRQ_E_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ATL_IRQ_E_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ATL_IRQ_E_A::ENABLE
    }
}
#[doc = "Field `ATL_IRQ_E` writer - ATL Interrupt Enable"]
pub type ATL_IRQ_E_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, USBINTR_SPEC, ATL_IRQ_E_A, O>;
impl<'a, const O: u8> ATL_IRQ_E_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ATL_IRQ_E_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ATL_IRQ_E_A::ENABLE)
    }
}
#[doc = "Field `ISO_IRQ_E` reader - ISO Interrupt Enable"]
pub type ISO_IRQ_E_R = crate::BitReader<ISO_IRQ_E_A>;
#[doc = "ISO Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISO_IRQ_E_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ISO_IRQ_E_A> for bool {
    #[inline(always)]
    fn from(variant: ISO_IRQ_E_A) -> Self {
        variant as u8 != 0
    }
}
impl ISO_IRQ_E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISO_IRQ_E_A {
        match self.bits {
            false => ISO_IRQ_E_A::DISABLE,
            true => ISO_IRQ_E_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ISO_IRQ_E_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ISO_IRQ_E_A::ENABLE
    }
}
#[doc = "Field `ISO_IRQ_E` writer - ISO Interrupt Enable"]
pub type ISO_IRQ_E_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, USBINTR_SPEC, ISO_IRQ_E_A, O>;
impl<'a, const O: u8> ISO_IRQ_E_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ISO_IRQ_E_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ISO_IRQ_E_A::ENABLE)
    }
}
#[doc = "Field `INT_IRQ_E` reader - INT Interrupt Enable"]
pub type INT_IRQ_E_R = crate::BitReader<INT_IRQ_E_A>;
#[doc = "INT Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_IRQ_E_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<INT_IRQ_E_A> for bool {
    #[inline(always)]
    fn from(variant: INT_IRQ_E_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_IRQ_E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_IRQ_E_A {
        match self.bits {
            false => INT_IRQ_E_A::DISABLE,
            true => INT_IRQ_E_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INT_IRQ_E_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INT_IRQ_E_A::ENABLE
    }
}
#[doc = "Field `INT_IRQ_E` writer - INT Interrupt Enable"]
pub type INT_IRQ_E_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, USBINTR_SPEC, INT_IRQ_E_A, O>;
impl<'a, const O: u8> INT_IRQ_E_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INT_IRQ_E_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INT_IRQ_E_A::ENABLE)
    }
}
#[doc = "Field `SOF_E` reader - SOF Interrupt Request"]
pub type SOF_E_R = crate::BitReader<bool>;
#[doc = "Field `SOF_E` writer - SOF Interrupt Request"]
pub type SOF_E_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, USBINTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Port Change Detect Interrupt Enable"]
    #[inline(always)]
    pub fn pcde(&self) -> PCDE_R {
        PCDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover Interrupt Enable"]
    #[inline(always)]
    pub fn flre(&self) -> FLRE_R {
        FLRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - ATL Interrupt Enable"]
    #[inline(always)]
    pub fn atl_irq_e(&self) -> ATL_IRQ_E_R {
        ATL_IRQ_E_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ISO Interrupt Enable"]
    #[inline(always)]
    pub fn iso_irq_e(&self) -> ISO_IRQ_E_R {
        ISO_IRQ_E_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - INT Interrupt Enable"]
    #[inline(always)]
    pub fn int_irq_e(&self) -> INT_IRQ_E_R {
        INT_IRQ_E_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SOF Interrupt Request"]
    #[inline(always)]
    pub fn sof_e(&self) -> SOF_E_R {
        SOF_E_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Port Change Detect Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcde(&mut self) -> PCDE_W<2> {
        PCDE_W::new(self)
    }
    #[doc = "Bit 3 - Frame List Rollover Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flre(&mut self) -> FLRE_W<3> {
        FLRE_W::new(self)
    }
    #[doc = "Bit 16 - ATL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn atl_irq_e(&mut self) -> ATL_IRQ_E_W<16> {
        ATL_IRQ_E_W::new(self)
    }
    #[doc = "Bit 17 - ISO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iso_irq_e(&mut self) -> ISO_IRQ_E_W<17> {
        ISO_IRQ_E_W::new(self)
    }
    #[doc = "Bit 18 - INT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int_irq_e(&mut self) -> INT_IRQ_E_W<18> {
        INT_IRQ_E_W::new(self)
    }
    #[doc = "Bit 19 - SOF Interrupt Request"]
    #[inline(always)]
    #[must_use]
    pub fn sof_e(&mut self) -> SOF_E_W<19> {
        SOF_E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbintr](index.html) module"]
pub struct USBINTR_SPEC;
impl crate::RegisterSpec for USBINTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbintr::R](R) reader structure"]
impl crate::Readable for USBINTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbintr::W](W) writer structure"]
impl crate::Writable for USBINTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x000f_0000;
}
#[doc = "`reset()` method sets USBINTR to value 0"]
impl crate::Resettable for USBINTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
