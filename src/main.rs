use rouille::Request;
use rouille::Response;
use sysinfo::{
    Components, Disks, Networks, System, CpuRefreshKind, RefreshKind
};




fn main(){
    println!("hello, World!");

    rouille::start_server("0.0.0.0:6900", move |request| {
        println!("recvd req to {}", request.url());
        let mut sys = System::new_all();
        sys.refresh_all();
        match request.url().to_ascii_lowercase().as_str() {
            "/ram_total" =>{
                let system_quantity = sys.total_memory();
                let response_string = system_quantity.to_string();
                Response::text(response_string)
            },
            "/ram_used" =>{
                let system_quantity = sys.used_memory();
                let response_string = system_quantity.to_string();
                Response::text(response_string)
            },
            "/swap_total" =>{
                let system_quantity = sys.total_swap();
                let response_string = system_quantity.to_string();
                Response::text(response_string)
            },
            "/swap_used" =>{
                let system_quantity = sys.used_swap();
                let response_string = system_quantity.to_string();
                Response::text(response_string)
            },
            "/global_cpu_usage" =>{
                let system_quantity = sys.global_cpu_usage();
                // Wait a bit because CPU usage is based on diff.
                std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
                // Refresh CPUs again to get actual value.
                sys.refresh_cpu_usage();
                let response_string = system_quantity.to_string();
                Response::text(response_string)
            },
            "/segmented_cpu_usage" =>{
                let system_quantity = sys.cpus();
                // Wait a bit because CPU usage is based on diff.
                std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
                // Refresh CPUs again to get actual value.
                sys.refresh_cpu_usage();
                let mut response_string = "".to_string();
                for cpu in sys.cpus() {
                    println!("{}%", cpu.cpu_usage());
                    response_string += (cpu.cpu_usage().to_string().as_str().to_owned() + "%,").as_str();
                }
                Response::text(response_string)
            },
            "/uptime" =>{
                let system_quantity = sysinfo::System::uptime();
                let response_string = system_quantity.to_string();
                Response::text(response_string)
            },
            "/physical_core_count" =>{
                let system_quantity = sys.physical_core_count().unwrap_or("ERROR".parse().unwrap());
                let response_string = system_quantity.to_string();
                Response::text(response_string)
            },
            "/name" =>{
                let system_quantity = sysinfo::System::name().unwrap_or("ERROR".parse().unwrap());
                let response_string = system_quantity.to_string();
                Response::text(response_string)
            },
            "/verbose_os_version" =>{
                let system_quantity = sysinfo::System::long_os_version().unwrap_or("ERROR".parse().unwrap());
                let response_string = system_quantity.to_string();
                Response::text(response_string)
            },
            "/cpu_arch" =>{
                let system_quantity = sysinfo::System::cpu_arch();
                let response_string = system_quantity.to_string();
                Response::text(response_string)
            },
            "/owo" =>{
                Response::text("UwU")
            },
            "/uwu" =>{
                Response::text("OwO")
            },
            "/index" => {
                Response::text("/ram_total, /ram_used, /swap_total, /swap_used, /cpu_total, /global_cpu_usage, /uptime, /owo, /uwu, /segmented_cpu_usage, /physical_core_count, /name, /verbose_os_version, /cpu_arch, ")
            }
            _ =>{
                Response::empty_404()
            }
        }
    });
}