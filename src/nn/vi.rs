#[allow(unused_imports)]
use self::super::root;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Display {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Layer {
    _unused: [u8; 0],
}
pub const ScalingMode_None: root::nn::vi::ScalingMode = 0;
pub const ScalingMode_Exact: root::nn::vi::ScalingMode = 1;
pub const ScalingMode_FitLayer: root::nn::vi::ScalingMode = 2;
pub const ScalingMode_ScaleAndCrop: root::nn::vi::ScalingMode = 3;
pub const ScalingMode_PreserveAspectRatio: root::nn::vi::ScalingMode = 4;
pub type ScalingMode = u32;
extern "C" {
    #[link_name = "\u{1}_ZN2nn2vi10InitializeEv"]
    pub fn Initialize();
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2vi18OpenDefaultDisplayEPPNS0_7DisplayE"]
    pub fn OpenDefaultDisplay(
        out_Disp: *mut *mut root::nn::vi::Display,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2vi11CreateLayerEPNS0_5LayerEPNS0_7DisplayE"]
    pub fn CreateLayer(
        out_Layer: *mut root::nn::vi::Layer,
        disp: *mut root::nn::vi::Display,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2vi19SetLayerScalingModeEPNS0_5LayerENS0_11ScalingModeE"]
    pub fn SetLayerScalingMode(
        layer: *mut root::nn::vi::Layer,
        scalingMode: root::nn::vi::ScalingMode,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2vi20GetDisplayVsyncEventEPNS_2os15SystemEventTypeEPNS0_7DisplayE"]
    pub fn GetDisplayVsyncEvent(
        arg1: *mut root::nn::os::SystemEventType,
        arg2: *mut root::nn::vi::Display,
    ) -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn2vi15GetNativeWindowEPPvPNS0_5LayerE"]
    pub fn GetNativeWindow(
        window: *mut *mut libc::c_void,
        arg1: *mut root::nn::vi::Layer,
    ) -> root::Result;
}