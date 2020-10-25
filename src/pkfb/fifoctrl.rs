#[doc = "Reader of register FIFOCTRL"]
pub type R = crate::R<u32, super::FIFOCTRL>;
#[doc = "Writer for register FIFOCTRL"]
pub type W = crate::W<u32, super::FIFOCTRL>;
#[doc = "Register FIFOCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "pf0_en\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF0_EN_A {
    #[doc = "0: Disabled."]
    DISABLE = 0,
    #[doc = "1: Enabled."]
    ENABLE = 1,
}
impl From<PF0_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PF0_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `pf0_en`"]
pub type PF0_EN_R = crate::R<bool, PF0_EN_A>;
impl PF0_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF0_EN_A {
        match self.bits {
            false => PF0_EN_A::DISABLE,
            true => PF0_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PF0_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PF0_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `pf0_en`"]
pub struct PF0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PF0_EN_A::DISABLE)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PF0_EN_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "pf0_push_mux\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF0_PUSH_MUX_A {
    #[doc = "0: M4."]
    M4 = 0,
    #[doc = "1: FFE."]
    FFE = 1,
}
impl From<PF0_PUSH_MUX_A> for bool {
    #[inline(always)]
    fn from(variant: PF0_PUSH_MUX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `pf0_push_mux`"]
pub type PF0_PUSH_MUX_R = crate::R<bool, PF0_PUSH_MUX_A>;
impl PF0_PUSH_MUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF0_PUSH_MUX_A {
        match self.bits {
            false => PF0_PUSH_MUX_A::M4,
            true => PF0_PUSH_MUX_A::FFE,
        }
    }
    #[doc = "Checks if the value of the field is `M4`"]
    #[inline(always)]
    pub fn is_m4(&self) -> bool {
        *self == PF0_PUSH_MUX_A::M4
    }
    #[doc = "Checks if the value of the field is `FFE`"]
    #[inline(always)]
    pub fn is_ffe(&self) -> bool {
        *self == PF0_PUSH_MUX_A::FFE
    }
}
#[doc = "Write proxy for field `pf0_push_mux`"]
pub struct PF0_PUSH_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_PUSH_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_PUSH_MUX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "M4."]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF0_PUSH_MUX_A::M4)
    }
    #[doc = "FFE."]
    #[inline(always)]
    pub fn ffe(self) -> &'a mut W {
        self.variant(PF0_PUSH_MUX_A::FFE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "pf0_pop_mux\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF0_POP_MUX_A {
    #[doc = "0: M4."]
    M4 = 0,
    #[doc = "1: AP."]
    AP = 1,
}
impl From<PF0_POP_MUX_A> for bool {
    #[inline(always)]
    fn from(variant: PF0_POP_MUX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `pf0_pop_mux`"]
pub type PF0_POP_MUX_R = crate::R<bool, PF0_POP_MUX_A>;
impl PF0_POP_MUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF0_POP_MUX_A {
        match self.bits {
            false => PF0_POP_MUX_A::M4,
            true => PF0_POP_MUX_A::AP,
        }
    }
    #[doc = "Checks if the value of the field is `M4`"]
    #[inline(always)]
    pub fn is_m4(&self) -> bool {
        *self == PF0_POP_MUX_A::M4
    }
    #[doc = "Checks if the value of the field is `AP`"]
    #[inline(always)]
    pub fn is_ap(&self) -> bool {
        *self == PF0_POP_MUX_A::AP
    }
}
#[doc = "Write proxy for field `pf0_pop_mux`"]
pub struct PF0_POP_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_POP_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_POP_MUX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "M4."]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF0_POP_MUX_A::M4)
    }
    #[doc = "AP."]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(PF0_POP_MUX_A::AP)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "pf0_push_int_mux\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF0_PUSH_INT_MUX_A {
    #[doc = "0: M4."]
    M4 = 0,
    #[doc = "1: AP."]
    AP = 1,
}
impl From<PF0_PUSH_INT_MUX_A> for bool {
    #[inline(always)]
    fn from(variant: PF0_PUSH_INT_MUX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `pf0_push_int_mux`"]
pub type PF0_PUSH_INT_MUX_R = crate::R<bool, PF0_PUSH_INT_MUX_A>;
impl PF0_PUSH_INT_MUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF0_PUSH_INT_MUX_A {
        match self.bits {
            false => PF0_PUSH_INT_MUX_A::M4,
            true => PF0_PUSH_INT_MUX_A::AP,
        }
    }
    #[doc = "Checks if the value of the field is `M4`"]
    #[inline(always)]
    pub fn is_m4(&self) -> bool {
        *self == PF0_PUSH_INT_MUX_A::M4
    }
    #[doc = "Checks if the value of the field is `AP`"]
    #[inline(always)]
    pub fn is_ap(&self) -> bool {
        *self == PF0_PUSH_INT_MUX_A::AP
    }
}
#[doc = "Write proxy for field `pf0_push_int_mux`"]
pub struct PF0_PUSH_INT_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_PUSH_INT_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_PUSH_INT_MUX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "M4."]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF0_PUSH_INT_MUX_A::M4)
    }
    #[doc = "AP."]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(PF0_PUSH_INT_MUX_A::AP)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "pf0_pop_int_mux\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF0_POP_INT_MUX_A {
    #[doc = "0: M4."]
    M4 = 0,
    #[doc = "1: AP."]
    AP = 1,
}
impl From<PF0_POP_INT_MUX_A> for bool {
    #[inline(always)]
    fn from(variant: PF0_POP_INT_MUX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `pf0_pop_int_mux`"]
pub type PF0_POP_INT_MUX_R = crate::R<bool, PF0_POP_INT_MUX_A>;
impl PF0_POP_INT_MUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF0_POP_INT_MUX_A {
        match self.bits {
            false => PF0_POP_INT_MUX_A::M4,
            true => PF0_POP_INT_MUX_A::AP,
        }
    }
    #[doc = "Checks if the value of the field is `M4`"]
    #[inline(always)]
    pub fn is_m4(&self) -> bool {
        *self == PF0_POP_INT_MUX_A::M4
    }
    #[doc = "Checks if the value of the field is `AP`"]
    #[inline(always)]
    pub fn is_ap(&self) -> bool {
        *self == PF0_POP_INT_MUX_A::AP
    }
}
#[doc = "Write proxy for field `pf0_pop_int_mux`"]
pub struct PF0_POP_INT_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_POP_INT_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_POP_INT_MUX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "M4."]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF0_POP_INT_MUX_A::M4)
    }
    #[doc = "AP."]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(PF0_POP_INT_MUX_A::AP)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "pf0_ffe_sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF0_FFE_SEL_A {
    #[doc = "0: FFE0."]
    FFE0 = 0,
    #[doc = "1: FFE1."]
    FFE1 = 1,
}
impl From<PF0_FFE_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: PF0_FFE_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `pf0_ffe_sel`"]
pub type PF0_FFE_SEL_R = crate::R<bool, PF0_FFE_SEL_A>;
impl PF0_FFE_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF0_FFE_SEL_A {
        match self.bits {
            false => PF0_FFE_SEL_A::FFE0,
            true => PF0_FFE_SEL_A::FFE1,
        }
    }
    #[doc = "Checks if the value of the field is `FFE0`"]
    #[inline(always)]
    pub fn is_ffe0(&self) -> bool {
        *self == PF0_FFE_SEL_A::FFE0
    }
    #[doc = "Checks if the value of the field is `FFE1`"]
    #[inline(always)]
    pub fn is_ffe1(&self) -> bool {
        *self == PF0_FFE_SEL_A::FFE1
    }
}
#[doc = "Write proxy for field `pf0_ffe_sel`"]
pub struct PF0_FFE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_FFE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_FFE_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FFE0."]
    #[inline(always)]
    pub fn ffe0(self) -> &'a mut W {
        self.variant(PF0_FFE_SEL_A::FFE0)
    }
    #[doc = "FFE1."]
    #[inline(always)]
    pub fn ffe1(self) -> &'a mut W {
        self.variant(PF0_FFE_SEL_A::FFE1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - pf0_en"]
    #[inline(always)]
    pub fn pf0_en(&self) -> PF0_EN_R {
        PF0_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - pf0_push_mux"]
    #[inline(always)]
    pub fn pf0_push_mux(&self) -> PF0_PUSH_MUX_R {
        PF0_PUSH_MUX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - pf0_pop_mux"]
    #[inline(always)]
    pub fn pf0_pop_mux(&self) -> PF0_POP_MUX_R {
        PF0_POP_MUX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - pf0_push_int_mux"]
    #[inline(always)]
    pub fn pf0_push_int_mux(&self) -> PF0_PUSH_INT_MUX_R {
        PF0_PUSH_INT_MUX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - pf0_pop_int_mux"]
    #[inline(always)]
    pub fn pf0_pop_int_mux(&self) -> PF0_POP_INT_MUX_R {
        PF0_POP_INT_MUX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - pf0_ffe_sel"]
    #[inline(always)]
    pub fn pf0_ffe_sel(&self) -> PF0_FFE_SEL_R {
        PF0_FFE_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - pf0_en"]
    #[inline(always)]
    pub fn pf0_en(&mut self) -> PF0_EN_W {
        PF0_EN_W { w: self }
    }
    #[doc = "Bit 1 - pf0_push_mux"]
    #[inline(always)]
    pub fn pf0_push_mux(&mut self) -> PF0_PUSH_MUX_W {
        PF0_PUSH_MUX_W { w: self }
    }
    #[doc = "Bit 2 - pf0_pop_mux"]
    #[inline(always)]
    pub fn pf0_pop_mux(&mut self) -> PF0_POP_MUX_W {
        PF0_POP_MUX_W { w: self }
    }
    #[doc = "Bit 3 - pf0_push_int_mux"]
    #[inline(always)]
    pub fn pf0_push_int_mux(&mut self) -> PF0_PUSH_INT_MUX_W {
        PF0_PUSH_INT_MUX_W { w: self }
    }
    #[doc = "Bit 4 - pf0_pop_int_mux"]
    #[inline(always)]
    pub fn pf0_pop_int_mux(&mut self) -> PF0_POP_INT_MUX_W {
        PF0_POP_INT_MUX_W { w: self }
    }
    #[doc = "Bit 5 - pf0_ffe_sel"]
    #[inline(always)]
    pub fn pf0_ffe_sel(&mut self) -> PF0_FFE_SEL_W {
        PF0_FFE_SEL_W { w: self }
    }
}
