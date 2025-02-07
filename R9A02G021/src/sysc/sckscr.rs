#[doc = "Register `SCKSCR` reader"]
pub type R = crate::R<SckscrSpec>;
#[doc = "Register `SCKSCR` writer"]
pub type W = crate::W<SckscrSpec>;
#[doc = "Clock Source Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cksel {
    #[doc = "0: HOCO"]
    _000 = 0,
    #[doc = "1: MOCO"]
    _001 = 1,
    #[doc = "2: LOCO"]
    _010 = 2,
    #[doc = "3: External clock input (EXTAL)"]
    _011 = 3,
    #[doc = "4: Sub-clock oscillator (SOSC)"]
    _100 = 4,
    #[doc = "5: Setting prohibited"]
    _101 = 5,
    #[doc = "6: Setting prohibited"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    _111 = 7,
}
impl From<Cksel> for u8 {
    #[inline(always)]
    fn from(variant: Cksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cksel {
    type Ux = u8;
}
impl crate::IsEnum for Cksel {}
#[doc = "Field `CKSEL` reader - Clock Source Select"]
pub type CkselR = crate::FieldReader<Cksel>;
impl CkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cksel {
        match self.bits {
            0 => Cksel::_000,
            1 => Cksel::_001,
            2 => Cksel::_010,
            3 => Cksel::_011,
            4 => Cksel::_100,
            5 => Cksel::_101,
            6 => Cksel::_110,
            7 => Cksel::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Cksel::_000
    }
    #[doc = "MOCO"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Cksel::_001
    }
    #[doc = "LOCO"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Cksel::_010
    }
    #[doc = "External clock input (EXTAL)"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Cksel::_011
    }
    #[doc = "Sub-clock oscillator (SOSC)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Cksel::_100
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Cksel::_101
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Cksel::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Cksel::_111
    }
}
#[doc = "Field `CKSEL` writer - Clock Source Select"]
pub type CkselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cksel, crate::Safe>;
impl<'a, REG> CkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::_000)
    }
    #[doc = "MOCO"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::_001)
    }
    #[doc = "LOCO"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::_010)
    }
    #[doc = "External clock input (EXTAL)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::_011)
    }
    #[doc = "Sub-clock oscillator (SOSC)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::_100)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::_101)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Source Select"]
    #[inline(always)]
    pub fn cksel(&self) -> CkselR {
        CkselR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Source Select"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CkselW<SckscrSpec> {
        CkselW::new(self, 0)
    }
}
#[doc = "System Clock Source Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sckscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SckscrSpec;
impl crate::RegisterSpec for SckscrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sckscr::R`](R) reader structure"]
impl crate::Readable for SckscrSpec {}
#[doc = "`write(|w| ..)` method takes [`sckscr::W`](W) writer structure"]
impl crate::Writable for SckscrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SCKSCR to value 0x01"]
impl crate::Resettable for SckscrSpec {
    const RESET_VALUE: u8 = 0x01;
}
