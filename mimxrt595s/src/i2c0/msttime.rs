#[doc = "Register `MSTTIME` reader"]
pub struct R(crate::R<MSTTIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTTIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTTIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTTIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTTIME` writer"]
pub struct W(crate::W<MSTTIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTTIME_SPEC>;
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
impl From<crate::W<MSTTIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTTIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTSCLLOW` reader - Master SCL Low time"]
pub type MSTSCLLOW_R = crate::FieldReader<u8, MSTSCLLOW_A>;
#[doc = "Master SCL Low time\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSTSCLLOW_A {
    #[doc = "0: 2 clocks. Minimum SCL low time is 2 clocks of the I2C clock pre-divider."]
    MSTSCLLOW_2CLOCKS = 0,
    #[doc = "1: 3 clocks. Minimum SCL low time is 3 clocks of the I2C clock pre-divider."]
    MSTSCLLOW_3CLOCKS = 1,
    #[doc = "2: 4 clocks. Minimum SCL low time is 4 clocks of the I2C clock pre-divider."]
    MSTSCLLOW_4CLOCKS = 2,
    #[doc = "3: 5 clocks. Minimum SCL low time is 5 clocks of the I2C clock pre-divider."]
    MSTSCLLOW_5CLOCKS = 3,
    #[doc = "4: 6 clocks. Minimum SCL low time is 6 clocks of the I2C clock pre-divider."]
    MSTSCLLOW_6CLOCKS = 4,
    #[doc = "5: 7 clocks. Minimum SCL low time is 7 clocks of the I2C clock pre-divider."]
    MSTSCLLOW_7CLOCKS = 5,
    #[doc = "6: 8 clocks. Minimum SCL low time is 8 clocks of the I2C clock pre-divider."]
    MSTSCLLOW_8CLOCKS = 6,
    #[doc = "7: 9 clocks. Minimum SCL low time is 9 clocks of the I2C clock pre-divider."]
    MSTSCLLOW_9CLOCKS = 7,
}
impl From<MSTSCLLOW_A> for u8 {
    #[inline(always)]
    fn from(variant: MSTSCLLOW_A) -> Self {
        variant as _
    }
}
impl MSTSCLLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTSCLLOW_A {
        match self.bits {
            0 => MSTSCLLOW_A::MSTSCLLOW_2CLOCKS,
            1 => MSTSCLLOW_A::MSTSCLLOW_3CLOCKS,
            2 => MSTSCLLOW_A::MSTSCLLOW_4CLOCKS,
            3 => MSTSCLLOW_A::MSTSCLLOW_5CLOCKS,
            4 => MSTSCLLOW_A::MSTSCLLOW_6CLOCKS,
            5 => MSTSCLLOW_A::MSTSCLLOW_7CLOCKS,
            6 => MSTSCLLOW_A::MSTSCLLOW_8CLOCKS,
            7 => MSTSCLLOW_A::MSTSCLLOW_9CLOCKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MSTSCLLOW_2CLOCKS`"]
    #[inline(always)]
    pub fn is_mstscllow_2clocks(&self) -> bool {
        *self == MSTSCLLOW_A::MSTSCLLOW_2CLOCKS
    }
    #[doc = "Checks if the value of the field is `MSTSCLLOW_3CLOCKS`"]
    #[inline(always)]
    pub fn is_mstscllow_3clocks(&self) -> bool {
        *self == MSTSCLLOW_A::MSTSCLLOW_3CLOCKS
    }
    #[doc = "Checks if the value of the field is `MSTSCLLOW_4CLOCKS`"]
    #[inline(always)]
    pub fn is_mstscllow_4clocks(&self) -> bool {
        *self == MSTSCLLOW_A::MSTSCLLOW_4CLOCKS
    }
    #[doc = "Checks if the value of the field is `MSTSCLLOW_5CLOCKS`"]
    #[inline(always)]
    pub fn is_mstscllow_5clocks(&self) -> bool {
        *self == MSTSCLLOW_A::MSTSCLLOW_5CLOCKS
    }
    #[doc = "Checks if the value of the field is `MSTSCLLOW_6CLOCKS`"]
    #[inline(always)]
    pub fn is_mstscllow_6clocks(&self) -> bool {
        *self == MSTSCLLOW_A::MSTSCLLOW_6CLOCKS
    }
    #[doc = "Checks if the value of the field is `MSTSCLLOW_7CLOCKS`"]
    #[inline(always)]
    pub fn is_mstscllow_7clocks(&self) -> bool {
        *self == MSTSCLLOW_A::MSTSCLLOW_7CLOCKS
    }
    #[doc = "Checks if the value of the field is `MSTSCLLOW_8CLOCKS`"]
    #[inline(always)]
    pub fn is_mstscllow_8clocks(&self) -> bool {
        *self == MSTSCLLOW_A::MSTSCLLOW_8CLOCKS
    }
    #[doc = "Checks if the value of the field is `MSTSCLLOW_9CLOCKS`"]
    #[inline(always)]
    pub fn is_mstscllow_9clocks(&self) -> bool {
        *self == MSTSCLLOW_A::MSTSCLLOW_9CLOCKS
    }
}
#[doc = "Field `MSTSCLLOW` writer - Master SCL Low time"]
pub type MSTSCLLOW_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MSTTIME_SPEC, u8, MSTSCLLOW_A, 3, O>;
impl<'a, const O: u8> MSTSCLLOW_W<'a, O> {
    #[doc = "2 clocks. Minimum SCL low time is 2 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn mstscllow_2clocks(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::MSTSCLLOW_2CLOCKS)
    }
    #[doc = "3 clocks. Minimum SCL low time is 3 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn mstscllow_3clocks(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::MSTSCLLOW_3CLOCKS)
    }
    #[doc = "4 clocks. Minimum SCL low time is 4 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn mstscllow_4clocks(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::MSTSCLLOW_4CLOCKS)
    }
    #[doc = "5 clocks. Minimum SCL low time is 5 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn mstscllow_5clocks(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::MSTSCLLOW_5CLOCKS)
    }
    #[doc = "6 clocks. Minimum SCL low time is 6 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn mstscllow_6clocks(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::MSTSCLLOW_6CLOCKS)
    }
    #[doc = "7 clocks. Minimum SCL low time is 7 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn mstscllow_7clocks(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::MSTSCLLOW_7CLOCKS)
    }
    #[doc = "8 clocks. Minimum SCL low time is 8 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn mstscllow_8clocks(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::MSTSCLLOW_8CLOCKS)
    }
    #[doc = "9 clocks. Minimum SCL low time is 9 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn mstscllow_9clocks(self) -> &'a mut W {
        self.variant(MSTSCLLOW_A::MSTSCLLOW_9CLOCKS)
    }
}
#[doc = "Field `MSTSCLHIGH` reader - Master SCL High time"]
pub type MSTSCLHIGH_R = crate::FieldReader<u8, MSTSCLHIGH_A>;
#[doc = "Master SCL High time\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSTSCLHIGH_A {
    #[doc = "0: 2 clocks. Minimum SCL high time is 2 clocks of the I2C clock pre-divider."]
    MSTSCLHIGH_2CLOCKS = 0,
    #[doc = "1: 3 clocks. Minimum SCL high time is 3 clocks of the I2C clock pre-divider ."]
    MSTSCLHIGH_3CLOCKS = 1,
    #[doc = "2: 4 clocks. Minimum SCL high time is 4 clocks of the I2C clock pre-divider."]
    MSTSCLHIGH_4CLOCKS = 2,
    #[doc = "3: 5 clocks. Minimum SCL high time is 5 clocks of the I2C clock pre-divider."]
    MSTSCLHIGH_5CLOCKS = 3,
    #[doc = "4: 6 clocks. Minimum SCL high time is 6 clocks of the I2C clock pre-divider."]
    MSTSCLHIGH_6CLOCKS = 4,
    #[doc = "5: 7 clocks. Minimum SCL high time is 7 clocks of the I2C clock pre-divider."]
    MSTSCLHIGH_7CLOCKS = 5,
    #[doc = "6: 8 clocks. Minimum SCL high time is 8 clocks of the I2C clock pre-divider."]
    MSTSCLHIGH_8CLOCKS = 6,
    #[doc = "7: 9 clocks. Minimum SCL high time is 9 clocks of the I2C clock pre-divider."]
    MSTSCLHIGH_9CLOCKS = 7,
}
impl From<MSTSCLHIGH_A> for u8 {
    #[inline(always)]
    fn from(variant: MSTSCLHIGH_A) -> Self {
        variant as _
    }
}
impl MSTSCLHIGH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTSCLHIGH_A {
        match self.bits {
            0 => MSTSCLHIGH_A::MSTSCLHIGH_2CLOCKS,
            1 => MSTSCLHIGH_A::MSTSCLHIGH_3CLOCKS,
            2 => MSTSCLHIGH_A::MSTSCLHIGH_4CLOCKS,
            3 => MSTSCLHIGH_A::MSTSCLHIGH_5CLOCKS,
            4 => MSTSCLHIGH_A::MSTSCLHIGH_6CLOCKS,
            5 => MSTSCLHIGH_A::MSTSCLHIGH_7CLOCKS,
            6 => MSTSCLHIGH_A::MSTSCLHIGH_8CLOCKS,
            7 => MSTSCLHIGH_A::MSTSCLHIGH_9CLOCKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MSTSCLHIGH_2CLOCKS`"]
    #[inline(always)]
    pub fn is_mstsclhigh_2clocks(&self) -> bool {
        *self == MSTSCLHIGH_A::MSTSCLHIGH_2CLOCKS
    }
    #[doc = "Checks if the value of the field is `MSTSCLHIGH_3CLOCKS`"]
    #[inline(always)]
    pub fn is_mstsclhigh_3clocks(&self) -> bool {
        *self == MSTSCLHIGH_A::MSTSCLHIGH_3CLOCKS
    }
    #[doc = "Checks if the value of the field is `MSTSCLHIGH_4CLOCKS`"]
    #[inline(always)]
    pub fn is_mstsclhigh_4clocks(&self) -> bool {
        *self == MSTSCLHIGH_A::MSTSCLHIGH_4CLOCKS
    }
    #[doc = "Checks if the value of the field is `MSTSCLHIGH_5CLOCKS`"]
    #[inline(always)]
    pub fn is_mstsclhigh_5clocks(&self) -> bool {
        *self == MSTSCLHIGH_A::MSTSCLHIGH_5CLOCKS
    }
    #[doc = "Checks if the value of the field is `MSTSCLHIGH_6CLOCKS`"]
    #[inline(always)]
    pub fn is_mstsclhigh_6clocks(&self) -> bool {
        *self == MSTSCLHIGH_A::MSTSCLHIGH_6CLOCKS
    }
    #[doc = "Checks if the value of the field is `MSTSCLHIGH_7CLOCKS`"]
    #[inline(always)]
    pub fn is_mstsclhigh_7clocks(&self) -> bool {
        *self == MSTSCLHIGH_A::MSTSCLHIGH_7CLOCKS
    }
    #[doc = "Checks if the value of the field is `MSTSCLHIGH_8CLOCKS`"]
    #[inline(always)]
    pub fn is_mstsclhigh_8clocks(&self) -> bool {
        *self == MSTSCLHIGH_A::MSTSCLHIGH_8CLOCKS
    }
    #[doc = "Checks if the value of the field is `MSTSCLHIGH_9CLOCKS`"]
    #[inline(always)]
    pub fn is_mstsclhigh_9clocks(&self) -> bool {
        *self == MSTSCLHIGH_A::MSTSCLHIGH_9CLOCKS
    }
}
#[doc = "Field `MSTSCLHIGH` writer - Master SCL High time"]
pub type MSTSCLHIGH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MSTTIME_SPEC, u8, MSTSCLHIGH_A, 3, O>;
impl<'a, const O: u8> MSTSCLHIGH_W<'a, O> {
    #[doc = "2 clocks. Minimum SCL high time is 2 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn mstsclhigh_2clocks(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::MSTSCLHIGH_2CLOCKS)
    }
    #[doc = "3 clocks. Minimum SCL high time is 3 clocks of the I2C clock pre-divider ."]
    #[inline(always)]
    pub fn mstsclhigh_3clocks(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::MSTSCLHIGH_3CLOCKS)
    }
    #[doc = "4 clocks. Minimum SCL high time is 4 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn mstsclhigh_4clocks(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::MSTSCLHIGH_4CLOCKS)
    }
    #[doc = "5 clocks. Minimum SCL high time is 5 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn mstsclhigh_5clocks(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::MSTSCLHIGH_5CLOCKS)
    }
    #[doc = "6 clocks. Minimum SCL high time is 6 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn mstsclhigh_6clocks(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::MSTSCLHIGH_6CLOCKS)
    }
    #[doc = "7 clocks. Minimum SCL high time is 7 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn mstsclhigh_7clocks(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::MSTSCLHIGH_7CLOCKS)
    }
    #[doc = "8 clocks. Minimum SCL high time is 8 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn mstsclhigh_8clocks(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::MSTSCLHIGH_8CLOCKS)
    }
    #[doc = "9 clocks. Minimum SCL high time is 9 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn mstsclhigh_9clocks(self) -> &'a mut W {
        self.variant(MSTSCLHIGH_A::MSTSCLHIGH_9CLOCKS)
    }
}
impl R {
    #[doc = "Bits 0:2 - Master SCL Low time"]
    #[inline(always)]
    pub fn mstscllow(&self) -> MSTSCLLOW_R {
        MSTSCLLOW_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Master SCL High time"]
    #[inline(always)]
    pub fn mstsclhigh(&self) -> MSTSCLHIGH_R {
        MSTSCLHIGH_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Master SCL Low time"]
    #[inline(always)]
    #[must_use]
    pub fn mstscllow(&mut self) -> MSTSCLLOW_W<0> {
        MSTSCLLOW_W::new(self)
    }
    #[doc = "Bits 4:6 - Master SCL High time"]
    #[inline(always)]
    #[must_use]
    pub fn mstsclhigh(&mut self) -> MSTSCLHIGH_W<4> {
        MSTSCLHIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msttime](index.html) module"]
pub struct MSTTIME_SPEC;
impl crate::RegisterSpec for MSTTIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msttime::R](R) reader structure"]
impl crate::Readable for MSTTIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msttime::W](W) writer structure"]
impl crate::Writable for MSTTIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTTIME to value 0x77"]
impl crate::Resettable for MSTTIME_SPEC {
    const RESET_VALUE: Self::Ux = 0x77;
}
