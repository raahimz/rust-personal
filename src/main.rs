use std::process::Command;

#[cfg(target_os = "macos")]
fn main() {
    let mut ssid = String::from("NA");
    let mut ip = String::from("NA");
    let mut bssid = String::from("NA");
    let mut frequency = String::from("NA");
    let mut rssi = String::from("NA");
    let mut speed = String::from("NA");

    // Getting Signal Level
    let output = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
        .arg("-I")
        .arg("|")
        .arg("grep")
        .arg("CtlRSSI")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8(output.stdout).expect("invalid utf8");
    let output: Vec<&str> = output.trim().split(":").collect();
    let output: Vec<&str> = output[1].trim().split("\n").collect();
    rssi = String::from(output[0].trim());
    
    // Getting rest of the information
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

    println!("SSID: {ssid} | BSSID: {bssid} | IP: {ip} | Frequency: {frequency} | RSSI: {rssi} | Speed: {speed}");
}

 #[cfg(target_os = "linux")]
 fn main() {
    let mut ssid = String::from("");
    let mut ip = String::from("");
    let mut bssid = String::from("");
    let mut frequency = String::from("");
    let mut rssi = String::from("");
    let mut speed = String::from("");

    // Getting IP
    let output = Command::new("hostname")
        .arg("-I")
        .output()
        .expect("failed to execute process");
        
    ip = String::from_utf8(output.stdout).expect("invalid utf8");

    // Getting rest of the data
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

        // Getting RSSI (dBm)
        if key_value_equals[0].trim() == "level" {
            let mut value = String::from(key_value_equals[1]);
            rssi = String::from(value.trim());
        }

        // Getting Speed (MB/s)
        if key_value_equals[0].trim() == "Rate" {
            let mut value = String::from(key_value_equals[1]);
            speed = String::from(value.trim());
        }

    }
    
    println!("SSID: {ssid} | BSSID: {bssid} | IP: {ip} | Frequency: {frequency} | RSSI: {rssi} | Speed: {speed}");
}

 #[cfg(target_os = "windows")]
 fn main() {
    let mut ssid = String::from("NA");
    let mut ip = String::from("NA");
    let mut bssid = String::from("NA");
    let mut frequency = String::from("NA");
    let mut rssi = String::from("NA");
    let mut speed = String::from("NA");

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

    // Getting rest of the information
    let output = Command::new("netsh")
        .arg("wlan")
        .arg("show")
        .arg("interfaces")
        .output()
        .expect("failed to execute process");
        
    let output = String::from_utf8(output.stdout).expect("invalid utf8");
    let output: Vec<&str> = output.trim().split("\n").collect();

    for word in output {
        let key_value: Vec<&str> = word.trim().split(":").collect();

        // Getting SSID
        if key_value[0].trim() == "SSID" {
            ssid = String::from(key_value[1].trim());
        }

        // Getting BSSID
        if key_value[0].trim() == "BSSID" {
            bssid = format!("{}:{}:{}:{}:{}:{}", key_value[1].trim(), key_value[2].trim(), key_value[3].trim(), key_value[4].trim(), key_value[5].trim(), key_value[6].trim());
        }

        // Getting Speed (MB/s)
        if key_value[0].trim() == "Receive rate (Mbps)" {
            let value = key_value[1].trim().parse::<i32>().unwrap();
            
            // Converting Mbps to MB/s
            let value = value / 8;

            speed = value.to_string();
        }

        // Getting RSSI (dBm)
        if key_value[0].trim() == "Signal" {
            let value: Vec<&str> = key_value[1].trim().split("%").collect();
            let value = value[0].parse::<i32>().unwrap();
            
            // Converting Percentage (%) to dBm
            let value = (value / 2) - 100;

            rssi = value.to_string();
        }
    }

    // println!("{:?}", output);
    
    println!("SSID: {ssid} | BSSID: {bssid} | IP: {ip} | Frequency: {frequency} | RSSI: {rssi} | Speed: {speed}");
}