use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "server")]
    pub http: Option<ServerConfig>,
    pub log: LogModuleConfig
}

#[derive(Deserialize, Debug)]
pub struct LogModuleConfig {
    pub file: LogConfig,
    pub console: LogConfig
}

// Configuration for the built-in webserver
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ServerConfig {
    HttpTls {
        #[serde(default)]
        port: HttpTlsPort,
        cert: String,
        key: String
    },
    Http {
        #[serde(default)]
        port: HttpPort
    }
}

#[derive(Deserialize, Debug)]
pub struct HttpPort(pub u16);
impl Default for HttpPort {
    fn default() -> Self {
        HttpPort(80)
    }
}

#[derive(Deserialize, Debug)]
pub struct HttpTlsPort(pub u16);
impl Default for HttpTlsPort {
    fn default() -> Self {
        HttpTlsPort(8080)
    }
}

#[derive(Deserialize, Debug)]
pub struct LogConfig {
    pub level: String
}
impl Default for LogConfig {
    fn default() -> Self {
        LogConfig { level: String::from("INFO") }
    }
}

impl Default for Config {
    fn default() -> Self {
       Config {
           http: Some(ServerConfig::Http { port: HttpPort(80) }),
           log: LogModuleConfig {
               file: Default::default(),
               console: Default::default()
           }
       } 
    }
}
