from setuptools import setup, find_namespace_packages
from setuptools_rust import RustExtension, Binding


setup(
    name="service-module",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
    ],
    packages=["service_module"],
    # packages=find_namespace_packages(include=["service_module.*"]),
    rust_extensions=[
        RustExtension(
            "service_module.rust",
            # path="Cargo.toml",
            binding=Binding.PyO3,
            # quiet=False,
            )
        ],
    install_requires=[
        "asyncclick",
        "anyio"
    ],
    entry_points={
        'console_scripts': [
            'yourscript = service_module.yourscript:cli',
        ],
    },
    include_package_data=True,
    zip_safe=False,
    extras_require={
        'dev': [
            'pytest>=3.5.0',
            'setuptools_rust~=0.12.0',
            'pytest-aiohttp',
        ]
    }
)

