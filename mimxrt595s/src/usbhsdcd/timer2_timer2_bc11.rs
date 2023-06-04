#[doc = "Register `TIMER2_BC11` reader"]
pub struct R(crate::R<TIMER2_TIMER2_BC11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER2_TIMER2_BC11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER2_TIMER2_BC11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER2_TIMER2_BC11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER2_BC11` writer"]
pub struct W(crate::W<TIMER2_TIMER2_BC11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER2_TIMER2_BC11_SPEC>;
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
impl From<crate::W<TIMER2_TIMER2_BC11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER2_TIMER2_BC11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHECK_DM` reader - Time Before Check of D- Line"]
pub type CHECK_DM_R = crate::FieldReader<u8, CHECK_DM_A>;
#[doc = "Time Before Check of D- Line\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHECK_DM_A {
    #[doc = "1: 1ms - 15ms"]
    MS_1 = 1,
    #[doc = "2: 1ms - 15ms"]
    MS_2 = 2,
    #[doc = "3: 1ms - 15ms"]
    MS_3 = 3,
    #[doc = "4: 1ms - 15ms"]
    MS_4 = 4,
    #[doc = "5: 1ms - 15ms"]
    MS_5 = 5,
    #[doc = "6: 1ms - 15ms"]
    MS_6 = 6,
    #[doc = "7: 1ms - 15ms"]
    MS_7 = 7,
    #[doc = "8: 1ms - 15ms"]
    MS_8 = 8,
    #[doc = "9: 1ms - 15ms"]
    MS_9 = 9,
    #[doc = "10: 1ms - 15ms"]
    MS_10 = 10,
}
impl From<CHECK_DM_A> for u8 {
    #[inline(always)]
    fn from(variant: CHECK_DM_A) -> Self {
        variant as _
    }
}
impl CHECK_DM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHECK_DM_A> {
        match self.bits {
            1 => Some(CHECK_DM_A::MS_1),
            2 => Some(CHECK_DM_A::MS_2),
            3 => Some(CHECK_DM_A::MS_3),
            4 => Some(CHECK_DM_A::MS_4),
            5 => Some(CHECK_DM_A::MS_5),
            6 => Some(CHECK_DM_A::MS_6),
            7 => Some(CHECK_DM_A::MS_7),
            8 => Some(CHECK_DM_A::MS_8),
            9 => Some(CHECK_DM_A::MS_9),
            10 => Some(CHECK_DM_A::MS_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MS_1`"]
    #[inline(always)]
    pub fn is_ms_1(&self) -> bool {
        *self == CHECK_DM_A::MS_1
    }
    #[doc = "Checks if the value of the field is `MS_2`"]
    #[inline(always)]
    pub fn is_ms_2(&self) -> bool {
        *self == CHECK_DM_A::MS_2
    }
    #[doc = "Checks if the value of the field is `MS_3`"]
    #[inline(always)]
    pub fn is_ms_3(&self) -> bool {
        *self == CHECK_DM_A::MS_3
    }
    #[doc = "Checks if the value of the field is `MS_4`"]
    #[inline(always)]
    pub fn is_ms_4(&self) -> bool {
        *self == CHECK_DM_A::MS_4
    }
    #[doc = "Checks if the value of the field is `MS_5`"]
    #[inline(always)]
    pub fn is_ms_5(&self) -> bool {
        *self == CHECK_DM_A::MS_5
    }
    #[doc = "Checks if the value of the field is `MS_6`"]
    #[inline(always)]
    pub fn is_ms_6(&self) -> bool {
        *self == CHECK_DM_A::MS_6
    }
    #[doc = "Checks if the value of the field is `MS_7`"]
    #[inline(always)]
    pub fn is_ms_7(&self) -> bool {
        *self == CHECK_DM_A::MS_7
    }
    #[doc = "Checks if the value of the field is `MS_8`"]
    #[inline(always)]
    pub fn is_ms_8(&self) -> bool {
        *self == CHECK_DM_A::MS_8
    }
    #[doc = "Checks if the value of the field is `MS_9`"]
    #[inline(always)]
    pub fn is_ms_9(&self) -> bool {
        *self == CHECK_DM_A::MS_9
    }
    #[doc = "Checks if the value of the field is `MS_10`"]
    #[inline(always)]
    pub fn is_ms_10(&self) -> bool {
        *self == CHECK_DM_A::MS_10
    }
}
#[doc = "Field `CHECK_DM` writer - Time Before Check of D- Line"]
pub type CHECK_DM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER2_TIMER2_BC11_SPEC, u8, CHECK_DM_A, 4, O>;
impl<'a, const O: u8> CHECK_DM_W<'a, O> {
    #[doc = "1ms - 15ms"]
    #[inline(always)]
    pub fn ms_1(self) -> &'a mut W {
        self.variant(CHECK_DM_A::MS_1)
    }
    #[doc = "1ms - 15ms"]
    #[inline(always)]
    pub fn ms_2(self) -> &'a mut W {
        self.variant(CHECK_DM_A::MS_2)
    }
    #[doc = "1ms - 15ms"]
    #[inline(always)]
    pub fn ms_3(self) -> &'a mut W {
        self.variant(CHECK_DM_A::MS_3)
    }
    #[doc = "1ms - 15ms"]
    #[inline(always)]
    pub fn ms_4(self) -> &'a mut W {
        self.variant(CHECK_DM_A::MS_4)
    }
    #[doc = "1ms - 15ms"]
    #[inline(always)]
    pub fn ms_5(self) -> &'a mut W {
        self.variant(CHECK_DM_A::MS_5)
    }
    #[doc = "1ms - 15ms"]
    #[inline(always)]
    pub fn ms_6(self) -> &'a mut W {
        self.variant(CHECK_DM_A::MS_6)
    }
    #[doc = "1ms - 15ms"]
    #[inline(always)]
    pub fn ms_7(self) -> &'a mut W {
        self.variant(CHECK_DM_A::MS_7)
    }
    #[doc = "1ms - 15ms"]
    #[inline(always)]
    pub fn ms_8(self) -> &'a mut W {
        self.variant(CHECK_DM_A::MS_8)
    }
    #[doc = "1ms - 15ms"]
    #[inline(always)]
    pub fn ms_9(self) -> &'a mut W {
        self.variant(CHECK_DM_A::MS_9)
    }
    #[doc = "1ms - 15ms"]
    #[inline(always)]
    pub fn ms_10(self) -> &'a mut W {
        self.variant(CHECK_DM_A::MS_10)
    }
}
#[doc = "Field `TVDPSRC_CON` reader - Time Period Before Enabling D+ Pullup"]
pub type TVDPSRC_CON_R = crate::FieldReader<u16, TVDPSRC_CON_A>;
#[doc = "Time Period Before Enabling D+ Pullup\n\nValue on reset: 40"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TVDPSRC_CON_A {
    #[doc = "1: 1ms - 1023ms"]
    MS_1 = 1,
    #[doc = "2: 1ms - 1023ms"]
    MS_2 = 2,
    #[doc = "3: 1ms - 1023ms"]
    MS_3 = 3,
    #[doc = "4: 1ms - 1023ms"]
    MS_4 = 4,
    #[doc = "5: 1ms - 1023ms"]
    MS_5 = 5,
    #[doc = "6: 1ms - 1023ms"]
    MS_6 = 6,
    #[doc = "7: 1ms - 1023ms"]
    MS_7 = 7,
    #[doc = "8: 1ms - 1023ms"]
    MS_8 = 8,
    #[doc = "9: 1ms - 1023ms"]
    MS_9 = 9,
    #[doc = "10: 1ms - 1023ms"]
    MS_10 = 10,
}
impl From<TVDPSRC_CON_A> for u16 {
    #[inline(always)]
    fn from(variant: TVDPSRC_CON_A) -> Self {
        variant as _
    }
}
impl TVDPSRC_CON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TVDPSRC_CON_A> {
        match self.bits {
            1 => Some(TVDPSRC_CON_A::MS_1),
            2 => Some(TVDPSRC_CON_A::MS_2),
            3 => Some(TVDPSRC_CON_A::MS_3),
            4 => Some(TVDPSRC_CON_A::MS_4),
            5 => Some(TVDPSRC_CON_A::MS_5),
            6 => Some(TVDPSRC_CON_A::MS_6),
            7 => Some(TVDPSRC_CON_A::MS_7),
            8 => Some(TVDPSRC_CON_A::MS_8),
            9 => Some(TVDPSRC_CON_A::MS_9),
            10 => Some(TVDPSRC_CON_A::MS_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MS_1`"]
    #[inline(always)]
    pub fn is_ms_1(&self) -> bool {
        *self == TVDPSRC_CON_A::MS_1
    }
    #[doc = "Checks if the value of the field is `MS_2`"]
    #[inline(always)]
    pub fn is_ms_2(&self) -> bool {
        *self == TVDPSRC_CON_A::MS_2
    }
    #[doc = "Checks if the value of the field is `MS_3`"]
    #[inline(always)]
    pub fn is_ms_3(&self) -> bool {
        *self == TVDPSRC_CON_A::MS_3
    }
    #[doc = "Checks if the value of the field is `MS_4`"]
    #[inline(always)]
    pub fn is_ms_4(&self) -> bool {
        *self == TVDPSRC_CON_A::MS_4
    }
    #[doc = "Checks if the value of the field is `MS_5`"]
    #[inline(always)]
    pub fn is_ms_5(&self) -> bool {
        *self == TVDPSRC_CON_A::MS_5
    }
    #[doc = "Checks if the value of the field is `MS_6`"]
    #[inline(always)]
    pub fn is_ms_6(&self) -> bool {
        *self == TVDPSRC_CON_A::MS_6
    }
    #[doc = "Checks if the value of the field is `MS_7`"]
    #[inline(always)]
    pub fn is_ms_7(&self) -> bool {
        *self == TVDPSRC_CON_A::MS_7
    }
    #[doc = "Checks if the value of the field is `MS_8`"]
    #[inline(always)]
    pub fn is_ms_8(&self) -> bool {
        *self == TVDPSRC_CON_A::MS_8
    }
    #[doc = "Checks if the value of the field is `MS_9`"]
    #[inline(always)]
    pub fn is_ms_9(&self) -> bool {
        *self == TVDPSRC_CON_A::MS_9
    }
    #[doc = "Checks if the value of the field is `MS_10`"]
    #[inline(always)]
    pub fn is_ms_10(&self) -> bool {
        *self == TVDPSRC_CON_A::MS_10
    }
}
#[doc = "Field `TVDPSRC_CON` writer - Time Period Before Enabling D+ Pullup"]
pub type TVDPSRC_CON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER2_TIMER2_BC11_SPEC, u16, TVDPSRC_CON_A, 10, O>;
impl<'a, const O: u8> TVDPSRC_CON_W<'a, O> {
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_1(self) -> &'a mut W {
        self.variant(TVDPSRC_CON_A::MS_1)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_2(self) -> &'a mut W {
        self.variant(TVDPSRC_CON_A::MS_2)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_3(self) -> &'a mut W {
        self.variant(TVDPSRC_CON_A::MS_3)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_4(self) -> &'a mut W {
        self.variant(TVDPSRC_CON_A::MS_4)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_5(self) -> &'a mut W {
        self.variant(TVDPSRC_CON_A::MS_5)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_6(self) -> &'a mut W {
        self.variant(TVDPSRC_CON_A::MS_6)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_7(self) -> &'a mut W {
        self.variant(TVDPSRC_CON_A::MS_7)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_8(self) -> &'a mut W {
        self.variant(TVDPSRC_CON_A::MS_8)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_9(self) -> &'a mut W {
        self.variant(TVDPSRC_CON_A::MS_9)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_10(self) -> &'a mut W {
        self.variant(TVDPSRC_CON_A::MS_10)
    }
}
impl R {
    #[doc = "Bits 0:3 - Time Before Check of D- Line"]
    #[inline(always)]
    pub fn check_dm(&self) -> CHECK_DM_R {
        CHECK_DM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:25 - Time Period Before Enabling D+ Pullup"]
    #[inline(always)]
    pub fn tvdpsrc_con(&self) -> TVDPSRC_CON_R {
        TVDPSRC_CON_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Time Before Check of D- Line"]
    #[inline(always)]
    #[must_use]
    pub fn check_dm(&mut self) -> CHECK_DM_W<0> {
        CHECK_DM_W::new(self)
    }
    #[doc = "Bits 16:25 - Time Period Before Enabling D+ Pullup"]
    #[inline(always)]
    #[must_use]
    pub fn tvdpsrc_con(&mut self) -> TVDPSRC_CON_W<16> {
        TVDPSRC_CON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMER2_BC11\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2_timer2_bc11](index.html) module"]
pub struct TIMER2_TIMER2_BC11_SPEC;
impl crate::RegisterSpec for TIMER2_TIMER2_BC11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer2_timer2_bc11::R](R) reader structure"]
impl crate::Readable for TIMER2_TIMER2_BC11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer2_timer2_bc11::W](W) writer structure"]
impl crate::Writable for TIMER2_TIMER2_BC11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER2_BC11 to value 0x0028_0001"]
impl crate::Resettable for TIMER2_TIMER2_BC11_SPEC {
    const RESET_VALUE: Self::Ux = 0x0028_0001;
}
