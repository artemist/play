#[doc = "Register `DEVICE_ID` reader"]
pub type R = crate::R<DeviceIdSpec>;
#[doc = "Field `DEVICEID` reader - Part ID numbers for LPC111x 0x041E 502B; 0x2516 D02B = LPC1111FHN33/101 0x2516 D02B = LPC1111FHN33/102 0x0416 502B; 0x2516 902B = LPC1111FHN33/201 0x2516 902B = LPC1111FHN33/202 0x042D 502B; 0x2524 D02B = LPC1112FHN33/101 0x2524 D02B = LPC1112FHN33/102 0x0425 502B; 0x2524 902B = LPC1112FHN33/201 0x2524 902B = LPC1112FHN33/202 0x2524 902B = LPC1112FHI33/202 0x0434 502B; 0x2532 902B = LPC1113FHN33/201 0x2532 902B = LPC1113FHN33/202 0x0434 102B; 0x2532 102B = LPC1113FHN33/301 0x2532 102B = LPC1113FHN33/302 0x0434 102B; 0x2532 102B = LPC1113FBD48/301 0x2532 102B = LPC1113FBD48/302 0x0444 502B; 0x2540 902B = LPC1114FHN33/201 0x2540 902B = LPC1114FHN33/202 0x0444 102B; 0x2540 102B = LPC1114FHN33/301 0x2540 102B = LPC1114FHN33/302 0x2540 102B = LPC1114FHI33/302 0x0444 102B; 0x2540 102B = LPC1114FBD48/301 0x2540 102B = LPC1114FBD48/302 0x2540 102B = LPC11D14FBD100/302"]
pub type DeviceidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Part ID numbers for LPC111x 0x041E 502B; 0x2516 D02B = LPC1111FHN33/101 0x2516 D02B = LPC1111FHN33/102 0x0416 502B; 0x2516 902B = LPC1111FHN33/201 0x2516 902B = LPC1111FHN33/202 0x042D 502B; 0x2524 D02B = LPC1112FHN33/101 0x2524 D02B = LPC1112FHN33/102 0x0425 502B; 0x2524 902B = LPC1112FHN33/201 0x2524 902B = LPC1112FHN33/202 0x2524 902B = LPC1112FHI33/202 0x0434 502B; 0x2532 902B = LPC1113FHN33/201 0x2532 902B = LPC1113FHN33/202 0x0434 102B; 0x2532 102B = LPC1113FHN33/301 0x2532 102B = LPC1113FHN33/302 0x0434 102B; 0x2532 102B = LPC1113FBD48/301 0x2532 102B = LPC1113FBD48/302 0x0444 502B; 0x2540 902B = LPC1114FHN33/201 0x2540 902B = LPC1114FHN33/202 0x0444 102B; 0x2540 102B = LPC1114FHN33/301 0x2540 102B = LPC1114FHN33/302 0x2540 102B = LPC1114FHI33/302 0x0444 102B; 0x2540 102B = LPC1114FBD48/301 0x2540 102B = LPC1114FBD48/302 0x2540 102B = LPC11D14FBD100/302"]
    #[inline(always)]
    pub fn deviceid(&self) -> DeviceidR {
        DeviceidR::new(self.bits)
    }
}
#[doc = "Device ID register 0 for parts LPC1100, LPC1100C, LPC1100L.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceIdSpec;
impl crate::RegisterSpec for DeviceIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`device_id::R`](R) reader structure"]
impl crate::Readable for DeviceIdSpec {}
#[doc = "`reset()` method sets DEVICE_ID to value 0"]
impl crate::Resettable for DeviceIdSpec {
    const RESET_VALUE: u32 = 0;
}
