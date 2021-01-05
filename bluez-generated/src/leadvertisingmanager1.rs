// This code was autogenerated with `dbus-codegen-rust --file=specs/org.bluez.LEAdvertisingManager1.xml --interfaces=org.bluez.LEAdvertisingManager1 --client=nonblock --methodtype=none`, see https://github.com/diwic/dbus-rs
#[allow(unused_imports)]
use dbus::arg;
use dbus::nonblock;

pub trait OrgBluezLEAdvertisingManager1 {
    fn register_advertisement(
        &self,
        advertisement: dbus::Path,
        options: arg::PropMap,
    ) -> nonblock::MethodReply<()>;
    fn unregister_advertisement(&self, service: dbus::Path) -> nonblock::MethodReply<()>;
    fn active_instances(&self) -> nonblock::MethodReply<u8>;
    fn supported_instances(&self) -> nonblock::MethodReply<u8>;
    fn supported_includes(&self) -> nonblock::MethodReply<Vec<String>>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target = T>> OrgBluezLEAdvertisingManager1
    for nonblock::Proxy<'a, C>
{
    fn register_advertisement(
        &self,
        advertisement: dbus::Path,
        options: arg::PropMap,
    ) -> nonblock::MethodReply<()> {
        self.method_call(
            "org.bluez.LEAdvertisingManager1",
            "RegisterAdvertisement",
            (advertisement, options),
        )
    }

    fn unregister_advertisement(&self, service: dbus::Path) -> nonblock::MethodReply<()> {
        self.method_call(
            "org.bluez.LEAdvertisingManager1",
            "UnregisterAdvertisement",
            (service,),
        )
    }

    fn active_instances(&self) -> nonblock::MethodReply<u8> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.LEAdvertisingManager1",
            "ActiveInstances",
        )
    }

    fn supported_instances(&self) -> nonblock::MethodReply<u8> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.LEAdvertisingManager1",
            "SupportedInstances",
        )
    }

    fn supported_includes(&self) -> nonblock::MethodReply<Vec<String>> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.LEAdvertisingManager1",
            "SupportedIncludes",
        )
    }
}
