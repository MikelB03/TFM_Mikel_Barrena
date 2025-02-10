#[doc = "Register `WDTCR` reader"]
pub type R = crate::R<WdtcrSpec>;
#[doc = "Register `WDTCR` writer"]
pub type W = crate::W<WdtcrSpec>;
#[doc = "Timeout Period Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tops {
    #[doc = "0: 1024 cycles (0x03FF)"]
    _00 = 0,
    #[doc = "1: 4096 cycles (0x0FFF)"]
    _01 = 1,
    #[doc = "2: 8192 cycles (0x1FFF)"]
    _10 = 2,
    #[doc = "3: 16384 cycles (0x3FFF)"]
    _11 = 3,
}
impl From<Tops> for u8 {
    #[inline(always)]
    fn from(variant: Tops) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tops {
    type Ux = u8;
}
impl crate::IsEnum for Tops {}
#[doc = "Field `TOPS` reader - Timeout Period Select"]
pub type TopsR = crate::FieldReader<Tops>;
impl TopsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tops {
        match self.bits {
            0 => Tops::_00,
            1 => Tops::_01,
            2 => Tops::_10,
            3 => Tops::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "1024 cycles (0x03FF)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Tops::_00
    }
    #[doc = "4096 cycles (0x0FFF)"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Tops::_01
    }
    #[doc = "8192 cycles (0x1FFF)"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Tops::_10
    }
    #[doc = "16384 cycles (0x3FFF)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Tops::_11
    }
}
#[doc = "Field `TOPS` writer - Timeout Period Select"]
pub type TopsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tops, crate::Safe>;
impl<'a, REG> TopsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1024 cycles (0x03FF)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Tops::_00)
    }
    #[doc = "4096 cycles (0x0FFF)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Tops::_01)
    }
    #[doc = "8192 cycles (0x1FFF)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Tops::_10)
    }
    #[doc = "16384 cycles (0x3FFF)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Tops::_11)
    }
}
#[doc = "Clock Division Ratio Select\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cks {
    #[doc = "1: PCLKB/4"]
    _0x1 = 1,
    #[doc = "4: PCLKB/64"]
    _0x4 = 4,
    #[doc = "15: PCLKB/128"]
    _0xF = 15,
    #[doc = "6: PCLKB/512"]
    _0x6 = 6,
    #[doc = "7: PCLKB/2048"]
    _0x7 = 7,
    #[doc = "8: PCLKB/8192"]
    _0x8 = 8,
    #[doc = "0: Setting prohibited"]
    Others = 0,
}
impl From<Cks> for u8 {
    #[inline(always)]
    fn from(variant: Cks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cks {
    type Ux = u8;
}
impl crate::IsEnum for Cks {}
#[doc = "Field `CKS` reader - Clock Division Ratio Select"]
pub type CksR = crate::FieldReader<Cks>;
impl CksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cks {
        match self.bits {
            1 => Cks::_0x1,
            4 => Cks::_0x4,
            15 => Cks::_0xF,
            6 => Cks::_0x6,
            7 => Cks::_0x7,
            8 => Cks::_0x8,
            _ => Cks::Others,
        }
    }
    #[doc = "PCLKB/4"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Cks::_0x1
    }
    #[doc = "PCLKB/64"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Cks::_0x4
    }
    #[doc = "PCLKB/128"]
    #[inline(always)]
    pub fn is_0x_f(&self) -> bool {
        *self == Cks::_0xF
    }
    #[doc = "PCLKB/512"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Cks::_0x6
    }
    #[doc = "PCLKB/2048"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == Cks::_0x7
    }
    #[doc = "PCLKB/8192"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Cks::_0x8
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Cks::Others)
    }
}
#[doc = "Field `CKS` writer - Clock Division Ratio Select"]
pub type CksW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cks, crate::Safe>;
impl<'a, REG> CksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLKB/4"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0x1)
    }
    #[doc = "PCLKB/64"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0x4)
    }
    #[doc = "PCLKB/128"]
    #[inline(always)]
    pub fn _0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0xF)
    }
    #[doc = "PCLKB/512"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0x6)
    }
    #[doc = "PCLKB/2048"]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0x7)
    }
    #[doc = "PCLKB/8192"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0x8)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::Others)
    }
}
#[doc = "Window End Position Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rpes {
    #[doc = "0: 75%"]
    _00 = 0,
    #[doc = "1: 50%"]
    _01 = 1,
    #[doc = "2: 25%"]
    _10 = 2,
    #[doc = "3: 0% (do not specify window end position)."]
    _11 = 3,
}
impl From<Rpes> for u8 {
    #[inline(always)]
    fn from(variant: Rpes) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rpes {
    type Ux = u8;
}
impl crate::IsEnum for Rpes {}
#[doc = "Field `RPES` reader - Window End Position Select"]
pub type RpesR = crate::FieldReader<Rpes>;
impl RpesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpes {
        match self.bits {
            0 => Rpes::_00,
            1 => Rpes::_01,
            2 => Rpes::_10,
            3 => Rpes::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "75%"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Rpes::_00
    }
    #[doc = "50%"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Rpes::_01
    }
    #[doc = "25%"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Rpes::_10
    }
    #[doc = "0% (do not specify window end position)."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Rpes::_11
    }
}
#[doc = "Field `RPES` writer - Window End Position Select"]
pub type RpesW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rpes, crate::Safe>;
impl<'a, REG> RpesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "75%"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Rpes::_00)
    }
    #[doc = "50%"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Rpes::_01)
    }
    #[doc = "25%"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Rpes::_10)
    }
    #[doc = "0% (do not specify window end position)."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Rpes::_11)
    }
}
#[doc = "Window Start Position Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rpss {
    #[doc = "0: 25%"]
    _00 = 0,
    #[doc = "1: 50%"]
    _01 = 1,
    #[doc = "2: 75%"]
    _10 = 2,
    #[doc = "3: 100% (do not specify window start position)."]
    _11 = 3,
}
impl From<Rpss> for u8 {
    #[inline(always)]
    fn from(variant: Rpss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rpss {
    type Ux = u8;
}
impl crate::IsEnum for Rpss {}
#[doc = "Field `RPSS` reader - Window Start Position Select"]
pub type RpssR = crate::FieldReader<Rpss>;
impl RpssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpss {
        match self.bits {
            0 => Rpss::_00,
            1 => Rpss::_01,
            2 => Rpss::_10,
            3 => Rpss::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "25%"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Rpss::_00
    }
    #[doc = "50%"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Rpss::_01
    }
    #[doc = "75%"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Rpss::_10
    }
    #[doc = "100% (do not specify window start position)."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Rpss::_11
    }
}
#[doc = "Field `RPSS` writer - Window Start Position Select"]
pub type RpssW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rpss, crate::Safe>;
impl<'a, REG> RpssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "25%"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Rpss::_00)
    }
    #[doc = "50%"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Rpss::_01)
    }
    #[doc = "75%"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Rpss::_10)
    }
    #[doc = "100% (do not specify window start position)."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Rpss::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Timeout Period Select"]
    #[inline(always)]
    pub fn tops(&self) -> TopsR {
        TopsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - Clock Division Ratio Select"]
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Window End Position Select"]
    #[inline(always)]
    pub fn rpes(&self) -> RpesR {
        RpesR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Window Start Position Select"]
    #[inline(always)]
    pub fn rpss(&self) -> RpssR {
        RpssR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timeout Period Select"]
    #[inline(always)]
    pub fn tops(&mut self) -> TopsW<WdtcrSpec> {
        TopsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Clock Division Ratio Select"]
    #[inline(always)]
    pub fn cks(&mut self) -> CksW<WdtcrSpec> {
        CksW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Window End Position Select"]
    #[inline(always)]
    pub fn rpes(&mut self) -> RpesW<WdtcrSpec> {
        RpesW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Window Start Position Select"]
    #[inline(always)]
    pub fn rpss(&mut self) -> RpssW<WdtcrSpec> {
        RpssW::new(self, 12)
    }
}
#[doc = "WDT Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtcrSpec;
impl crate::RegisterSpec for WdtcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wdtcr::R`](R) reader structure"]
impl crate::Readable for WdtcrSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtcr::W`](W) writer structure"]
impl crate::Writable for WdtcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WDTCR to value 0x33f3"]
impl crate::Resettable for WdtcrSpec {
    const RESET_VALUE: u16 = 0x33f3;
}
