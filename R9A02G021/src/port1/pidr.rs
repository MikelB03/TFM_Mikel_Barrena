#[doc = "Register `PIDR` reader"]
pub type R = crate::R<PidrSpec>;
#[doc = "Pin and Pjn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr00 {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr00> for bool {
    #[inline(always)]
    fn from(variant: Pidr00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR00` reader - Pin and Pjn State"]
pub type Pidr00R = crate::BitReader<Pidr00>;
impl Pidr00R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr00 {
        match self.bits {
            false => Pidr00::_0,
            true => Pidr00::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr00::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr00::_1
    }
}
#[doc = "Pin and Pjn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr01 {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr01> for bool {
    #[inline(always)]
    fn from(variant: Pidr01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR01` reader - Pin and Pjn State"]
pub type Pidr01R = crate::BitReader<Pidr01>;
impl Pidr01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr01 {
        match self.bits {
            false => Pidr01::_0,
            true => Pidr01::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr01::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr01::_1
    }
}
#[doc = "Pin and Pjn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr02 {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr02> for bool {
    #[inline(always)]
    fn from(variant: Pidr02) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR02` reader - Pin and Pjn State"]
pub type Pidr02R = crate::BitReader<Pidr02>;
impl Pidr02R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr02 {
        match self.bits {
            false => Pidr02::_0,
            true => Pidr02::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr02::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr02::_1
    }
}
#[doc = "Pin and Pjn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr03 {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr03> for bool {
    #[inline(always)]
    fn from(variant: Pidr03) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR03` reader - Pin and Pjn State"]
pub type Pidr03R = crate::BitReader<Pidr03>;
impl Pidr03R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr03 {
        match self.bits {
            false => Pidr03::_0,
            true => Pidr03::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr03::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr03::_1
    }
}
#[doc = "Pin and Pjn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr04 {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr04> for bool {
    #[inline(always)]
    fn from(variant: Pidr04) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR04` reader - Pin and Pjn State"]
pub type Pidr04R = crate::BitReader<Pidr04>;
impl Pidr04R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr04 {
        match self.bits {
            false => Pidr04::_0,
            true => Pidr04::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr04::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr04::_1
    }
}
#[doc = "Pin and Pjn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr05 {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr05> for bool {
    #[inline(always)]
    fn from(variant: Pidr05) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR05` reader - Pin and Pjn State"]
pub type Pidr05R = crate::BitReader<Pidr05>;
impl Pidr05R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr05 {
        match self.bits {
            false => Pidr05::_0,
            true => Pidr05::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr05::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr05::_1
    }
}
#[doc = "Pin and Pjn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr06 {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr06> for bool {
    #[inline(always)]
    fn from(variant: Pidr06) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR06` reader - Pin and Pjn State"]
pub type Pidr06R = crate::BitReader<Pidr06>;
impl Pidr06R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr06 {
        match self.bits {
            false => Pidr06::_0,
            true => Pidr06::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr06::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr06::_1
    }
}
#[doc = "Pin and Pjn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr07 {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr07> for bool {
    #[inline(always)]
    fn from(variant: Pidr07) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR07` reader - Pin and Pjn State"]
pub type Pidr07R = crate::BitReader<Pidr07>;
impl Pidr07R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr07 {
        match self.bits {
            false => Pidr07::_0,
            true => Pidr07::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr07::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr07::_1
    }
}
#[doc = "Pin and Pjn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr08 {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr08> for bool {
    #[inline(always)]
    fn from(variant: Pidr08) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR08` reader - Pin and Pjn State"]
pub type Pidr08R = crate::BitReader<Pidr08>;
impl Pidr08R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr08 {
        match self.bits {
            false => Pidr08::_0,
            true => Pidr08::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr08::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr08::_1
    }
}
#[doc = "Pin and Pjn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr09 {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr09> for bool {
    #[inline(always)]
    fn from(variant: Pidr09) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR09` reader - Pin and Pjn State"]
pub type Pidr09R = crate::BitReader<Pidr09>;
impl Pidr09R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr09 {
        match self.bits {
            false => Pidr09::_0,
            true => Pidr09::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr09::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr09::_1
    }
}
#[doc = "Pin and Pjn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr10 {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr10> for bool {
    #[inline(always)]
    fn from(variant: Pidr10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR10` reader - Pin and Pjn State"]
pub type Pidr10R = crate::BitReader<Pidr10>;
impl Pidr10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr10 {
        match self.bits {
            false => Pidr10::_0,
            true => Pidr10::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr10::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr10::_1
    }
}
#[doc = "Pin and Pjn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr11 {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr11> for bool {
    #[inline(always)]
    fn from(variant: Pidr11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR11` reader - Pin and Pjn State"]
pub type Pidr11R = crate::BitReader<Pidr11>;
impl Pidr11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr11 {
        match self.bits {
            false => Pidr11::_0,
            true => Pidr11::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr11::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr11::_1
    }
}
#[doc = "Pin and Pjn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr12 {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr12> for bool {
    #[inline(always)]
    fn from(variant: Pidr12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR12` reader - Pin and Pjn State"]
pub type Pidr12R = crate::BitReader<Pidr12>;
impl Pidr12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr12 {
        match self.bits {
            false => Pidr12::_0,
            true => Pidr12::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr12::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr12::_1
    }
}
#[doc = "Pin and Pjn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr13 {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr13> for bool {
    #[inline(always)]
    fn from(variant: Pidr13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR13` reader - Pin and Pjn State"]
pub type Pidr13R = crate::BitReader<Pidr13>;
impl Pidr13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr13 {
        match self.bits {
            false => Pidr13::_0,
            true => Pidr13::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr13::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr13::_1
    }
}
#[doc = "Pin and Pjn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr14 {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr14> for bool {
    #[inline(always)]
    fn from(variant: Pidr14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR14` reader - Pin and Pjn State"]
pub type Pidr14R = crate::BitReader<Pidr14>;
impl Pidr14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr14 {
        match self.bits {
            false => Pidr14::_0,
            true => Pidr14::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr14::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr14::_1
    }
}
#[doc = "Pin and Pjn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr15 {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr15> for bool {
    #[inline(always)]
    fn from(variant: Pidr15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR15` reader - Pin and Pjn State"]
pub type Pidr15R = crate::BitReader<Pidr15>;
impl Pidr15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr15 {
        match self.bits {
            false => Pidr15::_0,
            true => Pidr15::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr15::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr15::_1
    }
}
impl R {
    #[doc = "Bit 0 - Pin and Pjn State"]
    #[inline(always)]
    pub fn pidr00(&self) -> Pidr00R {
        Pidr00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin and Pjn State"]
    #[inline(always)]
    pub fn pidr01(&self) -> Pidr01R {
        Pidr01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin and Pjn State"]
    #[inline(always)]
    pub fn pidr02(&self) -> Pidr02R {
        Pidr02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin and Pjn State"]
    #[inline(always)]
    pub fn pidr03(&self) -> Pidr03R {
        Pidr03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin and Pjn State"]
    #[inline(always)]
    pub fn pidr04(&self) -> Pidr04R {
        Pidr04R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin and Pjn State"]
    #[inline(always)]
    pub fn pidr05(&self) -> Pidr05R {
        Pidr05R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin and Pjn State"]
    #[inline(always)]
    pub fn pidr06(&self) -> Pidr06R {
        Pidr06R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin and Pjn State"]
    #[inline(always)]
    pub fn pidr07(&self) -> Pidr07R {
        Pidr07R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pin and Pjn State"]
    #[inline(always)]
    pub fn pidr08(&self) -> Pidr08R {
        Pidr08R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pin and Pjn State"]
    #[inline(always)]
    pub fn pidr09(&self) -> Pidr09R {
        Pidr09R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pin and Pjn State"]
    #[inline(always)]
    pub fn pidr10(&self) -> Pidr10R {
        Pidr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pin and Pjn State"]
    #[inline(always)]
    pub fn pidr11(&self) -> Pidr11R {
        Pidr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pin and Pjn State"]
    #[inline(always)]
    pub fn pidr12(&self) -> Pidr12R {
        Pidr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pin and Pjn State"]
    #[inline(always)]
    pub fn pidr13(&self) -> Pidr13R {
        Pidr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pin and Pjn State"]
    #[inline(always)]
    pub fn pidr14(&self) -> Pidr14R {
        Pidr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pin and Pjn State"]
    #[inline(always)]
    pub fn pidr15(&self) -> Pidr15R {
        Pidr15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Port Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PidrSpec;
impl crate::RegisterSpec for PidrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pidr::R`](R) reader structure"]
impl crate::Readable for PidrSpec {}
#[doc = "`reset()` method sets PIDR to value 0"]
impl crate::Resettable for PidrSpec {
    const RESET_VALUE: u16 = 0;
}
