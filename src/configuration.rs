use std::fs;
use serde::Deserialize;
use crate::constants::{SCORE_MAX, TOP_SCORE_COUNT};


//cinfiguration / settings of the game
#[derive(Clone,Deserialize,PartialEq)]
pub struct Configuration {
    pub vsync: bool,
    pub display_index: i32,
    pub display_mode: i32,
    pub stars: i32,
    pub volume: i32,
    pub pause_on_hit: f32,
    pub levels: Vec<String>,
    pub score_table: Vec<i32>
}

impl Configuration {

    //default configuration if "config.toml" is not found
    pub fn default() -> Configuration {
        let levels = vec![
            "FFFFFFFFFF FFFFFFFFFF FFFFFFFFFF FFFFFFFFFF FFFFFFFFFF ".to_string(),
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF".to_string(),
            "HHHHHHHHHH FFFFFFFFFF FFFFFFFFFF FFFFFFFFFF FFFFFFFFFF ".to_string(),
            "HHHHHHHHHHHFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF".to_string(),
            "HHHHHHHHHHHHHHHHHHHHHHFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF".to_string(),
            "HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHFFFFFFFFFFFFFFFFFFFFFF".to_string(),
            "HFHFHFHFHFHFHFHFHFHFHFHFHFHFHFHFHFHFHFHFHFHFHFHFHFHFHFH".to_string(),
            "FFFFFFFFFFFFHHHHHHHHHFFHHHHHHHHHFFHHHHHHHHHFFFFFFFFFFFF".to_string(),
            "HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH".to_string(),
            "HHHHHHHHHHHHHHHHHHHHHHHDHDHDHDHDHFFFFFFFFFFFFFFFFFFFFFF".to_string(),
            "HHHHHHHHHHHHHHHHHHHHHHDDDDDDDDDDDFFFFFFFFFFFFFFFFFFFFFF".to_string(),
            "HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHDDDDDDDDDDDFFFFFFFFFFF".to_string(),
            "HHHHHHHHHHHHHHHHHHHHHHHDHDHDHDHDHDFDFDFDFDFDFFFFFFFFFFF".to_string(),
            "HHHHHHHHHHHHHHHHHHHHHHHFHFHFHFHFHFHFHFHFHFHFDDDDDDDDDDD".to_string(),
            "HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHDFFFFFFFFFDDDDDDDDDDDD".to_string(),
            "HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHFDFDFDFDFDFDDDDDDDDDDD".to_string(),
            "HHHHHHHHHHHHHHHHHHHHHHDHHHHHHHHHDDFFFFFFFFFDDDDDDDDDDDD".to_string(),
            "DHHHHHHHHHDDHHHHHHHHHDDHHHHHHHHHDDFFFFFFFFFDDFFFFFFFFFD".to_string(),
            "BHBHBHBHBHBHHHHHHHHHHHHHHHHHHHHHHFFFFFFFFFFFFFFFFFFFFFF".to_string(),
            "BBBBBBBBBBBHHHHHHHHHHHHFHFHFHFHFHFFFFFFFFFFFFFFFFFFFFFF".to_string(),
            "BBBBBBBBBBBHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHFFFFFFFFFFF".to_string(),
            "BBBBBBBBBBBHHHHHHHHHHHHHHHHHHHHHHHFHFHFHFHFHDFDFDFDFDFD".to_string(),
            "BBBBBBBBBBBHHHHHHHHHHHHHHHHHHHHHHFDFDFDFDFDFDFDFDFDFDFD".to_string(),
            "BBBBBBBBBBBHHHHHHHHHHHHHHHHHHHHHHFHFHFHFHFHFDDDDDDDDDDD".to_string(),
            "HHHHHHHHHHHHHHHBBBHHHHHHHHBBBHHHHFFFFBBBFFFFDDDDDDDDDDD".to_string(),
            "HHHBBBBBHHHHHHBBBBBHHHFFFBBBBBFFFFFFFFFFFFFFFFFDDDDDFFF".to_string(),
            "FDFDBBBDFDFDFDFBBBFDFDFDFDBBBDFDFDFDFDFDFDFDFDFDFDFDFDF".to_string(),
            "BBBBBBBBBBBHHHHHHHHHHHDHHHHHHHHHDDFFFFFFFFFDDDDDDDDDDDD".to_string(),
            "DBBBBBBBBBDDHHHHHHHHHDDHHHHHHHHHDDFFFFFFFFFDDFFFFFFFFFD".to_string(),
            "BBBBBBBBBBBHSHSHSHSHSHHHHHHHHHHHHFFFFFFFFFFFDDDDDDDDDDD".to_string(),
            "BBBBBBBBBBBSSSSSSSSSSSHHHHHHHHHHHFFFFFFFFFFFDDDDDDDDDDD".to_string(),
            "BBBBBBBBBBBSSSSSSSSSSSHSHSHSHSHSHHHHHHHHHHHHDDDDDDDDDDD".to_string(),
            "BBBBBBBBBBBHSHSHSHSHSHHSHSHSHSHSHFDFDFDFDFDFFFFFFFFFFFF".to_string(),
            "HBHBHBHBHBHHSHSHSHSHSHHDHDHDHDHDHFDFDFDFDFDFHFHFHFHFHFH".to_string(),
            "BBBBBBBBBBBBHBHBHBHBHBSSSSSSSSSSSDFDFDFDFDFDDDDDDDDDDDD".to_string(),
            "SBSBSBSBSBSDFDFDFDFDFDBSBSBSBSBSBFDFDFDFDFDFDDDDDDDDDDD".to_string(),
            "HSHSHSHSHSHHSHSHSHSHSHHSHSHSHSHSHFDFDFDFDFDFFDFDFDFDFDF".to_string(),
            "BSBSBSBSBSBHSHSHSHSHSHHSHSHSHSHSHFDFDFDFDFDFFDFDFDFDFDF".to_string(),
            "HSHSHSHSHSHHSHSHSHSHSHHDHDHDHDHDHFDFDFDFDFDFFDFDFDFDFDF".to_string(),
            "BSBSBSBSBSBHSHSHSHSHSHHDHDHDHDHDHDDFDFDFDFDDDDDDDDDDDDD".to_string(),
        ];

        Configuration {
            vsync: true,
            display_index: 0,
            display_mode: 0,
            stars: 300,
            volume: 20,
            pause_on_hit: 0.0,
            levels,
            score_table: vec![0; TOP_SCORE_COUNT]
        }
    }

    //read configuration from TOML file
    pub fn from_file(file_name: &str) -> Result<Configuration, String> {
        let cfg_string = match fs::read_to_string(file_name) {
            Ok(s) => s,
            Err(e) => {
                return Err(format!("Can't read \"{file_name}\" file. {e}\n"));
            },
        };

        let mut config: Configuration = match toml::from_str(&cfg_string) {
            Ok(c) => c,
            Err(e) => {
                return Err(format!("Parsing error in \"{file_name}\". {e}"));
            },
        };

        //limit score values and sort it in descending order
        config.score_table.iter_mut().for_each(|v| *v = (*v).clamp(0, SCORE_MAX) );
        config.score_table.sort_by(|a, b| b.cmp(a));
        Ok(config)
    }

    //serialize configuration into a single string that can be stored in file
    pub fn to_string(&self) -> String{
        let mut config_str = format!("vsync = {}\ndisplay_index = {}\ndisplay_mode = {}\nstars = {}\nvolume = {}\npause_on_hit = {}\n\n",
                                self.vsync,
                                self.display_index,
                                self.display_mode,
                                self.stars,
                                self.volume,
                                self.pause_on_hit);

        //levels
        config_str.push_str("levels = [\n");
        self.levels.iter().for_each(|l| {
            config_str.push('"');
            config_str.push_str(l);
            config_str.push_str("\",\n");
        });
        config_str.push_str("]\n\n");

        //score table
        config_str.push_str("score_table = [");
        self.score_table.iter()
            .take(TOP_SCORE_COUNT)
            .for_each(|s| config_str.push_str(&format!("{s}, ")));
        config_str.push_str("]\n\n");
        config_str
    }

    pub fn push_score(&mut self, score: i32) {
        self.score_table.push(score);
        self.score_table.sort_by(|a, b| b.cmp(a));
    }
}
