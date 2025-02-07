#[doc = "Register `POSR` writer"]
pub type W = crate::W<PosrSpec>;
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr00 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr00> for bool {
    #[inline(always)]
    fn from(variant: Posr00) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR00` writer - Pin and Pjn Output Set"]
pub type Posr00W<'a, REG> = crate::BitWriter<'a, REG, Posr00>;
impl<'a, REG> Posr00W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr00::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr00::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr01 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr01> for bool {
    #[inline(always)]
    fn from(variant: Posr01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR01` writer - Pin and Pjn Output Set"]
pub type Posr01W<'a, REG> = crate::BitWriter<'a, REG, Posr01>;
impl<'a, REG> Posr01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr01::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr01::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr02 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr02> for bool {
    #[inline(always)]
    fn from(variant: Posr02) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR02` writer - Pin and Pjn Output Set"]
pub type Posr02W<'a, REG> = crate::BitWriter<'a, REG, Posr02>;
impl<'a, REG> Posr02W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr02::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr02::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr03 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr03> for bool {
    #[inline(always)]
    fn from(variant: Posr03) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR03` writer - Pin and Pjn Output Set"]
pub type Posr03W<'a, REG> = crate::BitWriter<'a, REG, Posr03>;
impl<'a, REG> Posr03W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr03::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr03::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr04 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr04> for bool {
    #[inline(always)]
    fn from(variant: Posr04) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR04` writer - Pin and Pjn Output Set"]
pub type Posr04W<'a, REG> = crate::BitWriter<'a, REG, Posr04>;
impl<'a, REG> Posr04W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr04::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr04::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr05 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr05> for bool {
    #[inline(always)]
    fn from(variant: Posr05) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR05` writer - Pin and Pjn Output Set"]
pub type Posr05W<'a, REG> = crate::BitWriter<'a, REG, Posr05>;
impl<'a, REG> Posr05W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr05::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr05::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr06 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr06> for bool {
    #[inline(always)]
    fn from(variant: Posr06) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR06` writer - Pin and Pjn Output Set"]
pub type Posr06W<'a, REG> = crate::BitWriter<'a, REG, Posr06>;
impl<'a, REG> Posr06W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr06::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr06::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr07 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr07> for bool {
    #[inline(always)]
    fn from(variant: Posr07) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR07` writer - Pin and Pjn Output Set"]
pub type Posr07W<'a, REG> = crate::BitWriter<'a, REG, Posr07>;
impl<'a, REG> Posr07W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr07::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr07::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr08 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr08> for bool {
    #[inline(always)]
    fn from(variant: Posr08) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR08` writer - Pin and Pjn Output Set"]
pub type Posr08W<'a, REG> = crate::BitWriter<'a, REG, Posr08>;
impl<'a, REG> Posr08W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr08::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr08::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr09 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr09> for bool {
    #[inline(always)]
    fn from(variant: Posr09) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR09` writer - Pin and Pjn Output Set"]
pub type Posr09W<'a, REG> = crate::BitWriter<'a, REG, Posr09>;
impl<'a, REG> Posr09W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr09::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr09::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr10 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr10> for bool {
    #[inline(always)]
    fn from(variant: Posr10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR10` writer - Pin and Pjn Output Set"]
pub type Posr10W<'a, REG> = crate::BitWriter<'a, REG, Posr10>;
impl<'a, REG> Posr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr10::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr10::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr11 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr11> for bool {
    #[inline(always)]
    fn from(variant: Posr11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR11` writer - Pin and Pjn Output Set"]
pub type Posr11W<'a, REG> = crate::BitWriter<'a, REG, Posr11>;
impl<'a, REG> Posr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr11::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr11::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr12 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr12> for bool {
    #[inline(always)]
    fn from(variant: Posr12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR12` writer - Pin and Pjn Output Set"]
pub type Posr12W<'a, REG> = crate::BitWriter<'a, REG, Posr12>;
impl<'a, REG> Posr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr12::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr12::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr13 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr13> for bool {
    #[inline(always)]
    fn from(variant: Posr13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR13` writer - Pin and Pjn Output Set"]
pub type Posr13W<'a, REG> = crate::BitWriter<'a, REG, Posr13>;
impl<'a, REG> Posr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr13::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr13::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr14 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr14> for bool {
    #[inline(always)]
    fn from(variant: Posr14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR14` writer - Pin and Pjn Output Set"]
pub type Posr14W<'a, REG> = crate::BitWriter<'a, REG, Posr14>;
impl<'a, REG> Posr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr14::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr14::_1)
    }
}
#[doc = "Pin and Pjn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posr15 {
    #[doc = "0: No effect on output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Posr15> for bool {
    #[inline(always)]
    fn from(variant: Posr15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSR15` writer - Pin and Pjn Output Set"]
pub type Posr15W<'a, REG> = crate::BitWriter<'a, REG, Posr15>;
impl<'a, REG> Posr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr15::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr15::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr00(&mut self) -> Posr00W<PosrSpec> {
        Posr00W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr01(&mut self) -> Posr01W<PosrSpec> {
        Posr01W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr02(&mut self) -> Posr02W<PosrSpec> {
        Posr02W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr03(&mut self) -> Posr03W<PosrSpec> {
        Posr03W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr04(&mut self) -> Posr04W<PosrSpec> {
        Posr04W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr05(&mut self) -> Posr05W<PosrSpec> {
        Posr05W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr06(&mut self) -> Posr06W<PosrSpec> {
        Posr06W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr07(&mut self) -> Posr07W<PosrSpec> {
        Posr07W::new(self, 7)
    }
    #[doc = "Bit 8 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr08(&mut self) -> Posr08W<PosrSpec> {
        Posr08W::new(self, 8)
    }
    #[doc = "Bit 9 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr09(&mut self) -> Posr09W<PosrSpec> {
        Posr09W::new(self, 9)
    }
    #[doc = "Bit 10 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr10(&mut self) -> Posr10W<PosrSpec> {
        Posr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr11(&mut self) -> Posr11W<PosrSpec> {
        Posr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr12(&mut self) -> Posr12W<PosrSpec> {
        Posr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr13(&mut self) -> Posr13W<PosrSpec> {
        Posr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr14(&mut self) -> Posr14W<PosrSpec> {
        Posr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pin and Pjn Output Set"]
    #[inline(always)]
    pub fn posr15(&mut self) -> Posr15W<PosrSpec> {
        Posr15W::new(self, 15)
    }
}
#[doc = "Port Control Register 3\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`posr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PosrSpec;
impl crate::RegisterSpec for PosrSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`posr::W`](W) writer structure"]
impl crate::Writable for PosrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets POSR to value 0"]
impl crate::Resettable for PosrSpec {
    const RESET_VALUE: u16 = 0;
}
