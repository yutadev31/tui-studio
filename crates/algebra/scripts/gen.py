#!/bin/python3

SRC = "./src"
VEC2_DIR = f"{SRC}/vec2"
SCRIPTS = "./scripts"

type_list = ["8", "16", "32", "64", "size"]

def uvec2(size: str):
  with open("./scripts/uvec2.template", "r", encoding="utf-8") as f:
    template = f.read()

  return template.replace("%VALUE_TYPE%", "u" + size).replace("%I_TYPE_MODULE%", "i" + size).replace("%I_TYPE_NAME%", "I" + size.capitalize() + "Vec2").replace("%TYPE_NAME%", "U" + size.capitalize() + "Vec2")

def ivec2(size: str):
  with open("./scripts/ivec2.template", "r", encoding="utf-8") as f:
    template = f.read()

  return template.replace("%VALUE_TYPE%", "i" + size).replace("%TYPE_NAME%", "I" + size.capitalize() + "Vec2")

mod_rs = ""

for type in type_list:
  with open (f"{VEC2_DIR}/u{type}.rs", "w", encoding="utf-8") as f:
    mod_rs += f"pub mod u{type};\n"
    f.write(uvec2(type))

  with open (f"{VEC2_DIR}/i{type}.rs", "w", encoding="utf-8") as f:
    mod_rs += f"pub mod i{type};\n"
    f.write(ivec2(type))

with open ("./src/vec2/mod.rs", "w", encoding="utf-8") as f:
  f.write(mod_rs)
