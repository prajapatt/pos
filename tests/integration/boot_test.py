import re
import unittest
from pathlib import Path


class BootInfoContractTests(unittest.TestCase):
    def test_boot_contract_constants_are_present(self):
        text = Path("boot/common/boot_info.rs").read_text(encoding="utf-8")

        self.assertRegex(text, r"BOOT_INFO_VERSION:\s*u32\s*=\s*1")
        self.assertRegex(text, r"MEMORY_USABLE:\s*u32\s*=\s*7")
        self.assertRegex(text, r"BOOT_MAGIC:\s*u64\s*=\s*0x4348_5554_4f53_424f")

    def test_boot_info_header_has_expected_version_and_size_rules(self):
        text = Path("boot/common/boot_info.rs").read_text(encoding="utf-8")

        self.assertIn("version == BOOT_INFO_VERSION", text)
        self.assertIn("self.size >= core::mem::size_of::<Self>() as u32", text)


if __name__ == "__main__":
    unittest.main()
