use anyhow::{Context, Result};
use divan::Bencher;
use pretty_assertions::assert_eq;
use rustfft::{algorithm::Radix4, num_complex::Complex, Fft, FftDirection};
use std::{
    env,
    fmt::{self, Display},
    fs::{self, File},
    io::{prelude::*, LineWriter},
    path::Path,
    process::Command,
    sync::Mutex,
    time::Instant,
};

const INPUT_PRECISION: usize = 2;

fn round(n: f64, precision: i32) -> f64 {
    (n * 10.0_f64.powi(precision)).round() / 10.0_f64.powi(precision)
}

fn generate_inputs(len: usize) -> Vec<Complex<f64>> {
    (0..len)
        .map(|i| {
            let theta = i as f64 / len as f64 * std::f64::consts::PI;
            let re = 1.0 * (10.0 * theta).cos() + 0.5 * (25.0 * theta).cos();
            let im = 1.0 * (10.0 * theta).sin() + 0.5 * (25.0 * theta).sin();
            Complex { re, im }
        })
        .collect()
}

fn fft(mut buf: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let fft = Radix4::new(buf.len().next_power_of_two(), FftDirection::Forward);
    fft.process(&mut buf);
    let factor = 1. / (buf.len() as f64).sqrt();
    for datum in &mut buf {
        *datum *= factor;
    }
    buf
}

fn generate_data(data_dir: &Path, len: usize, output_files: bool) -> Result<Vec<Complex<f64>>> {
    assert!(len.is_power_of_two(), "len must be a power of two");
    eprintln!("INFO: generating test data where size={len:.<8}");

    let precision = INPUT_PRECISION as i32;
    let round = |cs: Vec<_>| {
        cs.into_iter()
            .map(|Complex { re, im }| Complex {
                re: round(re, precision),
                im: round(im, precision),
            })
            .collect::<Vec<_>>()
    };

    let inputs = round(generate_inputs(len));
    {
        let inputs_dir = data_dir.join("inputs");
        if !inputs_dir.exists() {
            fs::create_dir_all(&inputs_dir)?;
        }
        let fout = File::create(inputs_dir.join(format!("{len}.dat")))?;
        let mut fout = LineWriter::new(fout);
        for datum in &inputs {
            writeln!(
                fout,
                "{:.p$},{:.p$}",
                datum.re,
                datum.im,
                p = INPUT_PRECISION,
            )?;
        }
        fout.flush()?;
    }
    let outputs = round(fft(inputs));
    if output_files {
        let outputs_dir = data_dir.join("outputs");
        if !outputs_dir.exists() {
            fs::create_dir_all(&outputs_dir)?;
        }
        let fout = File::create(outputs_dir.join(format!("{len}.dat")))?;
        let mut fout = LineWriter::new(fout);
        for datum in &outputs {
            writeln!(
                fout,
                "{:.p$},{:.p$}",
                datum.re,
                datum.im,
                p = INPUT_PRECISION,
            )?;
        }
        fout.flush()?;
    }
    Ok(outputs)
}

fn main() {
    divan::main();
}

#[divan::bench(args = bench_setup().expect("bench demo setup failed"))]
fn bench_fft_demo(bencher: Bencher, demo: &Demo) {
    bencher.bench(|| {
        divan::black_box(
            demo.cmd
                .lock()
                .unwrap()
                .output()
                .expect("failed to execute the demo"),
        )
    });
}

fn bench_setup() -> Result<Vec<Demo>> {
    let sizes = &[
        // 4, 64, 256, 1024, 4096,
        16384,
    ];

    let cwd = env::current_dir()?;

    eprintln!("INFO: generating test data...");
    let data_dir = cwd.join("data");
    let sized_data = sizes
        .iter()
        .map(|&size| Ok((size, generate_data(&data_dir, size, false)?)))
        .collect::<Result<Vec<_>>>()?;

    let bins_dir = cwd.join("bins");
    if !bins_dir.exists() {
        fs::create_dir_all(&bins_dir)?;
    }

    eprintln!("INFO: compiling the Moonbit FFT demo...");
    let mbt_dir = cwd.join("mbt");
    let status = Command::new("moon")
        .args(["build", "--target=native"])
        .current_dir(&mbt_dir)
        .status()?;
    assert!(status.success());

    let mbt_exe_path = bins_dir.join("mbt.exe");
    eprintln!("INFO: copying the .exe to `{}`...", mbt_exe_path.display());
    let exe = fs::read_dir(mbt_dir.join("target/native/release/build/main"))?
        .find(|e| e.as_ref().is_ok_and(|e| e.file_name() == "main.exe"))
        .context("exe not found")??;
    fs::copy(exe.path(), &mbt_exe_path)?;

    eprintln!("INFO: compiling the OpenJDK FFT demo...");
    let java_dir = cwd.join("java");
    let status = Command::new("mvn")
        .arg("package")
        .current_dir(&java_dir)
        .status()?;
    assert!(status.success());

    let jar_path = bins_dir.join("java.jar");
    eprintln!("INFO: copying the .jar to `{}`...", jar_path.display());
    let jar = fs::read_dir(java_dir.join("target"))?
        .find(|e| {
            e.as_ref().is_ok_and(|e| {
                let name = e.file_name();
                let name = name.to_string_lossy();
                name.starts_with("fft") && name.ends_with(".jar")
            })
        })
        .context("jar not found")??;
    fs::copy(jar.path(), &jar_path)?;

    eprintln!("INFO: compiling the GraalVM FFT demo...");
    let status = Command::new("mvn")
        .args(["-Pnative", "package"])
        .current_dir(&java_dir)
        .status()?;
    assert!(status.success());

    let graalvm_exe_path = bins_dir.join("java.exe");
    eprintln!(
        "INFO: copying the .exe to `{}`...",
        graalvm_exe_path.display()
    );
    let exe = fs::read_dir(java_dir.join("target"))?
        .find(|e| e.as_ref().is_ok_and(|e| e.file_name() == "fft"))
        .context("exe not found")??;
    fs::copy(exe.path(), &graalvm_exe_path)?;

    let mut demos = vec![];

    eprintln!("INFO: checking the correctness of the Moonbit FFT demo...");
    eprintln!("WARN: currently the Moonbit demo has no I/O except stdout prints");
    let mut mbt_demo = Demo::new("MoonBit", {
        // TODO: Add stdin for both demos.
        Command::new(&mbt_exe_path)
    });
    mbt_demo.assert_working(&sized_data)?;
    demos.push(mbt_demo);

    eprintln!("INFO: checking the correctness of the OpenJDK FFT demo...");
    let mut openjdk_demo = Demo::new("OpenJDK", {
        let mut cmd = Command::new("java");
        cmd.arg("-jar").arg(&jar_path);
        // TODO: Add stdin for both demos.
        // .stdin(File::open(
        //     data_dir.join("inputs").join(format!("{size}.dat")),
        // )?)
        cmd
    });
    openjdk_demo.assert_working(&sized_data)?;
    demos.push(openjdk_demo);

    eprintln!("INFO: checking the correctness of the GraalVM FFT demo...");
    let mut graalvm_demo = Demo::new("GraalVM", {
        // TODO: Add stdin for both demos.
        Command::new(&graalvm_exe_path)
    });
    graalvm_demo.assert_working(&sized_data)?;
    demos.push(graalvm_demo);

    Ok(demos)
}

struct Demo {
    name: String,
    cmd: Mutex<Command>,
}

impl Display for Demo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl Demo {
    fn new(name: &str, cmd: Command) -> Self {
        Self {
            name: name.to_owned(),
            cmd: Mutex::new(cmd),
        }
    }

    fn assert_working(&mut self, sized_data: &[(usize, Vec<Complex<f64>>)]) -> Result<()> {
        for (size, expected) in sized_data {
            eprint!(
                "INFO: testing the {} FFT demo where size={size:.<8}",
                self.name
            );
            let now = Instant::now();
            let out = self.cmd.lock().unwrap().output()?;
            let elapsed = now.elapsed();
            eprint!("\telapsed {elapsed:?}");
            assert_eq!(
                &std::str::from_utf8(&out.stdout)?
                    .lines()
                    .map(|l| {
                        let (re, im) = l.split_once(',').context("expected a comma")?;
                        let re = re.parse::<f64>()?;
                        let im = im.parse::<f64>()?;
                        Ok(Complex { re, im })
                    })
                    .collect::<Result<Vec<_>>>()?,
                expected
            );
            eprintln!("\tOK");
        }
        Ok(())
    }
}
