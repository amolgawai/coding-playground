# Haskell Development setup and workflow

## Installation
1. Install (GHCup)[https://www.haskell.org/ghcup] - This will install the Haskell compiler(ghc) and interactive shell(ghci)
2. Use GHCup to install other essentials such as
   1. [haskell-language-server](https://haskell-language-server.readthedocs.io/en/stable/index.html)
   2. [The Haskell Tool Stack](https://docs.haskellstack.org/en/stable/#__tabbed_1_2)
   3. [The Haskell Cabal | Overview](https://www.haskell.org/cabal/) (which should get installed with GHCup)

## Build System - Haskell Stack
Use Haskell stack as a build system to get started with project and manage a reproducible environment.
See [The Haskell Tool Stack quick start guide](https://docs.haskellstack.org/en/stable/#quick-start-guide) to get started

## Testing
1. [Hspec: A Testing Framework for Haskell](https://hspec.github.io/)
2. [doctest](https://hackage.haskell.org/package/doctest)
3. [hedgehog](https://hackage.haskell.org/package/hedgehog) - property based testing
   1. [hspec-hedgehog](https://hackage.haskell.org/package/hspec-hedgehog) - Integrate into Hspec
4. [tasty: alternative for Hspec](https://github.com/UnkindPartition/tasty)