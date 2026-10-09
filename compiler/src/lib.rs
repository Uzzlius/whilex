use std::fs;
use std::path::PathBuf;
use std::process::Command;
use whilex;

use inkwell::context::Context;
use inkwell::targets::{Target, TargetMachine, TargetTriple, CodeModel, FileType, InitializationConfig, RelocMode};
use inkwell::OptimizationLevel;

use clap::{self, Parser};

use crate::compiler::Compiler;

mod util;
mod compiler;


#[derive(Debug)]
pub struct CompileConfig {
    file_path: String, 
    executable_output: PathBuf,
    object_output: PathBuf,
    target_triple: String,
    sysroot: Option<PathBuf>,
    cpu: String,
    features: String
}

#[derive(Debug, Parser)]
#[command(name = "compilex", version, about = "Whilex compiler")]
struct Cli {
    /// Source file *.whl
    file_path: String,

    /// Output executable path
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Output object file path
    #[arg(long)]
    object_output: Option<PathBuf>,

    /// Target triple
    #[arg(long)]
    target: Option<String>,

    /// Sysroot
    #[arg(long)]
    sysroot: Option<PathBuf>,

    /// Target CPU
    #[arg(long)]
    cpu: Option<String>,
    
    /// Target features
    #[arg(long)]
    features: Option<String>,
}

impl CompileConfig {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Self, String> {
        let cli = Cli::try_parse_from(args).map_err(|e| e.to_string())?;
        if !cli.file_path.ends_with(".whl") {
            return Err(String::from("Invalid file format; expected a .whl file"));
        }
        
        let mut default_executable = PathBuf::from(&cli.file_path);
        default_executable.set_extension("");

        let mut default_object = PathBuf::from(&cli.file_path);
        default_object.set_extension("");

        let host_triple = TargetMachine::get_default_triple().as_str().to_string_lossy().to_string();
        
        let do_native_build = cli.target.is_none();
        
        let target_triple = cli.target.unwrap_or(host_triple);
        
        let cpu = cli.cpu.unwrap_or_else(|| {
            if do_native_build {
                TargetMachine::get_host_cpu_name().to_string()
            } else {
                String::from("generic")
            }
        });

        let features = cli.features.unwrap_or_else(|| {
            if do_native_build {
                TargetMachine::get_host_cpu_features().to_string()
            } else {
                String::new()
            }
        });
       
        Ok(Self {
            file_path: cli.file_path,
            executable_output: cli.output.unwrap_or(default_executable),
            object_output: cli.object_output.unwrap_or(default_object),
            target_triple,
            sysroot: cli.sysroot,
            cpu,
            features,
        })
    }
}

pub fn compile(config: CompileConfig) -> Result<(), String> {
    let file_contents = match fs::read_to_string(&config.file_path) {
        Ok(file_contents) => file_contents,
        Err(_) => return Err(String::from("Could not read {config.file_path}")),
    };

    let lexer = whilex::lexer::Lexer::new(&file_contents);

    let mut parser = whilex::parser::Parser::new(lexer);
    let program = parser.program();

    let program = match program {
        Ok(program) => program,
        Err(_) => {
            for error in parser.errors {
                eprintln!("{}", error);
            }
            return Err(String::from("Parsing error"));
        }
        
    };


    // --- Set up build

    let context = Context::create();

    Target::initialize_all(&InitializationConfig::default());


    let triple = TargetTriple::create(&config.target_triple);
    let target = Target::from_triple(&triple).map_err(|e| e.to_string())?;
    let machine = target.create_target_machine(
        &triple,
        &config.cpu, 
        &config.features, 
        OptimizationLevel::Default, RelocMode::Default, CodeModel::Default
    ).ok_or("Failed to create machine")?;


    // --- Do the compiling

    let builder = context.create_builder();
    let module = context.create_module("whilex");

    let main_function = match Compiler::compile(&context, &builder, &module, &program) {
        Ok(function) => function,
        Err(err) => {
            return Err(String::from(err));
        }
    };

    main_function.print_to_stderr();

    // --- Write and link binary

    module.verify().map_err(|e| e.to_string())?;

    machine.write_to_file(&module, FileType::Object, &config.object_output).map_err(|e| e.to_string())?;

    let mut cmd = Command::new("clang");
    cmd.arg(format!("--target={}", &config.target_triple));

    if let Some(sysroot) = &config.sysroot {
        cmd.arg(format!("--sysroot={}", sysroot.to_string_lossy()));
    }
    
    cmd.arg("-fuse-ld=lld")
        .arg("-static")
        .arg(&config.object_output)
        .arg("runtime.o")
        .arg("-o")
        .arg(&config.executable_output);


    let status = cmd.status().map_err(|e| format!("Failed to run clang: {e}"))?;

    if !status.success() {
        return Err(format!("Linking failed with status: {status}"));
    }

    Ok(())
}
