#[doc = "Writer for register CGATCLR1"]
pub type W = crate::W<u32, super::CGATCLR1>;
#[doc = "Register CGATCLR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CGATCLR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LEDTS Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEDTSCU0_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<LEDTSCU0_AW> for bool {
    #[inline(always)]
    fn from(variant: LEDTSCU0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `LEDTSCU0`"]
pub struct LEDTSCU0_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDTSCU0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEDTSCU0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LEDTSCU0_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LEDTSCU0_AW::VALUE2)
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
#[doc = "MultiCAN Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCAN0_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<MCAN0_AW> for bool {
    #[inline(always)]
    fn from(variant: MCAN0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `MCAN0`"]
pub struct MCAN0_W<'a> {
    w: &'a mut W,
}
impl<'a> MCAN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCAN0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCAN0_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCAN0_AW::VALUE2)
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
#[doc = "DAC Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<DAC_AW> for bool {
    #[inline(always)]
    fn from(variant: DAC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DAC`"]
pub struct DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DAC_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DAC_AW::VALUE2)
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
#[doc = "USIC1 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC1_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<USIC1_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `USIC1`"]
pub struct USIC1_W<'a> {
    w: &'a mut W,
}
impl<'a> USIC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USIC1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIC1_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIC1_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "PORTS Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPORTS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<PPORTS_AW> for bool {
    #[inline(always)]
    fn from(variant: PPORTS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PPORTS`"]
pub struct PPORTS_W<'a> {
    w: &'a mut W,
}
impl<'a> PPORTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPORTS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPORTS_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPORTS_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl W {
    #[doc = "Bit 3 - LEDTS Gating Clear"]
    #[inline(always)]
    pub fn ledtscu0(&mut self) -> LEDTSCU0_W {
        LEDTSCU0_W { w: self }
    }
    #[doc = "Bit 4 - MultiCAN Gating Clear"]
    #[inline(always)]
    pub fn mcan0(&mut self) -> MCAN0_W {
        MCAN0_W { w: self }
    }
    #[doc = "Bit 5 - DAC Gating Clear"]
    #[inline(always)]
    pub fn dac(&mut self) -> DAC_W {
        DAC_W { w: self }
    }
    #[doc = "Bit 7 - USIC1 Gating Clear"]
    #[inline(always)]
    pub fn usic1(&mut self) -> USIC1_W {
        USIC1_W { w: self }
    }
    #[doc = "Bit 9 - PORTS Gating Clear"]
    #[inline(always)]
    pub fn pports(&mut self) -> PPORTS_W {
        PPORTS_W { w: self }
    }
}
