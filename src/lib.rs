use llvm_plugin::inkwell::module::Module;
use llvm_plugin::inkwell::values::CallSiteValue;
use llvm_plugin::{
    LlvmModulePass, ModuleAnalysisManager, PassBuilder, PipelineParsing, PreservedAnalyses,
};

// A name and version is required.
#[llvm_plugin::plugin(name = "plugin_name", version = "0.1")]
fn plugin_registrar(builder: &mut PassBuilder) {
    // Add a callback to parse a name from the textual representation of
    // the pipeline to be run.
    builder.add_module_pipeline_parsing_callback(|name, manager| {
        if name == "custom-pass" {
            // the input pipeline contains the name "custom-pass",
            // so we add our custom pass to the pass manager
            manager.add_pass(CustomPass);

            // we notify the caller that we were able to parse
            // the given name
            PipelineParsing::Parsed
        } else {
            // in any other cases, we notify the caller that our
            // callback wasn't able to parse the given name
            PipelineParsing::NotParsed
        }
    });
}

struct CustomPass;
impl LlvmModulePass for CustomPass {
    fn run_pass(&self, module: &mut Module, manager: &ModuleAnalysisManager) -> PreservedAnalyses {
        for f in module.get_functions() {
            for bb in f.get_basic_blocks() {
                for i in bb.get_instructions() {
                    if let Ok(call_site_value) = CallSiteValue::try_from(i) {
                        eprintln!("Found a call site: {:?}", call_site_value);
                    }
                }
            }
        }

        return PreservedAnalyses::All;
    }
}
