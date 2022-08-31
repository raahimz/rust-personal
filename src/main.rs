use std::process::Command;

#[cfg(target_os = "macos")]
fn main() {
    let mut ssid = String::from("");
    let mut ip = String::from("");
    let mut bssid = String::from("");
    
    let output = Command::new("ipconfig")
        .arg("getsummary")
        .arg("en0")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8(output.stdout).expect("invalid utf8");
    let output: Vec<&str> = output.split("\n").collect();

    for word in output {
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
    let mut frequency = String::from("");
    let mut signal_level = String::from("");

    // Getting IP
    let output = Command::new("hostname")
        .arg("-I")
        .output()
        .expect("failed to execute process");
        
    ip = String::from_utf8(output.stdout).expect("invalid utf8");

    let output = Command::new("iwconfig")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8(output.stdout).expect("invalid utf8");
    let output: Vec<&str> = output.trim().split(" ").collect();

    for (i, word) in output.iter().enumerate() {
        let key_value_colon: Vec<&str> = word.trim().split(":").collect();
        let key_value_equals: Vec<&str> = word.trim().split("=").collect();

        // Getting SSID
        if key_value_colon[0].trim() == "ESSID" {
            let mut value = String::from(key_value_colon[1]);
            value.remove(0);
            let value: Vec<&str> = value.split("\"").collect();
            ssid = String::from(value[0].trim());
        }

        // Getting BSSID
        if key_value_colon[0].trim() == "Point" {
            let mut value = String::from(output[i+1]);
            bssid = String::from(value.trim());
        }

        // Getting Frequency (GHz)
        if key_value_colon[0].trim() == "Frequency" {
            let mut value = String::from(key_value_colon[1]);
            frequency = String::from(value.trim());
        }

        // Getting Level (dBm)
        if key_value_equals[0].trim() == "level" {
            let mut value = String::from(key_value_equals[1]);
            signal_level = String::from(value.trim());
        }
    }
    // println!("{:?}", output);

    println!("SSID: {ssid} | BSSID: {bssid} | IP: {ip} | Frequency: {frequency} | Level: {signal_level}");
}

#[cfg(target_os = "windows")]
 fn main() {
    let mut ssid = String::from("");
    let mut ip = String::from("");
    let mut bssid = String::from("");
    let mut frequency = String::from("");
    let mut signal_level = String::from("");

    // Getting IP
    let output = Command::new("ipconfig")
        .output()
        .expect("failed to execute process");
        
    let output = String::from_utf8(output.stdout).expect("invalid utf8");
    let output: Vec<&str> = output.trim().split("\n").collect();

    for word in output {
        let key = word.trim();

        if key.contains("IPv4 Address") {
            let value: Vec<&str> = word.split(":").collect();
            let value: Vec<&str> = value[1].split("\r").collect();

            ip = String::from(value[0].trim());
        }
    }
    // ip = String::from(output[1]);

    // let output = Command::new("iwconfig")
    //     .output()
    //     .expect("failed to execute process");

    // let output = String::from_utf8(output.stdout).expect("invalid utf8");
    // let output: Vec<&str> = output.trim().split(" ").collect();

    // for (i, word) in output.iter().enumerate() {
    //     let key_value_colon: Vec<&str> = word.trim().split(":").collect();
    //     let key_value_equals: Vec<&str> = word.trim().split("=").collect();

    //     // Getting SSID
    //     if key_value_colon[0].trim() == "ESSID" {
    //         let mut value = String::from(key_value_colon[1]);
    //         value.remove(0);
    //         let value: Vec<&str> = value.split("\"").collect();
    //         ssid = String::from(value[0].trim());
    //     }

    //     // Getting BSSID
    //     if key_value_colon[0].trim() == "Point" {
    //         let mut value = String::from(output[i+1]);
    //         bssid = String::from(value.trim());
    //     }

    //     // Getting Frequency (GHz)
    //     if key_value_colon[0].trim() == "Frequency" {
    //         let mut value = String::from(key_value_colon[1]);
    //         frequency = String::from(value.trim());
    //     }

    //     // Getting Level (dBm)
    //     if key_value_equals[0].trim() == "level" {
    //         let mut value = String::from(key_value_equals[1]);
    //         signal_level = String::from(value.trim());
    //     }
    // }
    // println!("{:?}", output);

    println!("SSID: {ssid} | BSSID: {bssid} | IP: {ip} | Frequency: {frequency} | Level: {signal_level}");
}