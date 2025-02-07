#[doc = "Register `PSMCR` reader"]
pub type R = crate::R<PsmcrSpec>;
#[doc = "Register `PSMCR` writer"]
pub type W = crate::W<PsmcrSpec>;
#[doc = "Power Save Memory Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Psmc {
    #[doc = "0: All SRAMs are on in Software Standby mode"]
    _00 = 0,
    #[doc = "1: 8 KB SRAM (0x2000_0000 to 0x2000_0FFF and 0x2000_4000 to 0x2000_4FFF) is on, in Software Standby mode"]
    _01 = 1,
    #[doc = "2: Setting prohibited"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<Psmc> for u8 {
    #[inline(always)]
    fn from(variant: Psmc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Psmc {
    type Ux = u8;
}
impl crate::IsEnum for Psmc {}
#[doc = "Field `PSMC` reader - Power Save Memory Control"]
pub type PsmcR = crate::FieldReader<Psmc>;
impl PsmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psmc {
        match self.bits {
            0 => Psmc::_00,
            1 => Psmc::_01,
            2 => Psmc::_10,
            3 => Psmc::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "All SRAMs are on in Software Standby mode"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Psmc::_00
    }
    #[doc = "8 KB SRAM (0x2000_0000 to 0x2000_0FFF and 0x2000_4000 to 0x2000_4FFF) is on, in Software Standby mode"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Psmc::_01
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Psmc::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Psmc::_11
    }
}
#[doc = "Field `PSMC` writer - Power Save Memory Control"]
pub type PsmcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Psmc, crate::Safe>;
impl<'a, REG> PsmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All SRAMs are on in Software Standby mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Psmc::_00)
    }
    #[doc = "8 KB SRAM (0x2000_0000 to 0x2000_0FFF and 0x2000_4000 to 0x2000_4FFF) is on, in Software Standby mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Psmc::_01)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Psmc::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Psmc::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Power Save Memory Control"]
    #[inline(always)]
    pub fn psmc(&self) -> PsmcR {
        PsmcR::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power Save Memory Control"]
    #[inline(always)]
    pub fn psmc(&mut self) -> PsmcW<PsmcrSpec> {
        PsmcW::new(self, 0)
    }
}
#[doc = "Power Save Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsmcrSpec;
impl crate::RegisterSpec for PsmcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`psmcr::R`](R) reader structure"]
impl crate::Readable for PsmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`psmcr::W`](W) writer structure"]
impl crate::Writable for PsmcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PSMCR to value 0"]
impl crate::Resettable for PsmcrSpec {
    const RESET_VALUE: u8 = 0;
}
