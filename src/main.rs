use rouille::Response;
use sysinfo::{Disks, System, MINIMUM_CPU_UPDATE_INTERVAL};


fn all_data(newlines:bool, sys: &mut System)->String{
    let ram_total = sys.total_memory();
    let ram_used = sys.used_memory();
    let swap_total = sys.total_swap();
    let swap_used = sys.used_swap();

    let uptime = System::uptime();
    let cpu_arch = System::cpu_arch();

    let hostname = System::host_name().unwrap_or("ERROR".to_string());
    let name = System::name().unwrap_or("ERROR".to_string());
    let physical_core_count = sys.physical_core_count().unwrap_or(0usize);
    let verbose_os_version = System::long_os_version().unwrap_or("ERROR".to_string());

    let _ = sys.global_cpu_usage();
    let _ = sys.cpus();
    // Wait a bit because cpu usage is based on diff.
    std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    // Refresh CPUs again to get actual value.
    sys.refresh_cpu_usage();
    let global_cpu_usage = sys.global_cpu_usage();
    let segmented_cpu = sys.cpus();//cpu's contains freq and segmented usage
    let disks = Disks::new_with_refreshed_list();

    let mut cpu_frequency_string = String::new();
    let mut cpu_usage_string = String::new();
    let mut cpu_names_string = String::new();
    let mut storage_free_string = String::new();
    let mut storage_total_string = String::new();

    //populate string arrays
    for cpu in segmented_cpu{
        cpu_frequency_string += &*(cpu.frequency().to_string().to_owned() + ";");
        cpu_names_string += &*(cpu.name().to_string().to_owned() + ";");
        cpu_usage_string += &*(cpu.cpu_usage().to_string().to_owned() + ";");
    }
    for disk in &disks{
        storage_free_string += &*(disk.available_space().to_string().to_owned() + ";");
        storage_total_string += &*(disk.total_space().to_string().to_owned() + ";");
    }
    let logical_core_count = segmented_cpu.len();

    //strip final delimiter.
    cpu_frequency_string = cpu_frequency_string.strip_suffix(";").unwrap_or("ERROR").to_string();
    cpu_names_string = cpu_names_string.strip_suffix(";").unwrap_or("ERROR").to_string();
    cpu_usage_string = cpu_usage_string.strip_suffix(";").unwrap_or("ERROR").to_string();
    storage_free_string = storage_free_string.strip_suffix(";").unwrap_or("ERROR").to_string();
    storage_total_string = storage_total_string.strip_suffix(";").unwrap_or("ERROR").to_string();

    //encapsulate strings in brackets for easy parsing later on. Please note that these "arrays" may be any arbitrary length
    cpu_frequency_string = "[".to_string() + &*cpu_frequency_string.to_owned() + "]";
    cpu_names_string = "[".to_string() + &*cpu_names_string.to_owned() + "]";
    cpu_usage_string = "[".to_string() + &*cpu_usage_string.to_owned() + "]";
    storage_free_string = "[".to_string() + &*storage_free_string.to_owned() + "]";
    storage_total_string = "[".to_string() + &*storage_total_string.to_owned() + "]";

    let mut response_string = String::new();
    response_string += &*("ram_total=".to_string() + &*ram_total.to_string());
    response_string += &*(",ram_used=".to_string() + &*ram_used.to_string());
    response_string += &*(",swap_total=".to_string() + &*swap_total.to_string());
    response_string += &*(",swap_used=".to_string() + &*swap_used.to_string());
    response_string += &*(",uptime=".to_string() + &*uptime.to_string());
    response_string += &*(",cpu_arch=".to_string() + &*cpu_arch);
    response_string += &*(",name=".to_string() + &*name);
    response_string += &*(",host=".to_string() + &*hostname);
    response_string += &*(",physical_core_count=".to_string() + &*physical_core_count.to_string());
    response_string += &*(",logical_core_count=".to_string() + &*logical_core_count.to_string());
    response_string += &*(",verbose_os_version=".to_string() + &*verbose_os_version);
    response_string += &*(",global_cpu_usage=".to_string() + &*global_cpu_usage.to_string());
    response_string += &*(",frequency=".to_string() + &*cpu_frequency_string);
    response_string += &*(",core_usage=".to_string() + &*cpu_usage_string);
    response_string += &*(",core_names=".to_string() + &*cpu_names_string);
    response_string += &*(",free_storage=".to_string() + &*storage_free_string);
    response_string += &*(",total_storage=".to_string() + &*storage_total_string);

    if newlines{
        return response_string.replace(",", "\n");
    }
    response_string
}

fn main(){
    println!("hello, World!");

    rouille::start_server("0.0.0.0:6900", move |request| {
        println!("received request to {}", request.url());
        let mut sys = System::new_all();
        sys.refresh_all();
        match request.url().to_ascii_lowercase().as_str() {
            "/get_all_stats_csv" =>{
                Response::text(all_data(false, &mut sys))
            },
            "/get_all_stats" =>{
                Response::text(all_data(true, &mut sys))
            },
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
                let _ = sys.global_cpu_usage();
                // Wait a bit because CPU usage is based on diff.
                std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
                // Refresh CPUs again to get actual value.
                sys.refresh_cpu_usage();
                let response_string = sys.global_cpu_usage().to_string();
                Response::text(response_string)
            },
            "/segmented_cpu_usage" =>{
                let _ = sys.cpus();
                // Wait a bit because CPU usage is based on diff.
                std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
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
                let system_quantity = System::uptime();
                let response_string = system_quantity.to_string();
                Response::text(response_string)
            },
            "/hostname" =>{
                let system_quantity = System::host_name();
                let response_string = system_quantity.unwrap_or("ERROR".to_string());
                Response::text(response_string)
            },
            "/physical_core_count" =>{
                let system_quantity = sys.physical_core_count().unwrap_or(0usize);
                let response_string = system_quantity.to_string();
                Response::text(response_string)
            },
            "/name" =>{
                let response_string = System::name().unwrap_or("ERROR".to_string());
                Response::text(response_string)
            },
            "/verbose_os_version" =>{
                let response_string = System::long_os_version().unwrap_or("ERROR".parse().unwrap());
                Response::text(response_string)
            },
            "/cpu_arch" =>{
                let response_string = System::cpu_arch();
                Response::text(response_string)
            },
            "/free_storage" =>{
                let system_quantity = Disks::new_with_refreshed_list();
                let mut response_string = "".to_string();
                for disk in system_quantity.list() {
                    let free_space = disk.available_space();
                    println!("[{:?}] {}B", disk.name(), free_space);
                    let disk_name = disk.name().to_str().unwrap_or("ERROR");
                    response_string += match disk_name.to_string().as_str(){
                        ""=>"NoDiskName",
                        _ => disk_name,
                    };
                    response_string += "=>";
                    response_string += free_space.to_string().as_str();
                    response_string += "B,";
                }
                Response::text(response_string)
            },
            "/total_storage" =>{
                let system_quantity = Disks::new_with_refreshed_list();
                let mut response_string = "".to_string();
                for disk in system_quantity.list() {
                    let total_space = disk.total_space();
                    println!("[{:?}] {}B", disk.name(), total_space);
                    let disk_name = disk.name().to_str().unwrap_or("ERROR");
                    response_string += match disk_name.to_string().as_str(){
                        ""=>"NoDiskName",
                        _ => disk_name,
                    };
                    response_string += "=>";
                    response_string += total_space.to_string().as_str();
                    response_string += "B,";
                }
                Response::text(response_string)
            },
            "/cpu_frequency" =>{
                let _ = sys.cpus();
                // Wait a bit because CPU usage is based on diff.
                std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
                // Refresh CPUs again to get actual value.
                sys.refresh_cpu_usage();
                let mut response_string = "".to_string();
                for cpu in sys.cpus() {
                    println!("{}%", cpu.cpu_usage());
                    response_string += (cpu.frequency().to_string().as_str().to_owned() + "MHz,").as_str();
                }
                Response::text(response_string)
            },
            "/owo" =>{
                Response::text("UwU")
            },
            "/uwu" =>{
                Response::text("OwO")
            },
            "/units" =>{
                Response::text("OwO:UwU (String), UwU:OwO (String), Ram:Bytes (Int), Swap:Bytes (Int), Storage:Bytes (Int), CPU Usage:Percent (Float), Uptime:Seconds (Int), Counts:Quantity (Int), Frequency:MHz (Float), Storage:Bytes (Int)")
            },
            "/author" =>{
                Response::text("Created by Awesome_Tornado_ on GitHub, __Choco__ on Resonite. \nCC-BY-NC License, keeping this http endpoint message, and keeping any documentation that it exists (eg: the /index endpoint) will be considered appropriate attribution")
            },
            "/index" => {
                Response::text("/ram_total\n/ram_used\n/swap_total\n/swap_used\n/cpu_total\n/global_cpu_usage\n/uptime\n/owo\n/uwu\n/segmented_cpu_usage\n/physical_core_count\n/name\n/verbose_os_version\n/cpu_arch\n/cpu_frequency\n/free_storage\n/total_storage\n/get_all_stats\n/get_all_stats_csv\n/units\n/author\n/hostname")
            }
            _ =>{
                Response::empty_404()
            }
        }
    });
}