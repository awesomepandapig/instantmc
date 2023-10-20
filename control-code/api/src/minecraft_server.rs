use std::process::Command;
use std::net::{IpAddr, Ipv4Addr};
use crate::dns;

#[derive(Debug)]
// Define enums and Server struct
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator
}

#[derive(Debug)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard
}

#[derive(Debug)]
pub enum WorldType {
    Normal,
    Flat,
    LargeBiomes
}

pub struct Server {
    pub uuid: String,
    pub gamemode: Gamemode,
    pub difficulty: Difficulty,
    pub world_type: WorldType,
    pub hardcore: bool,
    pub pvp: bool,
    pub seed: i64,
    pub port: i64
}

impl Server {

    pub fn new(
        gamemode: Gamemode,
        difficulty: Difficulty,
        world_type: WorldType,
        hardcore: bool,
        pvp: bool,
        seed: i64,
        port: i64
    ) -> Self {
        let uuid = Self::start(
            &gamemode,
            &difficulty,
            &world_type,
            hardcore,
            pvp,
            seed,
            port,
        );
        let server = Server {
            uuid,
            gamemode,
            difficulty,
            world_type,
            hardcore,
            pvp,
            seed,
            port
        };
        server.print();
        let _ = dns::set_a(format!("{}.{}.", server.uuid, "instantmc.gg").as_str(), 300, IpAddr::V4(Ipv4Addr::new(73, 157, 184, 122)));
        
        Command::new("curl")
            .arg("-X")
            .arg("POST")
            .arg("-d")
            .arg(format!("host={}.instantmc.gg&backend=127.0.0.1:{}", server.uuid, server.port))
            .arg("http://127.0.0.1:8080/createhost")
            .output()
            .expect("Failed to execute command");

        server
    }

    pub fn start(
        gamemode: &Gamemode,
        difficulty: &Difficulty,
        world_type: &WorldType,
        hardcore: bool,
        pvp: bool,
        seed: i64,
        port: i64
    ) -> String {
        let output = Command::new("docker")
            .arg("run")
            .arg("-d")
            .arg("-it")
            .arg("-p")
            .arg(format!("{}:25565", port))
            .arg("-e")
            .arg("EULA=TRUE")
            .arg("-e")
            .arg("MOTD=A Minecraft Server")
            .arg("-e")
            .arg(format!("DIFFICULTY={:?}", difficulty))
            .arg("-e")
            .arg(format!("HARDCORE={:?}", hardcore))
            .arg("-e")
            .arg(format!("SEED=\"{:?}\"", seed))
            .arg("-e")
            .arg(format!("MODE={:?}", gamemode))
            .arg("-e")
            .arg(format!("PVP={:?}", pvp))
            .arg("-e")
            .arg(format!("LEVEL_TYPE={:?}", world_type))
            .arg("-e")
            .arg("SPAWN_PROTECTION=0")
            .arg("-e")
            .arg("MAX_PLAYERS=10")
            .arg("itzg/minecraft-server")
            .output()
            .expect("docker: Error");
        String::from_utf8_lossy(&output.stdout).chars().take(12).collect::<String>()
    }

    // Getters
    pub fn print(&self) {
        println!("Server UUID: {}", self.uuid);
        println!("  - Gamemode: {:?}", self.gamemode);
        println!("  - Difficulty: {:?}", self.difficulty);
        println!("  - WorldType: {:?}", self.world_type);
        println!("  - Hardcore: {:?}", self.hardcore);
        println!("  - PVP: {:?}", self.pvp);
        println!("  - Seed: {:?}", self.seed);
    }
}
