#[doc = "Register `FSTATR00` reader"]
pub type R = crate::R<Fstatr00Spec>;
#[doc = "Erase Error Flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ererr0 {
    #[doc = "0: Erasure terminates normally"]
    _0 = 0,
    #[doc = "1: An error occurs during erasure"]
    _1 = 1,
}
impl From<Ererr0> for bool {
    #[inline(always)]
    fn from(variant: Ererr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERERR0` reader - Erase Error Flag 0"]
pub type Ererr0R = crate::BitReader<Ererr0>;
impl Ererr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ererr0 {
        match self.bits {
            false => Ererr0::_0,
            true => Ererr0::_1,
        }
    }
    #[doc = "Erasure terminates normally"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ererr0::_0
    }
    #[doc = "An error occurs during erasure"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ererr0::_1
    }
}
#[doc = "Program Error Flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prgerr0 {
    #[doc = "0: Programming terminates normally"]
    _0 = 0,
    #[doc = "1: An error occurs during programming"]
    _1 = 1,
}
impl From<Prgerr0> for bool {
    #[inline(always)]
    fn from(variant: Prgerr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGERR0` reader - Program Error Flag 0"]
pub type Prgerr0R = crate::BitReader<Prgerr0>;
impl Prgerr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prgerr0 {
        match self.bits {
            false => Prgerr0::_0,
            true => Prgerr0::_1,
        }
    }
    #[doc = "Programming terminates normally"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Prgerr0::_0
    }
    #[doc = "An error occurs during programming"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Prgerr0::_1
    }
}
#[doc = "Blank Check Error Flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bcerr0 {
    #[doc = "0: Blank checking terminates normally"]
    _0 = 0,
    #[doc = "1: An error occurs during blank checking"]
    _1 = 1,
}
impl From<Bcerr0> for bool {
    #[inline(always)]
    fn from(variant: Bcerr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCERR0` reader - Blank Check Error Flag 0"]
pub type Bcerr0R = crate::BitReader<Bcerr0>;
impl Bcerr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bcerr0 {
        match self.bits {
            false => Bcerr0::_0,
            true => Bcerr0::_1,
        }
    }
    #[doc = "Blank checking terminates normally"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bcerr0::_0
    }
    #[doc = "An error occurs during blank checking"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bcerr0::_1
    }
}
#[doc = "Illegal Command Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilglerr {
    #[doc = "0: No illegal software command or illegal access is detected"]
    _0 = 0,
    #[doc = "1: An illegal command or illegal access is detected"]
    _1 = 1,
}
impl From<Ilglerr> for bool {
    #[inline(always)]
    fn from(variant: Ilglerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILGLERR` reader - Illegal Command Error Flag"]
pub type IlglerrR = crate::BitReader<Ilglerr>;
impl IlglerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ilglerr {
        match self.bits {
            false => Ilglerr::_0,
            true => Ilglerr::_1,
        }
    }
    #[doc = "No illegal software command or illegal access is detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ilglerr::_0
    }
    #[doc = "An illegal command or illegal access is detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ilglerr::_1
    }
}
#[doc = "Extra Area Illegal Command Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eilglerr {
    #[doc = "0: No illegal command or illegal access to the extra area is detected"]
    _0 = 0,
    #[doc = "1: An illegal command or illegal access to the extra area is detected"]
    _1 = 1,
}
impl From<Eilglerr> for bool {
    #[inline(always)]
    fn from(variant: Eilglerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EILGLERR` reader - Extra Area Illegal Command Error Flag"]
pub type EilglerrR = crate::BitReader<Eilglerr>;
impl EilglerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eilglerr {
        match self.bits {
            false => Eilglerr::_0,
            true => Eilglerr::_1,
        }
    }
    #[doc = "No illegal command or illegal access to the extra area is detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eilglerr::_0
    }
    #[doc = "An illegal command or illegal access to the extra area is detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eilglerr::_1
    }
}
impl R {
    #[doc = "Bit 0 - Erase Error Flag 0"]
    #[inline(always)]
    pub fn ererr0(&self) -> Ererr0R {
        Ererr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Program Error Flag 0"]
    #[inline(always)]
    pub fn prgerr0(&self) -> Prgerr0R {
        Prgerr0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Blank Check Error Flag 0"]
    #[inline(always)]
    pub fn bcerr0(&self) -> Bcerr0R {
        Bcerr0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Illegal Command Error Flag"]
    #[inline(always)]
    pub fn ilglerr(&self) -> IlglerrR {
        IlglerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Extra Area Illegal Command Error Flag"]
    #[inline(always)]
    pub fn eilglerr(&self) -> EilglerrR {
        EilglerrR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Flash Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fstatr00::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fstatr00Spec;
impl crate::RegisterSpec for Fstatr00Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fstatr00::R`](R) reader structure"]
impl crate::Readable for Fstatr00Spec {}
#[doc = "`reset()` method sets FSTATR00 to value 0"]
impl crate::Resettable for Fstatr00Spec {
    const RESET_VALUE: u16 = 0;
}
