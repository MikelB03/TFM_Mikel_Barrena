#[doc = "Register `SNZREQCR0` reader"]
pub type R = crate::R<Snzreqcr0Spec>;
#[doc = "Register `SNZREQCR0` writer"]
pub type W = crate::W<Snzreqcr0Spec>;
#[doc = "Enable IRQ0 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen0 {
    #[doc = "0: Disable the snooze request"]
    _0 = 0,
    #[doc = "1: Enable the snooze request"]
    _1 = 1,
}
impl From<Snzreqen0> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN0` reader - Enable IRQ0 pin snooze request"]
pub type Snzreqen0R = crate::BitReader<Snzreqen0>;
impl Snzreqen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen0 {
        match self.bits {
            false => Snzreqen0::_0,
            true => Snzreqen0::_1,
        }
    }
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen0::_0
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen0::_1
    }
}
#[doc = "Field `SNZREQEN0` writer - Enable IRQ0 pin snooze request"]
pub type Snzreqen0W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen0>;
impl<'a, REG> Snzreqen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen0::_0)
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen0::_1)
    }
}
#[doc = "Enable IRQ1 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen1 {
    #[doc = "0: Disable the snooze request"]
    _0 = 0,
    #[doc = "1: Enable the snooze request"]
    _1 = 1,
}
impl From<Snzreqen1> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN1` reader - Enable IRQ1 pin snooze request"]
pub type Snzreqen1R = crate::BitReader<Snzreqen1>;
impl Snzreqen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen1 {
        match self.bits {
            false => Snzreqen1::_0,
            true => Snzreqen1::_1,
        }
    }
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen1::_0
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen1::_1
    }
}
#[doc = "Field `SNZREQEN1` writer - Enable IRQ1 pin snooze request"]
pub type Snzreqen1W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen1>;
impl<'a, REG> Snzreqen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen1::_0)
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen1::_1)
    }
}
#[doc = "Enable IRQ2 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen2 {
    #[doc = "0: Disable the snooze request"]
    _0 = 0,
    #[doc = "1: Enable the snooze request"]
    _1 = 1,
}
impl From<Snzreqen2> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN2` reader - Enable IRQ2 pin snooze request"]
pub type Snzreqen2R = crate::BitReader<Snzreqen2>;
impl Snzreqen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen2 {
        match self.bits {
            false => Snzreqen2::_0,
            true => Snzreqen2::_1,
        }
    }
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen2::_0
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen2::_1
    }
}
#[doc = "Field `SNZREQEN2` writer - Enable IRQ2 pin snooze request"]
pub type Snzreqen2W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen2>;
impl<'a, REG> Snzreqen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen2::_0)
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen2::_1)
    }
}
#[doc = "Enable IRQ3 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen3 {
    #[doc = "0: Disable the snooze request"]
    _0 = 0,
    #[doc = "1: Enable the snooze request"]
    _1 = 1,
}
impl From<Snzreqen3> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN3` reader - Enable IRQ3 pin snooze request"]
pub type Snzreqen3R = crate::BitReader<Snzreqen3>;
impl Snzreqen3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen3 {
        match self.bits {
            false => Snzreqen3::_0,
            true => Snzreqen3::_1,
        }
    }
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen3::_0
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen3::_1
    }
}
#[doc = "Field `SNZREQEN3` writer - Enable IRQ3 pin snooze request"]
pub type Snzreqen3W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen3>;
impl<'a, REG> Snzreqen3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen3::_0)
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen3::_1)
    }
}
#[doc = "Enable IRQ4 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen4 {
    #[doc = "0: Disable the snooze request"]
    _0 = 0,
    #[doc = "1: Enable the snooze request"]
    _1 = 1,
}
impl From<Snzreqen4> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN4` reader - Enable IRQ4 pin snooze request"]
pub type Snzreqen4R = crate::BitReader<Snzreqen4>;
impl Snzreqen4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen4 {
        match self.bits {
            false => Snzreqen4::_0,
            true => Snzreqen4::_1,
        }
    }
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen4::_0
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen4::_1
    }
}
#[doc = "Field `SNZREQEN4` writer - Enable IRQ4 pin snooze request"]
pub type Snzreqen4W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen4>;
impl<'a, REG> Snzreqen4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen4::_0)
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen4::_1)
    }
}
#[doc = "Enable IRQ5 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen5 {
    #[doc = "0: Disable the snooze request"]
    _0 = 0,
    #[doc = "1: Enable the snooze request"]
    _1 = 1,
}
impl From<Snzreqen5> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN5` reader - Enable IRQ5 pin snooze request"]
pub type Snzreqen5R = crate::BitReader<Snzreqen5>;
impl Snzreqen5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen5 {
        match self.bits {
            false => Snzreqen5::_0,
            true => Snzreqen5::_1,
        }
    }
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen5::_0
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen5::_1
    }
}
#[doc = "Field `SNZREQEN5` writer - Enable IRQ5 pin snooze request"]
pub type Snzreqen5W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen5>;
impl<'a, REG> Snzreqen5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen5::_0)
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen5::_1)
    }
}
#[doc = "Enable IRQ6 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen6 {
    #[doc = "0: Disable the snooze request"]
    _0 = 0,
    #[doc = "1: Enable the snooze request"]
    _1 = 1,
}
impl From<Snzreqen6> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN6` reader - Enable IRQ6 pin snooze request"]
pub type Snzreqen6R = crate::BitReader<Snzreqen6>;
impl Snzreqen6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen6 {
        match self.bits {
            false => Snzreqen6::_0,
            true => Snzreqen6::_1,
        }
    }
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen6::_0
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen6::_1
    }
}
#[doc = "Field `SNZREQEN6` writer - Enable IRQ6 pin snooze request"]
pub type Snzreqen6W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen6>;
impl<'a, REG> Snzreqen6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen6::_0)
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen6::_1)
    }
}
#[doc = "Enable IRQ7 pin snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen7 {
    #[doc = "0: Disable the snooze request"]
    _0 = 0,
    #[doc = "1: Enable the snooze request"]
    _1 = 1,
}
impl From<Snzreqen7> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN7` reader - Enable IRQ7 pin snooze request"]
pub type Snzreqen7R = crate::BitReader<Snzreqen7>;
impl Snzreqen7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen7 {
        match self.bits {
            false => Snzreqen7::_0,
            true => Snzreqen7::_1,
        }
    }
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen7::_0
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen7::_1
    }
}
#[doc = "Field `SNZREQEN7` writer - Enable IRQ7 pin snooze request"]
pub type Snzreqen7W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen7>;
impl<'a, REG> Snzreqen7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen7::_0)
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen7::_1)
    }
}
#[doc = "Enable KEY_INTKR snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen17 {
    #[doc = "0: Disable the snooze request"]
    _0 = 0,
    #[doc = "1: Enable the snooze request"]
    _1 = 1,
}
impl From<Snzreqen17> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN17` reader - Enable KEY_INTKR snooze request"]
pub type Snzreqen17R = crate::BitReader<Snzreqen17>;
impl Snzreqen17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen17 {
        match self.bits {
            false => Snzreqen17::_0,
            true => Snzreqen17::_1,
        }
    }
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen17::_0
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen17::_1
    }
}
#[doc = "Field `SNZREQEN17` writer - Enable KEY_INTKR snooze request"]
pub type Snzreqen17W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen17>;
impl<'a, REG> Snzreqen17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen17::_0)
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen17::_1)
    }
}
#[doc = "Enable RTC alarm snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen24 {
    #[doc = "0: Disable the snooze request"]
    _0 = 0,
    #[doc = "1: Enable the snooze request"]
    _1 = 1,
}
impl From<Snzreqen24> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN24` reader - Enable RTC alarm snooze request"]
pub type Snzreqen24R = crate::BitReader<Snzreqen24>;
impl Snzreqen24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen24 {
        match self.bits {
            false => Snzreqen24::_0,
            true => Snzreqen24::_1,
        }
    }
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen24::_0
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen24::_1
    }
}
#[doc = "Field `SNZREQEN24` writer - Enable RTC alarm snooze request"]
pub type Snzreqen24W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen24>;
impl<'a, REG> Snzreqen24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen24::_0)
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen24::_1)
    }
}
#[doc = "Enable TML32 snooze request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Snzreqen26 {
    #[doc = "0: Disable the snooze request"]
    _0 = 0,
    #[doc = "1: Enable the snooze request"]
    _1 = 1,
}
impl From<Snzreqen26> for bool {
    #[inline(always)]
    fn from(variant: Snzreqen26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNZREQEN26` reader - Enable TML32 snooze request"]
pub type Snzreqen26R = crate::BitReader<Snzreqen26>;
impl Snzreqen26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Snzreqen26 {
        match self.bits {
            false => Snzreqen26::_0,
            true => Snzreqen26::_1,
        }
    }
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Snzreqen26::_0
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Snzreqen26::_1
    }
}
#[doc = "Field `SNZREQEN26` writer - Enable TML32 snooze request"]
pub type Snzreqen26W<'a, REG> = crate::BitWriter<'a, REG, Snzreqen26>;
impl<'a, REG> Snzreqen26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen26::_0)
    }
    #[doc = "Enable the snooze request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Snzreqen26::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable IRQ0 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen0(&self) -> Snzreqen0R {
        Snzreqen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable IRQ1 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen1(&self) -> Snzreqen1R {
        Snzreqen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable IRQ2 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen2(&self) -> Snzreqen2R {
        Snzreqen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable IRQ3 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen3(&self) -> Snzreqen3R {
        Snzreqen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable IRQ4 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen4(&self) -> Snzreqen4R {
        Snzreqen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable IRQ5 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen5(&self) -> Snzreqen5R {
        Snzreqen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable IRQ6 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen6(&self) -> Snzreqen6R {
        Snzreqen6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable IRQ7 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen7(&self) -> Snzreqen7R {
        Snzreqen7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable KEY_INTKR snooze request"]
    #[inline(always)]
    pub fn snzreqen17(&self) -> Snzreqen17R {
        Snzreqen17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable RTC alarm snooze request"]
    #[inline(always)]
    pub fn snzreqen24(&self) -> Snzreqen24R {
        Snzreqen24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable TML32 snooze request"]
    #[inline(always)]
    pub fn snzreqen26(&self) -> Snzreqen26R {
        Snzreqen26R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IRQ0 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen0(&mut self) -> Snzreqen0W<Snzreqcr0Spec> {
        Snzreqen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable IRQ1 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen1(&mut self) -> Snzreqen1W<Snzreqcr0Spec> {
        Snzreqen1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable IRQ2 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen2(&mut self) -> Snzreqen2W<Snzreqcr0Spec> {
        Snzreqen2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable IRQ3 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen3(&mut self) -> Snzreqen3W<Snzreqcr0Spec> {
        Snzreqen3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable IRQ4 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen4(&mut self) -> Snzreqen4W<Snzreqcr0Spec> {
        Snzreqen4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable IRQ5 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen5(&mut self) -> Snzreqen5W<Snzreqcr0Spec> {
        Snzreqen5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable IRQ6 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen6(&mut self) -> Snzreqen6W<Snzreqcr0Spec> {
        Snzreqen6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable IRQ7 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen7(&mut self) -> Snzreqen7W<Snzreqcr0Spec> {
        Snzreqen7W::new(self, 7)
    }
    #[doc = "Bit 17 - Enable KEY_INTKR snooze request"]
    #[inline(always)]
    pub fn snzreqen17(&mut self) -> Snzreqen17W<Snzreqcr0Spec> {
        Snzreqen17W::new(self, 17)
    }
    #[doc = "Bit 24 - Enable RTC alarm snooze request"]
    #[inline(always)]
    pub fn snzreqen24(&mut self) -> Snzreqen24W<Snzreqcr0Spec> {
        Snzreqen24W::new(self, 24)
    }
    #[doc = "Bit 26 - Enable TML32 snooze request"]
    #[inline(always)]
    pub fn snzreqen26(&mut self) -> Snzreqen26W<Snzreqcr0Spec> {
        Snzreqen26W::new(self, 26)
    }
}
#[doc = "Snooze Request Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`snzreqcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzreqcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Snzreqcr0Spec;
impl crate::RegisterSpec for Snzreqcr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`snzreqcr0::R`](R) reader structure"]
impl crate::Readable for Snzreqcr0Spec {}
#[doc = "`write(|w| ..)` method takes [`snzreqcr0::W`](W) writer structure"]
impl crate::Writable for Snzreqcr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SNZREQCR0 to value 0"]
impl crate::Resettable for Snzreqcr0Spec {
    const RESET_VALUE: u32 = 0;
}
