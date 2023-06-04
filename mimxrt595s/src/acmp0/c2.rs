#[doc = "Register `C2` reader"]
pub struct R(crate::R<C2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2` writer"]
pub struct W(crate::W<C2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2_SPEC>;
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
impl From<crate::W<C2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACOn` reader - ACOn"]
pub type ACON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACOn` writer - ACOn"]
pub type ACON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C2_SPEC, u8, u8, 6, O>;
#[doc = "Field `INITMOD` reader - Comparator and DAC initialization delay modulus."]
pub type INITMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INITMOD` writer - Comparator and DAC initialization delay modulus."]
pub type INITMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C2_SPEC, u8, u8, 6, O>;
#[doc = "Field `NSAM` reader - Number of sample clocks"]
pub type NSAM_R = crate::FieldReader<u8, NSAM_A>;
#[doc = "Number of sample clocks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NSAM_A {
    #[doc = "0: The comparison result is sampled as soon as the active channel is scanned in one round-robin clock."]
    NSAM_0 = 0,
    #[doc = "1: The sampling takes place 1 round-robin clock cycle after the next cycle of the round-robin clock."]
    NSAM_1 = 1,
    #[doc = "2: The sampling takes place 2 round-robin clock cycles after the next cycle of the round-robin clock."]
    NSAM_2 = 2,
    #[doc = "3: The sampling takes place 3 round-robin clock cycles after the next cycle of the round-robin clock."]
    NSAM_3 = 3,
}
impl From<NSAM_A> for u8 {
    #[inline(always)]
    fn from(variant: NSAM_A) -> Self {
        variant as _
    }
}
impl NSAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSAM_A {
        match self.bits {
            0 => NSAM_A::NSAM_0,
            1 => NSAM_A::NSAM_1,
            2 => NSAM_A::NSAM_2,
            3 => NSAM_A::NSAM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NSAM_0`"]
    #[inline(always)]
    pub fn is_nsam_0(&self) -> bool {
        *self == NSAM_A::NSAM_0
    }
    #[doc = "Checks if the value of the field is `NSAM_1`"]
    #[inline(always)]
    pub fn is_nsam_1(&self) -> bool {
        *self == NSAM_A::NSAM_1
    }
    #[doc = "Checks if the value of the field is `NSAM_2`"]
    #[inline(always)]
    pub fn is_nsam_2(&self) -> bool {
        *self == NSAM_A::NSAM_2
    }
    #[doc = "Checks if the value of the field is `NSAM_3`"]
    #[inline(always)]
    pub fn is_nsam_3(&self) -> bool {
        *self == NSAM_A::NSAM_3
    }
}
#[doc = "Field `NSAM` writer - Number of sample clocks"]
pub type NSAM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, C2_SPEC, u8, NSAM_A, 2, O>;
impl<'a, const O: u8> NSAM_W<'a, O> {
    #[doc = "The comparison result is sampled as soon as the active channel is scanned in one round-robin clock."]
    #[inline(always)]
    pub fn nsam_0(self) -> &'a mut W {
        self.variant(NSAM_A::NSAM_0)
    }
    #[doc = "The sampling takes place 1 round-robin clock cycle after the next cycle of the round-robin clock."]
    #[inline(always)]
    pub fn nsam_1(self) -> &'a mut W {
        self.variant(NSAM_A::NSAM_1)
    }
    #[doc = "The sampling takes place 2 round-robin clock cycles after the next cycle of the round-robin clock."]
    #[inline(always)]
    pub fn nsam_2(self) -> &'a mut W {
        self.variant(NSAM_A::NSAM_2)
    }
    #[doc = "The sampling takes place 3 round-robin clock cycles after the next cycle of the round-robin clock."]
    #[inline(always)]
    pub fn nsam_3(self) -> &'a mut W {
        self.variant(NSAM_A::NSAM_3)
    }
}
#[doc = "Field `CH0F` reader - CH0F"]
pub type CH0F_R = crate::BitReader<bool>;
#[doc = "Field `CH0F` writer - CH0F"]
pub type CH0F_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, C2_SPEC, bool, O>;
#[doc = "Field `CH1F` reader - CH1F"]
pub type CH1F_R = crate::BitReader<bool>;
#[doc = "Field `CH1F` writer - CH1F"]
pub type CH1F_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, C2_SPEC, bool, O>;
#[doc = "Field `CH2F` reader - CH2F"]
pub type CH2F_R = crate::BitReader<bool>;
#[doc = "Field `CH2F` writer - CH2F"]
pub type CH2F_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, C2_SPEC, bool, O>;
#[doc = "Field `CH3F` reader - CH3F"]
pub type CH3F_R = crate::BitReader<bool>;
#[doc = "Field `CH3F` writer - CH3F"]
pub type CH3F_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, C2_SPEC, bool, O>;
#[doc = "Field `CH4F` reader - CH4F"]
pub type CH4F_R = crate::BitReader<bool>;
#[doc = "Field `CH4F` writer - CH4F"]
pub type CH4F_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, C2_SPEC, bool, O>;
#[doc = "Field `CH5F` reader - CH5F"]
pub type CH5F_R = crate::BitReader<bool>;
#[doc = "Field `CH5F` writer - CH5F"]
pub type CH5F_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, C2_SPEC, bool, O>;
#[doc = "Field `FXMXCH` reader - Fixed channel selection"]
pub type FXMXCH_R = crate::FieldReader<u8, FXMXCH_A>;
#[doc = "Fixed channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FXMXCH_A {
    #[doc = "0: External Reference Input 0 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_0 = 0,
    #[doc = "1: External Reference Input 1 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_1 = 1,
    #[doc = "2: External Reference Input 2 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_2 = 2,
    #[doc = "3: External Reference Input 3 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_3 = 3,
    #[doc = "4: External Reference Input 4 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_4 = 4,
    #[doc = "5: External Reference Input 5 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_5 = 5,
    #[doc = "7: The 8bit DAC is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_7 = 7,
}
impl From<FXMXCH_A> for u8 {
    #[inline(always)]
    fn from(variant: FXMXCH_A) -> Self {
        variant as _
    }
}
impl FXMXCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FXMXCH_A> {
        match self.bits {
            0 => Some(FXMXCH_A::FXMXCH_0),
            1 => Some(FXMXCH_A::FXMXCH_1),
            2 => Some(FXMXCH_A::FXMXCH_2),
            3 => Some(FXMXCH_A::FXMXCH_3),
            4 => Some(FXMXCH_A::FXMXCH_4),
            5 => Some(FXMXCH_A::FXMXCH_5),
            7 => Some(FXMXCH_A::FXMXCH_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FXMXCH_0`"]
    #[inline(always)]
    pub fn is_fxmxch_0(&self) -> bool {
        *self == FXMXCH_A::FXMXCH_0
    }
    #[doc = "Checks if the value of the field is `FXMXCH_1`"]
    #[inline(always)]
    pub fn is_fxmxch_1(&self) -> bool {
        *self == FXMXCH_A::FXMXCH_1
    }
    #[doc = "Checks if the value of the field is `FXMXCH_2`"]
    #[inline(always)]
    pub fn is_fxmxch_2(&self) -> bool {
        *self == FXMXCH_A::FXMXCH_2
    }
    #[doc = "Checks if the value of the field is `FXMXCH_3`"]
    #[inline(always)]
    pub fn is_fxmxch_3(&self) -> bool {
        *self == FXMXCH_A::FXMXCH_3
    }
    #[doc = "Checks if the value of the field is `FXMXCH_4`"]
    #[inline(always)]
    pub fn is_fxmxch_4(&self) -> bool {
        *self == FXMXCH_A::FXMXCH_4
    }
    #[doc = "Checks if the value of the field is `FXMXCH_5`"]
    #[inline(always)]
    pub fn is_fxmxch_5(&self) -> bool {
        *self == FXMXCH_A::FXMXCH_5
    }
    #[doc = "Checks if the value of the field is `FXMXCH_7`"]
    #[inline(always)]
    pub fn is_fxmxch_7(&self) -> bool {
        *self == FXMXCH_A::FXMXCH_7
    }
}
#[doc = "Field `FXMXCH` writer - Fixed channel selection"]
pub type FXMXCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C2_SPEC, u8, FXMXCH_A, 3, O>;
impl<'a, const O: u8> FXMXCH_W<'a, O> {
    #[doc = "External Reference Input 0 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_0(self) -> &'a mut W {
        self.variant(FXMXCH_A::FXMXCH_0)
    }
    #[doc = "External Reference Input 1 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_1(self) -> &'a mut W {
        self.variant(FXMXCH_A::FXMXCH_1)
    }
    #[doc = "External Reference Input 2 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_2(self) -> &'a mut W {
        self.variant(FXMXCH_A::FXMXCH_2)
    }
    #[doc = "External Reference Input 3 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_3(self) -> &'a mut W {
        self.variant(FXMXCH_A::FXMXCH_3)
    }
    #[doc = "External Reference Input 4 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_4(self) -> &'a mut W {
        self.variant(FXMXCH_A::FXMXCH_4)
    }
    #[doc = "External Reference Input 5 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_5(self) -> &'a mut W {
        self.variant(FXMXCH_A::FXMXCH_5)
    }
    #[doc = "The 8bit DAC is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn fxmxch_7(self) -> &'a mut W {
        self.variant(FXMXCH_A::FXMXCH_7)
    }
}
#[doc = "Field `FXMP` reader - Fixed MUX Port"]
pub type FXMP_R = crate::BitReader<FXMP_A>;
#[doc = "Fixed MUX Port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FXMP_A {
    #[doc = "0: The Plus port is fixed. Only the inputs to the Minus port are swept in each round."]
    FXMP_0 = 0,
    #[doc = "1: The Minus port is fixed. Only the inputs to the Plus port are swept in each round."]
    FXMP_1 = 1,
}
impl From<FXMP_A> for bool {
    #[inline(always)]
    fn from(variant: FXMP_A) -> Self {
        variant as u8 != 0
    }
}
impl FXMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FXMP_A {
        match self.bits {
            false => FXMP_A::FXMP_0,
            true => FXMP_A::FXMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `FXMP_0`"]
    #[inline(always)]
    pub fn is_fxmp_0(&self) -> bool {
        *self == FXMP_A::FXMP_0
    }
    #[doc = "Checks if the value of the field is `FXMP_1`"]
    #[inline(always)]
    pub fn is_fxmp_1(&self) -> bool {
        *self == FXMP_A::FXMP_1
    }
}
#[doc = "Field `FXMP` writer - Fixed MUX Port"]
pub type FXMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2_SPEC, FXMP_A, O>;
impl<'a, const O: u8> FXMP_W<'a, O> {
    #[doc = "The Plus port is fixed. Only the inputs to the Minus port are swept in each round."]
    #[inline(always)]
    pub fn fxmp_0(self) -> &'a mut W {
        self.variant(FXMP_A::FXMP_0)
    }
    #[doc = "The Minus port is fixed. Only the inputs to the Plus port are swept in each round."]
    #[inline(always)]
    pub fn fxmp_1(self) -> &'a mut W {
        self.variant(FXMP_A::FXMP_1)
    }
}
#[doc = "Field `RRIE` reader - Round-Robin interrupt enable"]
pub type RRIE_R = crate::BitReader<RRIE_A>;
#[doc = "Round-Robin interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRIE_A {
    #[doc = "0: The round-robin interrupt is disabled."]
    RRIE_0 = 0,
    #[doc = "1: The round-robin interrupt is enabled when a comparison result changes from the last sample."]
    RRIE_1 = 1,
}
impl From<RRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRIE_A {
        match self.bits {
            false => RRIE_A::RRIE_0,
            true => RRIE_A::RRIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RRIE_0`"]
    #[inline(always)]
    pub fn is_rrie_0(&self) -> bool {
        *self == RRIE_A::RRIE_0
    }
    #[doc = "Checks if the value of the field is `RRIE_1`"]
    #[inline(always)]
    pub fn is_rrie_1(&self) -> bool {
        *self == RRIE_A::RRIE_1
    }
}
#[doc = "Field `RRIE` writer - Round-Robin interrupt enable"]
pub type RRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2_SPEC, RRIE_A, O>;
impl<'a, const O: u8> RRIE_W<'a, O> {
    #[doc = "The round-robin interrupt is disabled."]
    #[inline(always)]
    pub fn rrie_0(self) -> &'a mut W {
        self.variant(RRIE_A::RRIE_0)
    }
    #[doc = "The round-robin interrupt is enabled when a comparison result changes from the last sample."]
    #[inline(always)]
    pub fn rrie_1(self) -> &'a mut W {
        self.variant(RRIE_A::RRIE_1)
    }
}
impl R {
    #[doc = "Bits 0:5 - ACOn"]
    #[inline(always)]
    pub fn acon(&self) -> ACON_R {
        ACON_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Comparator and DAC initialization delay modulus."]
    #[inline(always)]
    pub fn initmod(&self) -> INITMOD_R {
        INITMOD_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Number of sample clocks"]
    #[inline(always)]
    pub fn nsam(&self) -> NSAM_R {
        NSAM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - CH0F"]
    #[inline(always)]
    pub fn ch0f(&self) -> CH0F_R {
        CH0F_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CH1F"]
    #[inline(always)]
    pub fn ch1f(&self) -> CH1F_R {
        CH1F_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CH2F"]
    #[inline(always)]
    pub fn ch2f(&self) -> CH2F_R {
        CH2F_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CH3F"]
    #[inline(always)]
    pub fn ch3f(&self) -> CH3F_R {
        CH3F_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CH4F"]
    #[inline(always)]
    pub fn ch4f(&self) -> CH4F_R {
        CH4F_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CH5F"]
    #[inline(always)]
    pub fn ch5f(&self) -> CH5F_R {
        CH5F_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Fixed channel selection"]
    #[inline(always)]
    pub fn fxmxch(&self) -> FXMXCH_R {
        FXMXCH_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 29 - Fixed MUX Port"]
    #[inline(always)]
    pub fn fxmp(&self) -> FXMP_R {
        FXMP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Round-Robin interrupt enable"]
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - ACOn"]
    #[inline(always)]
    #[must_use]
    pub fn acon(&mut self) -> ACON_W<0> {
        ACON_W::new(self)
    }
    #[doc = "Bits 8:13 - Comparator and DAC initialization delay modulus."]
    #[inline(always)]
    #[must_use]
    pub fn initmod(&mut self) -> INITMOD_W<8> {
        INITMOD_W::new(self)
    }
    #[doc = "Bits 14:15 - Number of sample clocks"]
    #[inline(always)]
    #[must_use]
    pub fn nsam(&mut self) -> NSAM_W<14> {
        NSAM_W::new(self)
    }
    #[doc = "Bit 16 - CH0F"]
    #[inline(always)]
    #[must_use]
    pub fn ch0f(&mut self) -> CH0F_W<16> {
        CH0F_W::new(self)
    }
    #[doc = "Bit 17 - CH1F"]
    #[inline(always)]
    #[must_use]
    pub fn ch1f(&mut self) -> CH1F_W<17> {
        CH1F_W::new(self)
    }
    #[doc = "Bit 18 - CH2F"]
    #[inline(always)]
    #[must_use]
    pub fn ch2f(&mut self) -> CH2F_W<18> {
        CH2F_W::new(self)
    }
    #[doc = "Bit 19 - CH3F"]
    #[inline(always)]
    #[must_use]
    pub fn ch3f(&mut self) -> CH3F_W<19> {
        CH3F_W::new(self)
    }
    #[doc = "Bit 20 - CH4F"]
    #[inline(always)]
    #[must_use]
    pub fn ch4f(&mut self) -> CH4F_W<20> {
        CH4F_W::new(self)
    }
    #[doc = "Bit 21 - CH5F"]
    #[inline(always)]
    #[must_use]
    pub fn ch5f(&mut self) -> CH5F_W<21> {
        CH5F_W::new(self)
    }
    #[doc = "Bits 25:27 - Fixed channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn fxmxch(&mut self) -> FXMXCH_W<25> {
        FXMXCH_W::new(self)
    }
    #[doc = "Bit 29 - Fixed MUX Port"]
    #[inline(always)]
    #[must_use]
    pub fn fxmp(&mut self) -> FXMP_W<29> {
        FXMP_W::new(self)
    }
    #[doc = "Bit 30 - Round-Robin interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rrie(&mut self) -> RRIE_W<30> {
        RRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMP Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](index.html) module"]
pub struct C2_SPEC;
impl crate::RegisterSpec for C2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2::R](R) reader structure"]
impl crate::Readable for C2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2::W](W) writer structure"]
impl crate::Writable for C2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x003f_0000;
}
#[doc = "`reset()` method sets C2 to value 0"]
impl crate::Resettable for C2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
