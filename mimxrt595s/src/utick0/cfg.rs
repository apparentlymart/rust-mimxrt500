#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPEN0` reader - Enable Capture 0"]
pub type CAPEN0_R = crate::BitReader<CAPEN0_A>;
#[doc = "Enable Capture 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPEN0_A {
    #[doc = "0: Disabled"]
    CAPEN0ISDISABLED = 0,
    #[doc = "1: Enabled"]
    CAPEN0ISENABLED = 1,
}
impl From<CAPEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CAPEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl CAPEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPEN0_A {
        match self.bits {
            false => CAPEN0_A::CAPEN0ISDISABLED,
            true => CAPEN0_A::CAPEN0ISENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `CAPEN0ISDISABLED`"]
    #[inline(always)]
    pub fn is_capen0isdisabled(&self) -> bool {
        *self == CAPEN0_A::CAPEN0ISDISABLED
    }
    #[doc = "Checks if the value of the field is `CAPEN0ISENABLED`"]
    #[inline(always)]
    pub fn is_capen0isenabled(&self) -> bool {
        *self == CAPEN0_A::CAPEN0ISENABLED
    }
}
#[doc = "Field `CAPEN0` writer - Enable Capture 0"]
pub type CAPEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, CAPEN0_A, O>;
impl<'a, const O: u8> CAPEN0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn capen0isdisabled(self) -> &'a mut W {
        self.variant(CAPEN0_A::CAPEN0ISDISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn capen0isenabled(self) -> &'a mut W {
        self.variant(CAPEN0_A::CAPEN0ISENABLED)
    }
}
#[doc = "Field `CAPEN1` reader - Enable Capture 1"]
pub type CAPEN1_R = crate::BitReader<CAPEN1_A>;
#[doc = "Enable Capture 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPEN1_A {
    #[doc = "0: Disabled"]
    CAPEN1ISDISABLED = 0,
    #[doc = "1: Enabled"]
    CAPEN1ISENABLED = 1,
}
impl From<CAPEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CAPEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl CAPEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPEN1_A {
        match self.bits {
            false => CAPEN1_A::CAPEN1ISDISABLED,
            true => CAPEN1_A::CAPEN1ISENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `CAPEN1ISDISABLED`"]
    #[inline(always)]
    pub fn is_capen1isdisabled(&self) -> bool {
        *self == CAPEN1_A::CAPEN1ISDISABLED
    }
    #[doc = "Checks if the value of the field is `CAPEN1ISENABLED`"]
    #[inline(always)]
    pub fn is_capen1isenabled(&self) -> bool {
        *self == CAPEN1_A::CAPEN1ISENABLED
    }
}
#[doc = "Field `CAPEN1` writer - Enable Capture 1"]
pub type CAPEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, CAPEN1_A, O>;
impl<'a, const O: u8> CAPEN1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn capen1isdisabled(self) -> &'a mut W {
        self.variant(CAPEN1_A::CAPEN1ISDISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn capen1isenabled(self) -> &'a mut W {
        self.variant(CAPEN1_A::CAPEN1ISENABLED)
    }
}
#[doc = "Field `CAPEN2` reader - Enable Capture 2"]
pub type CAPEN2_R = crate::BitReader<CAPEN2_A>;
#[doc = "Enable Capture 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPEN2_A {
    #[doc = "0: Disabled"]
    CAPEN2ISDISABLED = 0,
    #[doc = "1: Enabled"]
    CAPEN2ISENABLED = 1,
}
impl From<CAPEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CAPEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl CAPEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPEN2_A {
        match self.bits {
            false => CAPEN2_A::CAPEN2ISDISABLED,
            true => CAPEN2_A::CAPEN2ISENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `CAPEN2ISDISABLED`"]
    #[inline(always)]
    pub fn is_capen2isdisabled(&self) -> bool {
        *self == CAPEN2_A::CAPEN2ISDISABLED
    }
    #[doc = "Checks if the value of the field is `CAPEN2ISENABLED`"]
    #[inline(always)]
    pub fn is_capen2isenabled(&self) -> bool {
        *self == CAPEN2_A::CAPEN2ISENABLED
    }
}
#[doc = "Field `CAPEN2` writer - Enable Capture 2"]
pub type CAPEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, CAPEN2_A, O>;
impl<'a, const O: u8> CAPEN2_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn capen2isdisabled(self) -> &'a mut W {
        self.variant(CAPEN2_A::CAPEN2ISDISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn capen2isenabled(self) -> &'a mut W {
        self.variant(CAPEN2_A::CAPEN2ISENABLED)
    }
}
#[doc = "Field `CAPEN3` reader - Enable Capture 3"]
pub type CAPEN3_R = crate::BitReader<CAPEN3_A>;
#[doc = "Enable Capture 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPEN3_A {
    #[doc = "0: Disabled"]
    CAPEN3ISDISABLED = 0,
    #[doc = "1: Enabled"]
    CAPEN3ISENABLED = 1,
}
impl From<CAPEN3_A> for bool {
    #[inline(always)]
    fn from(variant: CAPEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl CAPEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPEN3_A {
        match self.bits {
            false => CAPEN3_A::CAPEN3ISDISABLED,
            true => CAPEN3_A::CAPEN3ISENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `CAPEN3ISDISABLED`"]
    #[inline(always)]
    pub fn is_capen3isdisabled(&self) -> bool {
        *self == CAPEN3_A::CAPEN3ISDISABLED
    }
    #[doc = "Checks if the value of the field is `CAPEN3ISENABLED`"]
    #[inline(always)]
    pub fn is_capen3isenabled(&self) -> bool {
        *self == CAPEN3_A::CAPEN3ISENABLED
    }
}
#[doc = "Field `CAPEN3` writer - Enable Capture 3"]
pub type CAPEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, CAPEN3_A, O>;
impl<'a, const O: u8> CAPEN3_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn capen3isdisabled(self) -> &'a mut W {
        self.variant(CAPEN3_A::CAPEN3ISDISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn capen3isenabled(self) -> &'a mut W {
        self.variant(CAPEN3_A::CAPEN3ISENABLED)
    }
}
#[doc = "Field `CAPPOL0` reader - Capture Polarity 0"]
pub type CAPPOL0_R = crate::BitReader<CAPPOL0_A>;
#[doc = "Capture Polarity 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPPOL0_A {
    #[doc = "0: Positive edge capture"]
    CAPPOL0POSEDGECAPTURE = 0,
    #[doc = "1: Negative edge capture"]
    CAPPOL0NEGEDGECAPTURE = 1,
}
impl From<CAPPOL0_A> for bool {
    #[inline(always)]
    fn from(variant: CAPPOL0_A) -> Self {
        variant as u8 != 0
    }
}
impl CAPPOL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPPOL0_A {
        match self.bits {
            false => CAPPOL0_A::CAPPOL0POSEDGECAPTURE,
            true => CAPPOL0_A::CAPPOL0NEGEDGECAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `CAPPOL0POSEDGECAPTURE`"]
    #[inline(always)]
    pub fn is_cappol0posedgecapture(&self) -> bool {
        *self == CAPPOL0_A::CAPPOL0POSEDGECAPTURE
    }
    #[doc = "Checks if the value of the field is `CAPPOL0NEGEDGECAPTURE`"]
    #[inline(always)]
    pub fn is_cappol0negedgecapture(&self) -> bool {
        *self == CAPPOL0_A::CAPPOL0NEGEDGECAPTURE
    }
}
#[doc = "Field `CAPPOL0` writer - Capture Polarity 0"]
pub type CAPPOL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, CAPPOL0_A, O>;
impl<'a, const O: u8> CAPPOL0_W<'a, O> {
    #[doc = "Positive edge capture"]
    #[inline(always)]
    pub fn cappol0posedgecapture(self) -> &'a mut W {
        self.variant(CAPPOL0_A::CAPPOL0POSEDGECAPTURE)
    }
    #[doc = "Negative edge capture"]
    #[inline(always)]
    pub fn cappol0negedgecapture(self) -> &'a mut W {
        self.variant(CAPPOL0_A::CAPPOL0NEGEDGECAPTURE)
    }
}
#[doc = "Field `CAPPOL1` reader - Capture Polarity 1"]
pub type CAPPOL1_R = crate::BitReader<CAPPOL1_A>;
#[doc = "Capture Polarity 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPPOL1_A {
    #[doc = "0: Positive edge capture"]
    CAPPOL1POSEDGECAPTURE = 0,
    #[doc = "1: Negative edge capture"]
    CAPPOL1NEGEDGECAPTURE = 1,
}
impl From<CAPPOL1_A> for bool {
    #[inline(always)]
    fn from(variant: CAPPOL1_A) -> Self {
        variant as u8 != 0
    }
}
impl CAPPOL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPPOL1_A {
        match self.bits {
            false => CAPPOL1_A::CAPPOL1POSEDGECAPTURE,
            true => CAPPOL1_A::CAPPOL1NEGEDGECAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `CAPPOL1POSEDGECAPTURE`"]
    #[inline(always)]
    pub fn is_cappol1posedgecapture(&self) -> bool {
        *self == CAPPOL1_A::CAPPOL1POSEDGECAPTURE
    }
    #[doc = "Checks if the value of the field is `CAPPOL1NEGEDGECAPTURE`"]
    #[inline(always)]
    pub fn is_cappol1negedgecapture(&self) -> bool {
        *self == CAPPOL1_A::CAPPOL1NEGEDGECAPTURE
    }
}
#[doc = "Field `CAPPOL1` writer - Capture Polarity 1"]
pub type CAPPOL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, CAPPOL1_A, O>;
impl<'a, const O: u8> CAPPOL1_W<'a, O> {
    #[doc = "Positive edge capture"]
    #[inline(always)]
    pub fn cappol1posedgecapture(self) -> &'a mut W {
        self.variant(CAPPOL1_A::CAPPOL1POSEDGECAPTURE)
    }
    #[doc = "Negative edge capture"]
    #[inline(always)]
    pub fn cappol1negedgecapture(self) -> &'a mut W {
        self.variant(CAPPOL1_A::CAPPOL1NEGEDGECAPTURE)
    }
}
#[doc = "Field `CAPPOL2` reader - Capture Polarity 2"]
pub type CAPPOL2_R = crate::BitReader<CAPPOL2_A>;
#[doc = "Capture Polarity 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPPOL2_A {
    #[doc = "0: Positive edge capture"]
    CAPPOL2POSEDGECAPTURE = 0,
    #[doc = "1: Negative edge capture"]
    CAPPOL2NEGEDGECAPTURE = 1,
}
impl From<CAPPOL2_A> for bool {
    #[inline(always)]
    fn from(variant: CAPPOL2_A) -> Self {
        variant as u8 != 0
    }
}
impl CAPPOL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPPOL2_A {
        match self.bits {
            false => CAPPOL2_A::CAPPOL2POSEDGECAPTURE,
            true => CAPPOL2_A::CAPPOL2NEGEDGECAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `CAPPOL2POSEDGECAPTURE`"]
    #[inline(always)]
    pub fn is_cappol2posedgecapture(&self) -> bool {
        *self == CAPPOL2_A::CAPPOL2POSEDGECAPTURE
    }
    #[doc = "Checks if the value of the field is `CAPPOL2NEGEDGECAPTURE`"]
    #[inline(always)]
    pub fn is_cappol2negedgecapture(&self) -> bool {
        *self == CAPPOL2_A::CAPPOL2NEGEDGECAPTURE
    }
}
#[doc = "Field `CAPPOL2` writer - Capture Polarity 2"]
pub type CAPPOL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, CAPPOL2_A, O>;
impl<'a, const O: u8> CAPPOL2_W<'a, O> {
    #[doc = "Positive edge capture"]
    #[inline(always)]
    pub fn cappol2posedgecapture(self) -> &'a mut W {
        self.variant(CAPPOL2_A::CAPPOL2POSEDGECAPTURE)
    }
    #[doc = "Negative edge capture"]
    #[inline(always)]
    pub fn cappol2negedgecapture(self) -> &'a mut W {
        self.variant(CAPPOL2_A::CAPPOL2NEGEDGECAPTURE)
    }
}
#[doc = "Field `CAPPOL3` reader - Capture Polarity 3"]
pub type CAPPOL3_R = crate::BitReader<CAPPOL3_A>;
#[doc = "Capture Polarity 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPPOL3_A {
    #[doc = "0: Positive edge capture"]
    CAPPOL3POSEDGECAPTURE = 0,
    #[doc = "1: Negative edge capture"]
    CAPPOL3NEGEDGECAPTURE = 1,
}
impl From<CAPPOL3_A> for bool {
    #[inline(always)]
    fn from(variant: CAPPOL3_A) -> Self {
        variant as u8 != 0
    }
}
impl CAPPOL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPPOL3_A {
        match self.bits {
            false => CAPPOL3_A::CAPPOL3POSEDGECAPTURE,
            true => CAPPOL3_A::CAPPOL3NEGEDGECAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `CAPPOL3POSEDGECAPTURE`"]
    #[inline(always)]
    pub fn is_cappol3posedgecapture(&self) -> bool {
        *self == CAPPOL3_A::CAPPOL3POSEDGECAPTURE
    }
    #[doc = "Checks if the value of the field is `CAPPOL3NEGEDGECAPTURE`"]
    #[inline(always)]
    pub fn is_cappol3negedgecapture(&self) -> bool {
        *self == CAPPOL3_A::CAPPOL3NEGEDGECAPTURE
    }
}
#[doc = "Field `CAPPOL3` writer - Capture Polarity 3"]
pub type CAPPOL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, CAPPOL3_A, O>;
impl<'a, const O: u8> CAPPOL3_W<'a, O> {
    #[doc = "Positive edge capture"]
    #[inline(always)]
    pub fn cappol3posedgecapture(self) -> &'a mut W {
        self.variant(CAPPOL3_A::CAPPOL3POSEDGECAPTURE)
    }
    #[doc = "Negative edge capture"]
    #[inline(always)]
    pub fn cappol3negedgecapture(self) -> &'a mut W {
        self.variant(CAPPOL3_A::CAPPOL3NEGEDGECAPTURE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Capture 0"]
    #[inline(always)]
    pub fn capen0(&self) -> CAPEN0_R {
        CAPEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Capture 1"]
    #[inline(always)]
    pub fn capen1(&self) -> CAPEN1_R {
        CAPEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Capture 2"]
    #[inline(always)]
    pub fn capen2(&self) -> CAPEN2_R {
        CAPEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Capture 3"]
    #[inline(always)]
    pub fn capen3(&self) -> CAPEN3_R {
        CAPEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture Polarity 0"]
    #[inline(always)]
    pub fn cappol0(&self) -> CAPPOL0_R {
        CAPPOL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture Polarity 1"]
    #[inline(always)]
    pub fn cappol1(&self) -> CAPPOL1_R {
        CAPPOL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture Polarity 2"]
    #[inline(always)]
    pub fn cappol2(&self) -> CAPPOL2_R {
        CAPPOL2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture Polarity 3"]
    #[inline(always)]
    pub fn cappol3(&self) -> CAPPOL3_R {
        CAPPOL3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Capture 0"]
    #[inline(always)]
    #[must_use]
    pub fn capen0(&mut self) -> CAPEN0_W<0> {
        CAPEN0_W::new(self)
    }
    #[doc = "Bit 1 - Enable Capture 1"]
    #[inline(always)]
    #[must_use]
    pub fn capen1(&mut self) -> CAPEN1_W<1> {
        CAPEN1_W::new(self)
    }
    #[doc = "Bit 2 - Enable Capture 2"]
    #[inline(always)]
    #[must_use]
    pub fn capen2(&mut self) -> CAPEN2_W<2> {
        CAPEN2_W::new(self)
    }
    #[doc = "Bit 3 - Enable Capture 3"]
    #[inline(always)]
    #[must_use]
    pub fn capen3(&mut self) -> CAPEN3_W<3> {
        CAPEN3_W::new(self)
    }
    #[doc = "Bit 8 - Capture Polarity 0"]
    #[inline(always)]
    #[must_use]
    pub fn cappol0(&mut self) -> CAPPOL0_W<8> {
        CAPPOL0_W::new(self)
    }
    #[doc = "Bit 9 - Capture Polarity 1"]
    #[inline(always)]
    #[must_use]
    pub fn cappol1(&mut self) -> CAPPOL1_W<9> {
        CAPPOL1_W::new(self)
    }
    #[doc = "Bit 10 - Capture Polarity 2"]
    #[inline(always)]
    #[must_use]
    pub fn cappol2(&mut self) -> CAPPOL2_W<10> {
        CAPPOL2_W::new(self)
    }
    #[doc = "Bit 11 - Capture Polarity 3"]
    #[inline(always)]
    #[must_use]
    pub fn cappol3(&mut self) -> CAPPOL3_W<11> {
        CAPPOL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
