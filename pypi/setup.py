from setuptools import setup, find_packages

setup(
    name="simvcce",
    version="1.0.2",
    author='Cheng Maohuaa',
    author_email='cmh@seu.edu.cn',
    packages=find_packages(),
    install_requires=["CoolProp", "phyprops"],
    url="https://github.com/thermalogic/SimVCCE",
    description='SimVCCE: The vapor-compression refrigeration cycle steady-state simulator for education',
    long_description=open("README.md", "r", encoding="utf8").read(),
    long_description_content_type="text/markdown",
    platforms=["Windows64", "Linux64"],
    classifiers=[
        "Programming Language :: Python :: 3",
        "Programming Language :: C",
        "Topic :: Scientific/Engineering",
        "Topic :: Scientific/Engineering :: Chemistry",
        "Topic :: Scientific/Engineering :: Physics",
        "Topic :: Software Development :: Libraries :: Python Modules"
    ]
)
