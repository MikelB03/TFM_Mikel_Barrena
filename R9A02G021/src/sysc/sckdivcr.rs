#[doc = "Register `SCKDIVCR` reader"]
pub type R = crate::R<SckdivcrSpec>;
#[doc = "Register `SCKDIVCR` writer"]
pub type W = crate::W<SckdivcrSpec>;
#[doc = "Peripheral Module Clock B (PCLKB) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pckb {
    #[doc = "0: x 1/1"]
    _000 = 0,
    #[doc = "1: x 1/2"]
    _001 = 1,
    #[doc = "2: x 1/4"]
    _010 = 2,
    #[doc = "3: x 1/8"]
    _011 = 3,
    #[doc = "4: x 1/16"]
    _100 = 4,
    #[doc = "5: x 1/32"]
    _101 = 5,
    #[doc = "6: x 1/64"]
    _110 = 6,
    #[doc = "7: Settings prohibited"]
    Others = 7,
}
impl From<Pckb> for u8 {
    #[inline(always)]
    fn from(variant: Pckb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pckb {
    type Ux = u8;
}
impl crate::IsEnum for Pckb {}
#[doc = "Field `PCKB` reader - Peripheral Module Clock B (PCLKB) Select"]
pub type PckbR = crate::FieldReader<Pckb>;
impl PckbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pckb {
        match self.bits {
            0 => Pckb::_000,
            1 => Pckb::_001,
            2 => Pckb::_010,
            3 => Pckb::_011,
            4 => Pckb::_100,
            5 => Pckb::_101,
            6 => Pckb::_110,
            7 => Pckb::Others,
            _ => unreachable!(),
        }
    }
    #[doc = "x 1/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Pckb::_000
    }
    #[doc = "x 1/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Pckb::_001
    }
    #[doc = "x 1/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Pckb::_010
    }
    #[doc = "x 1/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Pckb::_011
    }
    #[doc = "x 1/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Pckb::_100
    }
    #[doc = "x 1/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Pckb::_101
    }
    #[doc = "x 1/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Pckb::_110
    }
    #[doc = "Settings prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Pckb::Others
    }
}
#[doc = "Field `PCKB` writer - Peripheral Module Clock B (PCLKB) Select"]
pub type PckbW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pckb, crate::Safe>;
impl<'a, REG> PckbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "x 1/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Pckb::_000)
    }
    #[doc = "x 1/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Pckb::_001)
    }
    #[doc = "x 1/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Pckb::_010)
    }
    #[doc = "x 1/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Pckb::_011)
    }
    #[doc = "x 1/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Pckb::_100)
    }
    #[doc = "x 1/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Pckb::_101)
    }
    #[doc = "x 1/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Pckb::_110)
    }
    #[doc = "Settings prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Pckb::Others)
    }
}
#[doc = "System Clock (ICLK) Select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ick {
    #[doc = "0: x 1/1"]
    _000 = 0,
    #[doc = "1: x 1/2"]
    _001 = 1,
    #[doc = "2: x 1/4"]
    _010 = 2,
    #[doc = "3: x 1/8"]
    _011 = 3,
    #[doc = "4: x 1/16"]
    _100 = 4,
    #[doc = "5: x 1/32"]
    _101 = 5,
    #[doc = "6: x 1/64"]
    _110 = 6,
    #[doc = "7: Settings prohibited"]
    Others = 7,
}
impl From<Ick> for u8 {
    #[inline(always)]
    fn from(variant: Ick) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ick {
    type Ux = u8;
}
impl crate::IsEnum for Ick {}
#[doc = "Field `ICK` reader - System Clock (ICLK) Select"]
pub type IckR = crate::FieldReader<Ick>;
impl IckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ick {
        match self.bits {
            0 => Ick::_000,
            1 => Ick::_001,
            2 => Ick::_010,
            3 => Ick::_011,
            4 => Ick::_100,
            5 => Ick::_101,
            6 => Ick::_110,
            7 => Ick::Others,
            _ => unreachable!(),
        }
    }
    #[doc = "x 1/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ick::_000
    }
    #[doc = "x 1/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ick::_001
    }
    #[doc = "x 1/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Ick::_010
    }
    #[doc = "x 1/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ick::_011
    }
    #[doc = "x 1/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Ick::_100
    }
    #[doc = "x 1/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Ick::_101
    }
    #[doc = "x 1/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Ick::_110
    }
    #[doc = "Settings prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Ick::Others
    }
}
#[doc = "Field `ICK` writer - System Clock (ICLK) Select"]
pub type IckW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ick, crate::Safe>;
impl<'a, REG> IckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "x 1/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Ick::_000)
    }
    #[doc = "x 1/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Ick::_001)
    }
    #[doc = "x 1/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Ick::_010)
    }
    #[doc = "x 1/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Ick::_011)
    }
    #[doc = "x 1/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Ick::_100)
    }
    #[doc = "x 1/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Ick::_101)
    }
    #[doc = "x 1/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Ick::_110)
    }
    #[doc = "Settings prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Ick::Others)
    }
}
impl R {
    #[doc = "Bits 8:10 - Peripheral Module Clock B (PCLKB) Select"]
    #[inline(always)]
    pub fn pckb(&self) -> PckbR {
        PckbR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 24:26 - System Clock (ICLK) Select"]
    #[inline(always)]
    pub fn ick(&self) -> IckR {
        IckR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Peripheral Module Clock B (PCLKB) Select"]
    #[inline(always)]
    pub fn pckb(&mut self) -> PckbW<SckdivcrSpec> {
        PckbW::new(self, 8)
    }
    #[doc = "Bits 24:26 - System Clock (ICLK) Select"]
    #[inline(always)]
    pub fn ick(&mut self) -> IckW<SckdivcrSpec> {
        IckW::new(self, 24)
    }
}
#[doc = "System Clock Division Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sckdivcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckdivcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SckdivcrSpec;
impl crate::RegisterSpec for SckdivcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sckdivcr::R`](R) reader structure"]
impl crate::Readable for SckdivcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sckdivcr::W`](W) writer structure"]
impl crate::Writable for SckdivcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCKDIVCR to value 0x0400_0404"]
impl crate::Resettable for SckdivcrSpec {
    const RESET_VALUE: u32 = 0x0400_0404;
}
