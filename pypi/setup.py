from setuptools import setup, find_packages

setup(
    name="simvcce",
    version="1.0.0",
    author='Cheng Maohuaa',
    author_email='cmh@seu.edu.cn',
    packages=find_packages(),
    install_requires=["CoolProp", "phyprops"],
    description='SimVCCE',
)
