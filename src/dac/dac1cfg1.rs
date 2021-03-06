#[doc = "Reader of register DAC1CFG1"]
pub type R = crate::R<u32, super::DAC1CFG1>;
#[doc = "Writer for register DAC1CFG1"]
pub type W = crate::W<u32, super::DAC1CFG1>;
#[doc = "Register DAC1CFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC1CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Scale value for up- or downscale of the DAC1 input data in steps by the power of 2 (=shift operation)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCALE_A {
    #[doc = "0: no shift = multiplication/division by 1"]
    VALUE1 = 0,
    #[doc = "1: shift by 1 = multiplication/division by 2"]
    VALUE2 = 1,
    #[doc = "2: shift by 2 = multiplication/division by 4"]
    VALUE3 = 2,
    #[doc = "3: shift left by 3 = multiplication/division by 8"]
    VALUE4 = 3,
    #[doc = "4: shift left by 4 = multiplication/division by 16"]
    VALUE5 = 4,
    #[doc = "5: shift left by 5 = multiplication/division by 32"]
    VALUE6 = 5,
    #[doc = "6: shift left by 6 = multiplication/division by 64"]
    VALUE7 = 6,
    #[doc = "7: shift left by 7 = multiplication/division by 128"]
    VALUE8 = 7,
}
impl From<SCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: SCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SCALE`"]
pub type SCALE_R = crate::R<u8, SCALE_A>;
impl SCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCALE_A {
        match self.bits {
            0 => SCALE_A::VALUE1,
            1 => SCALE_A::VALUE2,
            2 => SCALE_A::VALUE3,
            3 => SCALE_A::VALUE4,
            4 => SCALE_A::VALUE5,
            5 => SCALE_A::VALUE6,
            6 => SCALE_A::VALUE7,
            7 => SCALE_A::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCALE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCALE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SCALE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SCALE_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == SCALE_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == SCALE_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == SCALE_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == SCALE_A::VALUE8
    }
}
#[doc = "Write proxy for field `SCALE`"]
pub struct SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCALE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "no shift = multiplication/division by 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE1)
    }
    #[doc = "shift by 1 = multiplication/division by 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE2)
    }
    #[doc = "shift by 2 = multiplication/division by 4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE3)
    }
    #[doc = "shift left by 3 = multiplication/division by 8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE4)
    }
    #[doc = "shift left by 4 = multiplication/division by 16"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE5)
    }
    #[doc = "shift left by 5 = multiplication/division by 32"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE6)
    }
    #[doc = "shift left by 6 = multiplication/division by 64"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE7)
    }
    #[doc = "shift left by 7 = multiplication/division by 128"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Switch between up- and downscale of the DAC1 input data values\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULDIV_A {
    #[doc = "0: downscale = division (shift SCALE positions to the right)"]
    VALUE1 = 0,
    #[doc = "1: upscale = multiplication (shift SCALE positions to the left)"]
    VALUE2 = 1,
}
impl From<MULDIV_A> for bool {
    #[inline(always)]
    fn from(variant: MULDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MULDIV`"]
pub type MULDIV_R = crate::R<bool, MULDIV_A>;
impl MULDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MULDIV_A {
        match self.bits {
            false => MULDIV_A::VALUE1,
            true => MULDIV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MULDIV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MULDIV_A::VALUE2
    }
}
#[doc = "Write proxy for field `MULDIV`"]
pub struct MULDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MULDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MULDIV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "downscale = division (shift SCALE positions to the right)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MULDIV_A::VALUE1)
    }
    #[doc = "upscale = multiplication (shift SCALE positions to the left)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MULDIV_A::VALUE2)
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
#[doc = "Reader of field `OFFS`"]
pub type OFFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OFFS`"]
pub struct OFFS_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | (((value as u32) & 0xff) << 4);
        self.w
    }
}
#[doc = "Reader of field `TRIGSEL`"]
pub type TRIGSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIGSEL`"]
pub struct TRIGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `SWTRIG`"]
pub type SWTRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWTRIG`"]
pub struct SWTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Select the trigger source for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGMOD_A {
    #[doc = "0: internal Trigger (integer divided clock - see FREQ parameter)"]
    VALUE1 = 0,
    #[doc = "1: external Trigger (preselected trigger by TRIGSEL parameter)"]
    VALUE2 = 1,
    #[doc = "2: software Trigger (see SWTRIG parameter)"]
    VALUE3 = 2,
}
impl From<TRIGMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRIGMOD`"]
pub type TRIGMOD_R = crate::R<u8, TRIGMOD_A>;
impl TRIGMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRIGMOD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRIGMOD_A::VALUE1),
            1 => Val(TRIGMOD_A::VALUE2),
            2 => Val(TRIGMOD_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRIGMOD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRIGMOD_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TRIGMOD_A::VALUE3
    }
}
#[doc = "Write proxy for field `TRIGMOD`"]
pub struct TRIGMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "internal Trigger (integer divided clock - see FREQ parameter)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRIGMOD_A::VALUE1)
    }
    #[doc = "external Trigger (preselected trigger by TRIGSEL parameter)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRIGMOD_A::VALUE2)
    }
    #[doc = "software Trigger (see SWTRIG parameter)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRIGMOD_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `ANACFG`"]
pub type ANACFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ANACFG`"]
pub struct ANACFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ANACFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Enable analog DAC for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANAEN_A {
    #[doc = "0: DAC1 is set to standby (analog output only)"]
    VALUE1 = 0,
    #[doc = "1: enable DAC1 (analog output only)"]
    VALUE2 = 1,
}
impl From<ANAEN_A> for bool {
    #[inline(always)]
    fn from(variant: ANAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ANAEN`"]
pub type ANAEN_R = crate::R<bool, ANAEN_A>;
impl ANAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANAEN_A {
        match self.bits {
            false => ANAEN_A::VALUE1,
            true => ANAEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ANAEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ANAEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `ANAEN`"]
pub struct ANAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ANAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC1 is set to standby (analog output only)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ANAEN_A::VALUE1)
    }
    #[doc = "enable DAC1 (analog output only)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ANAEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `REFCFGH`"]
pub type REFCFGH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFCFGH`"]
pub struct REFCFGH_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCFGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Scale value for up- or downscale of the DAC1 input data in steps by the power of 2 (=shift operation)"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Switch between up- and downscale of the DAC1 input data values"]
    #[inline(always)]
    pub fn muldiv(&self) -> MULDIV_R {
        MULDIV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:11 - 8-bit offset value addition"]
    #[inline(always)]
    pub fn offs(&self) -> OFFS_R {
        OFFS_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:14 - Selects one of the eight external trigger sources for DAC1"]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Software Trigger"]
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - Select the trigger source for channel 1"]
    #[inline(always)]
    pub fn trigmod(&self) -> TRIGMOD_R {
        TRIGMOD_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 19:23 - DAC1 analog configuration/calibration parameters"]
    #[inline(always)]
    pub fn anacfg(&self) -> ANACFG_R {
        ANACFG_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Enable analog DAC for channel 1"]
    #[inline(always)]
    pub fn anaen(&self) -> ANAEN_R {
        ANAEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - Higher 4 band-gap configuration/calibration parameters"]
    #[inline(always)]
    pub fn refcfgh(&self) -> REFCFGH_R {
        REFCFGH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Scale value for up- or downscale of the DAC1 input data in steps by the power of 2 (=shift operation)"]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W {
        SCALE_W { w: self }
    }
    #[doc = "Bit 3 - Switch between up- and downscale of the DAC1 input data values"]
    #[inline(always)]
    pub fn muldiv(&mut self) -> MULDIV_W {
        MULDIV_W { w: self }
    }
    #[doc = "Bits 4:11 - 8-bit offset value addition"]
    #[inline(always)]
    pub fn offs(&mut self) -> OFFS_W {
        OFFS_W { w: self }
    }
    #[doc = "Bits 12:14 - Selects one of the eight external trigger sources for DAC1"]
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W {
        TRIGSEL_W { w: self }
    }
    #[doc = "Bit 16 - Software Trigger"]
    #[inline(always)]
    pub fn swtrig(&mut self) -> SWTRIG_W {
        SWTRIG_W { w: self }
    }
    #[doc = "Bits 17:18 - Select the trigger source for channel 1"]
    #[inline(always)]
    pub fn trigmod(&mut self) -> TRIGMOD_W {
        TRIGMOD_W { w: self }
    }
    #[doc = "Bits 19:23 - DAC1 analog configuration/calibration parameters"]
    #[inline(always)]
    pub fn anacfg(&mut self) -> ANACFG_W {
        ANACFG_W { w: self }
    }
    #[doc = "Bit 24 - Enable analog DAC for channel 1"]
    #[inline(always)]
    pub fn anaen(&mut self) -> ANAEN_W {
        ANAEN_W { w: self }
    }
    #[doc = "Bits 28:31 - Higher 4 band-gap configuration/calibration parameters"]
    #[inline(always)]
    pub fn refcfgh(&mut self) -> REFCFGH_W {
        REFCFGH_W { w: self }
    }
}
