#[doc = "Register `BUS1ERRSTAT` reader"]
pub type R = crate::R<Bus1errstatSpec>;
#[doc = "Error Access Status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accstat {
    #[doc = "0: Read access"]
    _0 = 0,
    #[doc = "1: Write access"]
    _1 = 1,
}
impl From<Accstat> for bool {
    #[inline(always)]
    fn from(variant: Accstat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCSTAT` reader - Error Access Status flag"]
pub type AccstatR = crate::BitReader<Accstat>;
impl AccstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Accstat {
        match self.bits {
            false => Accstat::_0,
            true => Accstat::_1,
        }
    }
    #[doc = "Read access"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Accstat::_0
    }
    #[doc = "Write access"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Accstat::_1
    }
}
#[doc = "Illicit Memory Access Error Status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilterrstat {
    #[doc = "0: No bus error occurred"]
    _0 = 0,
    #[doc = "1: Bus error occurred"]
    _1 = 1,
}
impl From<Ilterrstat> for bool {
    #[inline(always)]
    fn from(variant: Ilterrstat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILTERRSTAT` reader - Illicit Memory Access Error Status flag"]
pub type IlterrstatR = crate::BitReader<Ilterrstat>;
impl IlterrstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ilterrstat {
        match self.bits {
            false => Ilterrstat::_0,
            true => Ilterrstat::_1,
        }
    }
    #[doc = "No bus error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ilterrstat::_0
    }
    #[doc = "Bus error occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ilterrstat::_1
    }
}
#[doc = "Slave Bus Error Status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slerrstat {
    #[doc = "0: No bus error occurred"]
    _0 = 0,
    #[doc = "1: Bus error occurred"]
    _1 = 1,
}
impl From<Slerrstat> for bool {
    #[inline(always)]
    fn from(variant: Slerrstat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLERRSTAT` reader - Slave Bus Error Status flag"]
pub type SlerrstatR = crate::BitReader<Slerrstat>;
impl SlerrstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slerrstat {
        match self.bits {
            false => Slerrstat::_0,
            true => Slerrstat::_1,
        }
    }
    #[doc = "No bus error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Slerrstat::_0
    }
    #[doc = "Bus error occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Slerrstat::_1
    }
}
#[doc = "Illegal Address Access Error Status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Illerrstat {
    #[doc = "0: No bus error occurred"]
    _0 = 0,
    #[doc = "1: Bus error occurred"]
    _1 = 1,
}
impl From<Illerrstat> for bool {
    #[inline(always)]
    fn from(variant: Illerrstat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILLERRSTAT` reader - Illegal Address Access Error Status flag"]
pub type IllerrstatR = crate::BitReader<Illerrstat>;
impl IllerrstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Illerrstat {
        match self.bits {
            false => Illerrstat::_0,
            true => Illerrstat::_1,
        }
    }
    #[doc = "No bus error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Illerrstat::_0
    }
    #[doc = "Bus error occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Illerrstat::_1
    }
}
impl R {
    #[doc = "Bit 0 - Error Access Status flag"]
    #[inline(always)]
    pub fn accstat(&self) -> AccstatR {
        AccstatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Illicit Memory Access Error Status flag"]
    #[inline(always)]
    pub fn ilterrstat(&self) -> IlterrstatR {
        IlterrstatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Slave Bus Error Status flag"]
    #[inline(always)]
    pub fn slerrstat(&self) -> SlerrstatR {
        SlerrstatR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Illegal Address Access Error Status flag"]
    #[inline(always)]
    pub fn illerrstat(&self) -> IllerrstatR {
        IllerrstatR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "BUS Error Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bus1errstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bus1errstatSpec;
impl crate::RegisterSpec for Bus1errstatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bus1errstat::R`](R) reader structure"]
impl crate::Readable for Bus1errstatSpec {}
#[doc = "`reset()` method sets BUS1ERRSTAT to value 0"]
impl crate::Resettable for Bus1errstatSpec {
    const RESET_VALUE: u8 = 0;
}
