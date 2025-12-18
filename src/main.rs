use clap::Parser;
use colored::*;
use std::net::{IpAddr, TcpStream, ToSocketAddrs};
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// ==========================================
// CONFIGURATION CLI
// ==========================================

#[derive(Parser, Debug)]
#[command(name = "rustrecon", author, version, about = "Fast multi-threaded port scanner", long_about = None)]
struct Args {
    /// Target IP or Hostname (e.g., 127.0.0.1 or google.com)
    target: String,

    /// Start port
    #[arg(short, long, default_value_t = 1)]
    start: u16,

    /// End port
    #[arg(short, long, default_value_t = 1024)]
    end: u16,

    /// Number of parallel threads (speed)
    #[arg(short, long, default_value_t = 100)]
    threads: u16,

    /// Timeout per port in milliseconds
    #[arg(long, default_value_t = 500)]
    timeout: u64,
}

// ==========================================
// MAIN LOGIC
// ==========================================

fn main() {
    let args = Args::parse();

    // 1. Résolution DNS de la cible
    let target_ip = match resolve_host(&args.target) {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("{} {}", "[ERROR]".red().bold(), e);
            process::exit(1);
        }
    };

    // CORRECTION ICI : Ajout de "{}" dans les println!
    println!("{}", "\n========================================".green());
    println!("   RUST RECON - OFFENSIVE/DEFENSIVE TOOL   ");
    println!("{}", "========================================".green());
    
    println!("Target:  {}", target_ip.to_string().cyan().bold());
    println!("Range:   {}-{}", args.start, args.end);
    println!("Threads: {}", args.threads);
    println!("Scanning...\n");

    let start_time = Instant::now();

    // 2. Préparation du port partagé (Atomic Counter pattern)
    let current_port = Arc::new(Mutex::new(args.start));
    let mut handles = vec![];

    // 3. Lancement des threads
    for _ in 0..args.threads {
        let current_port = Arc::clone(&current_port);
        let end_port = args.end;
        let timeout = Duration::from_millis(args.timeout);
        let ip = target_ip; 

        let handle = thread::spawn(move || {
            loop {
                // On récupère le prochain port à scanner
                let port = {
                    let mut num = current_port.lock().unwrap();
                    if *num > end_port {
                        break; // Fini
                    }
                    let p = *num;
                    *num += 1;
                    p
                };

                // Scan du port
                if is_port_open(ip, port, timeout) {
                    let service = get_common_service_name(port);
                    println!(
                        "[{}] Port {} ({}) is {}", 
                        "+".green().bold(), 
                        port.to_string().yellow().bold(),
                        service.cyan(),
                        "OPEN".green().bold()
                    );
                }
            }
        });

        handles.push(handle);
    }

    // 4. Attente de la fin de tous les threads
    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start_time.elapsed();
    println!("{}", "\n========================================".green());
    println!("Scan complete in {:.2?}", duration);
}

// ==========================================
// NETWORKING HELPERS
// ==========================================

fn resolve_host(host: &str) -> Result<IpAddr, String> {
    if let Ok(ip) = host.parse::<IpAddr>() {
        return Ok(ip);
    }
    
    let addr_string = format!("{}:80", host);
    match addr_string.to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(socket) => Ok(socket.ip()),
            None => Err("Could not resolve hostname".to_string()),
        },
        Err(_) => Err("DNS resolution failed".to_string()),
    }
}

fn is_port_open(ip: IpAddr, port: u16, timeout: Duration) -> bool {
    let socket_addr = std::net::SocketAddr::new(ip, port);
    match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn get_common_service_name(port: u16) -> &'static str {
    match port {
        21 => "FTP",
        22 => "SSH",
        23 => "Telnet",
        25 => "SMTP",
        53 => "DNS",
        80 => "HTTP",
        110 => "POP3",
        135 => "RPC",
        139 => "NetBIOS",
        143 => "IMAP",
        443 => "HTTPS",
        445 => "SMB",
        1433 => "MSSQL",
        3306 => "MySQL",
        3389 => "RDP",
        5432 => "PostgreSQL",
        6379 => "Redis",
        8080 => "HTTP-Alt",
        _ => "Unknown",
    }
}