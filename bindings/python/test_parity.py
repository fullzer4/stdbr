import json
import os
import unittest
from pathlib import Path

import stdbr

GOLDEN_PATH = os.environ.get("GOLDEN_JSON") or str(
    Path(__file__).resolve().parent.parent.parent / "tests" / "parity" / "golden.json"
)
GOLDEN = json.loads(Path(GOLDEN_PATH).read_text())


class TestCpf(unittest.TestCase):
    def test_parse(self):
        for c in GOLDEN["cpf"]["parse"]:
            with self.subTest(input=c["input"]):
                cpf = stdbr.Cpf.parse(c["input"])
                self.assertEqual(cpf.as_str(), c["digits_only"])
                self.assertEqual(cpf.formatted(), c["formatted"])
                self.assertEqual(cpf.masked(), c["masked"])
                self.assertEqual(int(cpf.fiscal_region), c["fiscal_region"])
                self.assertEqual(cpf.check_digits[0], c["check_digits"][0])
                self.assertEqual(cpf.check_digits[1], c["check_digits"][1])

    def test_is_valid(self):
        for c in GOLDEN["cpf"]["is_valid"]:
            with self.subTest(input=c["input"]):
                self.assertEqual(stdbr.cpf_is_valid(c["input"]), c["expected"])

    def test_is_valid_strict(self):
        for c in GOLDEN["cpf"]["is_valid_strict"]:
            with self.subTest(input=c["input"]):
                if c["valid"]:
                    stdbr.cpf_is_valid_strict(c["input"])
                else:
                    with self.assertRaises(ValueError) as ctx:
                        stdbr.cpf_is_valid_strict(c["input"])
                    self.assertEqual(str(ctx.exception), c["error"])

    def test_format(self):
        for c in GOLDEN["cpf"]["format"]:
            with self.subTest(input=c["input"]):
                self.assertEqual(stdbr.cpf_format(c["input"]), c["expected"])

    def test_remove_symbols(self):
        for c in GOLDEN["cpf"]["remove_symbols"]:
            with self.subTest(input=c["input"]):
                self.assertEqual(stdbr.cpf_remove_symbols(c["input"]), c["expected"])

    def test_compute_check_digits(self):
        for c in GOLDEN["cpf"]["compute_check_digits"]:
            with self.subTest(base=c["base"]):
                result = stdbr.cpf_compute_check_digits(c["base"])
                if c["expected"] is None:
                    self.assertIsNone(result)
                else:
                    self.assertEqual(result[0], c["expected"][0])
                    self.assertEqual(result[1], c["expected"][1])

    def test_generate_roundtrip(self):
        raw = stdbr.cpf_generate()
        self.assertTrue(stdbr.cpf_is_valid(raw))
        self.assertEqual(stdbr.Cpf.parse(raw).as_str(), raw)


class TestCnpj(unittest.TestCase):
    def test_parse(self):
        for c in GOLDEN["cnpj"]["parse"]:
            with self.subTest(input=c["input"]):
                cnpj = stdbr.Cnpj.parse(c["input"])
                self.assertEqual(cnpj.as_str(), c["digits_only"])
                self.assertEqual(cnpj.formatted(), c["formatted"])
                self.assertEqual(cnpj.masked(), c["masked"])
                self.assertEqual(cnpj.raiz(), c["raiz"])
                self.assertEqual(cnpj.ordem(), c["ordem"])
                self.assertEqual(int(cnpj.kind), c["kind"])
                self.assertEqual(int(cnpj.establishment_type), c["establishment_type"])
                self.assertEqual(cnpj.check_digits[0], c["check_digits"][0])
                self.assertEqual(cnpj.check_digits[1], c["check_digits"][1])

    def test_is_valid(self):
        for c in GOLDEN["cnpj"]["is_valid"]:
            with self.subTest(input=c["input"]):
                self.assertEqual(stdbr.cnpj_is_valid(c["input"]), c["expected"])

    def test_is_valid_strict(self):
        for c in GOLDEN["cnpj"]["is_valid_strict"]:
            with self.subTest(input=c["input"]):
                if c["valid"]:
                    stdbr.cnpj_is_valid_strict(c["input"])
                else:
                    with self.assertRaises(ValueError) as ctx:
                        stdbr.cnpj_is_valid_strict(c["input"])
                    self.assertEqual(str(ctx.exception), c["error"])

    def test_format(self):
        for c in GOLDEN["cnpj"]["format"]:
            with self.subTest(input=c["input"]):
                self.assertEqual(stdbr.cnpj_format(c["input"]), c["expected"])

    def test_remove_symbols(self):
        for c in GOLDEN["cnpj"]["remove_symbols"]:
            with self.subTest(input=c["input"]):
                self.assertEqual(stdbr.cnpj_remove_symbols(c["input"]), c["expected"])

    def test_compute_check_digits(self):
        for c in GOLDEN["cnpj"]["compute_check_digits"]:
            with self.subTest(base=c["base"]):
                result = stdbr.cnpj_compute_check_digits(c["base"])
                if c["expected"] is None:
                    self.assertIsNone(result)
                else:
                    self.assertEqual(result[0], c["expected"][0])
                    self.assertEqual(result[1], c["expected"][1])

    def test_generate_roundtrip(self):
        raw = stdbr.cnpj_generate(stdbr.CnpjKind.Numeric)
        self.assertTrue(stdbr.cnpj_is_valid(raw))
        self.assertEqual(stdbr.Cnpj.parse(raw).as_str(), raw)


class TestCep(unittest.TestCase):
    def test_parse(self):
        for c in GOLDEN["cep"]["parse"]:
            with self.subTest(input=c["input"]):
                cep = stdbr.Cep.parse(c["input"])
                self.assertEqual(cep.as_str(), c["digits_only"])
                self.assertEqual(cep.formatted(), c["formatted"])
                self.assertEqual(cep.masked(), c["masked"])
                self.assertEqual(int(cep.postal_region), c["postal_region"])
                state = cep.state
                abbr = stdbr.state_abbreviation(state) if state is not None else None
                self.assertEqual(abbr, c["state"])

    def test_is_valid(self):
        for c in GOLDEN["cep"]["is_valid"]:
            with self.subTest(input=c["input"]):
                self.assertEqual(stdbr.cep_is_valid(c["input"]), c["expected"])

    def test_is_valid_strict(self):
        for c in GOLDEN["cep"]["is_valid_strict"]:
            with self.subTest(input=c["input"]):
                if c["valid"]:
                    stdbr.cep_is_valid_strict(c["input"])
                else:
                    with self.assertRaises(ValueError) as ctx:
                        stdbr.cep_is_valid_strict(c["input"])
                    self.assertEqual(str(ctx.exception), c["error"])

    def test_format(self):
        for c in GOLDEN["cep"]["format"]:
            with self.subTest(input=c["input"]):
                self.assertEqual(stdbr.cep_format(c["input"]), c["expected"])

    def test_remove_symbols(self):
        for c in GOLDEN["cep"]["remove_symbols"]:
            with self.subTest(input=c["input"]):
                self.assertEqual(stdbr.cep_remove_symbols(c["input"]), c["expected"])

    def test_generate_roundtrip(self):
        raw = stdbr.cep_generate()
        self.assertTrue(stdbr.cep_is_valid(raw))
        self.assertEqual(stdbr.Cep.parse(raw).as_str(), raw)


class TestUf(unittest.TestCase):
    def test_states(self):
        states = stdbr.all_states()
        self.assertEqual(len(states), len(GOLDEN["uf"]["states"]))
        for i, g in enumerate(GOLDEN["uf"]["states"]):
            with self.subTest(i=i):
                self.assertEqual(stdbr.state_abbreviation(states[i]), g["abbreviation"])
                self.assertEqual(stdbr.state_name(states[i]), g["name"])

    def test_from_abbreviation(self):
        for c in GOLDEN["uf"]["from_abbreviation"]:
            with self.subTest(input=c["input"]):
                s = stdbr.state_from_abbreviation(c["input"])
                result = stdbr.state_abbreviation(s) if s is not None else None
                self.assertEqual(result, c["expected"])


class TestMunicipio(unittest.TestCase):
    def test_count(self):
        self.assertEqual(stdbr.municipio_count(), GOLDEN["municipio"]["count"])

    def test_from_ibge_code(self):
        for c in GOLDEN["municipio"]["from_ibge_code"]:
            with self.subTest(code=c["code"]):
                m = stdbr.municipio_from_ibge_code(c["code"])
                if c.get("expected") is None and "name" not in c:
                    self.assertIsNone(m)
                else:
                    self.assertEqual(m.name, c["name"])
                    self.assertEqual(stdbr.state_abbreviation(m.state), c["state"])
                    self.assertEqual(m.is_capital, c["is_capital"])

    def test_capital_of(self):
        for c in GOLDEN["municipio"]["capital_of"]:
            with self.subTest(state=c["state"]):
                cap = stdbr.municipio_capital_of(stdbr.state_from_abbreviation(c["state"]))
                self.assertEqual(cap.name, c["name"])
                self.assertEqual(cap.ibge_code, c["ibge_code"])

    def test_search_by_name(self):
        for s in GOLDEN["municipio"]["search_by_name"]:
            with self.subTest(query=s["query"]):
                results = stdbr.municipio_search_by_name(s["query"])
                self.assertEqual(len(results), len(s["results"]))
                for i, e in enumerate(s["results"]):
                    self.assertEqual(results[i].ibge_code, e["ibge_code"])
                    self.assertEqual(results[i].name, e["name"])

    def test_by_state_count(self):
        for c in GOLDEN["municipio"]["by_state_count"]:
            with self.subTest(state=c["state"]):
                st = stdbr.state_from_abbreviation(c["state"])
                self.assertEqual(len(stdbr.municipios_by_state(st)), c["count"])


if __name__ == "__main__":
    unittest.main()
