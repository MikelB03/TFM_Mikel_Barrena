#[doc = "Register `IICF%s` reader"]
pub type R = crate::R<IicfSpec>;
#[doc = "Register `IICF%s` writer"]
pub type W = crate::W<IicfSpec>;
#[doc = "Communication reservation function disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicrsv {
    #[doc = "0: Enable communication reservation"]
    _0 = 0,
    #[doc = "1: Disable communication reservation"]
    _1 = 1,
}
impl From<Iicrsv> for bool {
    #[inline(always)]
    fn from(variant: Iicrsv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICRSV` reader - Communication reservation function disable bit"]
pub type IicrsvR = crate::BitReader<Iicrsv>;
impl IicrsvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iicrsv {
        match self.bits {
            false => Iicrsv::_0,
            true => Iicrsv::_1,
        }
    }
    #[doc = "Enable communication reservation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicrsv::_0
    }
    #[doc = "Disable communication reservation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicrsv::_1
    }
}
#[doc = "Field `IICRSV` writer - Communication reservation function disable bit"]
pub type IicrsvW<'a, REG> = crate::BitWriter<'a, REG, Iicrsv>;
impl<'a, REG> IicrsvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable communication reservation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicrsv::_0)
    }
    #[doc = "Disable communication reservation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicrsv::_1)
    }
}
#[doc = "Initial start enable trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stcen {
    #[doc = "0: After operation is enabled (IICCTLn0.IICE = 1), enable generation of a start condition upon detection of a stop condition."]
    _0 = 0,
    #[doc = "1: After operation is enabled (IICCTLn0.IICE = 1), enable generation of a start condition without detecting a stop condition."]
    _1 = 1,
}
impl From<Stcen> for bool {
    #[inline(always)]
    fn from(variant: Stcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STCEN` reader - Initial start enable trigger"]
pub type StcenR = crate::BitReader<Stcen>;
impl StcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stcen {
        match self.bits {
            false => Stcen::_0,
            true => Stcen::_1,
        }
    }
    #[doc = "After operation is enabled (IICCTLn0.IICE = 1), enable generation of a start condition upon detection of a stop condition."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Stcen::_0
    }
    #[doc = "After operation is enabled (IICCTLn0.IICE = 1), enable generation of a start condition without detecting a stop condition."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Stcen::_1
    }
}
#[doc = "Field `STCEN` writer - Initial start enable trigger"]
pub type StcenW<'a, REG> = crate::BitWriter<'a, REG, Stcen>;
impl<'a, REG> StcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "After operation is enabled (IICCTLn0.IICE = 1), enable generation of a start condition upon detection of a stop condition."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Stcen::_0)
    }
    #[doc = "After operation is enabled (IICCTLn0.IICE = 1), enable generation of a start condition without detecting a stop condition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Stcen::_1)
    }
}
#[doc = "I2C bus status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicbsy {
    #[doc = "0: Bus release status (communication initial status when STCEN = 1)"]
    _0 = 0,
    #[doc = "1: Bus communication status (communication initial status when STCEN = 0)"]
    _1 = 1,
}
impl From<Iicbsy> for bool {
    #[inline(always)]
    fn from(variant: Iicbsy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICBSY` reader - I2C bus status flag"]
pub type IicbsyR = crate::BitReader<Iicbsy>;
impl IicbsyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iicbsy {
        match self.bits {
            false => Iicbsy::_0,
            true => Iicbsy::_1,
        }
    }
    #[doc = "Bus release status (communication initial status when STCEN = 1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicbsy::_0
    }
    #[doc = "Bus communication status (communication initial status when STCEN = 0)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicbsy::_1
    }
}
#[doc = "IICCTLn0.STT clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stcf {
    #[doc = "0: Generate start condition"]
    _0 = 0,
    #[doc = "1: Start condition generation unsuccessful: clear the IICCTLn0.STT flag"]
    _1 = 1,
}
impl From<Stcf> for bool {
    #[inline(always)]
    fn from(variant: Stcf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STCF` reader - IICCTLn0.STT clear flag"]
pub type StcfR = crate::BitReader<Stcf>;
impl StcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stcf {
        match self.bits {
            false => Stcf::_0,
            true => Stcf::_1,
        }
    }
    #[doc = "Generate start condition"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Stcf::_0
    }
    #[doc = "Start condition generation unsuccessful: clear the IICCTLn0.STT flag"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Stcf::_1
    }
}
impl R {
    #[doc = "Bit 0 - Communication reservation function disable bit"]
    #[inline(always)]
    pub fn iicrsv(&self) -> IicrsvR {
        IicrsvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Initial start enable trigger"]
    #[inline(always)]
    pub fn stcen(&self) -> StcenR {
        StcenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C bus status flag"]
    #[inline(always)]
    pub fn iicbsy(&self) -> IicbsyR {
        IicbsyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IICCTLn0.STT clear flag"]
    #[inline(always)]
    pub fn stcf(&self) -> StcfR {
        StcfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Communication reservation function disable bit"]
    #[inline(always)]
    pub fn iicrsv(&mut self) -> IicrsvW<IicfSpec> {
        IicrsvW::new(self, 0)
    }
    #[doc = "Bit 1 - Initial start enable trigger"]
    #[inline(always)]
    pub fn stcen(&mut self) -> StcenW<IicfSpec> {
        StcenW::new(self, 1)
    }
}
#[doc = "IICA Flag Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`iicf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iicf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IicfSpec;
impl crate::RegisterSpec for IicfSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iicf::R`](R) reader structure"]
impl crate::Readable for IicfSpec {}
#[doc = "`write(|w| ..)` method takes [`iicf::W`](W) writer structure"]
impl crate::Writable for IicfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IICF%s to value 0"]
impl crate::Resettable for IicfSpec {
    const RESET_VALUE: u8 = 0;
}
