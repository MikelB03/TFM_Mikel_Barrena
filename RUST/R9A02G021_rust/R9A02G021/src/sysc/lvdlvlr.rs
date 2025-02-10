#[doc = "Register `LVDLVLR` reader"]
pub type R = crate::R<LvdlvlrSpec>;
#[doc = "Register `LVDLVLR` writer"]
pub type W = crate::W<LvdlvlrSpec>;
#[doc = "Voltage Detection 1 Level Select (Standard voltage during fall in voltage)\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lvd1lvl {
    #[doc = "0: Vdet1_0"]
    _0x00 = 0,
    #[doc = "1: Vdet1_1"]
    _0x01 = 1,
    #[doc = "2: Vdet1_2"]
    _0x02 = 2,
    #[doc = "3: Vdet1_3"]
    _0x03 = 3,
    #[doc = "4: Vdet1_4"]
    _0x04 = 4,
    #[doc = "5: Vdet1_5"]
    _0x05 = 5,
    #[doc = "6: Vdet1_6"]
    _0x06 = 6,
    #[doc = "7: Vdet1_7"]
    _0x07 = 7,
    #[doc = "8: Vdet1_8"]
    _0x08 = 8,
    #[doc = "9: Vdet1_9"]
    _0x09 = 9,
    #[doc = "10: Vdet1_A"]
    _0x0a = 10,
    #[doc = "11: Vdet1_B"]
    _0x0b = 11,
    #[doc = "12: Vdet1_C"]
    _0x0c = 12,
    #[doc = "13: Vdet1_D"]
    _0x0d = 13,
    #[doc = "14: Vdet1_E"]
    _0x0e = 14,
    #[doc = "15: Vdet1_F"]
    _0x0f = 15,
    #[doc = "16: Setting prohibited"]
    Others = 16,
}
impl From<Lvd1lvl> for u8 {
    #[inline(always)]
    fn from(variant: Lvd1lvl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lvd1lvl {
    type Ux = u8;
}
impl crate::IsEnum for Lvd1lvl {}
#[doc = "Field `LVD1LVL` reader - Voltage Detection 1 Level Select (Standard voltage during fall in voltage)"]
pub type Lvd1lvlR = crate::FieldReader<Lvd1lvl>;
impl Lvd1lvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd1lvl {
        match self.bits {
            0 => Lvd1lvl::_0x00,
            1 => Lvd1lvl::_0x01,
            2 => Lvd1lvl::_0x02,
            3 => Lvd1lvl::_0x03,
            4 => Lvd1lvl::_0x04,
            5 => Lvd1lvl::_0x05,
            6 => Lvd1lvl::_0x06,
            7 => Lvd1lvl::_0x07,
            8 => Lvd1lvl::_0x08,
            9 => Lvd1lvl::_0x09,
            10 => Lvd1lvl::_0x0a,
            11 => Lvd1lvl::_0x0b,
            12 => Lvd1lvl::_0x0c,
            13 => Lvd1lvl::_0x0d,
            14 => Lvd1lvl::_0x0e,
            15 => Lvd1lvl::_0x0f,
            _ => Lvd1lvl::Others,
        }
    }
    #[doc = "Vdet1_0"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == Lvd1lvl::_0x00
    }
    #[doc = "Vdet1_1"]
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == Lvd1lvl::_0x01
    }
    #[doc = "Vdet1_2"]
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == Lvd1lvl::_0x02
    }
    #[doc = "Vdet1_3"]
    #[inline(always)]
    pub fn is_0x03(&self) -> bool {
        *self == Lvd1lvl::_0x03
    }
    #[doc = "Vdet1_4"]
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == Lvd1lvl::_0x04
    }
    #[doc = "Vdet1_5"]
    #[inline(always)]
    pub fn is_0x05(&self) -> bool {
        *self == Lvd1lvl::_0x05
    }
    #[doc = "Vdet1_6"]
    #[inline(always)]
    pub fn is_0x06(&self) -> bool {
        *self == Lvd1lvl::_0x06
    }
    #[doc = "Vdet1_7"]
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == Lvd1lvl::_0x07
    }
    #[doc = "Vdet1_8"]
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == Lvd1lvl::_0x08
    }
    #[doc = "Vdet1_9"]
    #[inline(always)]
    pub fn is_0x09(&self) -> bool {
        *self == Lvd1lvl::_0x09
    }
    #[doc = "Vdet1_A"]
    #[inline(always)]
    pub fn is_0x0a(&self) -> bool {
        *self == Lvd1lvl::_0x0a
    }
    #[doc = "Vdet1_B"]
    #[inline(always)]
    pub fn is_0x0b(&self) -> bool {
        *self == Lvd1lvl::_0x0b
    }
    #[doc = "Vdet1_C"]
    #[inline(always)]
    pub fn is_0x0c(&self) -> bool {
        *self == Lvd1lvl::_0x0c
    }
    #[doc = "Vdet1_D"]
    #[inline(always)]
    pub fn is_0x0d(&self) -> bool {
        *self == Lvd1lvl::_0x0d
    }
    #[doc = "Vdet1_E"]
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == Lvd1lvl::_0x0e
    }
    #[doc = "Vdet1_F"]
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == Lvd1lvl::_0x0f
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Lvd1lvl::Others)
    }
}
#[doc = "Field `LVD1LVL` writer - Voltage Detection 1 Level Select (Standard voltage during fall in voltage)"]
pub type Lvd1lvlW<'a, REG> = crate::FieldWriter<'a, REG, 5, Lvd1lvl, crate::Safe>;
impl<'a, REG> Lvd1lvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Vdet1_0"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x00)
    }
    #[doc = "Vdet1_1"]
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x01)
    }
    #[doc = "Vdet1_2"]
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x02)
    }
    #[doc = "Vdet1_3"]
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x03)
    }
    #[doc = "Vdet1_4"]
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x04)
    }
    #[doc = "Vdet1_5"]
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x05)
    }
    #[doc = "Vdet1_6"]
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x06)
    }
    #[doc = "Vdet1_7"]
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x07)
    }
    #[doc = "Vdet1_8"]
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x08)
    }
    #[doc = "Vdet1_9"]
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x09)
    }
    #[doc = "Vdet1_A"]
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x0a)
    }
    #[doc = "Vdet1_B"]
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x0b)
    }
    #[doc = "Vdet1_C"]
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x0c)
    }
    #[doc = "Vdet1_D"]
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x0d)
    }
    #[doc = "Vdet1_E"]
    #[inline(always)]
    pub fn _0x0e(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x0e)
    }
    #[doc = "Vdet1_F"]
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::_0x0f)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1lvl::Others)
    }
}
#[doc = "Voltage Detection 2 Level Select (Standard voltage during fall in voltage)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lvd2lvl {
    #[doc = "0: Vdet2_0"]
    _000 = 0,
    #[doc = "1: Vdet2_1"]
    _001 = 1,
    #[doc = "2: Vdet2_2"]
    _010 = 2,
    #[doc = "3: Vdet2_3"]
    _011 = 3,
    #[doc = "4: Setting prohibited"]
    Others = 4,
}
impl From<Lvd2lvl> for u8 {
    #[inline(always)]
    fn from(variant: Lvd2lvl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lvd2lvl {
    type Ux = u8;
}
impl crate::IsEnum for Lvd2lvl {}
#[doc = "Field `LVD2LVL` reader - Voltage Detection 2 Level Select (Standard voltage during fall in voltage)"]
pub type Lvd2lvlR = crate::FieldReader<Lvd2lvl>;
impl Lvd2lvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd2lvl {
        match self.bits {
            0 => Lvd2lvl::_000,
            1 => Lvd2lvl::_001,
            2 => Lvd2lvl::_010,
            3 => Lvd2lvl::_011,
            _ => Lvd2lvl::Others,
        }
    }
    #[doc = "Vdet2_0"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Lvd2lvl::_000
    }
    #[doc = "Vdet2_1"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Lvd2lvl::_001
    }
    #[doc = "Vdet2_2"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Lvd2lvl::_010
    }
    #[doc = "Vdet2_3"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Lvd2lvl::_011
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Lvd2lvl::Others)
    }
}
#[doc = "Field `LVD2LVL` writer - Voltage Detection 2 Level Select (Standard voltage during fall in voltage)"]
pub type Lvd2lvlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lvd2lvl, crate::Safe>;
impl<'a, REG> Lvd2lvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Vdet2_0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2lvl::_000)
    }
    #[doc = "Vdet2_1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2lvl::_001)
    }
    #[doc = "Vdet2_2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2lvl::_010)
    }
    #[doc = "Vdet2_3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2lvl::_011)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2lvl::Others)
    }
}
impl R {
    #[doc = "Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during fall in voltage)"]
    #[inline(always)]
    pub fn lvd1lvl(&self) -> Lvd1lvlR {
        Lvd1lvlR::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:7 - Voltage Detection 2 Level Select (Standard voltage during fall in voltage)"]
    #[inline(always)]
    pub fn lvd2lvl(&self) -> Lvd2lvlR {
        Lvd2lvlR::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:4 - Voltage Detection 1 Level Select (Standard voltage during fall in voltage)"]
    #[inline(always)]
    pub fn lvd1lvl(&mut self) -> Lvd1lvlW<LvdlvlrSpec> {
        Lvd1lvlW::new(self, 0)
    }
    #[doc = "Bits 5:7 - Voltage Detection 2 Level Select (Standard voltage during fall in voltage)"]
    #[inline(always)]
    pub fn lvd2lvl(&mut self) -> Lvd2lvlW<LvdlvlrSpec> {
        Lvd2lvlW::new(self, 5)
    }
}
#[doc = "Voltage Detection Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lvdlvlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdlvlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvdlvlrSpec;
impl crate::RegisterSpec for LvdlvlrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvdlvlr::R`](R) reader structure"]
impl crate::Readable for LvdlvlrSpec {}
#[doc = "`write(|w| ..)` method takes [`lvdlvlr::W`](W) writer structure"]
impl crate::Writable for LvdlvlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LVDLVLR to value 0x07"]
impl crate::Resettable for LvdlvlrSpec {
    const RESET_VALUE: u8 = 0x07;
}
