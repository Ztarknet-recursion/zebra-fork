 ~/.asdf/shims/scarb --profile release build
 
cp target/release/stwo_wrapper.executable.json compiled/old_stwo_wrapper.executable.json
 
stwo_run_and_prove \
		--program ../bootloaders/simple_bootloader_compiled.json \
		--program_input program_input_step_1.json \
		--prover_params_json prover_params_step_1.json \
		--proofs_dir target \
		--proof-format binary \
		--verify