use std::path::*;

#[derive(Debug)]
pub struct Config{
    out_path : Option::<PathBuf>,
    input_path : Option::<PathBuf>,
    on_stdout : bool
}

impl Config{
    pub fn new() -> Self{
        let mut args = std::env::args();

        // init
        let mut out : Option::<PathBuf> = None;
        let mut input : Option::<PathBuf> = None;
        let mut on_stdout : bool = false;

        // handle arguments
        loop{
            match args.next(){
                Some(s) => {
                    match s.as_str(){
                        "-o" => match args.next() {
                            Some(b) => out = Some(PathBuf::from(b)),
                            None => ()                                           
                        },

                        "-stdout" | "--stdout"=> on_stdout = true,
                        _ => input = Some(PathBuf::from(s)),
                    }
                }
                None => break,
            }
        }

        Self{
            out_path : out,
            input_path : input,
            on_stdout
        }
    }

    pub fn get_out_path(&self) -> PathBuf{
        match &self.out_path{
            Some(p) => p.clone(),
            None => PathBuf::from(String::from("sim_risc16.mem"))
        }
    }

    pub fn get_input_path(&self) -> PathBuf{
        match &self.input_path{
            Some(p) => p.clone(),
            None => panic!("No input file")
        }
    }

    pub fn is_on_stdout(&self) ->  bool{
        self.on_stdout
    }
}
