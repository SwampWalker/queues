from os import path as path


def find_target() -> str:
    cd = path.abspath(".")
    maybe = path.join(cd, "target")
    while not path.exists(maybe):
        cd = path.abspath(path.join(cd, ".."))
        maybe = path.join(cd, "target")
    return maybe