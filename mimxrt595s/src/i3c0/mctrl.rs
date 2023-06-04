#[doc = "Register `MCTRL` reader"]
pub struct R(crate::R<MCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTRL` writer"]
pub struct W(crate::W<MCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTRL_SPEC>;
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
impl From<crate::W<MCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQUEST` reader - Request"]
pub type REQUEST_R = crate::FieldReader<u8, REQUEST_A>;
#[doc = "Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REQUEST_A {
    #[doc = "0: NONE"]
    NONE = 0,
    #[doc = "1: EMITSTARTADDR"]
    EMITSTARTADDR = 1,
    #[doc = "2: EMITSTOP"]
    EMITSTOP = 2,
    #[doc = "3: IBIACKNACK"]
    IBIACKNACK = 3,
    #[doc = "4: PROCESSDAA"]
    PROCESSDAA = 4,
    #[doc = "6: FORCEEXIT and IBHR"]
    FORCEEXIT = 6,
    #[doc = "7: AUTOIBI"]
    AUTOIBI = 7,
}
impl From<REQUEST_A> for u8 {
    #[inline(always)]
    fn from(variant: REQUEST_A) -> Self {
        variant as _
    }
}
impl REQUEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REQUEST_A> {
        match self.bits {
            0 => Some(REQUEST_A::NONE),
            1 => Some(REQUEST_A::EMITSTARTADDR),
            2 => Some(REQUEST_A::EMITSTOP),
            3 => Some(REQUEST_A::IBIACKNACK),
            4 => Some(REQUEST_A::PROCESSDAA),
            6 => Some(REQUEST_A::FORCEEXIT),
            7 => Some(REQUEST_A::AUTOIBI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == REQUEST_A::NONE
    }
    #[doc = "Checks if the value of the field is `EMITSTARTADDR`"]
    #[inline(always)]
    pub fn is_emitstartaddr(&self) -> bool {
        *self == REQUEST_A::EMITSTARTADDR
    }
    #[doc = "Checks if the value of the field is `EMITSTOP`"]
    #[inline(always)]
    pub fn is_emitstop(&self) -> bool {
        *self == REQUEST_A::EMITSTOP
    }
    #[doc = "Checks if the value of the field is `IBIACKNACK`"]
    #[inline(always)]
    pub fn is_ibiacknack(&self) -> bool {
        *self == REQUEST_A::IBIACKNACK
    }
    #[doc = "Checks if the value of the field is `PROCESSDAA`"]
    #[inline(always)]
    pub fn is_processdaa(&self) -> bool {
        *self == REQUEST_A::PROCESSDAA
    }
    #[doc = "Checks if the value of the field is `FORCEEXIT`"]
    #[inline(always)]
    pub fn is_forceexit(&self) -> bool {
        *self == REQUEST_A::FORCEEXIT
    }
    #[doc = "Checks if the value of the field is `AUTOIBI`"]
    #[inline(always)]
    pub fn is_autoibi(&self) -> bool {
        *self == REQUEST_A::AUTOIBI
    }
}
#[doc = "Field `REQUEST` writer - Request"]
pub type REQUEST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCTRL_SPEC, u8, REQUEST_A, 3, O>;
impl<'a, const O: u8> REQUEST_W<'a, O> {
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(REQUEST_A::NONE)
    }
    #[doc = "EMITSTARTADDR"]
    #[inline(always)]
    pub fn emitstartaddr(self) -> &'a mut W {
        self.variant(REQUEST_A::EMITSTARTADDR)
    }
    #[doc = "EMITSTOP"]
    #[inline(always)]
    pub fn emitstop(self) -> &'a mut W {
        self.variant(REQUEST_A::EMITSTOP)
    }
    #[doc = "IBIACKNACK"]
    #[inline(always)]
    pub fn ibiacknack(self) -> &'a mut W {
        self.variant(REQUEST_A::IBIACKNACK)
    }
    #[doc = "PROCESSDAA"]
    #[inline(always)]
    pub fn processdaa(self) -> &'a mut W {
        self.variant(REQUEST_A::PROCESSDAA)
    }
    #[doc = "FORCEEXIT and IBHR"]
    #[inline(always)]
    pub fn forceexit(self) -> &'a mut W {
        self.variant(REQUEST_A::FORCEEXIT)
    }
    #[doc = "AUTOIBI"]
    #[inline(always)]
    pub fn autoibi(self) -> &'a mut W {
        self.variant(REQUEST_A::AUTOIBI)
    }
}
#[doc = "Field `TYPE` reader - Bus type with START"]
pub type TYPE_R = crate::FieldReader<u8, TYPE_A>;
#[doc = "Bus type with START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TYPE_A {
    #[doc = "0: I3C"]
    I3C = 0,
    #[doc = "1: I2C"]
    I2C = 1,
    #[doc = "2: DDR"]
    DDR = 2,
    #[doc = "3: For ForcedExit, this is forced IBHR."]
    FORCEDIBHR = 3,
}
impl From<TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as _
    }
}
impl TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE_A {
        match self.bits {
            0 => TYPE_A::I3C,
            1 => TYPE_A::I2C,
            2 => TYPE_A::DDR,
            3 => TYPE_A::FORCEDIBHR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `I3C`"]
    #[inline(always)]
    pub fn is_i3c(&self) -> bool {
        *self == TYPE_A::I3C
    }
    #[doc = "Checks if the value of the field is `I2C`"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == TYPE_A::I2C
    }
    #[doc = "Checks if the value of the field is `DDR`"]
    #[inline(always)]
    pub fn is_ddr(&self) -> bool {
        *self == TYPE_A::DDR
    }
    #[doc = "Checks if the value of the field is `FORCEDIBHR`"]
    #[inline(always)]
    pub fn is_forcedibhr(&self) -> bool {
        *self == TYPE_A::FORCEDIBHR
    }
}
#[doc = "Field `TYPE` writer - Bus type with START"]
pub type TYPE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MCTRL_SPEC, u8, TYPE_A, 2, O>;
impl<'a, const O: u8> TYPE_W<'a, O> {
    #[doc = "I3C"]
    #[inline(always)]
    pub fn i3c(self) -> &'a mut W {
        self.variant(TYPE_A::I3C)
    }
    #[doc = "I2C"]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut W {
        self.variant(TYPE_A::I2C)
    }
    #[doc = "DDR"]
    #[inline(always)]
    pub fn ddr(self) -> &'a mut W {
        self.variant(TYPE_A::DDR)
    }
    #[doc = "For ForcedExit, this is forced IBHR."]
    #[inline(always)]
    pub fn forcedibhr(self) -> &'a mut W {
        self.variant(TYPE_A::FORCEDIBHR)
    }
}
#[doc = "Field `IBIRESP` reader - In-Band Interrupt (IBI) response"]
pub type IBIRESP_R = crate::FieldReader<u8, IBIRESP_A>;
#[doc = "In-Band Interrupt (IBI) response\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IBIRESP_A {
    #[doc = "0: ACK"]
    ACK = 0,
    #[doc = "1: NACK"]
    NACK = 1,
    #[doc = "2: ACK_WITH_MANDATORY"]
    ACK_WITH_MANDATORY = 2,
    #[doc = "3: MANUAL"]
    MANUAL = 3,
}
impl From<IBIRESP_A> for u8 {
    #[inline(always)]
    fn from(variant: IBIRESP_A) -> Self {
        variant as _
    }
}
impl IBIRESP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBIRESP_A {
        match self.bits {
            0 => IBIRESP_A::ACK,
            1 => IBIRESP_A::NACK,
            2 => IBIRESP_A::ACK_WITH_MANDATORY,
            3 => IBIRESP_A::MANUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == IBIRESP_A::ACK
    }
    #[doc = "Checks if the value of the field is `NACK`"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == IBIRESP_A::NACK
    }
    #[doc = "Checks if the value of the field is `ACK_WITH_MANDATORY`"]
    #[inline(always)]
    pub fn is_ack_with_mandatory(&self) -> bool {
        *self == IBIRESP_A::ACK_WITH_MANDATORY
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == IBIRESP_A::MANUAL
    }
}
#[doc = "Field `IBIRESP` writer - In-Band Interrupt (IBI) response"]
pub type IBIRESP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MCTRL_SPEC, u8, IBIRESP_A, 2, O>;
impl<'a, const O: u8> IBIRESP_W<'a, O> {
    #[doc = "ACK"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(IBIRESP_A::ACK)
    }
    #[doc = "NACK"]
    #[inline(always)]
    pub fn nack(self) -> &'a mut W {
        self.variant(IBIRESP_A::NACK)
    }
    #[doc = "ACK_WITH_MANDATORY"]
    #[inline(always)]
    pub fn ack_with_mandatory(self) -> &'a mut W {
        self.variant(IBIRESP_A::ACK_WITH_MANDATORY)
    }
    #[doc = "MANUAL"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut W {
        self.variant(IBIRESP_A::MANUAL)
    }
}
#[doc = "Field `DIR` reader - DIR"]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "DIR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: DIRWRITE: Write"]
    DIRWRITE = 0,
    #[doc = "1: DIRREAD: Read"]
    DIRREAD = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::DIRWRITE,
            true => DIR_A::DIRREAD,
        }
    }
    #[doc = "Checks if the value of the field is `DIRWRITE`"]
    #[inline(always)]
    pub fn is_dirwrite(&self) -> bool {
        *self == DIR_A::DIRWRITE
    }
    #[doc = "Checks if the value of the field is `DIRREAD`"]
    #[inline(always)]
    pub fn is_dirread(&self) -> bool {
        *self == DIR_A::DIRREAD
    }
}
#[doc = "Field `DIR` writer - DIR"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCTRL_SPEC, DIR_A, O>;
impl<'a, const O: u8> DIR_W<'a, O> {
    #[doc = "DIRWRITE: Write"]
    #[inline(always)]
    pub fn dirwrite(self) -> &'a mut W {
        self.variant(DIR_A::DIRWRITE)
    }
    #[doc = "DIRREAD: Read"]
    #[inline(always)]
    pub fn dirread(self) -> &'a mut W {
        self.variant(DIR_A::DIRREAD)
    }
}
#[doc = "Field `ADDR` reader - ADDR"]
pub type ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR` writer - ADDR"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `RDTERM` reader - Read terminate"]
pub type RDTERM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDTERM` writer - Read terminate"]
pub type RDTERM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:2 - Request"]
    #[inline(always)]
    pub fn request(&self) -> REQUEST_R {
        REQUEST_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - Bus type with START"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - In-Band Interrupt (IBI) response"]
    #[inline(always)]
    pub fn ibiresp(&self) -> IBIRESP_R {
        IBIRESP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - Read terminate"]
    #[inline(always)]
    pub fn rdterm(&self) -> RDTERM_R {
        RDTERM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Request"]
    #[inline(always)]
    #[must_use]
    pub fn request(&mut self) -> REQUEST_W<0> {
        REQUEST_W::new(self)
    }
    #[doc = "Bits 4:5 - Bus type with START"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TYPE_W<4> {
        TYPE_W::new(self)
    }
    #[doc = "Bits 6:7 - In-Band Interrupt (IBI) response"]
    #[inline(always)]
    #[must_use]
    pub fn ibiresp(&mut self) -> IBIRESP_W<6> {
        IBIRESP_W::new(self)
    }
    #[doc = "Bit 8 - DIR"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<8> {
        DIR_W::new(self)
    }
    #[doc = "Bits 9:15 - ADDR"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<9> {
        ADDR_W::new(self)
    }
    #[doc = "Bits 16:23 - Read terminate"]
    #[inline(always)]
    #[must_use]
    pub fn rdterm(&mut self) -> RDTERM_W<16> {
        RDTERM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Main Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctrl](index.html) module"]
pub struct MCTRL_SPEC;
impl crate::RegisterSpec for MCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctrl::R](R) reader structure"]
impl crate::Readable for MCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctrl::W](W) writer structure"]
impl crate::Writable for MCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTRL to value 0"]
impl crate::Resettable for MCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
