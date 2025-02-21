use std::collections::HashMap;
use strum_macros::{AsRefStr, EnumString};

// Instruction ------------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Instruction {
    pub name: String,
    pub summary: String,
    pub forms: Vec<InstructionForm>,
    pub url: Option<String>,
    pub arch: Option<Arch>,
}

impl Default for Instruction {
    fn default() -> Self {
        let name = String::new();
        let summary = String::new();
        let forms = vec![];
        let url = None;
        let arch = None;

        Self {
            name,
            summary,
            forms,
            url,
            arch,
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // basic fields
        let header: String;
        if let Some(arch) = &self.arch {
            header = format!("{} [{}]", &self.name, arch.as_ref());
        } else {
            header = self.name.clone();
        }

        let mut v: Vec<&'_ str> = vec![&header, &self.summary, "\n", "## Forms", "\n"];

        // instruction forms
        let instruction_form_strs: Vec<String> =
            self.forms.iter().map(|f| format!("{}", f)).collect();
        for item in instruction_form_strs.iter() {
            v.push(item.as_str());
        }

        // url
        let more_info: String;
        match &self.url {
            None => {}
            Some(url_) => {
                more_info = format!("\nMore info: {}", url_);
                v.push(&more_info);
            }
        }

        let s = v.join("\n");
        write!(f, "{}", s)?;
        Ok(())
    }
}

impl<'own> Instruction {
    /// Add a new form at the current instruction
    pub fn push_form(&mut self, form: InstructionForm) {
        self.forms.push(form);
    }

    /// get the names of all the associated commands (includes Go and Gas forms)
    pub fn get_associated_names(&'own self) -> Vec<&'own str> {
        let mut names = Vec::<&'own str>::new();
        names.push(&self.name);

        for f in &self.forms {
            for name in [&f.gas_name, &f.go_name].iter().copied().flatten() {
                names.push(name);
            }
        }

        names
    }
}

// InstructionForm --------------------------------------------------------------------------------
#[derive(Default, Debug, Clone)]
pub struct InstructionForm {
    pub gas_name: Option<String>,
    pub go_name: Option<String>,
    pub mmx_mode: Option<MMXMode>,
    pub xmm_mode: Option<XMMMode>,
    pub cancelling_inputs: Option<bool>,
    pub nacl_version: Option<u8>,
    pub nacl_zero_extends_outputs: Option<bool>,
    pub isa: Option<ISA>,
    pub operands: Vec<Operand>,
}

impl std::fmt::Display for InstructionForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        if let Some(val) = &self.gas_name {
            s += &format!("*GAS*: {} | ", val);
        }
        if let Some(val) = &self.go_name {
            s += &format!("*GO*: {} | ", val);
        }

        if let Some(val) = &self.mmx_mode {
            s += &(format!("*MMX*: {} | ", val.as_ref()));
        }
        if let Some(val) = &self.xmm_mode {
            s += &(format!("*XMM*: {} | ", val.as_ref()));
        }

        // cancelling inputs
        // nacl_version
        // nacl_zero_extends_outputs

        // ISA
        if let Some(val) = &self.isa {
            s += &format!("*ISA*: {} | ", val.as_ref());
        }

        if !s.is_empty() {
            s = format!("- {}\n\n", &s[..s.len() - 3]);
        }

        // Operands
        let operands_str: String = self
            .operands
            .iter()
            .map(|op| {
                let mut s = format!("  + {:<8}", format!("[{}]", op.type_.as_ref()));
                if let Some(input) = op.input {
                    s += &format!(" input = {:<5} ", input)
                }
                if let Some(output) = op.output {
                    s += &format!(" output = {:<5}", output)
                }
                if let Some(extended_size) = op.extended_size {
                    s += &format!(" extended-size = {}", extended_size)
                }

                s
            })
            .collect::<Vec<String>>()
            .join("\n");
        s = s + &operands_str + "\n";

        write!(f, "{}", s)?;
        Ok(())
    }
}

// helper structs, types and functions ------------------------------------------------------------
pub type NameToInstructionMap<'instruction> = HashMap<&'instruction str, &'instruction Instruction>;

#[derive(Debug, Clone, EnumString, AsRefStr)]
pub enum XMMMode {
    SSE,
    AVX,
}

#[derive(Debug, Clone, EnumString, AsRefStr)]
pub enum MMXMode {
    FPU,
    MMX,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, EnumString, AsRefStr)]
pub enum Arch {
    X86,
    X86_64,
}

// Instruction Set Architecture -------------------------------------------------------------------
#[derive(Debug, Clone, EnumString, AsRefStr)]
pub enum ISA {
    RDTSC,
    RDTSCP,
    CPUID,
    CMOV,
    MMX,
    #[strum(serialize = "MMX+")]
    MMXPlus,
    FEMMS,
    #[strum(serialize = "3dnow!")]
    _3DNow,
    #[strum(serialize = "3dnow!+")]
    _3DNowPlus,
    #[strum(serialize = "3dnow! Geode")]
    _3DNowGeode,
    SSE,
    SSE2,
    SSE3,
    SSSE3,
    #[strum(serialize = "SSE4.1")]
    SSE4_1, //
    #[strum(serialize = "SSE4.2")]
    SSE4_2, //
    SSE4A,
    AVX,
    AVX2,
    XOP,
    FMA3,
    FMA4,
    F16C,
    PCLMULQDQ,
    AES,
    SHA,
    RDRAND,
    RDSEED,
    MOVBE,
    POPCNT,
    LZCNT,
    BMI,
    BMI2,
    TBM,
    ADX,
    CLFLUSH,
    CLFLUSHOPT,
    CLWB,
    CLZERO,
    PREFETCH,
    PREFETCHW,
    PREFETCHWT1,
    MONITOR,
    MONITORX,
    AVX512F,
    AVX512BW,
    AVX512DQ,
    AVX512VL,
    AVX512PF,
    AVX512ER,
    AVX512CD,
    AVX512VBMI,
    AVX512IFMA,
    AVX512VPOPCNTDQ,
}

// Operand ----------------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Operand {
    pub type_: OperandType,
    pub input: Option<bool>,
    pub output: Option<bool>,
    pub extended_size: Option<usize>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, EnumString, AsRefStr)]
pub enum OperandType {
    #[strum(serialize = "1")]
    _1,
    #[strum(serialize = "3")]
    _3,
    imm4,
    imm8,
    imm16,
    imm32,
    imm64,
    al,
    cl,
    r8,
    r8l,
    ax,
    r16,
    r16l,
    eax,
    r32,
    r32l,
    rax,
    r64,
    mm,
    xmm0,
    xmm,
    #[strum(serialize = "xmm{k}")]
    xmm_k,
    #[strum(serialize = "xmm{k}{z}")]
    xmm_k_z,
    ymm,
    #[strum(serialize = "ymm{k}")]
    ymm_k,
    #[strum(serialize = "ymm{k}{z}")]
    ymm_k_z,
    zmm,
    #[strum(serialize = "zmm{k}")]
    zmm_k,
    #[strum(serialize = "zmm{k}{z}")]
    zmm_k_z,
    k,
    #[strum(serialize = "k{k}")]
    k_k,
    moffs32,
    moffs64,
    m,
    m8,
    m16,
    #[strum(serialize = "m16{k}{z}")]
    m16_k_z,
    m32,
    #[strum(serialize = "m32{k}")]
    m32_k,
    #[strum(serialize = "m32{k}{z}")]
    m32_k_z,
    m64,
    #[strum(serialize = "m64{k}")]
    m64_k,
    #[strum(serialize = "m64{k}{z}")]
    m64_k_z,
    m128,
    #[strum(serialize = "m128{k}{z}")]
    m128_k_z,
    m256,
    #[strum(serialize = "m256{k}{z}")]
    m256_k_z,
    m512,
    #[strum(serialize = "m512{k}{z}")]
    m512_k_z,
    #[strum(serialize = "m64/m32bcst")]
    m64_m32bcst,
    #[strum(serialize = "m128/m32bcst")]
    m128_m32bcst,
    #[strum(serialize = "m256/m32bcst")]
    m256_m32bcst,
    #[strum(serialize = "m512/m32bcst")]
    m512_m32bcst,
    #[strum(serialize = "m128/m64bcst")]
    m128_m64bcst,
    #[strum(serialize = "m256/m64bcst")]
    m256_m64bcst,
    #[strum(serialize = "m512/m64bcst")]
    m512_m64bcst,
    vm32x,
    #[strum(serialize = "vm32x{k}")]
    vm32x_k,
    vm64x,
    #[strum(serialize = "vm64x{k}")]
    vm64xk,
    vm32y,
    #[strum(serialize = "vm32y{k}")]
    vm32yk_,
    vm64y,
    #[strum(serialize = "vm64y{k}")]
    vm64y_k,
    vm32z,
    #[strum(serialize = "vm32z{k}")]
    vm32z_k,
    vm64z,
    #[strum(serialize = "vm64z{k}")]
    vm64z_k,
    rel8,
    rel32,
    #[strum(serialize = "{er}")]
    er,
    #[strum(serialize = "{sae}")]
    sae,
}

// lsp types --------------------------------------------------------------------------------------

/// Represents a text cursor between characters, pointing at the next character in the buffer.
pub type Column = usize;
