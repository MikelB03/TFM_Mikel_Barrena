#[doc = "Register `FCR` reader"]
pub type R = crate::R<FcrSpec>;
#[doc = "Register `FCR` writer"]
pub type W = crate::W<FcrSpec>;
#[doc = "Software Command Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd {
    #[doc = "1: Program"]
    _0x1 = 1,
    #[doc = "3: Blank check (code flash)"]
    _0x3 = 3,
    #[doc = "4: Block erase"]
    _0x4 = 4,
    #[doc = "6: Chip erase"]
    _0x6 = 6,
    #[doc = "11: Blank check (data flash)"]
    _0xB = 11,
    #[doc = "0: Setting prohibited."]
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
            1 => Cmd::_0x1,
            3 => Cmd::_0x3,
            4 => Cmd::_0x4,
            6 => Cmd::_0x6,
            11 => Cmd::_0xB,
            _ => Cmd::Others,
        }
    }
    #[doc = "Program"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Cmd::_0x1
    }
    #[doc = "Blank check (code flash)"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == Cmd::_0x3
    }
    #[doc = "Block erase"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Cmd::_0x4
    }
    #[doc = "Chip erase"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Cmd::_0x6
    }
    #[doc = "Blank check (data flash)"]
    #[inline(always)]
    pub fn is_0x_b(&self) -> bool {
        *self == Cmd::_0xB
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Cmd::Others)
    }
}
#[doc = "Field `CMD` writer - Software Command Setting"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cmd, crate::Safe>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Program"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::_0x1)
    }
    #[doc = "Blank check (code flash)"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::_0x3)
    }
    #[doc = "Block erase"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::_0x4)
    }
    #[doc = "Chip erase"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::_0x6)
    }
    #[doc = "Blank check (data flash)"]
    #[inline(always)]
    pub fn _0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::_0xB)
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Others)
    }
}
#[doc = "Field `STOP` reader - Forced Processing Stop"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Forced Processing Stop"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bits 0:3 - Software Command Setting"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 6 - Forced Processing Stop"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Processing Start"]
    #[inline(always)]
    pub fn opst(&self) -> OpstR {
        OpstR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Software Command Setting"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<FcrSpec> {
        CmdW::new(self, 0)
    }
    #[doc = "Bit 6 - Forced Processing Stop"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<FcrSpec> {
        StopW::new(self, 6)
    }
    #[doc = "Bit 7 - Processing Start"]
    #[inline(always)]
    pub fn opst(&mut self) -> OpstW<FcrSpec> {
        OpstW::new(self, 7)
    }
}
#[doc = "Flash Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcrSpec;
impl crate::RegisterSpec for FcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fcr::R`](R) reader structure"]
impl crate::Readable for FcrSpec {}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FcrSpec {
    const RESET_VALUE: u8 = 0;
}
