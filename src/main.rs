use clap::{Parser, Subcommand, ValueEnum};
use com_policy_config::{IPolicyConfig, PolicyConfigClient};
use windows::core::{Result, PCWSTR};
use windows::Win32::Devices::Display::{
    SetDisplayConfig, SDC_APPLY, SDC_TOPOLOGY_CLONE, SDC_TOPOLOGY_EXTEND, SDC_TOPOLOGY_EXTERNAL,
    SDC_TOPOLOGY_INTERNAL,
};
use windows::Win32::Media::Audio::{
    eConsole, eRender, IMMDeviceEnumerator, MMDeviceEnumerator, DEVICE_STATE_ACTIVE,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
};

#[derive(Parser)]
#[command(about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Display { topology: DisplayTopology },
    Audio { device_index: u32 },
}

#[derive(ValueEnum, Clone)]
enum DisplayTopology {
    Internal,
    External,
    Clone,
    Extend,
}

fn main() {
    match Cli::parse().command {
        Commands::Display { topology } => set_display(topology),
        Commands::Audio { device_index } => set_audio(device_index).unwrap(),
    }
}

fn set_display(topology: DisplayTopology) {
    let topo = match topology {
        DisplayTopology::Internal => SDC_TOPOLOGY_INTERNAL,
        DisplayTopology::External => SDC_TOPOLOGY_EXTERNAL,
        DisplayTopology::Clone => SDC_TOPOLOGY_CLONE,
        DisplayTopology::Extend => SDC_TOPOLOGY_EXTEND,
    };

    unsafe {
        let result = SetDisplayConfig(None, None, topo | SDC_APPLY);
        assert_eq!(result, 0)
    }
}

fn set_audio(device_index: u32) -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)?;

        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let device = enumerator
            .EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?
            .Item(device_index)?;

        let device_id = PCWSTR(device.GetId()?.0);
        let policy_config: IPolicyConfig = CoCreateInstance(&PolicyConfigClient, None, CLSCTX_ALL)?;
        policy_config.SetDefaultEndpoint(device_id, eConsole)?;

        CoUninitialize();
    }

    Ok(())
}
