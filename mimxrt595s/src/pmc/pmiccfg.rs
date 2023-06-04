#[doc = "Register `PMICCFG` reader"]
pub struct R(crate::R<PMICCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMICCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMICCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMICCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMICCFG` writer"]
pub struct W(crate::W<PMICCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMICCFG_SPEC>;
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
impl From<crate::W<PMICCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMICCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDDCOREM0` reader - vddcore state in PMIC mode 0"]
pub type VDDCOREM0_R = crate::BitReader<VDDCOREM0_A>;
#[doc = "vddcore state in PMIC mode 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDCOREM0_A {
    #[doc = "0: Off"]
    DISABLE = 0,
    #[doc = "1: Powered"]
    ENABLE = 1,
}
impl From<VDDCOREM0_A> for bool {
    #[inline(always)]
    fn from(variant: VDDCOREM0_A) -> Self {
        variant as u8 != 0
    }
}
impl VDDCOREM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDCOREM0_A {
        match self.bits {
            false => VDDCOREM0_A::DISABLE,
            true => VDDCOREM0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VDDCOREM0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VDDCOREM0_A::ENABLE
    }
}
#[doc = "Field `VDDCOREM0` writer - vddcore state in PMIC mode 0"]
pub type VDDCOREM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMICCFG_SPEC, VDDCOREM0_A, O>;
impl<'a, const O: u8> VDDCOREM0_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VDDCOREM0_A::DISABLE)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VDDCOREM0_A::ENABLE)
    }
}
#[doc = "Field `VDDCOREM1` reader - vddcore state in PMIC mode 1"]
pub type VDDCOREM1_R = crate::BitReader<VDDCOREM1_A>;
#[doc = "vddcore state in PMIC mode 1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDCOREM1_A {
    #[doc = "0: Off"]
    DISABLE = 0,
    #[doc = "1: Powered"]
    ENABLE = 1,
}
impl From<VDDCOREM1_A> for bool {
    #[inline(always)]
    fn from(variant: VDDCOREM1_A) -> Self {
        variant as u8 != 0
    }
}
impl VDDCOREM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDCOREM1_A {
        match self.bits {
            false => VDDCOREM1_A::DISABLE,
            true => VDDCOREM1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VDDCOREM1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VDDCOREM1_A::ENABLE
    }
}
#[doc = "Field `VDDCOREM1` writer - vddcore state in PMIC mode 1"]
pub type VDDCOREM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMICCFG_SPEC, VDDCOREM1_A, O>;
impl<'a, const O: u8> VDDCOREM1_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VDDCOREM1_A::DISABLE)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VDDCOREM1_A::ENABLE)
    }
}
#[doc = "Field `VDDCOREM2` reader - vddcore state in PMIC mode 2"]
pub type VDDCOREM2_R = crate::BitReader<VDDCOREM2_A>;
#[doc = "vddcore state in PMIC mode 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDCOREM2_A {
    #[doc = "0: Off"]
    DISABLE = 0,
    #[doc = "1: Powered"]
    ENABLE = 1,
}
impl From<VDDCOREM2_A> for bool {
    #[inline(always)]
    fn from(variant: VDDCOREM2_A) -> Self {
        variant as u8 != 0
    }
}
impl VDDCOREM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDCOREM2_A {
        match self.bits {
            false => VDDCOREM2_A::DISABLE,
            true => VDDCOREM2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VDDCOREM2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VDDCOREM2_A::ENABLE
    }
}
#[doc = "Field `VDDCOREM2` writer - vddcore state in PMIC mode 2"]
pub type VDDCOREM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMICCFG_SPEC, VDDCOREM2_A, O>;
impl<'a, const O: u8> VDDCOREM2_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VDDCOREM2_A::DISABLE)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VDDCOREM2_A::ENABLE)
    }
}
#[doc = "Field `VDDCOREM3` reader - vddcore state in PMIC mode 3"]
pub type VDDCOREM3_R = crate::BitReader<VDDCOREM3_A>;
#[doc = "vddcore state in PMIC mode 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDCOREM3_A {
    #[doc = "0: Off"]
    DISABLE = 0,
    #[doc = "1: Powered"]
    ENABLE = 1,
}
impl From<VDDCOREM3_A> for bool {
    #[inline(always)]
    fn from(variant: VDDCOREM3_A) -> Self {
        variant as u8 != 0
    }
}
impl VDDCOREM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDCOREM3_A {
        match self.bits {
            false => VDDCOREM3_A::DISABLE,
            true => VDDCOREM3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VDDCOREM3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VDDCOREM3_A::ENABLE
    }
}
#[doc = "Field `VDDCOREM3` writer - vddcore state in PMIC mode 3"]
pub type VDDCOREM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMICCFG_SPEC, VDDCOREM3_A, O>;
impl<'a, const O: u8> VDDCOREM3_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VDDCOREM3_A::DISABLE)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VDDCOREM3_A::ENABLE)
    }
}
#[doc = "Field `VDD1V8M0` reader - vdd1v8 state in PMIC mode 0"]
pub type VDD1V8M0_R = crate::BitReader<VDD1V8M0_A>;
#[doc = "vdd1v8 state in PMIC mode 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDD1V8M0_A {
    #[doc = "0: Off"]
    DISABLE = 0,
    #[doc = "1: Powered"]
    ENABLE = 1,
}
impl From<VDD1V8M0_A> for bool {
    #[inline(always)]
    fn from(variant: VDD1V8M0_A) -> Self {
        variant as u8 != 0
    }
}
impl VDD1V8M0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDD1V8M0_A {
        match self.bits {
            false => VDD1V8M0_A::DISABLE,
            true => VDD1V8M0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VDD1V8M0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VDD1V8M0_A::ENABLE
    }
}
#[doc = "Field `VDD1V8M0` writer - vdd1v8 state in PMIC mode 0"]
pub type VDD1V8M0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMICCFG_SPEC, VDD1V8M0_A, O>;
impl<'a, const O: u8> VDD1V8M0_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VDD1V8M0_A::DISABLE)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VDD1V8M0_A::ENABLE)
    }
}
#[doc = "Field `VDD1V8M1` reader - vdd1v8 state in PMIC mode 1"]
pub type VDD1V8M1_R = crate::BitReader<VDD1V8M1_A>;
#[doc = "vdd1v8 state in PMIC mode 1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDD1V8M1_A {
    #[doc = "0: Off"]
    DISABLE = 0,
    #[doc = "1: Powered"]
    ENABLE = 1,
}
impl From<VDD1V8M1_A> for bool {
    #[inline(always)]
    fn from(variant: VDD1V8M1_A) -> Self {
        variant as u8 != 0
    }
}
impl VDD1V8M1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDD1V8M1_A {
        match self.bits {
            false => VDD1V8M1_A::DISABLE,
            true => VDD1V8M1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VDD1V8M1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VDD1V8M1_A::ENABLE
    }
}
#[doc = "Field `VDD1V8M1` writer - vdd1v8 state in PMIC mode 1"]
pub type VDD1V8M1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMICCFG_SPEC, VDD1V8M1_A, O>;
impl<'a, const O: u8> VDD1V8M1_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VDD1V8M1_A::DISABLE)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VDD1V8M1_A::ENABLE)
    }
}
#[doc = "Field `VDD1V8M2` reader - vdd1v8 state in PMIC mode 2"]
pub type VDD1V8M2_R = crate::BitReader<VDD1V8M2_A>;
#[doc = "vdd1v8 state in PMIC mode 2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDD1V8M2_A {
    #[doc = "0: Off"]
    DISABLE = 0,
    #[doc = "1: Powered"]
    ENABLE = 1,
}
impl From<VDD1V8M2_A> for bool {
    #[inline(always)]
    fn from(variant: VDD1V8M2_A) -> Self {
        variant as u8 != 0
    }
}
impl VDD1V8M2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDD1V8M2_A {
        match self.bits {
            false => VDD1V8M2_A::DISABLE,
            true => VDD1V8M2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VDD1V8M2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VDD1V8M2_A::ENABLE
    }
}
#[doc = "Field `VDD1V8M2` writer - vdd1v8 state in PMIC mode 2"]
pub type VDD1V8M2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMICCFG_SPEC, VDD1V8M2_A, O>;
impl<'a, const O: u8> VDD1V8M2_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VDD1V8M2_A::DISABLE)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VDD1V8M2_A::ENABLE)
    }
}
#[doc = "Field `VDD1V8M3` reader - vdd1v8 state in PMIC mode 3"]
pub type VDD1V8M3_R = crate::BitReader<VDD1V8M3_A>;
#[doc = "vdd1v8 state in PMIC mode 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDD1V8M3_A {
    #[doc = "0: Off"]
    DISABLE = 0,
    #[doc = "1: Powered"]
    ENABLE = 1,
}
impl From<VDD1V8M3_A> for bool {
    #[inline(always)]
    fn from(variant: VDD1V8M3_A) -> Self {
        variant as u8 != 0
    }
}
impl VDD1V8M3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDD1V8M3_A {
        match self.bits {
            false => VDD1V8M3_A::DISABLE,
            true => VDD1V8M3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VDD1V8M3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VDD1V8M3_A::ENABLE
    }
}
#[doc = "Field `VDD1V8M3` writer - vdd1v8 state in PMIC mode 3"]
pub type VDD1V8M3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMICCFG_SPEC, VDD1V8M3_A, O>;
impl<'a, const O: u8> VDD1V8M3_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VDD1V8M3_A::DISABLE)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VDD1V8M3_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - vddcore state in PMIC mode 0"]
    #[inline(always)]
    pub fn vddcorem0(&self) -> VDDCOREM0_R {
        VDDCOREM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - vddcore state in PMIC mode 1"]
    #[inline(always)]
    pub fn vddcorem1(&self) -> VDDCOREM1_R {
        VDDCOREM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - vddcore state in PMIC mode 2"]
    #[inline(always)]
    pub fn vddcorem2(&self) -> VDDCOREM2_R {
        VDDCOREM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - vddcore state in PMIC mode 3"]
    #[inline(always)]
    pub fn vddcorem3(&self) -> VDDCOREM3_R {
        VDDCOREM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - vdd1v8 state in PMIC mode 0"]
    #[inline(always)]
    pub fn vdd1v8m0(&self) -> VDD1V8M0_R {
        VDD1V8M0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - vdd1v8 state in PMIC mode 1"]
    #[inline(always)]
    pub fn vdd1v8m1(&self) -> VDD1V8M1_R {
        VDD1V8M1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - vdd1v8 state in PMIC mode 2"]
    #[inline(always)]
    pub fn vdd1v8m2(&self) -> VDD1V8M2_R {
        VDD1V8M2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - vdd1v8 state in PMIC mode 3"]
    #[inline(always)]
    pub fn vdd1v8m3(&self) -> VDD1V8M3_R {
        VDD1V8M3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - vddcore state in PMIC mode 0"]
    #[inline(always)]
    #[must_use]
    pub fn vddcorem0(&mut self) -> VDDCOREM0_W<0> {
        VDDCOREM0_W::new(self)
    }
    #[doc = "Bit 1 - vddcore state in PMIC mode 1"]
    #[inline(always)]
    #[must_use]
    pub fn vddcorem1(&mut self) -> VDDCOREM1_W<1> {
        VDDCOREM1_W::new(self)
    }
    #[doc = "Bit 2 - vddcore state in PMIC mode 2"]
    #[inline(always)]
    #[must_use]
    pub fn vddcorem2(&mut self) -> VDDCOREM2_W<2> {
        VDDCOREM2_W::new(self)
    }
    #[doc = "Bit 3 - vddcore state in PMIC mode 3"]
    #[inline(always)]
    #[must_use]
    pub fn vddcorem3(&mut self) -> VDDCOREM3_W<3> {
        VDDCOREM3_W::new(self)
    }
    #[doc = "Bit 4 - vdd1v8 state in PMIC mode 0"]
    #[inline(always)]
    #[must_use]
    pub fn vdd1v8m0(&mut self) -> VDD1V8M0_W<4> {
        VDD1V8M0_W::new(self)
    }
    #[doc = "Bit 5 - vdd1v8 state in PMIC mode 1"]
    #[inline(always)]
    #[must_use]
    pub fn vdd1v8m1(&mut self) -> VDD1V8M1_W<5> {
        VDD1V8M1_W::new(self)
    }
    #[doc = "Bit 6 - vdd1v8 state in PMIC mode 2"]
    #[inline(always)]
    #[must_use]
    pub fn vdd1v8m2(&mut self) -> VDD1V8M2_W<6> {
        VDD1V8M2_W::new(self)
    }
    #[doc = "Bit 7 - vdd1v8 state in PMIC mode 3"]
    #[inline(always)]
    #[must_use]
    pub fn vdd1v8m3(&mut self) -> VDD1V8M3_W<7> {
        VDD1V8M3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMIC Power Mode Select Control Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmiccfg](index.html) module"]
pub struct PMICCFG_SPEC;
impl crate::RegisterSpec for PMICCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmiccfg::R](R) reader structure"]
impl crate::Readable for PMICCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmiccfg::W](W) writer structure"]
impl crate::Writable for PMICCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMICCFG to value 0x73"]
impl crate::Resettable for PMICCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x73;
}
