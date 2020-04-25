use std::io::Write;

pub struct TestHelper {
    dir: std::path::PathBuf,
    mint: goldenfile::Mint,
}

impl TestHelper {
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> Self {
        let dir = std::path::Path::new("tests").join(path);
        let mint = goldenfile::Mint::new(dir.clone());

        TestHelper {
            dir: dir.to_path_buf(),
            mint: mint,
        }
    }

    pub fn load<T: serde::de::DeserializeOwned, P: AsRef<std::path::Path>>(
        self: &Self,
        path: P,
    ) -> T {
        let file = std::fs::File::open(self.dir.join(path)).unwrap();
        ron::de::from_reader(file).unwrap()
    }

    pub fn save<T: serde::ser::Serialize, P: AsRef<std::path::Path>>(
        self: &mut Self,
        path: P,
        value: &T,
    ) {
        let config = ron::ser::PrettyConfig::default();
        let serialized_value = ron::ser::to_string_pretty(&value, config).unwrap();

        let mut output_file = self.mint.new_goldenfile(path).unwrap();
        write!(output_file, "{}", serialized_value).unwrap();
    }
}

pub fn run_test<P: AsRef<std::path::Path>>(path: P) {
    let mut t = TestHelper::new(path);

    let mut game = mafia::Game::new_from_state(t.load("in.init.ron"));
    let inputs: mafia::Inputs = t.load("in.input.ron");

    for input in inputs {
        game.apply(&input);
        match input {
            mafia::Input::AdvancePhase => {
                t.save(
                    format!(
                        "out.{}.{}_log.ron",
                        game.phase.prev().num(),
                        game.phase.prev().kind_str(),
                    ),
                    &game.log,
                );
                t.save(
                    format!("out.{}.{}.ron", game.phase.num(), game.phase.kind_str(),),
                    &game.state,
                );
            }
            _ => {}
        }
    }
}
