use rouille::Request;
use rouille::Response;
use sysinfo::{
    Components, Disks, Networks, System,
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
                let response_string = system_quantity.to_string();
                Response::text(response_string)
            },
            "/uptime" =>{
                let system_quantity = sysinfo::System::uptime();
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
                Response::text("/ram_total, /ram_used, /swap_total, /swap_used, /cpu_total, /global_cpu_usage")
            }
            _ =>{
                Response::empty_404()
            }
        }
    });
}