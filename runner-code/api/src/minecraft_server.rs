use std::process::Command;

#[derive(Debug)]
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

pub struct Options {
    pub gamemode: Gamemode,
    pub difficulty: Difficulty,
    pub world_type: WorldType,
    pub hardcore: bool,
    pub pvp: bool,
    pub seed: i64,
    pub port: i64
}

pub struct Server {
    pub uuid: String,
    pub options: Options
}

impl Server {
    pub fn new(options: Options, file_id: String) -> Self {
        let uuid = Server::download_data(&file_id);
        Server::run(&options, &file_id);
        let server = Server { uuid, options };
        server.print();
        server
    }

    fn run(options: &Options, file_id: &str) -> String {
        let output = Command::new("docker")
                .arg("run")
                .arg("-d")
                .arg("-it")
                .arg("-v")
                .arg(format!("/minecraft/{}", file_id))
                .arg("-p")
                .arg(format!("{}:25565", options.port))
                .arg("-e")
                .arg("EULA=TRUE")
                .arg("-e")
                .arg("MOTD=A Minecraft Server")
                .arg("-e")
                .arg(format!("DIFFICULTY={:?}", options.difficulty))
                .arg("-e")
                .arg(format!("HARDCORE={:?}", options.hardcore))
                .arg("-e")
                .arg(format!("SEED=\"{:?}\"", options.seed))
                .arg("-e")
                .arg(format!("MODE={:?}", options.gamemode))
                .arg("-e")
                .arg(format!("PVP={:?}", options.pvp))
                .arg("-e")
                .arg(format!("LEVEL_TYPE={:?}", options.world_type))
                .arg("-e")
                .arg("SPAWN_PROTECTION=0")
                .arg("-e")
                .arg("MAX_PLAYERS=10")
                .arg("itzg/minecraft-server")
                .output()
                .expect("docker: Error");
        String::from_utf8_lossy(&output.stdout).chars().take(12).collect::<String>()
    }

    fn print(&self) {
        println!("Server UUID: {}", self.uuid);
        println!("  - Gamemode: {:?}", self.options.gamemode);
        println!("  - Difficulty: {:?}", self.options.difficulty);
        println!("  - WorldType: {:?}", self.options.world_type);
        println!("  - Hardcore: {:?}", self.options.hardcore);
        println!("  - PVP: {:?}", self.options.pvp);
        println!("  - Seed: {:?}", self.options.seed);
    }

    fn download_data(file_id: &str) -> String {
        let output = Command::new("curl")
            .arg(format!("127.0.0.1:8080/{}", file_id))
            .arg("-o")
            .arg(format!("{}.zst", file_id))
            .output()
            .expect("curl: Error");
        String::from_utf8_lossy(&output.stdout).to_string()
    }

    fn gate_create_host(&self) {
        let output = Command::new("curl")
            .arg("-X")
            .arg("POST")
            .arg("-d")
            .arg(format!("host={}.instantmc.gg&backend=127.0.0.1:{}", self.uuid, self.options.port))
            .arg("http://127.0.0.1:3000/createhost")
            .output()
            .expect("curl: Error");
    }
    
    fn gate_delete_host(&self) {
        let output = Command::new("curl")
            .arg("-X")
            .arg("DELETE")
            .arg(format!("http://127.0.0.1:3000/deletehost/{}.instantmc.gg", self.uuid))
            .output()
            .expect("curl: Error");
    }
}