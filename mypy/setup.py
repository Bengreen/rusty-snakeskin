from setuptools import setup


setup(
    name="mypy",
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
    packages=[
        "mypy"
    ],
    install_requires=[
        "asyncio"
    ],
    entry_points={
        'console_scripts': [
            'myscript = mypy.myscript:cli',
        ],
    },
    include_package_data=True,
    zip_safe=False,
    extras_require={
        'dev': [
            'pytest>=3.5.0',
        ]
    }
)

