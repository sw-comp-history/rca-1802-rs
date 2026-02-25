use crate::assembler::assemble;
use crate::cpu::{Cpu, Instruction, execute_instruction};
use components::{
    Header, LegendItem, MemoryViewer, Modal, ProgramArea, Register, RegisterPanel, Sidebar,
    SidebarButton,
};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // CPU state
    let cpu = use_state(|| Cpu::new());
    let program_size = use_state(|| 0usize);
    let assembly_lines = use_state(|| Vec::<String>::new());
    let error_message = use_state(|| None::<String>);
    let last_registers = use_state(|| vec![0u16; 16]);
    let last_d = use_state(|| 0u8);
    let last_p = use_state(|| 0u8);
    let last_x = use_state(|| 0u8);

    // Editor code state
    let editor_code = use_state(|| {
        String::from(
            "; Add two numbers\nLDI 0x05    ; D = 5\nPHI R1      ; R1.high = 5\nLDI 0x0A    ; D = 10\nPHI R2      ; R2.high = 10\nGHI R1      ; D = R1.high\nSTR R3      ; Memory[R3] = D\nGHI R2      ; D = R2.high\nADD         ; D = D + Memory[R3]\nIDL         ; Stop execution (Idle)",
        )
    });

    // Modal states
    let tutorial_open = use_state(|| false);
    let examples_open = use_state(|| false);
    let challenges_open = use_state(|| false);
    let isa_open = use_state(|| false);
    let help_open = use_state(|| false);

    // Current example/challenge
    let _current_example = use_state(|| 0usize);
    let current_challenge = use_state(|| None::<usize>);
    let challenge_result = use_state(|| None::<String>);

    // Example programs
    let examples = vec![
        (
            "Example 1: Simple Addition",
            "; Add two numbers\nLDI 0x05    ; D = 5\nPHI R1      ; R1.high = 5\nLDI 0x0A    ; D = 10\nPHI R2      ; R2.high = 10\nGHI R1      ; D = R1.high\nSTR R3      ; Memory[R3] = D\nGHI R2      ; D = R2.high\nADD         ; D = D + Memory[R3]\nIDL         ; Stop execution (Idle)",
        ),
        (
            "Example 2: Register Loading",
            "; Load immediate values\nLDI 0xFF    ; D = 255\nPLO R5      ; R5.low = 255\nLDI 0x12    ; D = 18\nPHI R5      ; R5.high = 18\nGLO R5      ; D = R5.low (255)\nIDL         ; Stop execution",
        ),
        (
            "Example 3: Loop (Count to 10)",
            "; Count from 0 to 10 in R5 low byte\nLDI 0x00    ; D = 0\nPLO R5      ; R5.low = 0 (counter in LOW byte)\nLOOP:\nINC R5      ; R5++\nGLO R5      ; D = R5.low (not high byte!)\nXRI 0x0A    ; D = D XOR 10\nBNZ LOOP    ; Branch if not zero (loop while D != 0)\nIDL         ; Stop when R5.low = 10",
        ),
        (
            "Example 4: Conditional Branch",
            "; Branch based on comparison\nLDI 0x42    ; D = 66\nPHI R4      ; R4.high = 66\nLDI 0x42    ; D = 66\nXRI 0x42    ; D = D XOR 66 (result = 0)\nBZ EQUAL    ; Branch if zero (they're equal)\nLDI 0xFF    ; Not equal path\nIDL\nEQUAL:\nLDI 0x00    ; Equal path (D = 0)\nIDL",
        ),
    ];

    // Challenge definitions with detailed validation
    let challenges = use_memo((), |_| {
        vec![
            (
                "Challenge 1: Load a Value",
                "Load the value 42 (0x2A) into register R1's high byte and halt.",
                |cpu: &Cpu| -> Result<String, String> {
                    if !cpu.halted {
                        return Err(
                            "❌ CPU not halted. Make sure your program ends with IDL.".to_string()
                        );
                    }
                    let r1_high = cpu.registers[1] >> 8;
                    if r1_high != 0x2A {
                        return Err(format!(
                            "❌ R1.high = 0x{:02X} (expected 0x2A). Use LDI 0x2A followed by PHI R1.",
                            r1_high
                        ));
                    }
                    Ok(
                        "✅ Challenge completed! You successfully loaded 42 into R1's high byte!"
                            .to_string(),
                    )
                } as fn(&Cpu) -> Result<String, String>,
            ),
            (
                "Challenge 2: Simple Addition",
                "Add 5 + 7 and store the result (12) in R2's high byte, then halt.",
                |cpu: &Cpu| -> Result<String, String> {
                    if !cpu.halted {
                        return Err(
                            "❌ CPU not halted. Make sure your program ends with IDL.".to_string()
                        );
                    }
                    let r2_high = cpu.registers[2] >> 8;
                    if r2_high != 12 {
                        return Err(format!(
                            "❌ R2.high = {} (expected 12). Remember: 5 + 7 = 12.",
                            r2_high
                        ));
                    }
                    Ok(
                        "✅ Challenge completed! You correctly added 5 + 7 and stored 12!"
                            .to_string(),
                    )
                } as fn(&Cpu) -> Result<String, String>,
            ),
            (
                "Challenge 3: Memory Write",
                "Store the value 0xFF at the memory location pointed to by R3 (initially 0x0010), then halt.",
                |cpu: &Cpu| -> Result<String, String> {
                    if !cpu.halted {
                        return Err(
                            "❌ CPU not halted. Make sure your program ends with IDL.".to_string()
                        );
                    }
                    let mem_val = cpu.read_byte(0x0010).unwrap_or(0);
                    if mem_val != 0xFF {
                        return Err(format!(
                            "❌ Memory[0x0010] = 0x{:02X} (expected 0xFF). Use LDI 0xFF, then STR R3 to write to memory.",
                            mem_val
                        ));
                    }
                    Ok(
                        "✅ Challenge completed! You successfully wrote 0xFF to memory!"
                            .to_string(),
                    )
                } as fn(&Cpu) -> Result<String, String>,
            ),
            (
                "Challenge 4: Count to Five",
                "Use a loop to count from 0 to 5, storing the final value (5) in R4's low byte, then halt.\n\nSuccess criteria:\n- CPU halted (program ends with IDL)\n- R4.low = 5 (count reached exactly 5)",
                |cpu: &Cpu| -> Result<String, String> {
                    if !cpu.halted {
                        return Err(
                            "❌ CPU not halted. Make sure your loop exits and ends with IDL."
                                .to_string(),
                        );
                    }
                    let r4_low = cpu.registers[4] & 0xFF;
                    if r4_low < 5 {
                        return Err(format!(
                            "❌ R4.low = {} (expected 5). Your loop stopped too early. Check your loop condition.",
                            r4_low
                        ));
                    } else if r4_low > 5 {
                        return Err(format!(
                            "❌ R4.low = {} (expected 5). Your loop counted too high. Check your exit condition (should stop when R4.low = 5).",
                            r4_low
                        ));
                    }
                    Ok("✅ Challenge completed! Your loop correctly counted to 5!".to_string())
                } as fn(&Cpu) -> Result<String, String>,
            ),
        ]
    });

    // Event handlers
    let handle_assemble = {
        let cpu = cpu.clone();
        let program_size = program_size.clone();
        let assembly_lines = assembly_lines.clone();
        let error_message = error_message.clone();

        Callback::from(move |code: String| {
            error_message.set(None);

            match assemble(&code) {
                Ok(output) => {
                    let mut new_cpu = Cpu::new();
                    if let Err(e) = new_cpu.load_program(&output.machine_code, 0) {
                        error_message.set(Some(format!("Failed to load program: {}", e)));
                        return;
                    }

                    new_cpu.p = 0;
                    new_cpu.registers[0] = 0;
                    new_cpu.halted = false;

                    program_size.set(output.machine_code.len());
                    cpu.set(new_cpu);

                    // Store disassembly lines for highlighting
                    assembly_lines.set(output.disassembly);
                }
                Err(e) => {
                    error_message.set(Some(format!("Assembly error: {}", e)));
                }
            }
        })
    };

    let handle_step = {
        let cpu = cpu.clone();
        let error_message = error_message.clone();
        let last_registers = last_registers.clone();
        let last_d = last_d.clone();
        let last_p = last_p.clone();
        let last_x = last_x.clone();

        Callback::from(move |_| {
            error_message.set(None);

            let mut new_cpu = (*cpu).clone();

            if new_cpu.halted {
                error_message.set(Some("CPU is halted".to_string()));
                return;
            }

            // Save old state for change tracking
            last_registers.set(new_cpu.registers.to_vec());
            last_d.set(new_cpu.d);
            last_p.set(new_cpu.p);
            last_x.set(new_cpu.x);

            let pc = new_cpu.get_pc();

            // Decode instruction
            let instr_bytes: Vec<u8> = (0..3)
                .filter_map(|i| new_cpu.read_byte(pc.wrapping_add(i)).ok())
                .collect();

            if instr_bytes.is_empty() {
                error_message.set(Some("Failed to read instruction".to_string()));
                return;
            }

            let instruction = match Instruction::decode(&instr_bytes) {
                Some(instr) => instr,
                None => {
                    error_message.set(Some("Failed to decode instruction".to_string()));
                    return;
                }
            };

            // Advance PC
            let instruction_length = instruction.opcode.length() as u16;
            new_cpu.set_pc(pc.wrapping_add(instruction_length));

            // Execute
            if let Err(e) = execute_instruction(&mut new_cpu, &instruction) {
                error_message.set(Some(format!("Execution error: {}", e)));
                return;
            }

            cpu.set(new_cpu);
        })
    };

    let handle_run = {
        let cpu = cpu.clone();
        let error_message = error_message.clone();

        Callback::from(move |_| {
            error_message.set(None);

            let mut new_cpu = (*cpu).clone();
            let start_cycles = new_cpu.cycles;
            let max_cycles = 10000u64;

            while !new_cpu.halted && (new_cpu.cycles - start_cycles) < max_cycles {
                let pc = new_cpu.get_pc();
                let instr_bytes: Vec<u8> = (0..3)
                    .filter_map(|i| new_cpu.read_byte(pc.wrapping_add(i)).ok())
                    .collect();

                if instr_bytes.is_empty() {
                    break;
                }

                if let Some(instruction) = Instruction::decode(&instr_bytes) {
                    let instruction_length = instruction.opcode.length() as u16;
                    new_cpu.set_pc(pc.wrapping_add(instruction_length));

                    if execute_instruction(&mut new_cpu, &instruction).is_err() {
                        break;
                    }
                } else {
                    break;
                }
            }

            cpu.set(new_cpu);
        })
    };

    let handle_reset = {
        let cpu = cpu.clone();
        let program_size = program_size.clone();
        let assembly_lines = assembly_lines.clone();
        let error_message = error_message.clone();
        let challenge_result = challenge_result.clone();

        Callback::from(move |_| {
            cpu.set(Cpu::new());
            program_size.set(0);
            assembly_lines.set(Vec::new());
            error_message.set(None);
            challenge_result.set(None);
        })
    };

    // Challenge check handler
    let handle_check_challenge = {
        let cpu = cpu.clone();
        let current_challenge = current_challenge.clone();
        let challenge_result = challenge_result.clone();
        let challenges_clone = challenges.clone();

        Callback::from(move |_| {
            if let Some(idx) = *current_challenge {
                if idx < challenges_clone.len() {
                    let (_title, _description, validator) = challenges_clone[idx];
                    match validator(&cpu) {
                        Ok(success_msg) => challenge_result.set(Some(success_msg)),
                        Err(error_msg) => challenge_result.set(Some(error_msg)),
                    }
                }
            }
        })
    };

    // Dismiss challenge result
    let dismiss_result = {
        let challenge_result = challenge_result.clone();
        Callback::from(move |_| {
            challenge_result.set(None);
        })
    };

    // Modal handlers
    let close_tutorial = {
        let tutorial_open = tutorial_open.clone();
        Callback::from(move |_| tutorial_open.set(false))
    };

    let close_examples = {
        let examples_open = examples_open.clone();
        Callback::from(move |_| examples_open.set(false))
    };

    let close_challenges = {
        let challenges_open = challenges_open.clone();
        Callback::from(move |_| challenges_open.set(false))
    };

    let close_isa = {
        let isa_open = isa_open.clone();
        Callback::from(move |_| isa_open.set(false))
    };

    let close_help = {
        let help_open = help_open.clone();
        Callback::from(move |_| help_open.set(false))
    };

    // Sidebar buttons
    let sidebar_buttons = vec![
        SidebarButton {
            emoji: "📚".to_string(),
            label: "Tutorial".to_string(),
            onclick: {
                let tutorial_open = tutorial_open.clone();
                Callback::from(move |_| tutorial_open.set(true))
            },
            title: Some("Learn RCA 1802 basics".to_string()),
        },
        SidebarButton {
            emoji: "📝".to_string(),
            label: "Examples".to_string(),
            onclick: {
                let examples_open = examples_open.clone();
                Callback::from(move |_| examples_open.set(true))
            },
            title: Some("Load example programs".to_string()),
        },
        SidebarButton {
            emoji: "🎯".to_string(),
            label: "Challenges".to_string(),
            onclick: {
                let challenges_open = challenges_open.clone();
                Callback::from(move |_| challenges_open.set(true))
            },
            title: Some("Test your skills".to_string()),
        },
        SidebarButton {
            emoji: "📖".to_string(),
            label: "ISA Reference".to_string(),
            onclick: {
                let isa_open = isa_open.clone();
                Callback::from(move |_| isa_open.set(true))
            },
            title: Some("Instruction set reference".to_string()),
        },
        SidebarButton {
            emoji: "❓".to_string(),
            label: "Help".to_string(),
            onclick: {
                let help_open = help_open.clone();
                Callback::from(move |_| help_open.set(true))
            },
            title: Some("How to use this tool".to_string()),
        },
    ];

    // Build register list
    let registers: Vec<Register> = (0..16)
        .map(|i| {
            let changed = cpu.registers[i] != (*last_registers)[i];
            Register {
                name: format!("R{:X}", i),
                value: format!("0x{:04X}", cpu.registers[i]),
                changed,
            }
        })
        .collect();

    // Build legend items
    let legend_items = vec![
        LegendItem {
            label: "D = Accumulator".to_string(),
            value: format!("0x{:02X}", cpu.d),
            changed: cpu.d != *last_d,
        },
        LegendItem {
            label: "P = PC Selector".to_string(),
            value: format!("R{:X}", cpu.p),
            changed: cpu.p != *last_p,
        },
        LegendItem {
            label: "X = Index Selector".to_string(),
            value: format!("R{:X}", cpu.x),
            changed: cpu.x != *last_x,
        },
        LegendItem {
            label: "DF = Data Flag".to_string(),
            value: if cpu.df {
                "1".to_string()
            } else {
                "0".to_string()
            },
            changed: false,
        },
        LegendItem {
            label: "Q = Output".to_string(),
            value: if cpu.q {
                "1".to_string()
            } else {
                "0".to_string()
            },
            changed: false,
        },
        LegendItem {
            label: "IE = Interrupts".to_string(),
            value: if cpu.ie {
                "ON".to_string()
            } else {
                "OFF".to_string()
            },
            changed: false,
        },
    ];

    // Get memory
    let memory: Vec<u8> = (0..128)
        .map(|addr| cpu.read_byte(addr).unwrap_or(0))
        .collect();

    html! {
        <>
            <Header title="RCA 1802 (COSMAC ELF) Assembly Game" />

            <Sidebar buttons={sidebar_buttons} />

            <div class="main-content">
                <ProgramArea
                    on_assemble={handle_assemble}
                    on_step={handle_step}
                    on_run={handle_run}
                    on_reset={handle_reset}
                    assembly_output={if assembly_lines.is_empty() {
                        None
                    } else {
                        let pc = cpu.get_pc();
                        Some(html! {
                            <div>
                                {for assembly_lines.iter().map(|line| {
                                    // Parse address from line (format: "0000: F8 05 | LDI 0x05")
                                    let addr_str = line.split(':').next().unwrap_or("");
                                    let is_current = if let Ok(addr) = u16::from_str_radix(addr_str, 16) {
                                        addr == pc
                                    } else {
                                        false
                                    };

                                    let class = if is_current {
                                        "assembly-line current"
                                    } else {
                                        "assembly-line"
                                    };

                                    html! {
                                        <div class={class}>{line}</div>
                                    }
                                })}
                            </div>
                        })
                    }}
                    initial_code={Some((*editor_code).clone())}
                    step_enabled={!cpu.halted}
                    run_enabled={!cpu.halted}
                />

                <div class="right-panels">
                    // Registers Panel
                    <div class="registers-panel">
                        <div class="panel-title">{"Registers & Flags"}</div>
                        <div class="registers-container">
                            <RegisterPanel
                                registers={registers}
                                legend_items={legend_items}
                            />
                        </div>

                        // Flags
                        <div class="flags">
                            <div class="flag">
                                <div class={if cpu.df { "flag-indicator set" } else { "flag-indicator" }}></div>
                                <span>{"DF (Data Flag)"}</span>
                            </div>
                            <div class="flag">
                                <div class={if cpu.q { "flag-indicator set" } else { "flag-indicator" }}></div>
                                <span>{"Q (Output)"}</span>
                            </div>
                            <div class="flag">
                                <div class={if cpu.ie { "flag-indicator set" } else { "flag-indicator" }}></div>
                                <span>{"IE (Interrupt)"}</span>
                            </div>
                        </div>

                        // CPU Status
                        <div class="cpu-status">
                            <div class="status-item">
                                <span class="status-label">{"Cycles:"}</span>
                                <span class="status-value">{cpu.cycles}</span>
                            </div>
                            <div class="status-item">
                                <span class="status-label">{"Instructions:"}</span>
                                <span class="status-value">{cpu.instructions_executed}</span>
                            </div>
                            <div class="status-item">
                                <span class="status-label">{"Status:"}</span>
                                <span class="status-value">
                                    {if cpu.halted { "HALTED" } else { "RUNNING" }}
                                </span>
                            </div>
                        </div>
                    </div>

                    // Memory Viewer
                    <MemoryViewer
                        memory={memory}
                        pc={cpu.get_pc()}
                        title={Some("Memory (First 128 Bytes)".to_string())}
                        bytes_per_row={16}
                        bytes_to_show={128}
                    />
                </div>
            </div>

            // Challenge status banner
            if current_challenge.is_some() {
                <div class="challenge-banner">
                    <div class="challenge-info">
                        <strong>{"Challenge Mode Active"}</strong>
                        {if let Some(idx) = *current_challenge {
                            format!(" - {}", challenges[idx].0)
                        } else {
                            String::new()
                        }}
                    </div>
                    <button class="check-solution-btn" onclick={handle_check_challenge}>
                        {"Check Solution"}
                    </button>
                </div>
            }

            // Challenge result banner
            if let Some(result) = (*challenge_result).clone() {
                <div class={if result.contains("✅") {
                    "success-banner"
                } else {
                    "error-banner"
                }}>
                    <div class="banner-content">
                        {result}
                    </div>
                    <button class="dismiss-btn" onclick={dismiss_result.clone()}>
                        {"×"}
                    </button>
                </div>
            }

            // Error banner
            if let Some(err) = (*error_message).clone() {
                <div class="error-banner">
                    {err}
                </div>
            }

            // Tutorial Modal
            <Modal
                id="tutorialModal"
                title="RCA 1802 Tutorial"
                active={*tutorial_open}
                on_close={close_tutorial}
            >
                <div class="tutorial-content">
                    <h3>{"Welcome to the RCA 1802 Assembly Game!"}</h3>
                    <p>{"The RCA 1802, also known as the COSMAC, is a historic 8-bit microprocessor."}</p>

                    <h4>{"Key Features"}</h4>
                    <ul>
                        <li>{"16 general-purpose 16-bit registers (R0-RF)"}</li>
                        <li>{"8-bit accumulator (D register)"}</li>
                        <li>{"Unique register-based program counter"}</li>
                        <li>{"Data flag (DF) for carry/borrow"}</li>
                    </ul>

                    <h4>{"Historic Space Missions"}</h4>
                    <p>{"The RCA 1802 was renowned for its radiation-hardened design, making it ideal for space exploration:"}</p>
                    <ul>
                        <li><strong>{"Galileo"}</strong>{" - Jupiter orbiter's command and data systems"}</li>
                        <li><strong>{"Magellan"}</strong>{" - Venus mapper spacecraft"}</li>
                        <li><strong>{"Hubble Space Telescope"}</strong>{" - Wide Field and Planetary Camera (WFPC)"}</li>
                        <li><strong>{"Ulysses"}</strong>{" - Solar study spacecraft's Plasma Wave Analyzer"}</li>
                        <li><strong>{"Magsat"}</strong>{" - Earth orbit satellite with redundant 1802s"}</li>
                        <li><strong>{"Amateur Radio Satellites"}</strong>{" - Numerous OSCAR satellites"}</li>
                    </ul>

                    <h4>{"Getting Started"}</h4>
                    <ol>
                        <li>{"Click 'Examples' to see sample programs"}</li>
                        <li>{"Click 'Assemble' to convert assembly to machine code"}</li>
                        <li>{"Click 'Step' to execute one instruction"}</li>
                        <li>{"Click 'Run' to execute until HALT"}</li>
                        <li>{"Click 'Reset' to clear CPU state"}</li>
                    </ol>
                </div>
            </Modal>

            // Examples Modal
            <Modal
                id="examplesModal"
                title="Examples"
                active={*examples_open}
                on_close={close_examples.clone()}
            >
                <div class="examples-list">
                    {for examples.iter().enumerate().map(|(idx, (title, code))| {
                        let editor_code = editor_code.clone();
                        let examples_open = examples_open.clone();
                        let cpu = cpu.clone();
                        let assembly_lines = assembly_lines.clone();
                        let error_message = error_message.clone();
                        let code = code.to_string();

                        let load_example = Callback::from(move |_: MouseEvent| {
                            // Reset CPU and clear assembly output
                            cpu.set(Cpu::new());
                            assembly_lines.set(Vec::new());
                            error_message.set(None);

                            // Load new code
                            editor_code.set(code.clone());
                            examples_open.set(false);
                        });

                        html! {
                            <div class="example-item" key={idx} onclick={load_example}>
                                <h4>{title}</h4>
                                <p>{"Click to load this example"}</p>
                            </div>
                        }
                    })}
                </div>
            </Modal>

            // Challenges Modal
            <Modal
                id="challengesModal"
                title="Challenges"
                active={*challenges_open}
                on_close={close_challenges.clone()}
            >
                <div class="challenges-list">
                    {for challenges.iter().enumerate().map(|(idx, (title, description, _validator))| {
                        let editor_code = editor_code.clone();
                        let challenges_open = challenges_open.clone();
                        let cpu = cpu.clone();
                        let assembly_lines = assembly_lines.clone();
                        let error_message = error_message.clone();
                        let current_challenge = current_challenge.clone();
                        let challenge_result = challenge_result.clone();
                        let title_str = title.to_string();
                        let desc_str = description.to_string();

                        let load_challenge = Callback::from(move |_: MouseEvent| {
                            // Reset CPU and clear assembly output
                            cpu.set(Cpu::new());
                            assembly_lines.set(Vec::new());
                            error_message.set(None);
                            challenge_result.set(None);

                            // Set current challenge
                            current_challenge.set(Some(idx));

                            // Load empty code (user writes solution)
                            editor_code.set(format!("; {}\n; {}\n\n", title_str, desc_str));
                            challenges_open.set(false);
                        });

                        html! {
                            <div class="challenge-item" key={idx} onclick={load_challenge}>
                                <h4>{title}</h4>
                                <p>{description}</p>
                            </div>
                        }
                    })}
                </div>
            </Modal>

            // ISA Reference Modal
            <Modal
                id="isaModal"
                title="ISA Reference"
                active={*isa_open}
                on_close={close_isa}
            >
                <p>{"Instruction set reference coming soon!"}</p>
            </Modal>

            // Help Modal
            <Modal
                id="helpModal"
                title="Help"
                active={*help_open}
                on_close={close_help}
            >
                <h3>{"How to Use"}</h3>
                <p>{"1. Write or load assembly code in the editor"}</p>
                <p>{"2. Click Assemble to convert to machine code"}</p>
                <p>{"3. Use Step/Run to execute"}</p>
                <p>{"4. Watch registers and memory change"}</p>
            </Modal>

            // GitHub Corner
            <a href="https://github.com/sw-comp-history/rca-1802-rs" class="github-corner" aria-label="View source on GitHub" target="_blank">
                <svg width="80" height="80" viewBox="0 0 250 250" style="fill:#00d9ff; color:#1a1a2e; position: absolute; top: 0; border: 0; right: 0;" aria-hidden="true">
                    <path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z"></path>
                    <path d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2" fill="currentColor" style="transform-origin: 130px 106px;" class="octo-arm"></path>
                    <path d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z" fill="currentColor" class="octo-body"></path>
                </svg>
            </a>

            // Footer
            <footer class="app-footer">
                <div class="footer-left">
                    <span>{"MIT License"}</span>
                    <span>{"© 2026 Michael A Wright"}</span>
                </div>
                <div class="footer-right">
                    <span>{format!("{} | {} | {}", env!("VERGEN_BUILD_HOST"), env!("VERGEN_GIT_SHA_SHORT"), env!("VERGEN_BUILD_TIMESTAMP"))}</span>
                </div>
            </footer>
        </>
    }
}
