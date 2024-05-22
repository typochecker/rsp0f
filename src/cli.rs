use clap::Parser;

#[derive(Parser, Debug)] // Add this line
pub struct Cli {
    // 注意下面的注释是三个斜杠!!!
    /// use which method
    #[arg(short, long)]
    pub method: Option<String>,

    /// Optional name to call
    pub name: Option<String>,

    /// reads fingerprint database (p0f.fp) from the specified location.
    ///
    /// See section 5 for more information about the contents of this file.
    ///
    /// The default location is ./p0f.fp. If you want to install p0f, you may want to change FP_FILE in config.h to /etc/p0f.fp.
    #[arg(short, long = "fp", default_value = "./p0f.fp")]
    pub fp: Option<String>,

    /// asks p0f to listen on a specific network interface.
    ///
    /// On un*x, you should reference the interface by name (e.g., eth0). On Windows,
    ///
    /// you can use adapter index instead (0, 1, 2...).
    ///
    /// Multiple -i parameters are not supported; you need to run
    ///
    /// separate instances of p0f for that. On Linux, you can specify
    ///
    /// 'any' to access a pseudo-device that combines the traffic on
    /// all other interfaces; the only limitation is that libpcap will
    ///
    /// not recognize VLAN-tagged frames in this mode, which may be
    /// an issue in some of the more exotic setups.
    ///
    /// If you do not specify an interface, libpcap will probably pick
    /// the first working interface in your system.
    #[arg(short, long)]
    pub interface: Option<String>,

    /// lists all available network interfaces, then quits.
    ///
    /// Particularly useful on Windows, where the system-generated interface names
    /// are impossible to memorize.
    //#[arg(short, long, default_value = "false")]
    #[arg(short = 'L', default_value = "false")]
    pub laani_flag: bool,

    /// instead of listening for live traffic, reads pcap captures from the specified file.
    ///
    /// The data can be collected with tcpdump or any
    /// other compatible tool. Make sure that snapshot length (-s
    /// option in tcpdump) is large enough not to truncate packets; the
    /// default may be too small.
    ///
    /// As with -i, only one -r option can be specified at any given time.
    #[arg(short = 'r', long)]
    pub pcap: Option<String>,

    /// appends grep-friendly log data to the specified file.
    ///
    /// The log contains all observations made by p0f about every matching
    /// connection, and may grow large; plan accordingly.
    ///
    /// Only one instance of p0f should be writing to a particular file
    /// at any given time; where supported, advisory locking is used to avoid problems.
    #[arg(short, long)]
    pub output: Option<String>,

    /// listens for API queries on the specified filesystem socket.
    ///
    /// This allows other programs to ask p0f about its current thoughts about
    /// a particular host. More information about the API protocol can be
    /// found in section 4 below.
    ///
    /// Only one instance of p0f can be listening on a particular socket
    /// at any given time. The mode is also incompatible with -r.
    #[arg(short, long)]
    pub socket: Option<String>,

    /// runs p0f in daemon mode
    ///
    /// the program will fork into background
    /// and continue writing to the specified log file or API socket. It
    /// will continue running until killed, until the listening interface
    /// is shut down, or until some other fatal error is encountered.
    ///
    /// This mode requires either -o or -s to be specified.
    ///
    /// To continue capturing p0f debug output and error messages (but
    /// not signatures), redirect stderr to another non-TTY destination,
    /// e.g.:
    ///
    /// ./p0f -o /var/log/p0f.log -d 2>>/var/log/p0f.error
    ///
    /// Note that if -d is specified and stderr points to a TTY, error
    /// messages will be lost.
    #[arg(short, long)]
    pub daemon: bool,

    /// causes p0f to drop privileges, switching to the specified user
    /// and chroot()ing itself to said user's home directory.
    ///
    /// This mode is *highly* advisable (but not required) on un*x
    /// systems, especially in daemon mode. See section 7 for more info.
    #[arg(short, long)]
    pub user: Option<String>,

    /// puts the interface specified with -i in promiscuous mode.
    ///
    /// If supported by the firmware, the card will also process frames not addressed to it.
    #[arg(short, long)]
    pub promisc: bool,
}
