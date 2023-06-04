#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOWN_L` reader - Down Counter Low"]
pub type DOWN_L_R = crate::BitReader<DOWN_L_A>;
#[doc = "Down Counter Low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOWN_L_A {
    #[doc = "0: Up. The L or unified counter is counting up."]
    UP = 0,
    #[doc = "1: Down. The L or unified counter is counting down."]
    DOWN = 1,
}
impl From<DOWN_L_A> for bool {
    #[inline(always)]
    fn from(variant: DOWN_L_A) -> Self {
        variant as u8 != 0
    }
}
impl DOWN_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOWN_L_A {
        match self.bits {
            false => DOWN_L_A::UP,
            true => DOWN_L_A::DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == DOWN_L_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == DOWN_L_A::DOWN
    }
}
#[doc = "Field `DOWN_L` writer - Down Counter Low"]
pub type DOWN_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, DOWN_L_A, O>;
impl<'a, const O: u8> DOWN_L_W<'a, O> {
    #[doc = "Up. The L or unified counter is counting up."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(DOWN_L_A::UP)
    }
    #[doc = "Down. The L or unified counter is counting down."]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(DOWN_L_A::DOWN)
    }
}
#[doc = "Field `STOP_L` reader - Stop Counter Low"]
pub type STOP_L_R = crate::BitReader<STOP_L_A>;
#[doc = "Stop Counter Low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_L_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<STOP_L_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_L_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_L_A {
        match self.bits {
            false => STOP_L_A::DISABLE,
            true => STOP_L_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STOP_L_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STOP_L_A::ENABLE
    }
}
#[doc = "Field `STOP_L` writer - Stop Counter Low"]
pub type STOP_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, STOP_L_A, O>;
impl<'a, const O: u8> STOP_L_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(STOP_L_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(STOP_L_A::ENABLE)
    }
}
#[doc = "Field `HALT_L` reader - Halt Counter Low"]
pub type HALT_L_R = crate::BitReader<HALT_L_A>;
#[doc = "Halt Counter Low\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALT_L_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<HALT_L_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_L_A) -> Self {
        variant as u8 != 0
    }
}
impl HALT_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_L_A {
        match self.bits {
            false => HALT_L_A::DISABLE,
            true => HALT_L_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HALT_L_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HALT_L_A::ENABLE
    }
}
#[doc = "Field `HALT_L` writer - Halt Counter Low"]
pub type HALT_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, HALT_L_A, O>;
impl<'a, const O: u8> HALT_L_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HALT_L_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HALT_L_A::ENABLE)
    }
}
#[doc = "Field `CLRCTR_L` reader - Clear Counter Low"]
pub type CLRCTR_L_R = crate::BitReader<bool>;
#[doc = "Field `CLRCTR_L` writer - Clear Counter Low"]
pub type CLRCTR_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BIDIR_L` reader - Bidirectional Select Low"]
pub type BIDIR_L_R = crate::BitReader<BIDIR_L_A>;
#[doc = "Bidirectional Select Low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIDIR_L_A {
    #[doc = "0: Up. The counter counts up to a limit condition, then is cleared to zero."]
    UP = 0,
    #[doc = "1: Up-down. The counter counts up to a limit, then counts down to a limit condition or to 0."]
    UP_DOWN = 1,
}
impl From<BIDIR_L_A> for bool {
    #[inline(always)]
    fn from(variant: BIDIR_L_A) -> Self {
        variant as u8 != 0
    }
}
impl BIDIR_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIDIR_L_A {
        match self.bits {
            false => BIDIR_L_A::UP,
            true => BIDIR_L_A::UP_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == BIDIR_L_A::UP
    }
    #[doc = "Checks if the value of the field is `UP_DOWN`"]
    #[inline(always)]
    pub fn is_up_down(&self) -> bool {
        *self == BIDIR_L_A::UP_DOWN
    }
}
#[doc = "Field `BIDIR_L` writer - Bidirectional Select Low"]
pub type BIDIR_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, BIDIR_L_A, O>;
impl<'a, const O: u8> BIDIR_L_W<'a, O> {
    #[doc = "Up. The counter counts up to a limit condition, then is cleared to zero."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(BIDIR_L_A::UP)
    }
    #[doc = "Up-down. The counter counts up to a limit, then counts down to a limit condition or to 0."]
    #[inline(always)]
    pub fn up_down(self) -> &'a mut W {
        self.variant(BIDIR_L_A::UP_DOWN)
    }
}
#[doc = "Field `PRE_L` reader - Prescaler for Low Counter"]
pub type PRE_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRE_L` writer - Prescaler for Low Counter"]
pub type PRE_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `DOWN_H` reader - Down Counter High"]
pub type DOWN_H_R = crate::BitReader<DOWN_H_A>;
#[doc = "Down Counter High\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOWN_H_A {
    #[doc = "0: Up. The H counter is counting up."]
    UP = 0,
    #[doc = "1: Down. The H counter is counting down."]
    DOWN = 1,
}
impl From<DOWN_H_A> for bool {
    #[inline(always)]
    fn from(variant: DOWN_H_A) -> Self {
        variant as u8 != 0
    }
}
impl DOWN_H_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOWN_H_A {
        match self.bits {
            false => DOWN_H_A::UP,
            true => DOWN_H_A::DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == DOWN_H_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == DOWN_H_A::DOWN
    }
}
#[doc = "Field `DOWN_H` writer - Down Counter High"]
pub type DOWN_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, DOWN_H_A, O>;
impl<'a, const O: u8> DOWN_H_W<'a, O> {
    #[doc = "Up. The H counter is counting up."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(DOWN_H_A::UP)
    }
    #[doc = "Down. The H counter is counting down."]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(DOWN_H_A::DOWN)
    }
}
#[doc = "Field `STOP_H` reader - Stop Counter High"]
pub type STOP_H_R = crate::BitReader<STOP_H_A>;
#[doc = "Stop Counter High\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_H_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<STOP_H_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_H_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_H_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_H_A {
        match self.bits {
            false => STOP_H_A::DISABLE,
            true => STOP_H_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STOP_H_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STOP_H_A::ENABLE
    }
}
#[doc = "Field `STOP_H` writer - Stop Counter High"]
pub type STOP_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, STOP_H_A, O>;
impl<'a, const O: u8> STOP_H_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(STOP_H_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(STOP_H_A::ENABLE)
    }
}
#[doc = "Field `HALT_H` reader - Halt Counter High"]
pub type HALT_H_R = crate::BitReader<HALT_H_A>;
#[doc = "Halt Counter High\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALT_H_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<HALT_H_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_H_A) -> Self {
        variant as u8 != 0
    }
}
impl HALT_H_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_H_A {
        match self.bits {
            false => HALT_H_A::DISABLE,
            true => HALT_H_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HALT_H_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HALT_H_A::ENABLE
    }
}
#[doc = "Field `HALT_H` writer - Halt Counter High"]
pub type HALT_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, HALT_H_A, O>;
impl<'a, const O: u8> HALT_H_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HALT_H_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HALT_H_A::ENABLE)
    }
}
#[doc = "Field `CLRCTR_H` reader - Clear Counter High"]
pub type CLRCTR_H_R = crate::BitReader<bool>;
#[doc = "Field `CLRCTR_H` writer - Clear Counter High"]
pub type CLRCTR_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BIDIR_H` reader - Bidirectional Select High"]
pub type BIDIR_H_R = crate::BitReader<BIDIR_H_A>;
#[doc = "Bidirectional Select High\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIDIR_H_A {
    #[doc = "0: Up. The H counter counts up to its limit condition, then is cleared to zero."]
    UP = 0,
    #[doc = "1: Up-down. The H counter counts up to its limit, then counts down to a limit condition or to 0."]
    UP_DOWN = 1,
}
impl From<BIDIR_H_A> for bool {
    #[inline(always)]
    fn from(variant: BIDIR_H_A) -> Self {
        variant as u8 != 0
    }
}
impl BIDIR_H_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIDIR_H_A {
        match self.bits {
            false => BIDIR_H_A::UP,
            true => BIDIR_H_A::UP_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == BIDIR_H_A::UP
    }
    #[doc = "Checks if the value of the field is `UP_DOWN`"]
    #[inline(always)]
    pub fn is_up_down(&self) -> bool {
        *self == BIDIR_H_A::UP_DOWN
    }
}
#[doc = "Field `BIDIR_H` writer - Bidirectional Select High"]
pub type BIDIR_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, BIDIR_H_A, O>;
impl<'a, const O: u8> BIDIR_H_W<'a, O> {
    #[doc = "Up. The H counter counts up to its limit condition, then is cleared to zero."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(BIDIR_H_A::UP)
    }
    #[doc = "Up-down. The H counter counts up to its limit, then counts down to a limit condition or to 0."]
    #[inline(always)]
    pub fn up_down(self) -> &'a mut W {
        self.variant(BIDIR_H_A::UP_DOWN)
    }
}
#[doc = "Field `PRE_H` reader - Prescaler for High Counter"]
pub type PRE_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRE_H` writer - Prescaler for High Counter"]
pub type PRE_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Down Counter Low"]
    #[inline(always)]
    pub fn down_l(&self) -> DOWN_L_R {
        DOWN_L_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stop Counter Low"]
    #[inline(always)]
    pub fn stop_l(&self) -> STOP_L_R {
        STOP_L_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Halt Counter Low"]
    #[inline(always)]
    pub fn halt_l(&self) -> HALT_L_R {
        HALT_L_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear Counter Low"]
    #[inline(always)]
    pub fn clrctr_l(&self) -> CLRCTR_L_R {
        CLRCTR_L_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bidirectional Select Low"]
    #[inline(always)]
    pub fn bidir_l(&self) -> BIDIR_L_R {
        BIDIR_L_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:12 - Prescaler for Low Counter"]
    #[inline(always)]
    pub fn pre_l(&self) -> PRE_L_R {
        PRE_L_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Down Counter High"]
    #[inline(always)]
    pub fn down_h(&self) -> DOWN_H_R {
        DOWN_H_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Stop Counter High"]
    #[inline(always)]
    pub fn stop_h(&self) -> STOP_H_R {
        STOP_H_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Halt Counter High"]
    #[inline(always)]
    pub fn halt_h(&self) -> HALT_H_R {
        HALT_H_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clear Counter High"]
    #[inline(always)]
    pub fn clrctr_h(&self) -> CLRCTR_H_R {
        CLRCTR_H_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bidirectional Select High"]
    #[inline(always)]
    pub fn bidir_h(&self) -> BIDIR_H_R {
        BIDIR_H_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:28 - Prescaler for High Counter"]
    #[inline(always)]
    pub fn pre_h(&self) -> PRE_H_R {
        PRE_H_R::new(((self.bits >> 21) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Down Counter Low"]
    #[inline(always)]
    #[must_use]
    pub fn down_l(&mut self) -> DOWN_L_W<0> {
        DOWN_L_W::new(self)
    }
    #[doc = "Bit 1 - Stop Counter Low"]
    #[inline(always)]
    #[must_use]
    pub fn stop_l(&mut self) -> STOP_L_W<1> {
        STOP_L_W::new(self)
    }
    #[doc = "Bit 2 - Halt Counter Low"]
    #[inline(always)]
    #[must_use]
    pub fn halt_l(&mut self) -> HALT_L_W<2> {
        HALT_L_W::new(self)
    }
    #[doc = "Bit 3 - Clear Counter Low"]
    #[inline(always)]
    #[must_use]
    pub fn clrctr_l(&mut self) -> CLRCTR_L_W<3> {
        CLRCTR_L_W::new(self)
    }
    #[doc = "Bit 4 - Bidirectional Select Low"]
    #[inline(always)]
    #[must_use]
    pub fn bidir_l(&mut self) -> BIDIR_L_W<4> {
        BIDIR_L_W::new(self)
    }
    #[doc = "Bits 5:12 - Prescaler for Low Counter"]
    #[inline(always)]
    #[must_use]
    pub fn pre_l(&mut self) -> PRE_L_W<5> {
        PRE_L_W::new(self)
    }
    #[doc = "Bit 16 - Down Counter High"]
    #[inline(always)]
    #[must_use]
    pub fn down_h(&mut self) -> DOWN_H_W<16> {
        DOWN_H_W::new(self)
    }
    #[doc = "Bit 17 - Stop Counter High"]
    #[inline(always)]
    #[must_use]
    pub fn stop_h(&mut self) -> STOP_H_W<17> {
        STOP_H_W::new(self)
    }
    #[doc = "Bit 18 - Halt Counter High"]
    #[inline(always)]
    #[must_use]
    pub fn halt_h(&mut self) -> HALT_H_W<18> {
        HALT_H_W::new(self)
    }
    #[doc = "Bit 19 - Clear Counter High"]
    #[inline(always)]
    #[must_use]
    pub fn clrctr_h(&mut self) -> CLRCTR_H_W<19> {
        CLRCTR_H_W::new(self)
    }
    #[doc = "Bit 20 - Bidirectional Select High"]
    #[inline(always)]
    #[must_use]
    pub fn bidir_h(&mut self) -> BIDIR_H_W<20> {
        BIDIR_H_W::new(self)
    }
    #[doc = "Bits 21:28 - Prescaler for High Counter"]
    #[inline(always)]
    #[must_use]
    pub fn pre_h(&mut self) -> PRE_H_W<21> {
        PRE_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0004_0004"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0004;
}
