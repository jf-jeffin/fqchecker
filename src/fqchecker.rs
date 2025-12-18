pub mod fqc {

    use std::fs::{File, metadata};
    use std::io::{BufRead, BufReader};
    use tokio::sync::watch::Sender;

    use plotters::{prelude::*, style::full_palette::BLUE_700};
    use std::collections::HashMap;
    use std::time::{Duration, Instant};
    use tabled::{
        builder::Builder,
        settings::{Alignment, Style, Width},
    };
   

    #[derive(Debug)]
    pub struct ReadInfo {
        total: i64,
        length: HashMap<usize, usize>,
        // max_bases: HashMap<char, usize>,
        // max_quality:HashMap<char, usize>,
        // gc: HashMap<f32, usize>,
        // only_a_base_reads:HashMap<char, usize>,
    }
    impl ReadInfo {
        fn init() -> Self {
            ReadInfo {
                total: 0,
                length: HashMap::new(),
            }
        }
    }
    #[derive(Debug, Default, Clone)]
    pub struct CrrFileProcessInfo {
        pub read_count: usize,
        pub base_count: usize,
        pub file_name: String,
        pub file_size: u64,
        pub header: String,
        pub is_file_reading: bool

    }
    impl CrrFileProcessInfo {
        pub fn new(filename: &str) -> Self {
            let mut temp = 0;
            if let Ok(md) = metadata(filename){
                if !md.is_file() {
                    panic!("Not a file {}", filename);
                } else {
                    temp = md.len();
                } 
            }
            CrrFileProcessInfo {
                    read_count: 0,
                    base_count: 0,
                    file_size: temp,
                    header: String::new(),
                    file_name: filename.to_string(),
                    is_file_reading: false

            }
        }
    }

    pub const PHRED_SCORE_RANGE: usize = 94;
    #[derive(Debug)]
    pub struct BaseProfile {
        total: i64,
        a: i64,
        c: i64,
        g: i64,
        t: i64,
        n: i64,
    }

    pub trait Helpers {
        fn init() -> Self;
        fn dsply_table(self: &Self);
        fn plot_graph(self: &Self);
    }
    #[derive(Debug, Clone)]
    pub struct Counter {
        letter: char,
        count: i64,
    }

    pub fn total_quality(bqf: &BaseQualityProfile) -> (i64, Vec<Counter>) {
        let mut total_quality: Vec<Counter> = Vec::new();
        let mut total = 0;
        let mut total_bases = 0;
        for val in 0..PHRED_SCORE_RANGE {
            total = bqf.a_quality[val]
                + bqf.g_quality[val]
                + bqf.c_quality[val]
                + bqf.t_quality[val]
                + bqf.n_quality[val];

            if total != 0 {
                total_bases += total;
                total_quality.push(Counter {
                    letter: ((val + 33) as u8) as char,
                    count: total,
                });
            }
        }

        (total_bases, total_quality)
    }

    impl Helpers for BaseProfile {
        fn init() -> Self {
            BaseProfile {
                total: 0,
                a: 0,
                c: 0,
                g: 0,
                t: 0,
                n: 0,
            }
        }

        fn dsply_table(&self) {
            println!("self.total = {}", self.total);
            let mut builder_t = Builder::with_capacity(6, 2);
            builder_t.push_record(["Bases", "A", "C", "G", "T", "N"]);
            builder_t.push_record([
                "Count",
                format!("{} [{}%]", self.a, percentage_value(self.a, self.total)).as_str(),
                format!("{} [{}%]", self.c, percentage_value(self.c, self.total)).as_str(),
                format!("{} [{}%]", self.g, percentage_value(self.g, self.total)).as_str(),
                format!("{} [{}%]", self.t, percentage_value(self.t, self.total)).as_str(),
                format!("{} [{}%]", self.n, percentage_value(self.n, self.total)).as_str(),
            ]);

            println!(
                "{}",
                builder_t.build().with((
                    Style::sharp(),
                    Alignment::center(),
                    // Padding::new(1, 1, 1, 1)
                    Width::justify(15)
                ))
            );
        }

        fn plot_graph(&self) {
            let (width, height): (i32, i32) = (1024, 768);
            let mut bm =
                BitMapBackend::new("plot.png", (width as u32, height as u32)).into_drawing_area();
            let title_style = TextStyle::from(("times-new-roman", 50).into_font()).color(&(WHITE));

            match bm.titled("Base Compositions", title_style) {
                Ok(d) => bm = d,
                Err(e) => println!("Error: {:?}", e),
            }
            // let (mut title_area, mut plot_area) = bm.split_vertically(60);

            if let Err(e) = bm.fill(&WHITE) {
                println!("Error drawing plot.png: {}", e);
            }

            let sizes = [
                percentage_value(self.a, self.t) as f64,
                percentage_value(self.c, self.t) as f64,
                percentage_value(self.g, self.t) as f64,
                percentage_value(self.t, self.t) as f64,
                percentage_value(self.n, self.t) as f64,
            ];

            let centre = (width / 2, height / 2);
            let mut pie = Pie::new(
                &centre,
                &200.0,
                &sizes,
                &[RED, BLUE, GREEN, YELLOW, BLACK],
                &["A", "C", "G", "T", "N"],
            );
            pie.label_style((("sans-serif", 33).into_font()).color(&(BLUE_700)));
            pie.percentages((("sans-serif", 50.0 * 0.08).into_font()).color(&BLACK));
            pie.start_angle(-90.0);
            pie.donut_hole(100.0);
            let _ = bm.draw(&pie);
            bm.present().expect("Could not present");

            println!("Done");
        }
    }

    impl Helpers for BaseQualityProfile {
        fn init() -> Self {
            BaseQualityProfile {
                g_quality: [0; PHRED_SCORE_RANGE],
                t_quality: [0; PHRED_SCORE_RANGE],
                c_quality: [0; PHRED_SCORE_RANGE],
                a_quality: [0; PHRED_SCORE_RANGE],
                n_quality: [0; PHRED_SCORE_RANGE],
            }
        }

        fn dsply_table(self: &Self) {
            let ava_qual = total_quality(self);
            let mut builder_t = Builder::new();
            builder_t.push_column(["Quality", "Counts"]);
            let _ = ava_qual.1.iter().for_each(|qual| {
                builder_t.push_column([
                    format!("'{}'", qual.letter).as_str(),
                    format!(
                        "{} [{}%] ",
                        qual.count,
                        percentage_value(qual.count, ava_qual.0)
                    )
                    .as_str(),
                ]);
            });
            builder_t.push_column(["Total", format!("{}", ava_qual.0).as_str()]);
            println!(
                "{}",
                builder_t.build().with((
                    Style::sharp(),
                    Alignment::center(),
                    // Padding::new(1, 1, 1, 1)
                    Width::justify(15)
                ))
            );

            let mut builder = Builder::new();
            builder.push_record(["Quality", "G", "T", "C", "A", "N"]);
            let _ = ava_qual.1.iter().for_each(|qual| {
                let index = (qual.letter as usize) - 33;
                builder.push_record([
                    format!("'{}'", qual.letter).as_str(),
                    format!("{}", self.g_quality[index]).as_str(),
                    format!("{}", self.t_quality[index]).as_str(),
                    format!("{}", self.c_quality[index]).as_str(),
                    format!("{}", self.a_quality[index]).as_str(),
                    format!("{}", self.n_quality[index]).as_str(),
                ])
            });
            println!(
                "{}",
                builder.build().with((
                    Style::sharp(),
                    Alignment::center(),
                    // Padding::new(1, 1, 1, 1)
                    Width::justify(15)
                ))
            );
        }

        fn plot_graph(&self) {
            println!("ss")
        }
    }

    #[derive(Debug)]
    pub struct BaseQualityProfile {
        g_quality: [i64; PHRED_SCORE_RANGE],
        a_quality: [i64; PHRED_SCORE_RANGE],
        t_quality: [i64; PHRED_SCORE_RANGE],
        c_quality: [i64; PHRED_SCORE_RANGE],
        n_quality: [i64; PHRED_SCORE_RANGE],
    }

    #[inline]
    fn count_bases<'a>(
        seq_ln: &Vec<u8>,
        quality_val_ln: &Vec<u8>,
        b_profiles: &'a mut BaseProfile,
        bqf: &'a mut BaseQualityProfile,
    ) -> (&'a BaseProfile, &'a BaseQualityProfile) {
        for val in 0..seq_ln.len() {
            match seq_ln[val] {
                65 => {
                    b_profiles.a += 1;
                    bqf.a_quality[(quality_val_ln[val] - 33) as usize] += 1;
                }
                67 => {
                    b_profiles.c += 1;
                    bqf.c_quality[(quality_val_ln[val] - 33) as usize] += 1;
                }
                84 => {
                    b_profiles.t += 1;
                    bqf.t_quality[(quality_val_ln[val] - 33) as usize] += 1;
                }
                71 => {
                    b_profiles.g += 1;
                    bqf.g_quality[(quality_val_ln[val] - 33) as usize] += 1;
                }
                _ => {
                    b_profiles.n += 1;
                    bqf.n_quality[(quality_val_ln[val] - 33) as usize] += 1;
                }
            }
        }
        (b_profiles, bqf)
    }

    #[inline]
    pub fn extract_count(
        input_file_reader: BufReader<File>,
        infos: &mut CrrFileProcessInfo,
        sp: Sender<CrrFileProcessInfo>,
    ) -> (BaseProfile, BaseQualityProfile) {
        let mut new_seq: bool = false;
        let mut quality_ln: bool = false;
        let mut b_profiles: BaseProfile = BaseProfile::init();
        let mut bqf: BaseQualityProfile = BaseQualityProfile::init();
        let mut read_info = ReadInfo::init();
        let mut seq_ln: Vec<u8> = Vec::new();
        let mut quality_val_ln: Vec<u8> = Vec::new();
        infos.is_file_reading = true;
        
        // let mut total_reads: i64 = 0;
        for line in input_file_reader.lines() {
            if let Ok(ln) = line {
                // std::thread::sleep(Duration::from_millis(9));
                if ln.starts_with("@") {
                    new_seq = true;
                    infos.header = ln.clone();
                    continue;
                } else if ln.starts_with("+") {
                    quality_ln = true;
                    continue;
                }
                if new_seq {
                    b_profiles.total += ln.len() as i64;
                    seq_ln = ln.into_bytes();
                    new_seq = false;
                    continue;
                }
                if quality_ln {
                    quality_val_ln = ln.into_bytes();
                    quality_ln = false;
                }

                // read_info.total += 1;
                // infos.read_count += 1;
                infos.read_count += 1;

                *read_info.length.entry(seq_ln.len()).or_insert(0) += 1;

                // for base in  &seq_ln {
                let (b_profiles, bqf) =
                    count_bases(&seq_ln, &quality_val_ln, &mut b_profiles, &mut bqf);
                infos.base_count += b_profiles.total as usize;
                match sp.send(infos.clone()) {
                    Ok(_) => {}
                    Err(e) => {
                        panic!("Not working: {}",e)
                    }
                }

            }
        }
        infos.is_file_reading =  false;
        match sp.send(infos.clone()) {
                    Ok(_) => {}
                    Err(e) => {
                        panic!("Not working: {}",e)
                    }
                }
        drop(sp);
        (b_profiles, bqf)
        
    }

    pub fn fq_init(infos: &mut CrrFileProcessInfo, sp: Sender<CrrFileProcessInfo>) {
        let c = "C:\\Users\\Jef Finn\\Downloads\\downsampled_SampleB_R1.fastq\\downsampled_SampleB_R1.fastq";
        let file = File::open(infos.file_name.as_str());
        match file {
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1)
            }
            Ok(f) => {
                let input_file_reader = BufReader::new(f);
                let g = extract_count(input_file_reader, infos, sp);
                // g.0.dsply_table();
                // g.0.plot_graph();
                // g.1.dsply_table();
            }
        }
    }
    fn percentage_value(val: i64, total: i64) -> f32 {
        (((val as f32 / total as f32) * 100.0) * 100.0).round() / 100.0
    }
}
