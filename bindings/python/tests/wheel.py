import importlib.metadata
import unittest

import stdbr


class TestInstalledWheel(unittest.TestCase):
    def test_typing_files_are_installed(self):
        files = {str(path) for path in importlib.metadata.files("stdbr") or ()}
        self.assertIn("stdbr/stdbr.pyi", files)
        self.assertIn("stdbr/py.typed", files)

    def test_native_api(self):
        cpf = stdbr.Cpf.parse("529.982.247-25")
        self.assertEqual(cpf.as_str(), "52998224725")
        self.assertTrue(stdbr.cpf_is_valid(cpf.as_str()))


if __name__ == "__main__":
    unittest.main()
