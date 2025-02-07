#[doc = "Register `ULBS` reader"]
pub type R = crate::R<UlbsSpec>;
#[doc = "Register `ULBS` writer"]
pub type W = crate::W<UlbsSpec>;
#[doc = "Selection of the UART0 loopback function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ulbs0 {
    #[doc = "0: Inputs the states of the RxD0 pin of serial array unit UART0 to the reception shift register"]
    _0 = 0,
    #[doc = "1: Loops back output from the transmission shift register to the reception shift register"]
    _1 = 1,
}
impl From<Ulbs0> for bool {
    #[inline(always)]
    fn from(variant: Ulbs0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULBS0` reader - Selection of the UART0 loopback function"]
pub type Ulbs0R = crate::BitReader<Ulbs0>;
impl Ulbs0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ulbs0 {
        match self.bits {
            false => Ulbs0::_0,
            true => Ulbs0::_1,
        }
    }
    #[doc = "Inputs the states of the RxD0 pin of serial array unit UART0 to the reception shift register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ulbs0::_0
    }
    #[doc = "Loops back output from the transmission shift register to the reception shift register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ulbs0::_1
    }
}
#[doc = "Field `ULBS0` writer - Selection of the UART0 loopback function"]
pub type Ulbs0W<'a, REG> = crate::BitWriter<'a, REG, Ulbs0>;
impl<'a, REG> Ulbs0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inputs the states of the RxD0 pin of serial array unit UART0 to the reception shift register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbs0::_0)
    }
    #[doc = "Loops back output from the transmission shift register to the reception shift register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbs0::_1)
    }
}
#[doc = "Selection of the UART1 loopback function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ulbs1 {
    #[doc = "0: Inputs the states of the RxD1 pin of serial array unit UART1 to the reception shift register"]
    _0 = 0,
    #[doc = "1: Loops back output from the transmission shift register to the reception shift register"]
    _1 = 1,
}
impl From<Ulbs1> for bool {
    #[inline(always)]
    fn from(variant: Ulbs1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULBS1` reader - Selection of the UART1 loopback function"]
pub type Ulbs1R = crate::BitReader<Ulbs1>;
impl Ulbs1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ulbs1 {
        match self.bits {
            false => Ulbs1::_0,
            true => Ulbs1::_1,
        }
    }
    #[doc = "Inputs the states of the RxD1 pin of serial array unit UART1 to the reception shift register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ulbs1::_0
    }
    #[doc = "Loops back output from the transmission shift register to the reception shift register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ulbs1::_1
    }
}
#[doc = "Field `ULBS1` writer - Selection of the UART1 loopback function"]
pub type Ulbs1W<'a, REG> = crate::BitWriter<'a, REG, Ulbs1>;
impl<'a, REG> Ulbs1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inputs the states of the RxD1 pin of serial array unit UART1 to the reception shift register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbs1::_0)
    }
    #[doc = "Loops back output from the transmission shift register to the reception shift register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbs1::_1)
    }
}
#[doc = "Selection of the UART2 loopback function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ulbs2 {
    #[doc = "0: Inputs the states of the RxD2 pin of serial array unit UART2 to the reception shift register"]
    _0 = 0,
    #[doc = "1: Loops back output from the transmission shift register to the reception shift register"]
    _1 = 1,
}
impl From<Ulbs2> for bool {
    #[inline(always)]
    fn from(variant: Ulbs2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULBS2` reader - Selection of the UART2 loopback function"]
pub type Ulbs2R = crate::BitReader<Ulbs2>;
impl Ulbs2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ulbs2 {
        match self.bits {
            false => Ulbs2::_0,
            true => Ulbs2::_1,
        }
    }
    #[doc = "Inputs the states of the RxD2 pin of serial array unit UART2 to the reception shift register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ulbs2::_0
    }
    #[doc = "Loops back output from the transmission shift register to the reception shift register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ulbs2::_1
    }
}
#[doc = "Field `ULBS2` writer - Selection of the UART2 loopback function"]
pub type Ulbs2W<'a, REG> = crate::BitWriter<'a, REG, Ulbs2>;
impl<'a, REG> Ulbs2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inputs the states of the RxD2 pin of serial array unit UART2 to the reception shift register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbs2::_0)
    }
    #[doc = "Loops back output from the transmission shift register to the reception shift register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbs2::_1)
    }
}
#[doc = "Selection of the UARTA0 loopback function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ulbs4 {
    #[doc = "0: Inputs the states of the RxDA0 pin of serial interface UARTA0 to the reception shift register"]
    _0 = 0,
    #[doc = "1: Loops back output from the transmission shift register to the reception shift register"]
    _1 = 1,
}
impl From<Ulbs4> for bool {
    #[inline(always)]
    fn from(variant: Ulbs4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULBS4` reader - Selection of the UARTA0 loopback function"]
pub type Ulbs4R = crate::BitReader<Ulbs4>;
impl Ulbs4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ulbs4 {
        match self.bits {
            false => Ulbs4::_0,
            true => Ulbs4::_1,
        }
    }
    #[doc = "Inputs the states of the RxDA0 pin of serial interface UARTA0 to the reception shift register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ulbs4::_0
    }
    #[doc = "Loops back output from the transmission shift register to the reception shift register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ulbs4::_1
    }
}
#[doc = "Field `ULBS4` writer - Selection of the UARTA0 loopback function"]
pub type Ulbs4W<'a, REG> = crate::BitWriter<'a, REG, Ulbs4>;
impl<'a, REG> Ulbs4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inputs the states of the RxDA0 pin of serial interface UARTA0 to the reception shift register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbs4::_0)
    }
    #[doc = "Loops back output from the transmission shift register to the reception shift register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbs4::_1)
    }
}
#[doc = "Selection of the UARTA1 loopback function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ulbs5 {
    #[doc = "0: Inputs the states of the RxDA1 pin of serial interface UARTA1 to the reception shift register"]
    _0 = 0,
    #[doc = "1: Loops back output from the transmission shift register to the reception shift register"]
    _1 = 1,
}
impl From<Ulbs5> for bool {
    #[inline(always)]
    fn from(variant: Ulbs5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULBS5` reader - Selection of the UARTA1 loopback function"]
pub type Ulbs5R = crate::BitReader<Ulbs5>;
impl Ulbs5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ulbs5 {
        match self.bits {
            false => Ulbs5::_0,
            true => Ulbs5::_1,
        }
    }
    #[doc = "Inputs the states of the RxDA1 pin of serial interface UARTA1 to the reception shift register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ulbs5::_0
    }
    #[doc = "Loops back output from the transmission shift register to the reception shift register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ulbs5::_1
    }
}
#[doc = "Field `ULBS5` writer - Selection of the UARTA1 loopback function"]
pub type Ulbs5W<'a, REG> = crate::BitWriter<'a, REG, Ulbs5>;
impl<'a, REG> Ulbs5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inputs the states of the RxDA1 pin of serial interface UARTA1 to the reception shift register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbs5::_0)
    }
    #[doc = "Loops back output from the transmission shift register to the reception shift register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbs5::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Selection of the UART0 loopback function"]
    #[inline(always)]
    pub fn ulbs0(&self) -> Ulbs0R {
        Ulbs0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selection of the UART1 loopback function"]
    #[inline(always)]
    pub fn ulbs1(&self) -> Ulbs1R {
        Ulbs1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selection of the UART2 loopback function"]
    #[inline(always)]
    pub fn ulbs2(&self) -> Ulbs2R {
        Ulbs2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Selection of the UARTA0 loopback function"]
    #[inline(always)]
    pub fn ulbs4(&self) -> Ulbs4R {
        Ulbs4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selection of the UARTA1 loopback function"]
    #[inline(always)]
    pub fn ulbs5(&self) -> Ulbs5R {
        Ulbs5R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selection of the UART0 loopback function"]
    #[inline(always)]
    pub fn ulbs0(&mut self) -> Ulbs0W<UlbsSpec> {
        Ulbs0W::new(self, 0)
    }
    #[doc = "Bit 1 - Selection of the UART1 loopback function"]
    #[inline(always)]
    pub fn ulbs1(&mut self) -> Ulbs1W<UlbsSpec> {
        Ulbs1W::new(self, 1)
    }
    #[doc = "Bit 2 - Selection of the UART2 loopback function"]
    #[inline(always)]
    pub fn ulbs2(&mut self) -> Ulbs2W<UlbsSpec> {
        Ulbs2W::new(self, 2)
    }
    #[doc = "Bit 4 - Selection of the UARTA0 loopback function"]
    #[inline(always)]
    pub fn ulbs4(&mut self) -> Ulbs4W<UlbsSpec> {
        Ulbs4W::new(self, 4)
    }
    #[doc = "Bit 5 - Selection of the UARTA1 loopback function"]
    #[inline(always)]
    pub fn ulbs5(&mut self) -> Ulbs5W<UlbsSpec> {
        Ulbs5W::new(self, 5)
    }
}
#[doc = "UART Loopback Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ulbs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulbs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UlbsSpec;
impl crate::RegisterSpec for UlbsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ulbs::R`](R) reader structure"]
impl crate::Readable for UlbsSpec {}
#[doc = "`write(|w| ..)` method takes [`ulbs::W`](W) writer structure"]
impl crate::Writable for UlbsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ULBS to value 0"]
impl crate::Resettable for UlbsSpec {
    const RESET_VALUE: u8 = 0;
}
