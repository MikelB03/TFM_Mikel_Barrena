#[doc = "Register `CCDE` reader"]
pub type R = crate::R<CcdeSpec>;
#[doc = "Register `CCDE` writer"]
pub type W = crate::W<CcdeSpec>;
#[doc = "CCDE00 (P100) output control function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccde00 {
    #[doc = "0: Disable (digital I/O)"]
    _0 = 0,
    #[doc = "1: Enable (current control function)"]
    _1 = 1,
}
impl From<Ccde00> for bool {
    #[inline(always)]
    fn from(variant: Ccde00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCDE00` reader - CCDE00 (P100) output control function"]
pub type Ccde00R = crate::BitReader<Ccde00>;
impl Ccde00R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccde00 {
        match self.bits {
            false => Ccde00::_0,
            true => Ccde00::_1,
        }
    }
    #[doc = "Disable (digital I/O)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ccde00::_0
    }
    #[doc = "Enable (current control function)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ccde00::_1
    }
}
#[doc = "Field `CCDE00` writer - CCDE00 (P100) output control function"]
pub type Ccde00W<'a, REG> = crate::BitWriter<'a, REG, Ccde00>;
impl<'a, REG> Ccde00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (digital I/O)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccde00::_0)
    }
    #[doc = "Enable (current control function)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccde00::_1)
    }
}
#[doc = "CCDE01 (P302) output control function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccde01 {
    #[doc = "0: Disable (digital I/O)"]
    _0 = 0,
    #[doc = "1: Enable (current control function)"]
    _1 = 1,
}
impl From<Ccde01> for bool {
    #[inline(always)]
    fn from(variant: Ccde01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCDE01` reader - CCDE01 (P302) output control function"]
pub type Ccde01R = crate::BitReader<Ccde01>;
impl Ccde01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccde01 {
        match self.bits {
            false => Ccde01::_0,
            true => Ccde01::_1,
        }
    }
    #[doc = "Disable (digital I/O)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ccde01::_0
    }
    #[doc = "Enable (current control function)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ccde01::_1
    }
}
#[doc = "Field `CCDE01` writer - CCDE01 (P302) output control function"]
pub type Ccde01W<'a, REG> = crate::BitWriter<'a, REG, Ccde01>;
impl<'a, REG> Ccde01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (digital I/O)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccde01::_0)
    }
    #[doc = "Enable (current control function)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccde01::_1)
    }
}
#[doc = "CCDE02 (P303) output control function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccde02 {
    #[doc = "0: Disable (digital I/O)"]
    _0 = 0,
    #[doc = "1: Enable (current control function)"]
    _1 = 1,
}
impl From<Ccde02> for bool {
    #[inline(always)]
    fn from(variant: Ccde02) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCDE02` reader - CCDE02 (P303) output control function"]
pub type Ccde02R = crate::BitReader<Ccde02>;
impl Ccde02R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccde02 {
        match self.bits {
            false => Ccde02::_0,
            true => Ccde02::_1,
        }
    }
    #[doc = "Disable (digital I/O)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ccde02::_0
    }
    #[doc = "Enable (current control function)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ccde02::_1
    }
}
#[doc = "Field `CCDE02` writer - CCDE02 (P303) output control function"]
pub type Ccde02W<'a, REG> = crate::BitWriter<'a, REG, Ccde02>;
impl<'a, REG> Ccde02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (digital I/O)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccde02::_0)
    }
    #[doc = "Enable (current control function)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccde02::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CCDE00 (P100) output control function"]
    #[inline(always)]
    pub fn ccde00(&self) -> Ccde00R {
        Ccde00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCDE01 (P302) output control function"]
    #[inline(always)]
    pub fn ccde01(&self) -> Ccde01R {
        Ccde01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCDE02 (P303) output control function"]
    #[inline(always)]
    pub fn ccde02(&self) -> Ccde02R {
        Ccde02R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCDE00 (P100) output control function"]
    #[inline(always)]
    pub fn ccde00(&mut self) -> Ccde00W<CcdeSpec> {
        Ccde00W::new(self, 0)
    }
    #[doc = "Bit 1 - CCDE01 (P302) output control function"]
    #[inline(always)]
    pub fn ccde01(&mut self) -> Ccde01W<CcdeSpec> {
        Ccde01W::new(self, 1)
    }
    #[doc = "Bit 2 - CCDE02 (P303) output control function"]
    #[inline(always)]
    pub fn ccde02(&mut self) -> Ccde02W<CcdeSpec> {
        Ccde02W::new(self, 2)
    }
}
#[doc = "Output Current Control Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccde::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccde::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcdeSpec;
impl crate::RegisterSpec for CcdeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccde::R`](R) reader structure"]
impl crate::Readable for CcdeSpec {}
#[doc = "`write(|w| ..)` method takes [`ccde::W`](W) writer structure"]
impl crate::Writable for CcdeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CCDE to value 0"]
impl crate::Resettable for CcdeSpec {
    const RESET_VALUE: u8 = 0;
}
