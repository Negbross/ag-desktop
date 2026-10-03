use windows::core::GUID;
use windows::Win32::Media::Audio::{
    eConsole, eRender, IMMDeviceEnumerator, MMDeviceEnumerator, Endpoints::IAudioEndpointVolume
};
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED};


pub fn get_volume_interface() -> anyhow::Result<IAudioEndpointVolume> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER)?;
        let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;
        let volume: IAudioEndpointVolume = device.Activate(CLSCTX_INPROC_SERVER, None)?;
        Ok(volume)
    }
}

pub fn get_volume() -> anyhow::Result<f32> {
    let v = get_volume_interface()?;
    unsafe { Ok(v.GetMasterVolumeLevelScalar()? * 100.0) }
}

pub fn set_volume(percent: f32) -> anyhow::Result<()> {
    let v = get_volume_interface()?;
    let guid = GUID::zeroed();
    unsafe { v.SetMasterVolumeLevelScalar(percent / 100.0, &guid)? }
    Ok(())
}

pub fn mute_volume(state: bool) -> anyhow::Result<()> {
    let v = get_volume_interface()?;
    let guid = GUID::zeroed();
    unsafe { v.SetMute(state, &guid)? }
    Ok(())
}