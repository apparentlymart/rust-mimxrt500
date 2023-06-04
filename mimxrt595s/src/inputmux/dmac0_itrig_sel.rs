#[doc = "Register `DMAC0_ITRIG_SEL[%s]` reader"]
pub struct R(crate::R<DMAC0_ITRIG_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC0_ITRIG_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC0_ITRIG_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC0_ITRIG_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC0_ITRIG_SEL[%s]` writer"]
pub struct W(crate::W<DMAC0_ITRIG_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC0_ITRIG_SEL_SPEC>;
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
impl From<crate::W<DMAC0_ITRIG_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC0_ITRIG_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAC0_ITRIG_SEL` reader - DMA Input Trigger Selection."]
pub type DMAC0_ITRIG_SEL_R = crate::FieldReader<u8, DMAC0_ITRIG_SEL_A>;
#[doc = "DMA Input Trigger Selection.\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMAC0_ITRIG_SEL_A {
    #[doc = "0: GPIO_INT0"]
    GPIO_INT0 = 0,
    #[doc = "1: GPIO_INT1"]
    GPIO_INT1 = 1,
    #[doc = "2: GPIO_INT2"]
    GPIO_INT2 = 2,
    #[doc = "3: GPIO_INT3"]
    GPIO_INT3 = 3,
    #[doc = "4: T0_DMAREQ_M0"]
    T0_DMAREQ_0 = 4,
    #[doc = "5: T0_DMAREQ_M1"]
    T0_DMAREQ_M1 = 5,
    #[doc = "6: T1_DMAREQ_M0"]
    T0_DMAREQ_6 = 6,
    #[doc = "7: T1_DMAREQ_M1"]
    T0_DMAREQ_7 = 7,
    #[doc = "8: T2_DMAREQ_M0"]
    T0_DMAREQ_8 = 8,
    #[doc = "9: T2_DMAREQ_M1"]
    T0_DMAREQ_9 = 9,
    #[doc = "10: T3_DMAREQ_M0"]
    T0_DMAREQ_A = 10,
    #[doc = "11: T3_DMAREQ_M1"]
    T0_DMAREQ_B = 11,
    #[doc = "12: T4_DMAREQ_M0"]
    T0_DMAREQ_C = 12,
    #[doc = "13: T4_DMAREQ_M1"]
    T0_DMAREQ_D = 13,
    #[doc = "14: DMA0_TRIGOUT_A"]
    T0_DMAREQ_E = 14,
    #[doc = "15: DMA0_TRIGOUT_B"]
    DMA0_TRIGOUT_B = 15,
    #[doc = "16: DMA0_TRIGOUT_C"]
    DMA0_TRIGOUT_C = 16,
    #[doc = "17: DMA0_TRIGOUT_D"]
    DMA0_TRIGOUT_D = 17,
    #[doc = "18: SCT_DMAC0_REQ0"]
    SCT0_DMAC0 = 18,
    #[doc = "19: SCT_DMAC1_REQ1"]
    SCT_DMAC1 = 19,
    #[doc = "20: HASHCRYPT_OUT_DMA"]
    HASHCRYPT_OUT_DMA = 20,
    #[doc = "21: ACMP_DMA"]
    ACMP_DMA = 21,
    #[doc = "22: FlexSPI0_RX_DMA"]
    FLEX_SPI0_RX_DMA = 22,
    #[doc = "23: FlexSPI0_TX_DMA"]
    FLEX_SPI0_TX_DMA = 23,
    #[doc = "24: ADC_DMA"]
    ADC_DMA = 24,
    #[doc = "25: FlexSPI1_RX_DMA"]
    FLEX_SPI1_RX_DMA = 25,
    #[doc = "26: FlexSPI1_TX_DMA"]
    FLEX_SPI1_TX_DMA = 26,
}
impl From<DMAC0_ITRIG_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAC0_ITRIG_SEL_A) -> Self {
        variant as _
    }
}
impl DMAC0_ITRIG_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMAC0_ITRIG_SEL_A> {
        match self.bits {
            0 => Some(DMAC0_ITRIG_SEL_A::GPIO_INT0),
            1 => Some(DMAC0_ITRIG_SEL_A::GPIO_INT1),
            2 => Some(DMAC0_ITRIG_SEL_A::GPIO_INT2),
            3 => Some(DMAC0_ITRIG_SEL_A::GPIO_INT3),
            4 => Some(DMAC0_ITRIG_SEL_A::T0_DMAREQ_0),
            5 => Some(DMAC0_ITRIG_SEL_A::T0_DMAREQ_M1),
            6 => Some(DMAC0_ITRIG_SEL_A::T0_DMAREQ_6),
            7 => Some(DMAC0_ITRIG_SEL_A::T0_DMAREQ_7),
            8 => Some(DMAC0_ITRIG_SEL_A::T0_DMAREQ_8),
            9 => Some(DMAC0_ITRIG_SEL_A::T0_DMAREQ_9),
            10 => Some(DMAC0_ITRIG_SEL_A::T0_DMAREQ_A),
            11 => Some(DMAC0_ITRIG_SEL_A::T0_DMAREQ_B),
            12 => Some(DMAC0_ITRIG_SEL_A::T0_DMAREQ_C),
            13 => Some(DMAC0_ITRIG_SEL_A::T0_DMAREQ_D),
            14 => Some(DMAC0_ITRIG_SEL_A::T0_DMAREQ_E),
            15 => Some(DMAC0_ITRIG_SEL_A::DMA0_TRIGOUT_B),
            16 => Some(DMAC0_ITRIG_SEL_A::DMA0_TRIGOUT_C),
            17 => Some(DMAC0_ITRIG_SEL_A::DMA0_TRIGOUT_D),
            18 => Some(DMAC0_ITRIG_SEL_A::SCT0_DMAC0),
            19 => Some(DMAC0_ITRIG_SEL_A::SCT_DMAC1),
            20 => Some(DMAC0_ITRIG_SEL_A::HASHCRYPT_OUT_DMA),
            21 => Some(DMAC0_ITRIG_SEL_A::ACMP_DMA),
            22 => Some(DMAC0_ITRIG_SEL_A::FLEX_SPI0_RX_DMA),
            23 => Some(DMAC0_ITRIG_SEL_A::FLEX_SPI0_TX_DMA),
            24 => Some(DMAC0_ITRIG_SEL_A::ADC_DMA),
            25 => Some(DMAC0_ITRIG_SEL_A::FLEX_SPI1_RX_DMA),
            26 => Some(DMAC0_ITRIG_SEL_A::FLEX_SPI1_TX_DMA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_INT0`"]
    #[inline(always)]
    pub fn is_gpio_int0(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::GPIO_INT0
    }
    #[doc = "Checks if the value of the field is `GPIO_INT1`"]
    #[inline(always)]
    pub fn is_gpio_int1(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::GPIO_INT1
    }
    #[doc = "Checks if the value of the field is `GPIO_INT2`"]
    #[inline(always)]
    pub fn is_gpio_int2(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::GPIO_INT2
    }
    #[doc = "Checks if the value of the field is `GPIO_INT3`"]
    #[inline(always)]
    pub fn is_gpio_int3(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::GPIO_INT3
    }
    #[doc = "Checks if the value of the field is `T0_DMAREQ_0`"]
    #[inline(always)]
    pub fn is_t0_dmareq_0(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::T0_DMAREQ_0
    }
    #[doc = "Checks if the value of the field is `T0_DMAREQ_M1`"]
    #[inline(always)]
    pub fn is_t0_dmareq_m1(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::T0_DMAREQ_M1
    }
    #[doc = "Checks if the value of the field is `T0_DMAREQ_6`"]
    #[inline(always)]
    pub fn is_t0_dmareq_6(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::T0_DMAREQ_6
    }
    #[doc = "Checks if the value of the field is `T0_DMAREQ_7`"]
    #[inline(always)]
    pub fn is_t0_dmareq_7(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::T0_DMAREQ_7
    }
    #[doc = "Checks if the value of the field is `T0_DMAREQ_8`"]
    #[inline(always)]
    pub fn is_t0_dmareq_8(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::T0_DMAREQ_8
    }
    #[doc = "Checks if the value of the field is `T0_DMAREQ_9`"]
    #[inline(always)]
    pub fn is_t0_dmareq_9(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::T0_DMAREQ_9
    }
    #[doc = "Checks if the value of the field is `T0_DMAREQ_A`"]
    #[inline(always)]
    pub fn is_t0_dmareq_a(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::T0_DMAREQ_A
    }
    #[doc = "Checks if the value of the field is `T0_DMAREQ_B`"]
    #[inline(always)]
    pub fn is_t0_dmareq_b(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::T0_DMAREQ_B
    }
    #[doc = "Checks if the value of the field is `T0_DMAREQ_C`"]
    #[inline(always)]
    pub fn is_t0_dmareq_c(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::T0_DMAREQ_C
    }
    #[doc = "Checks if the value of the field is `T0_DMAREQ_D`"]
    #[inline(always)]
    pub fn is_t0_dmareq_d(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::T0_DMAREQ_D
    }
    #[doc = "Checks if the value of the field is `T0_DMAREQ_E`"]
    #[inline(always)]
    pub fn is_t0_dmareq_e(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::T0_DMAREQ_E
    }
    #[doc = "Checks if the value of the field is `DMA0_TRIGOUT_B`"]
    #[inline(always)]
    pub fn is_dma0_trigout_b(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::DMA0_TRIGOUT_B
    }
    #[doc = "Checks if the value of the field is `DMA0_TRIGOUT_C`"]
    #[inline(always)]
    pub fn is_dma0_trigout_c(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::DMA0_TRIGOUT_C
    }
    #[doc = "Checks if the value of the field is `DMA0_TRIGOUT_D`"]
    #[inline(always)]
    pub fn is_dma0_trigout_d(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::DMA0_TRIGOUT_D
    }
    #[doc = "Checks if the value of the field is `SCT0_DMAC0`"]
    #[inline(always)]
    pub fn is_sct0_dmac0(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::SCT0_DMAC0
    }
    #[doc = "Checks if the value of the field is `SCT_DMAC1`"]
    #[inline(always)]
    pub fn is_sct_dmac1(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::SCT_DMAC1
    }
    #[doc = "Checks if the value of the field is `HASHCRYPT_OUT_DMA`"]
    #[inline(always)]
    pub fn is_hashcrypt_out_dma(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::HASHCRYPT_OUT_DMA
    }
    #[doc = "Checks if the value of the field is `ACMP_DMA`"]
    #[inline(always)]
    pub fn is_acmp_dma(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::ACMP_DMA
    }
    #[doc = "Checks if the value of the field is `FLEX_SPI0_RX_DMA`"]
    #[inline(always)]
    pub fn is_flex_spi0_rx_dma(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::FLEX_SPI0_RX_DMA
    }
    #[doc = "Checks if the value of the field is `FLEX_SPI0_TX_DMA`"]
    #[inline(always)]
    pub fn is_flex_spi0_tx_dma(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::FLEX_SPI0_TX_DMA
    }
    #[doc = "Checks if the value of the field is `ADC_DMA`"]
    #[inline(always)]
    pub fn is_adc_dma(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::ADC_DMA
    }
    #[doc = "Checks if the value of the field is `FLEX_SPI1_RX_DMA`"]
    #[inline(always)]
    pub fn is_flex_spi1_rx_dma(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::FLEX_SPI1_RX_DMA
    }
    #[doc = "Checks if the value of the field is `FLEX_SPI1_TX_DMA`"]
    #[inline(always)]
    pub fn is_flex_spi1_tx_dma(&self) -> bool {
        *self == DMAC0_ITRIG_SEL_A::FLEX_SPI1_TX_DMA
    }
}
#[doc = "Field `DMAC0_ITRIG_SEL` writer - DMA Input Trigger Selection."]
pub type DMAC0_ITRIG_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMAC0_ITRIG_SEL_SPEC, u8, DMAC0_ITRIG_SEL_A, 5, O>;
impl<'a, const O: u8> DMAC0_ITRIG_SEL_W<'a, O> {
    #[doc = "GPIO_INT0"]
    #[inline(always)]
    pub fn gpio_int0(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::GPIO_INT0)
    }
    #[doc = "GPIO_INT1"]
    #[inline(always)]
    pub fn gpio_int1(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::GPIO_INT1)
    }
    #[doc = "GPIO_INT2"]
    #[inline(always)]
    pub fn gpio_int2(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::GPIO_INT2)
    }
    #[doc = "GPIO_INT3"]
    #[inline(always)]
    pub fn gpio_int3(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::GPIO_INT3)
    }
    #[doc = "T0_DMAREQ_M0"]
    #[inline(always)]
    pub fn t0_dmareq_0(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::T0_DMAREQ_0)
    }
    #[doc = "T0_DMAREQ_M1"]
    #[inline(always)]
    pub fn t0_dmareq_m1(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::T0_DMAREQ_M1)
    }
    #[doc = "T1_DMAREQ_M0"]
    #[inline(always)]
    pub fn t0_dmareq_6(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::T0_DMAREQ_6)
    }
    #[doc = "T1_DMAREQ_M1"]
    #[inline(always)]
    pub fn t0_dmareq_7(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::T0_DMAREQ_7)
    }
    #[doc = "T2_DMAREQ_M0"]
    #[inline(always)]
    pub fn t0_dmareq_8(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::T0_DMAREQ_8)
    }
    #[doc = "T2_DMAREQ_M1"]
    #[inline(always)]
    pub fn t0_dmareq_9(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::T0_DMAREQ_9)
    }
    #[doc = "T3_DMAREQ_M0"]
    #[inline(always)]
    pub fn t0_dmareq_a(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::T0_DMAREQ_A)
    }
    #[doc = "T3_DMAREQ_M1"]
    #[inline(always)]
    pub fn t0_dmareq_b(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::T0_DMAREQ_B)
    }
    #[doc = "T4_DMAREQ_M0"]
    #[inline(always)]
    pub fn t0_dmareq_c(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::T0_DMAREQ_C)
    }
    #[doc = "T4_DMAREQ_M1"]
    #[inline(always)]
    pub fn t0_dmareq_d(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::T0_DMAREQ_D)
    }
    #[doc = "DMA0_TRIGOUT_A"]
    #[inline(always)]
    pub fn t0_dmareq_e(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::T0_DMAREQ_E)
    }
    #[doc = "DMA0_TRIGOUT_B"]
    #[inline(always)]
    pub fn dma0_trigout_b(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::DMA0_TRIGOUT_B)
    }
    #[doc = "DMA0_TRIGOUT_C"]
    #[inline(always)]
    pub fn dma0_trigout_c(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::DMA0_TRIGOUT_C)
    }
    #[doc = "DMA0_TRIGOUT_D"]
    #[inline(always)]
    pub fn dma0_trigout_d(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::DMA0_TRIGOUT_D)
    }
    #[doc = "SCT_DMAC0_REQ0"]
    #[inline(always)]
    pub fn sct0_dmac0(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::SCT0_DMAC0)
    }
    #[doc = "SCT_DMAC1_REQ1"]
    #[inline(always)]
    pub fn sct_dmac1(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::SCT_DMAC1)
    }
    #[doc = "HASHCRYPT_OUT_DMA"]
    #[inline(always)]
    pub fn hashcrypt_out_dma(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::HASHCRYPT_OUT_DMA)
    }
    #[doc = "ACMP_DMA"]
    #[inline(always)]
    pub fn acmp_dma(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::ACMP_DMA)
    }
    #[doc = "FlexSPI0_RX_DMA"]
    #[inline(always)]
    pub fn flex_spi0_rx_dma(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::FLEX_SPI0_RX_DMA)
    }
    #[doc = "FlexSPI0_TX_DMA"]
    #[inline(always)]
    pub fn flex_spi0_tx_dma(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::FLEX_SPI0_TX_DMA)
    }
    #[doc = "ADC_DMA"]
    #[inline(always)]
    pub fn adc_dma(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::ADC_DMA)
    }
    #[doc = "FlexSPI1_RX_DMA"]
    #[inline(always)]
    pub fn flex_spi1_rx_dma(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::FLEX_SPI1_RX_DMA)
    }
    #[doc = "FlexSPI1_TX_DMA"]
    #[inline(always)]
    pub fn flex_spi1_tx_dma(self) -> &'a mut W {
        self.variant(DMAC0_ITRIG_SEL_A::FLEX_SPI1_TX_DMA)
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA Input Trigger Selection."]
    #[inline(always)]
    pub fn dmac0_itrig_sel(&self) -> DMAC0_ITRIG_SEL_R {
        DMAC0_ITRIG_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA Input Trigger Selection."]
    #[inline(always)]
    #[must_use]
    pub fn dmac0_itrig_sel(&mut self) -> DMAC0_ITRIG_SEL_W<0> {
        DMAC0_ITRIG_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC0 Input Trigger Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac0_itrig_sel](index.html) module"]
pub struct DMAC0_ITRIG_SEL_SPEC;
impl crate::RegisterSpec for DMAC0_ITRIG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac0_itrig_sel::R](R) reader structure"]
impl crate::Readable for DMAC0_ITRIG_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac0_itrig_sel::W](W) writer structure"]
impl crate::Writable for DMAC0_ITRIG_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAC0_ITRIG_SEL[%s]
to value 0x1f"]
impl crate::Resettable for DMAC0_ITRIG_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
