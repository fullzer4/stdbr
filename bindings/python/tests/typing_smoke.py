from typing import Optional

import stdbr

cpf: stdbr.Cpf = stdbr.Cpf.parse("529.982.247-25")
formatted: str = cpf.formatted()
valid: bool = stdbr.cpf_is_valid(formatted)
state: Optional[stdbr.State] = stdbr.state_from_abbreviation("SP")
