#[doc = "Register `MSTPCRB` reader"]
pub type R = crate::R<MstpcrbSpec>;
#[doc = "Register `MSTPCRB` writer"]
pub type W = crate::W<MstpcrbSpec>;
#[doc = "I2C Bus Interface 1 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb8 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpb8> for bool {
    #[inline(always)]
    fn from(variant: Mstpb8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPB8` reader - I2C Bus Interface 1 Module Stop"]
pub type Mstpb8R = crate::BitReader<Mstpb8>;
impl Mstpb8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb8 {
        match self.bits {
            false => Mstpb8::_0,
            true => Mstpb8::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb8::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb8::_1
    }
}
#[doc = "Field `MSTPB8` writer - I2C Bus Interface 1 Module Stop"]
pub type Mstpb8W<'a, REG> = crate::BitWriter<'a, REG, Mstpb8>;
impl<'a, REG> Mstpb8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb8::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb8::_1)
    }
}
#[doc = "I2C Bus Interface 0 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb9 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpb9> for bool {
    #[inline(always)]
    fn from(variant: Mstpb9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPB9` reader - I2C Bus Interface 0 Module Stop"]
pub type Mstpb9R = crate::BitReader<Mstpb9>;
impl Mstpb9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb9 {
        match self.bits {
            false => Mstpb9::_0,
            true => Mstpb9::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb9::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb9::_1
    }
}
#[doc = "Field `MSTPB9` writer - I2C Bus Interface 0 Module Stop"]
pub type Mstpb9W<'a, REG> = crate::BitWriter<'a, REG, Mstpb9>;
impl<'a, REG> Mstpb9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb9::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb9::_1)
    }
}
#[doc = "Serial Interface UARTA0/UARTA1 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb17 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpb17> for bool {
    #[inline(always)]
    fn from(variant: Mstpb17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPB17` reader - Serial Interface UARTA0/UARTA1 Module Stop"]
pub type Mstpb17R = crate::BitReader<Mstpb17>;
impl Mstpb17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb17 {
        match self.bits {
            false => Mstpb17::_0,
            true => Mstpb17::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb17::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb17::_1
    }
}
#[doc = "Field `MSTPB17` writer - Serial Interface UARTA0/UARTA1 Module Stop"]
pub type Mstpb17W<'a, REG> = crate::BitWriter<'a, REG, Mstpb17>;
impl<'a, REG> Mstpb17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb17::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb17::_1)
    }
}
#[doc = "Serial Array Unit 1 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb20 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpb20> for bool {
    #[inline(always)]
    fn from(variant: Mstpb20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPB20` reader - Serial Array Unit 1 Module Stop"]
pub type Mstpb20R = crate::BitReader<Mstpb20>;
impl Mstpb20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb20 {
        match self.bits {
            false => Mstpb20::_0,
            true => Mstpb20::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb20::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb20::_1
    }
}
#[doc = "Field `MSTPB20` writer - Serial Array Unit 1 Module Stop"]
pub type Mstpb20W<'a, REG> = crate::BitWriter<'a, REG, Mstpb20>;
impl<'a, REG> Mstpb20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb20::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb20::_1)
    }
}
#[doc = "Serial Array Unit 0 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpb21 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpb21> for bool {
    #[inline(always)]
    fn from(variant: Mstpb21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPB21` reader - Serial Array Unit 0 Module Stop"]
pub type Mstpb21R = crate::BitReader<Mstpb21>;
impl Mstpb21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpb21 {
        match self.bits {
            false => Mstpb21::_0,
            true => Mstpb21::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpb21::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpb21::_1
    }
}
#[doc = "Field `MSTPB21` writer - Serial Array Unit 0 Module Stop"]
pub type Mstpb21W<'a, REG> = crate::BitWriter<'a, REG, Mstpb21>;
impl<'a, REG> Mstpb21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb21::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpb21::_1)
    }
}
impl R {
    #[doc = "Bit 8 - I2C Bus Interface 1 Module Stop"]
    #[inline(always)]
    pub fn mstpb8(&self) -> Mstpb8R {
        Mstpb8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Bus Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb9(&self) -> Mstpb9R {
        Mstpb9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Serial Interface UARTA0/UARTA1 Module Stop"]
    #[inline(always)]
    pub fn mstpb17(&self) -> Mstpb17R {
        Mstpb17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Serial Array Unit 1 Module Stop"]
    #[inline(always)]
    pub fn mstpb20(&self) -> Mstpb20R {
        Mstpb20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Serial Array Unit 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb21(&self) -> Mstpb21R {
        Mstpb21R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - I2C Bus Interface 1 Module Stop"]
    #[inline(always)]
    pub fn mstpb8(&mut self) -> Mstpb8W<MstpcrbSpec> {
        Mstpb8W::new(self, 8)
    }
    #[doc = "Bit 9 - I2C Bus Interface 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb9(&mut self) -> Mstpb9W<MstpcrbSpec> {
        Mstpb9W::new(self, 9)
    }
    #[doc = "Bit 17 - Serial Interface UARTA0/UARTA1 Module Stop"]
    #[inline(always)]
    pub fn mstpb17(&mut self) -> Mstpb17W<MstpcrbSpec> {
        Mstpb17W::new(self, 17)
    }
    #[doc = "Bit 20 - Serial Array Unit 1 Module Stop"]
    #[inline(always)]
    pub fn mstpb20(&mut self) -> Mstpb20W<MstpcrbSpec> {
        Mstpb20W::new(self, 20)
    }
    #[doc = "Bit 21 - Serial Array Unit 0 Module Stop"]
    #[inline(always)]
    pub fn mstpb21(&mut self) -> Mstpb21W<MstpcrbSpec> {
        Mstpb21W::new(self, 21)
    }
}
#[doc = "Module Stop Control Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`mstpcrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstpcrbSpec;
impl crate::RegisterSpec for MstpcrbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstpcrb::R`](R) reader structure"]
impl crate::Readable for MstpcrbSpec {}
#[doc = "`write(|w| ..)` method takes [`mstpcrb::W`](W) writer structure"]
impl crate::Writable for MstpcrbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTPCRB to value 0xffff_ffff"]
impl crate::Resettable for MstpcrbSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
