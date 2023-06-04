#[doc = "Register `PWD_TOG` reader"]
pub struct R(crate::R<PWD_TOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWD_TOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWD_TOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWD_TOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWD_TOG` writer"]
pub struct W(crate::W<PWD_TOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWD_TOG_SPEC>;
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
impl From<crate::W<PWD_TOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWD_TOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXPWDFS` reader - Power down USB FS drivers."]
pub type TXPWDFS_R = crate::BitReader<bool>;
#[doc = "Field `TXPWDFS` writer - Power down USB FS drivers."]
pub type TXPWDFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_TOG_SPEC, bool, O>;
#[doc = "Field `TXPWDIBIAS` reader - Power down USB PHY current bias block."]
pub type TXPWDIBIAS_R = crate::BitReader<TXPWDIBIAS_A>;
#[doc = "Power down USB PHY current bias block.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXPWDIBIAS_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding PWD bit"]
    ENABLE = 1,
}
impl From<TXPWDIBIAS_A> for bool {
    #[inline(always)]
    fn from(variant: TXPWDIBIAS_A) -> Self {
        variant as u8 != 0
    }
}
impl TXPWDIBIAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPWDIBIAS_A {
        match self.bits {
            false => TXPWDIBIAS_A::DISABLE,
            true => TXPWDIBIAS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXPWDIBIAS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXPWDIBIAS_A::ENABLE
    }
}
#[doc = "Field `TXPWDIBIAS` writer - Power down USB PHY current bias block."]
pub type TXPWDIBIAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_TOG_SPEC, TXPWDIBIAS_A, O>;
impl<'a, const O: u8> TXPWDIBIAS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXPWDIBIAS_A::DISABLE)
    }
    #[doc = "Toggles the corresponding PWD bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXPWDIBIAS_A::ENABLE)
    }
}
#[doc = "Field `TXPWDV2I` reader - Power down USB PHY V-I converter and current mirror."]
pub type TXPWDV2I_R = crate::BitReader<TXPWDV2I_A>;
#[doc = "Power down USB PHY V-I converter and current mirror.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXPWDV2I_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding PWD bit"]
    ENABLE = 1,
}
impl From<TXPWDV2I_A> for bool {
    #[inline(always)]
    fn from(variant: TXPWDV2I_A) -> Self {
        variant as u8 != 0
    }
}
impl TXPWDV2I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPWDV2I_A {
        match self.bits {
            false => TXPWDV2I_A::DISABLE,
            true => TXPWDV2I_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXPWDV2I_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXPWDV2I_A::ENABLE
    }
}
#[doc = "Field `TXPWDV2I` writer - Power down USB PHY V-I converter and current mirror."]
pub type TXPWDV2I_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_TOG_SPEC, TXPWDV2I_A, O>;
impl<'a, const O: u8> TXPWDV2I_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXPWDV2I_A::DISABLE)
    }
    #[doc = "Toggles the corresponding PWD bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXPWDV2I_A::ENABLE)
    }
}
#[doc = "Field `RXPWDENV` reader - Power down USB HS receiver envelope detector."]
pub type RXPWDENV_R = crate::BitReader<RXPWDENV_A>;
#[doc = "Power down USB HS receiver envelope detector.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXPWDENV_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding PWD bit"]
    ENABLE = 1,
}
impl From<RXPWDENV_A> for bool {
    #[inline(always)]
    fn from(variant: RXPWDENV_A) -> Self {
        variant as u8 != 0
    }
}
impl RXPWDENV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPWDENV_A {
        match self.bits {
            false => RXPWDENV_A::DISABLE,
            true => RXPWDENV_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXPWDENV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXPWDENV_A::ENABLE
    }
}
#[doc = "Field `RXPWDENV` writer - Power down USB HS receiver envelope detector."]
pub type RXPWDENV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_TOG_SPEC, RXPWDENV_A, O>;
impl<'a, const O: u8> RXPWDENV_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXPWDENV_A::DISABLE)
    }
    #[doc = "Toggles the corresponding PWD bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXPWDENV_A::ENABLE)
    }
}
#[doc = "Field `RXPWD1PT1` reader - Power down USB FS differential receiver."]
pub type RXPWD1PT1_R = crate::BitReader<RXPWD1PT1_A>;
#[doc = "Power down USB FS differential receiver.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXPWD1PT1_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding PWD bit"]
    ENABLE = 1,
}
impl From<RXPWD1PT1_A> for bool {
    #[inline(always)]
    fn from(variant: RXPWD1PT1_A) -> Self {
        variant as u8 != 0
    }
}
impl RXPWD1PT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPWD1PT1_A {
        match self.bits {
            false => RXPWD1PT1_A::DISABLE,
            true => RXPWD1PT1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXPWD1PT1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXPWD1PT1_A::ENABLE
    }
}
#[doc = "Field `RXPWD1PT1` writer - Power down USB FS differential receiver."]
pub type RXPWD1PT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_TOG_SPEC, RXPWD1PT1_A, O>;
impl<'a, const O: u8> RXPWD1PT1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXPWD1PT1_A::DISABLE)
    }
    #[doc = "Toggles the corresponding PWD bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXPWD1PT1_A::ENABLE)
    }
}
#[doc = "Field `RXPWDDIFF` reader - Power down USB HS differential receiver."]
pub type RXPWDDIFF_R = crate::BitReader<RXPWDDIFF_A>;
#[doc = "Power down USB HS differential receiver.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXPWDDIFF_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding PWD bit"]
    ENABLE = 1,
}
impl From<RXPWDDIFF_A> for bool {
    #[inline(always)]
    fn from(variant: RXPWDDIFF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXPWDDIFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPWDDIFF_A {
        match self.bits {
            false => RXPWDDIFF_A::DISABLE,
            true => RXPWDDIFF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXPWDDIFF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXPWDDIFF_A::ENABLE
    }
}
#[doc = "Field `RXPWDDIFF` writer - Power down USB HS differential receiver."]
pub type RXPWDDIFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_TOG_SPEC, RXPWDDIFF_A, O>;
impl<'a, const O: u8> RXPWDDIFF_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXPWDDIFF_A::DISABLE)
    }
    #[doc = "Toggles the corresponding PWD bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXPWDDIFF_A::ENABLE)
    }
}
#[doc = "Field `RXPWDRX` reader - Power down USB PHY receiver except the FS differential."]
pub type RXPWDRX_R = crate::BitReader<RXPWDRX_A>;
#[doc = "Power down USB PHY receiver except the FS differential.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXPWDRX_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
    #[doc = "1: Toggles the corresponding PWD bit"]
    ENABLE = 1,
}
impl From<RXPWDRX_A> for bool {
    #[inline(always)]
    fn from(variant: RXPWDRX_A) -> Self {
        variant as u8 != 0
    }
}
impl RXPWDRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPWDRX_A {
        match self.bits {
            false => RXPWDRX_A::DISABLE,
            true => RXPWDRX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXPWDRX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXPWDRX_A::ENABLE
    }
}
#[doc = "Field `RXPWDRX` writer - Power down USB PHY receiver except the FS differential."]
pub type RXPWDRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_TOG_SPEC, RXPWDRX_A, O>;
impl<'a, const O: u8> RXPWDRX_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXPWDRX_A::DISABLE)
    }
    #[doc = "Toggles the corresponding PWD bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXPWDRX_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 10 - Power down USB FS drivers."]
    #[inline(always)]
    pub fn txpwdfs(&self) -> TXPWDFS_R {
        TXPWDFS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Power down USB PHY current bias block."]
    #[inline(always)]
    pub fn txpwdibias(&self) -> TXPWDIBIAS_R {
        TXPWDIBIAS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Power down USB PHY V-I converter and current mirror."]
    #[inline(always)]
    pub fn txpwdv2i(&self) -> TXPWDV2I_R {
        TXPWDV2I_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - Power down USB HS receiver envelope detector."]
    #[inline(always)]
    pub fn rxpwdenv(&self) -> RXPWDENV_R {
        RXPWDENV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Power down USB FS differential receiver."]
    #[inline(always)]
    pub fn rxpwd1pt1(&self) -> RXPWD1PT1_R {
        RXPWD1PT1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Power down USB HS differential receiver."]
    #[inline(always)]
    pub fn rxpwddiff(&self) -> RXPWDDIFF_R {
        RXPWDDIFF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Power down USB PHY receiver except the FS differential."]
    #[inline(always)]
    pub fn rxpwdrx(&self) -> RXPWDRX_R {
        RXPWDRX_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Power down USB FS drivers."]
    #[inline(always)]
    #[must_use]
    pub fn txpwdfs(&mut self) -> TXPWDFS_W<10> {
        TXPWDFS_W::new(self)
    }
    #[doc = "Bit 11 - Power down USB PHY current bias block."]
    #[inline(always)]
    #[must_use]
    pub fn txpwdibias(&mut self) -> TXPWDIBIAS_W<11> {
        TXPWDIBIAS_W::new(self)
    }
    #[doc = "Bit 12 - Power down USB PHY V-I converter and current mirror."]
    #[inline(always)]
    #[must_use]
    pub fn txpwdv2i(&mut self) -> TXPWDV2I_W<12> {
        TXPWDV2I_W::new(self)
    }
    #[doc = "Bit 17 - Power down USB HS receiver envelope detector."]
    #[inline(always)]
    #[must_use]
    pub fn rxpwdenv(&mut self) -> RXPWDENV_W<17> {
        RXPWDENV_W::new(self)
    }
    #[doc = "Bit 18 - Power down USB FS differential receiver."]
    #[inline(always)]
    #[must_use]
    pub fn rxpwd1pt1(&mut self) -> RXPWD1PT1_W<18> {
        RXPWD1PT1_W::new(self)
    }
    #[doc = "Bit 19 - Power down USB HS differential receiver."]
    #[inline(always)]
    #[must_use]
    pub fn rxpwddiff(&mut self) -> RXPWDDIFF_W<19> {
        RXPWDDIFF_W::new(self)
    }
    #[doc = "Bit 20 - Power down USB PHY receiver except the FS differential."]
    #[inline(always)]
    #[must_use]
    pub fn rxpwdrx(&mut self) -> RXPWDRX_W<20> {
        RXPWDRX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Down Register Toggle\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwd_tog](index.html) module"]
pub struct PWD_TOG_SPEC;
impl crate::RegisterSpec for PWD_TOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwd_tog::R](R) reader structure"]
impl crate::Readable for PWD_TOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwd_tog::W](W) writer structure"]
impl crate::Writable for PWD_TOG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWD_TOG to value 0x001e_1c00"]
impl crate::Resettable for PWD_TOG_SPEC {
    const RESET_VALUE: Self::Ux = 0x001e_1c00;
}
