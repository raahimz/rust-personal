use std::process::Command;

#[cfg(target_os = "macos")]
fn main() {
    let output = Command::new("ipconfig")
        .arg("getsummary")
        .arg("en0")
        .output()
        .expect("failed to execute process");

    let mut ssid = String::from("");
    let mut ip = String::from("");
    let mut bssid = String::from("");

    let output = String::from_utf8(output.stdout).expect("invalid utf8");
    // dbg!(one_liner(output.as_str()));
    // let mut words = Vec::new();
    let words = output.split("\n");
    let vec: Vec<&str> = words.collect();

    for word in vec {
        let key_value: Vec<&str> = word.trim().split(":").collect();
        
        // Getting SSID
        if key_value[0].trim() == "SSID" {
            ssid = String::from(key_value[1].trim());
        }

        // Getting BSSID
        if key_value[0].trim() == "BSSID" {
            let bssid_key_value: Vec<&str> = word.trim().split(" ").collect();
            
            bssid = String::from(bssid_key_value[2]);
        }

        // Getting IP Address
        let ip_key_value: Vec<&str> = key_value[0].trim().split("=").collect();

        if ip_key_value[0].trim() == "yiaddr" {
            ip = String::from(ip_key_value[1].trim());
        }
    }

    println!("SSID: {ssid} | BSSID: {bssid} | IP: {ip}");
}

 #[cfg(target_os = "linux")]
 fn main() {
    let mut ssid = String::from("");
    let mut ip = String::from("");
    let mut bssid = String::from("");

    // Getting IP
    let output = Command::new("hostname")
        .arg("-I")
        .output()
        .expect("failed to execute process");
        
    ip = String::from_utf8(output.stdout).expect("invalid utf8");

    // Getting SSID
    let output = Command::new("iwgetid")
        .arg("wlan0")
        .arg("--raw")
        .output()
        .expect("failed to execute process");
        
    ssid = String::from_utf8(output.stdout).expect("invalid utf8");

    
    println!("SSID: {ssid} | BSSID: {bssid} | IP: {ip}");
}