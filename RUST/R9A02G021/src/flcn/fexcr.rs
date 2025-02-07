#[doc = "Register `FEXCR` reader"]
pub type R = crate::R<FexcrSpec>;
#[doc = "Register `FEXCR` writer"]
pub type W = crate::W<FexcrSpec>;
#[doc = "Software Command Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd {
    #[doc = "1: Protection setting"]
    _001 = 1,
    #[doc = "2: Access window information program"]
    _010 = 2,
    #[doc = "3: User ID1 program"]
    _011 = 3,
    #[doc = "4: User ID2 program"]
    _100 = 4,
    #[doc = "5: User ID3 program"]
    _101 = 5,
    #[doc = "6: User ID4 program"]
    _110 = 6,
    #[doc = "0: Setting prohibited"]
    Others = 0,
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(variant: Cmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd {
    type Ux = u8;
}
impl crate::IsEnum for Cmd {}
#[doc = "Field `CMD` reader - Software Command Setting"]
pub type CmdR = crate::FieldReader<Cmd>;
impl CmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd {
        match self.bits {
            1 => Cmd::_001,
            2 => Cmd::_010,
            3 => Cmd::_011,
            4 => Cmd::_100,
            5 => Cmd::_101,
            6 => Cmd::_110,
            _ => Cmd::Others,
        }
    }
    #[doc = "Protection setting"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Cmd::_001
    }
    #[doc = "Access window information program"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Cmd::_010
    }
    #[doc = "User ID1 program"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Cmd::_011
    }
    #[doc = "User ID2 program"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Cmd::_100
    }
    #[doc = "User ID3 program"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Cmd::_101
    }
    #[doc = "User ID4 program"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Cmd::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Cmd::Others)
    }
}
#[doc = "Field `CMD` writer - Software Command Setting"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cmd, crate::Safe>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Protection setting"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::_001)
    }
    #[doc = "Access window information program"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::_010)
    }
    #[doc = "User ID1 program"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::_011)
    }
    #[doc = "User ID2 program"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::_100)
    }
    #[doc = "User ID3 program"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::_101)
    }
    #[doc = "User ID4 program"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Others)
    }
}
#[doc = "Processing Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opst {
    #[doc = "0: Processing stops"]
    _0 = 0,
    #[doc = "1: Processing starts"]
    _1 = 1,
}
impl From<Opst> for bool {
    #[inline(always)]
    fn from(variant: Opst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPST` reader - Processing Start"]
pub type OpstR = crate::BitReader<Opst>;
impl OpstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opst {
        match self.bits {
            false => Opst::_0,
            true => Opst::_1,
        }
    }
    #[doc = "Processing stops"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Opst::_0
    }
    #[doc = "Processing starts"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Opst::_1
    }
}
#[doc = "Field `OPST` writer - Processing Start"]
pub type OpstW<'a, REG> = crate::BitWriter<'a, REG, Opst>;
impl<'a, REG> OpstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Processing stops"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Opst::_0)
    }
    #[doc = "Processing starts"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Opst::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Software Command Setting"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(self.bits & 7)
    }
    #[doc = "Bit 7 - Processing Start"]
    #[inline(always)]
    pub fn opst(&self) -> OpstR {
        OpstR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Software Command Setting"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<FexcrSpec> {
        CmdW::new(self, 0)
    }
    #[doc = "Bit 7 - Processing Start"]
    #[inline(always)]
    pub fn opst(&mut self) -> OpstW<FexcrSpec> {
        OpstW::new(self, 7)
    }
}
#[doc = "Flash Extra Area Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fexcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fexcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FexcrSpec;
impl crate::RegisterSpec for FexcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fexcr::R`](R) reader structure"]
impl crate::Readable for FexcrSpec {}
#[doc = "`write(|w| ..)` method takes [`fexcr::W`](W) writer structure"]
impl crate::Writable for FexcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FEXCR to value 0"]
impl crate::Resettable for FexcrSpec {
    const RESET_VALUE: u8 = 0;
}
