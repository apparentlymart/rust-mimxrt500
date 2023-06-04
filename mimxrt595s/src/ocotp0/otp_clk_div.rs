#[doc = "Register `OTP_CLK_DIV` reader"]
pub struct R(crate::R<OTP_CLK_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTP_CLK_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTP_CLK_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTP_CLK_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTP_CLK_DIV` writer"]
pub struct W(crate::W<OTP_CLK_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTP_CLK_DIV_SPEC>;
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
impl From<crate::W<OTP_CLK_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTP_CLK_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Clock divider value by -1 encoding"]
pub type DIV_R = crate::FieldReader<u8, DIV_A>;
#[doc = "Clock divider value by -1 encoding\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIV_A {
    #[doc = "0: Divide by 1"]
    DIV_1 = 0,
    #[doc = "1: Divide by 2"]
    DIV_2 = 1,
    #[doc = "2: Divide by 3"]
    DIV_3 = 2,
    #[doc = "3: Divide by 4"]
    DIV_4 = 3,
    #[doc = "4: Divide by 5"]
    DIV_5 = 4,
    #[doc = "5: Divide by 6"]
    DIV_6 = 5,
    #[doc = "6: Divide by 7"]
    DIV_7 = 6,
    #[doc = "7: Divide by 8"]
    DIV_8 = 7,
    #[doc = "8: Divide by 9"]
    DIV_9 = 8,
    #[doc = "9: Divide by 10"]
    DIV_10 = 9,
    #[doc = "10: Divide by 11"]
    DIV_11 = 10,
    #[doc = "11: Divide by 12"]
    DIV_12 = 11,
    #[doc = "12: Divide by 13"]
    DIV_13 = 12,
    #[doc = "13: Divide by 14"]
    DIV_14 = 13,
    #[doc = "14: Divide by 15"]
    DIV_15 = 14,
    #[doc = "15: Divide by 16"]
    DIV_16 = 15,
}
impl From<DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: DIV_A) -> Self {
        variant as _
    }
}
impl DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIV_A {
        match self.bits {
            0 => DIV_A::DIV_1,
            1 => DIV_A::DIV_2,
            2 => DIV_A::DIV_3,
            3 => DIV_A::DIV_4,
            4 => DIV_A::DIV_5,
            5 => DIV_A::DIV_6,
            6 => DIV_A::DIV_7,
            7 => DIV_A::DIV_8,
            8 => DIV_A::DIV_9,
            9 => DIV_A::DIV_10,
            10 => DIV_A::DIV_11,
            11 => DIV_A::DIV_12,
            12 => DIV_A::DIV_13,
            13 => DIV_A::DIV_14,
            14 => DIV_A::DIV_15,
            15 => DIV_A::DIV_16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV_1`"]
    #[inline(always)]
    pub fn is_div_1(&self) -> bool {
        *self == DIV_A::DIV_1
    }
    #[doc = "Checks if the value of the field is `DIV_2`"]
    #[inline(always)]
    pub fn is_div_2(&self) -> bool {
        *self == DIV_A::DIV_2
    }
    #[doc = "Checks if the value of the field is `DIV_3`"]
    #[inline(always)]
    pub fn is_div_3(&self) -> bool {
        *self == DIV_A::DIV_3
    }
    #[doc = "Checks if the value of the field is `DIV_4`"]
    #[inline(always)]
    pub fn is_div_4(&self) -> bool {
        *self == DIV_A::DIV_4
    }
    #[doc = "Checks if the value of the field is `DIV_5`"]
    #[inline(always)]
    pub fn is_div_5(&self) -> bool {
        *self == DIV_A::DIV_5
    }
    #[doc = "Checks if the value of the field is `DIV_6`"]
    #[inline(always)]
    pub fn is_div_6(&self) -> bool {
        *self == DIV_A::DIV_6
    }
    #[doc = "Checks if the value of the field is `DIV_7`"]
    #[inline(always)]
    pub fn is_div_7(&self) -> bool {
        *self == DIV_A::DIV_7
    }
    #[doc = "Checks if the value of the field is `DIV_8`"]
    #[inline(always)]
    pub fn is_div_8(&self) -> bool {
        *self == DIV_A::DIV_8
    }
    #[doc = "Checks if the value of the field is `DIV_9`"]
    #[inline(always)]
    pub fn is_div_9(&self) -> bool {
        *self == DIV_A::DIV_9
    }
    #[doc = "Checks if the value of the field is `DIV_10`"]
    #[inline(always)]
    pub fn is_div_10(&self) -> bool {
        *self == DIV_A::DIV_10
    }
    #[doc = "Checks if the value of the field is `DIV_11`"]
    #[inline(always)]
    pub fn is_div_11(&self) -> bool {
        *self == DIV_A::DIV_11
    }
    #[doc = "Checks if the value of the field is `DIV_12`"]
    #[inline(always)]
    pub fn is_div_12(&self) -> bool {
        *self == DIV_A::DIV_12
    }
    #[doc = "Checks if the value of the field is `DIV_13`"]
    #[inline(always)]
    pub fn is_div_13(&self) -> bool {
        *self == DIV_A::DIV_13
    }
    #[doc = "Checks if the value of the field is `DIV_14`"]
    #[inline(always)]
    pub fn is_div_14(&self) -> bool {
        *self == DIV_A::DIV_14
    }
    #[doc = "Checks if the value of the field is `DIV_15`"]
    #[inline(always)]
    pub fn is_div_15(&self) -> bool {
        *self == DIV_A::DIV_15
    }
    #[doc = "Checks if the value of the field is `DIV_16`"]
    #[inline(always)]
    pub fn is_div_16(&self) -> bool {
        *self == DIV_A::DIV_16
    }
}
#[doc = "Field `DIV` writer - Clock divider value by -1 encoding"]
pub type DIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OTP_CLK_DIV_SPEC, u8, DIV_A, 4, O>;
impl<'a, const O: u8> DIV_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div_1(self) -> &'a mut W {
        self.variant(DIV_A::DIV_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div_2(self) -> &'a mut W {
        self.variant(DIV_A::DIV_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn div_3(self) -> &'a mut W {
        self.variant(DIV_A::DIV_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div_4(self) -> &'a mut W {
        self.variant(DIV_A::DIV_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn div_5(self) -> &'a mut W {
        self.variant(DIV_A::DIV_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn div_6(self) -> &'a mut W {
        self.variant(DIV_A::DIV_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn div_7(self) -> &'a mut W {
        self.variant(DIV_A::DIV_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div_8(self) -> &'a mut W {
        self.variant(DIV_A::DIV_8)
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn div_9(self) -> &'a mut W {
        self.variant(DIV_A::DIV_9)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn div_10(self) -> &'a mut W {
        self.variant(DIV_A::DIV_10)
    }
    #[doc = "Divide by 11"]
    #[inline(always)]
    pub fn div_11(self) -> &'a mut W {
        self.variant(DIV_A::DIV_11)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn div_12(self) -> &'a mut W {
        self.variant(DIV_A::DIV_12)
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn div_13(self) -> &'a mut W {
        self.variant(DIV_A::DIV_13)
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn div_14(self) -> &'a mut W {
        self.variant(DIV_A::DIV_14)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn div_15(self) -> &'a mut W {
        self.variant(DIV_A::DIV_15)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div_16(self) -> &'a mut W {
        self.variant(DIV_A::DIV_16)
    }
}
#[doc = "Field `RESET` reader - Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_CLK_DIV_SPEC, bool, O>;
#[doc = "Field `HALT` reader - Halts the divider counter"]
pub type HALT_R = crate::BitReader<bool>;
#[doc = "Field `HALT` writer - Halts the divider counter"]
pub type HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTP_CLK_DIV_SPEC, bool, O>;
#[doc = "Field `REQFLAG` reader - Divider status flag"]
pub type REQFLAG_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Clock divider value by -1 encoding"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Halts the divider counter"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Divider status flag"]
    #[inline(always)]
    pub fn reqflag(&self) -> REQFLAG_R {
        REQFLAG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock divider value by -1 encoding"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Bit 29 - Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<29> {
        RESET_W::new(self)
    }
    #[doc = "Bit 30 - Halts the divider counter"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HALT_W<30> {
        HALT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTP clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otp_clk_div](index.html) module"]
pub struct OTP_CLK_DIV_SPEC;
impl crate::RegisterSpec for OTP_CLK_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otp_clk_div::R](R) reader structure"]
impl crate::Readable for OTP_CLK_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otp_clk_div::W](W) writer structure"]
impl crate::Writable for OTP_CLK_DIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTP_CLK_DIV to value 0"]
impl crate::Resettable for OTP_CLK_DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
