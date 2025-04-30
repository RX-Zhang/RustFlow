from dataclasses import dataclass

# global variables
llm_call_count:int = 0


test_constraints_prompt = [
    "Use standalone functions rather than struct methods.",
    "Provide only the translated Rust function, without additional explanations and a main function.",
]

#-----------------------------------------------------------------------------------------------
#   Feedback Strategy  |   Feedback   |  Hint Information  |  Initiate Dialogue (Add History)  |
#       my_method       | fallback_opt |      hinted        |         conversation             |
#-----------------------|--------------|--------------------|-----------------------------------|
#       BaseRepair      |     fix      |         -          |             False                |
#         CAPR          |     fix      |         -          |              True                |
#        Hinted         |   restart    |        True        |               -                  |
#        Restart        |   restart    |       False        |               -                  |
#-----------------------------------------------------------------------------------------------

@dataclass(frozen=True)
class Options:

    benchmark_name: str  # choices = ["libopenaptx", "opl","cpw"]
    submodule_name: str

    model: str = "claude3" # choices=["claude3", "gpt4", "mistral" ,"gemini"]
    feedback_strategy: str = "Restart"   # choices=["BaseRepair", "CAPR", "Hinted" ,"Restart"]

    # Few sample prompts to improve translation quality
    few_shot: bool = False # True  or False

    # Parameters related to the thought chain
    cot: bool = False # True or False
    cot_version: str = "ast" # choices = ["ast", "go", "llvm", "explain"]

    # Candidate number (set to 1 without activation)
    num_candidates: int = 1

    # Feedback strategy related parameters
    fallback_opt: str = "fix" if feedback_strategy == "BaseRepair" or feedback_strategy == "CAPR" else "restart"
    hinted: bool = True if feedback_strategy == "Hinted" and few_shot else False
    conversation: bool = True if feedback_strategy == "CAPR" else False

    # Other
    comp_fix: str = "msft"  # choices = [ "msft", "no"]
    sem_fix: str = "base"  # choices=["base","llm-explain"]  # default "base"  -> Extra
    language: str = "c"
    restart_budget: int = 3
    fix_budget: int = 3
    comp_fix_attempt_budget: int = 3
    n_prompt_examples: int = 2
    transpl_attempt_budget: int = 3
    conversation_window_size: int = 2
    initial_temperature: float = 0.2


    @property
    def work_dir(self) -> str:
        return (
            f"transpilations/{self.language}/{self.benchmark_name}/{self.model}/"   # Basic information
            f"{self.feedback_strategy}/{self.is_few_shot}/{self.is_cot}/candidates-{self.num_candidates}/"  # Application Strategy
            f"comp-fix-{self.comp_fix}/sem-fix-{self.sem_fix}/"   # Repair Strategy
            f"{self.submodule_name}"
        )

    @property
    def res_dir(self) -> str:
        return f"{self.work_dir}/results"

    @property
    def is_cot(self) -> str:
        return f"cot-{self.cot_version}" if self.cot else "cot-False"

    @property
    def is_few_shot(self) -> str:
        return "few-shot-True" if self.few_shot else "few-shot-False"