import { readFileSync } from "fs";
import { fileURLToPath } from "url";
import { createRequire } from "module";
import { dirname, join } from "path";
import { describe, it } from "node:test";
import { strictEqual, ok } from "node:assert";

const __dirname = dirname(fileURLToPath(import.meta.url));
const goldenPath = process.env.GOLDEN_JSON || join(__dirname, "../../tests/parity/golden.json");
const golden = JSON.parse(readFileSync(goldenPath, "utf8"));

const require = createRequire(import.meta.url);
const stdbr = require("./stdbr.node");
const {
  Cpf, cpfIsValid, cpfIsValidStrict, cpfFormat, cpfRemoveSymbols,
  cpfGenerate, cpfComputeCheckDigits,
  Cnpj, cnpjIsValid, cnpjIsValidStrict, cnpjFormat, cnpjRemoveSymbols,
  cnpjGenerate, cnpjComputeCheckDigits, CnpjKind,
  Cep, cepIsValid, cepIsValidStrict, cepFormat, cepRemoveSymbols,
  cepGenerate,
  stateAbbreviation, stateName, stateFromAbbreviation, allStates,
  municipioFromIbgeCode, municipioCapitalOf, municipiosByState,
  municipioSearchByName, municipioCount,
} = stdbr;

function strictTest(validateFn, cases) {
  for (const c of cases) {
    it(`${validateFn.name}(${c.input})`, () => {
      if (c.valid) {
        validateFn(c.input);
      } else {
        try {
          validateFn(c.input);
          ok(false, "should throw");
        } catch (e) {
          strictEqual(e.message, c.error);
        }
      }
    });
  }
}

function checkDigitsTest(computeFn, cases) {
  for (const c of cases) {
    it(`${computeFn.name}(${c.base})`, () => {
      const result = computeFn(c.base);
      if (c.expected === null) {
        strictEqual(result, null);
      } else {
        strictEqual(result[0], c.expected[0]);
        strictEqual(result[1], c.expected[1]);
      }
    });
  }
}

describe("CPF", () => {
  describe("parse", () => {
    for (const c of golden.cpf.parse) {
      it(c.input, () => {
        const cpf = Cpf.parse(c.input);
        strictEqual(cpf.asStr(), c.digits_only);
        strictEqual(cpf.formatted(), c.formatted);
        strictEqual(cpf.masked(), c.masked);
        strictEqual(cpf.fiscalRegion, c.fiscal_region);
        strictEqual(cpf.checkDigits[0], c.check_digits[0]);
        strictEqual(cpf.checkDigits[1], c.check_digits[1]);
      });
    }
  });

  describe("is_valid", () => {
    for (const c of golden.cpf.is_valid)
      it(c.input || "(empty)", () => strictEqual(cpfIsValid(c.input), c.expected));
  });

  describe("is_valid_strict", () => strictTest(cpfIsValidStrict, golden.cpf.is_valid_strict));

  describe("format", () => {
    for (const c of golden.cpf.format)
      it(c.input || "(empty)", () => strictEqual(cpfFormat(c.input) ?? null, c.expected));
  });

  describe("remove_symbols", () => {
    for (const c of golden.cpf.remove_symbols)
      it(c.input, () => strictEqual(cpfRemoveSymbols(c.input), c.expected));
  });

  describe("compute_check_digits", () => checkDigitsTest(cpfComputeCheckDigits, golden.cpf.compute_check_digits));

  it("generate roundtrip", () => {
    const raw = cpfGenerate();
    ok(cpfIsValid(raw));
    strictEqual(Cpf.parse(raw).asStr(), raw);
  });
});

describe("CNPJ", () => {
  describe("parse", () => {
    for (const c of golden.cnpj.parse) {
      it(c.input, () => {
        const cnpj = Cnpj.parse(c.input);
        strictEqual(cnpj.asStr(), c.digits_only);
        strictEqual(cnpj.formatted(), c.formatted);
        strictEqual(cnpj.masked(), c.masked);
        strictEqual(cnpj.raiz(), c.raiz);
        strictEqual(cnpj.ordem(), c.ordem);
        strictEqual(cnpj.kind, c.kind);
        strictEqual(cnpj.establishmentType, c.establishment_type);
        strictEqual(cnpj.checkDigits[0], c.check_digits[0]);
        strictEqual(cnpj.checkDigits[1], c.check_digits[1]);
      });
    }
  });

  describe("is_valid", () => {
    for (const c of golden.cnpj.is_valid)
      it(c.input || "(empty)", () => strictEqual(cnpjIsValid(c.input), c.expected));
  });

  describe("is_valid_strict", () => strictTest(cnpjIsValidStrict, golden.cnpj.is_valid_strict));

  describe("format", () => {
    for (const c of golden.cnpj.format)
      it(c.input || "(empty)", () => strictEqual(cnpjFormat(c.input) ?? null, c.expected));
  });

  describe("remove_symbols", () => {
    for (const c of golden.cnpj.remove_symbols)
      it(c.input, () => strictEqual(cnpjRemoveSymbols(c.input), c.expected));
  });

  describe("compute_check_digits", () => checkDigitsTest(cnpjComputeCheckDigits, golden.cnpj.compute_check_digits));

  it("generate roundtrip", () => {
    const raw = cnpjGenerate(CnpjKind.Numeric);
    ok(cnpjIsValid(raw));
    strictEqual(Cnpj.parse(raw).asStr(), raw);
  });
});

describe("CEP", () => {
  describe("parse", () => {
    for (const c of golden.cep.parse) {
      it(c.input, () => {
        const cep = Cep.parse(c.input);
        strictEqual(cep.asStr(), c.digits_only);
        strictEqual(cep.formatted(), c.formatted);
        strictEqual(cep.masked(), c.masked);
        strictEqual(cep.postalRegion, c.postal_region);
        const abbr = cep.state != null ? stateAbbreviation(cep.state) : null;
        strictEqual(abbr, c.state);
      });
    }
  });

  describe("is_valid", () => {
    for (const c of golden.cep.is_valid)
      it(c.input || "(empty)", () => strictEqual(cepIsValid(c.input), c.expected));
  });

  describe("is_valid_strict", () => strictTest(cepIsValidStrict, golden.cep.is_valid_strict));

  describe("format", () => {
    for (const c of golden.cep.format)
      it(c.input || "(empty)", () => strictEqual(cepFormat(c.input) ?? null, c.expected));
  });

  describe("remove_symbols", () => {
    for (const c of golden.cep.remove_symbols)
      it(c.input, () => strictEqual(cepRemoveSymbols(c.input), c.expected));
  });

  it("generate roundtrip", () => {
    const raw = cepGenerate();
    ok(cepIsValid(raw));
    strictEqual(Cep.parse(raw).asStr(), raw);
  });
});

describe("UF", () => {
  it("states", () => {
    const states = allStates();
    strictEqual(states.length, golden.uf.states.length);
    for (let i = 0; i < golden.uf.states.length; i++) {
      strictEqual(stateAbbreviation(states[i]), golden.uf.states[i].abbreviation);
      strictEqual(stateName(states[i]), golden.uf.states[i].name);
    }
  });

  describe("from_abbreviation", () => {
    for (const c of golden.uf.from_abbreviation) {
      it(c.input || "(empty)", () => {
        const s = stateFromAbbreviation(c.input);
        strictEqual(s != null ? stateAbbreviation(s) : null, c.expected);
      });
    }
  });
});

describe("Municipio", () => {
  it("count", () => strictEqual(municipioCount(), golden.municipio.count));

  describe("from_ibge_code", () => {
    for (const c of golden.municipio.from_ibge_code) {
      it(String(c.code), () => {
        const m = municipioFromIbgeCode(c.code);
        if (c.expected === null) {
          strictEqual(m, null);
        } else {
          strictEqual(m.name, c.name);
          strictEqual(stateAbbreviation(m.state), c.state);
          strictEqual(m.isCapital, c.is_capital);
        }
      });
    }
  });

  describe("capital_of", () => {
    for (const c of golden.municipio.capital_of) {
      it(c.state, () => {
        const cap = municipioCapitalOf(stateFromAbbreviation(c.state));
        strictEqual(cap.name, c.name);
        strictEqual(cap.ibgeCode, c.ibge_code);
      });
    }
  });

  describe("search_by_name", () => {
    for (const s of golden.municipio.search_by_name) {
      it(s.query, () => {
        const results = municipioSearchByName(s.query);
        strictEqual(results.length, s.results.length);
        for (let i = 0; i < s.results.length; i++) {
          strictEqual(results[i].ibgeCode, s.results[i].ibge_code);
          strictEqual(results[i].name, s.results[i].name);
        }
      });
    }
  });

  describe("by_state_count", () => {
    for (const c of golden.municipio.by_state_count) {
      it(c.state, () => {
        strictEqual(municipiosByState(stateFromAbbreviation(c.state)).length, c.count);
      });
    }
  });
});
