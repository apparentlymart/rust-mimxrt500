#[doc = "Register `TIMER1` reader"]
pub struct R(crate::R<TIMER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1` writer"]
pub struct W(crate::W<TIMER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_SPEC>;
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
impl From<crate::W<TIMER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TVDPSRC_ON` reader - Time Period Comparator Enabled"]
pub type TVDPSRC_ON_R = crate::FieldReader<u16, TVDPSRC_ON_A>;
#[doc = "Time Period Comparator Enabled\n\nValue on reset: 40"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TVDPSRC_ON_A {
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
impl From<TVDPSRC_ON_A> for u16 {
    #[inline(always)]
    fn from(variant: TVDPSRC_ON_A) -> Self {
        variant as _
    }
}
impl TVDPSRC_ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TVDPSRC_ON_A> {
        match self.bits {
            1 => Some(TVDPSRC_ON_A::MS_1),
            2 => Some(TVDPSRC_ON_A::MS_2),
            3 => Some(TVDPSRC_ON_A::MS_3),
            4 => Some(TVDPSRC_ON_A::MS_4),
            5 => Some(TVDPSRC_ON_A::MS_5),
            6 => Some(TVDPSRC_ON_A::MS_6),
            7 => Some(TVDPSRC_ON_A::MS_7),
            8 => Some(TVDPSRC_ON_A::MS_8),
            9 => Some(TVDPSRC_ON_A::MS_9),
            10 => Some(TVDPSRC_ON_A::MS_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MS_1`"]
    #[inline(always)]
    pub fn is_ms_1(&self) -> bool {
        *self == TVDPSRC_ON_A::MS_1
    }
    #[doc = "Checks if the value of the field is `MS_2`"]
    #[inline(always)]
    pub fn is_ms_2(&self) -> bool {
        *self == TVDPSRC_ON_A::MS_2
    }
    #[doc = "Checks if the value of the field is `MS_3`"]
    #[inline(always)]
    pub fn is_ms_3(&self) -> bool {
        *self == TVDPSRC_ON_A::MS_3
    }
    #[doc = "Checks if the value of the field is `MS_4`"]
    #[inline(always)]
    pub fn is_ms_4(&self) -> bool {
        *self == TVDPSRC_ON_A::MS_4
    }
    #[doc = "Checks if the value of the field is `MS_5`"]
    #[inline(always)]
    pub fn is_ms_5(&self) -> bool {
        *self == TVDPSRC_ON_A::MS_5
    }
    #[doc = "Checks if the value of the field is `MS_6`"]
    #[inline(always)]
    pub fn is_ms_6(&self) -> bool {
        *self == TVDPSRC_ON_A::MS_6
    }
    #[doc = "Checks if the value of the field is `MS_7`"]
    #[inline(always)]
    pub fn is_ms_7(&self) -> bool {
        *self == TVDPSRC_ON_A::MS_7
    }
    #[doc = "Checks if the value of the field is `MS_8`"]
    #[inline(always)]
    pub fn is_ms_8(&self) -> bool {
        *self == TVDPSRC_ON_A::MS_8
    }
    #[doc = "Checks if the value of the field is `MS_9`"]
    #[inline(always)]
    pub fn is_ms_9(&self) -> bool {
        *self == TVDPSRC_ON_A::MS_9
    }
    #[doc = "Checks if the value of the field is `MS_10`"]
    #[inline(always)]
    pub fn is_ms_10(&self) -> bool {
        *self == TVDPSRC_ON_A::MS_10
    }
}
#[doc = "Field `TVDPSRC_ON` writer - Time Period Comparator Enabled"]
pub type TVDPSRC_ON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER1_SPEC, u16, TVDPSRC_ON_A, 10, O>;
impl<'a, const O: u8> TVDPSRC_ON_W<'a, O> {
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_1(self) -> &'a mut W {
        self.variant(TVDPSRC_ON_A::MS_1)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_2(self) -> &'a mut W {
        self.variant(TVDPSRC_ON_A::MS_2)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_3(self) -> &'a mut W {
        self.variant(TVDPSRC_ON_A::MS_3)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_4(self) -> &'a mut W {
        self.variant(TVDPSRC_ON_A::MS_4)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_5(self) -> &'a mut W {
        self.variant(TVDPSRC_ON_A::MS_5)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_6(self) -> &'a mut W {
        self.variant(TVDPSRC_ON_A::MS_6)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_7(self) -> &'a mut W {
        self.variant(TVDPSRC_ON_A::MS_7)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_8(self) -> &'a mut W {
        self.variant(TVDPSRC_ON_A::MS_8)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_9(self) -> &'a mut W {
        self.variant(TVDPSRC_ON_A::MS_9)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_10(self) -> &'a mut W {
        self.variant(TVDPSRC_ON_A::MS_10)
    }
}
#[doc = "Field `TDCD_DBNC` reader - Time Period to Debounce D+ Signal"]
pub type TDCD_DBNC_R = crate::FieldReader<u16, TDCD_DBNC_A>;
#[doc = "Time Period to Debounce D+ Signal\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TDCD_DBNC_A {
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
impl From<TDCD_DBNC_A> for u16 {
    #[inline(always)]
    fn from(variant: TDCD_DBNC_A) -> Self {
        variant as _
    }
}
impl TDCD_DBNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TDCD_DBNC_A> {
        match self.bits {
            1 => Some(TDCD_DBNC_A::MS_1),
            2 => Some(TDCD_DBNC_A::MS_2),
            3 => Some(TDCD_DBNC_A::MS_3),
            4 => Some(TDCD_DBNC_A::MS_4),
            5 => Some(TDCD_DBNC_A::MS_5),
            6 => Some(TDCD_DBNC_A::MS_6),
            7 => Some(TDCD_DBNC_A::MS_7),
            8 => Some(TDCD_DBNC_A::MS_8),
            9 => Some(TDCD_DBNC_A::MS_9),
            10 => Some(TDCD_DBNC_A::MS_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MS_1`"]
    #[inline(always)]
    pub fn is_ms_1(&self) -> bool {
        *self == TDCD_DBNC_A::MS_1
    }
    #[doc = "Checks if the value of the field is `MS_2`"]
    #[inline(always)]
    pub fn is_ms_2(&self) -> bool {
        *self == TDCD_DBNC_A::MS_2
    }
    #[doc = "Checks if the value of the field is `MS_3`"]
    #[inline(always)]
    pub fn is_ms_3(&self) -> bool {
        *self == TDCD_DBNC_A::MS_3
    }
    #[doc = "Checks if the value of the field is `MS_4`"]
    #[inline(always)]
    pub fn is_ms_4(&self) -> bool {
        *self == TDCD_DBNC_A::MS_4
    }
    #[doc = "Checks if the value of the field is `MS_5`"]
    #[inline(always)]
    pub fn is_ms_5(&self) -> bool {
        *self == TDCD_DBNC_A::MS_5
    }
    #[doc = "Checks if the value of the field is `MS_6`"]
    #[inline(always)]
    pub fn is_ms_6(&self) -> bool {
        *self == TDCD_DBNC_A::MS_6
    }
    #[doc = "Checks if the value of the field is `MS_7`"]
    #[inline(always)]
    pub fn is_ms_7(&self) -> bool {
        *self == TDCD_DBNC_A::MS_7
    }
    #[doc = "Checks if the value of the field is `MS_8`"]
    #[inline(always)]
    pub fn is_ms_8(&self) -> bool {
        *self == TDCD_DBNC_A::MS_8
    }
    #[doc = "Checks if the value of the field is `MS_9`"]
    #[inline(always)]
    pub fn is_ms_9(&self) -> bool {
        *self == TDCD_DBNC_A::MS_9
    }
    #[doc = "Checks if the value of the field is `MS_10`"]
    #[inline(always)]
    pub fn is_ms_10(&self) -> bool {
        *self == TDCD_DBNC_A::MS_10
    }
}
#[doc = "Field `TDCD_DBNC` writer - Time Period to Debounce D+ Signal"]
pub type TDCD_DBNC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER1_SPEC, u16, TDCD_DBNC_A, 10, O>;
impl<'a, const O: u8> TDCD_DBNC_W<'a, O> {
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_1(self) -> &'a mut W {
        self.variant(TDCD_DBNC_A::MS_1)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_2(self) -> &'a mut W {
        self.variant(TDCD_DBNC_A::MS_2)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_3(self) -> &'a mut W {
        self.variant(TDCD_DBNC_A::MS_3)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_4(self) -> &'a mut W {
        self.variant(TDCD_DBNC_A::MS_4)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_5(self) -> &'a mut W {
        self.variant(TDCD_DBNC_A::MS_5)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_6(self) -> &'a mut W {
        self.variant(TDCD_DBNC_A::MS_6)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_7(self) -> &'a mut W {
        self.variant(TDCD_DBNC_A::MS_7)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_8(self) -> &'a mut W {
        self.variant(TDCD_DBNC_A::MS_8)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_9(self) -> &'a mut W {
        self.variant(TDCD_DBNC_A::MS_9)
    }
    #[doc = "1ms - 1023ms"]
    #[inline(always)]
    pub fn ms_10(self) -> &'a mut W {
        self.variant(TDCD_DBNC_A::MS_10)
    }
}
impl R {
    #[doc = "Bits 0:9 - Time Period Comparator Enabled"]
    #[inline(always)]
    pub fn tvdpsrc_on(&self) -> TVDPSRC_ON_R {
        TVDPSRC_ON_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Time Period to Debounce D+ Signal"]
    #[inline(always)]
    pub fn tdcd_dbnc(&self) -> TDCD_DBNC_R {
        TDCD_DBNC_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Time Period Comparator Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tvdpsrc_on(&mut self) -> TVDPSRC_ON_W<0> {
        TVDPSRC_ON_W::new(self)
    }
    #[doc = "Bits 16:25 - Time Period to Debounce D+ Signal"]
    #[inline(always)]
    #[must_use]
    pub fn tdcd_dbnc(&mut self) -> TDCD_DBNC_W<16> {
        TDCD_DBNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1](index.html) module"]
pub struct TIMER1_SPEC;
impl crate::RegisterSpec for TIMER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1::R](R) reader structure"]
impl crate::Readable for TIMER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1::W](W) writer structure"]
impl crate::Writable for TIMER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER1 to value 0x000a_0028"]
impl crate::Resettable for TIMER1_SPEC {
    const RESET_VALUE: Self::Ux = 0x000a_0028;
}
