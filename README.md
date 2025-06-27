## Solfunmeme Dioxus 

# todo 

### Idea

split each declaration to own file
vectorize -> emit code from vector
vector = code = binary 
executable vectors
look for duplicates
cannonical directories

### auto populate menu
### embed code in app
### embed json version of code in app
### embed bert embedding of code and json in app
### create emojis for every item in app

idea

1. dynamically generate the menu
2. to do so query the code at runtime
3. to do so embed the source in the app
4. parse the source with syn to json
5. embed the source and the json into the app
6. later we will vectorize both into a dimensional structure. use dummy embedding for now.
7. then we assign an emoji to each item.
8. then we can visualze the code as a hyperspace. 

# Status

1. setting up testing and coverage for code
2. removing warnings
3. removing unused code
4. creating coverage

## Functionality

### Login with wallet
#### Create secrets for access to apis
#### Encrypt secrets locally with secondary password

#### Store secrets on aws ssm parameters
##### AWS Credentials
##### Github
##### Huggingface
##### Openstreetmap
##### Archive.org
##### Wikidata
##### Codeberg
##### Gitlab
##### Gitgud

##### AI Credentials
###### Grok
###### Groq
###### OpenAI
###### Google
###### Bedrock
##### Solana
##### SSH Keys

#### Store public data
##### SSH keys
##### On solana
##### On git 

#### Interpreting data :

### Formats
#### JSON
#### xml schema
#### turtle/rdf
#### Emojis
#### Tickets
#### Org Mode
#### Markdown  - pandoc
#### Wikidata
#### Wikimedia
#### JPG/PNG
#### Video
#### Trace
#### Corefile
#### Elf
#### Dwarf
#### Nix/Guix
#### Github Actions

### Languages
#### Rust
#### Lean
#### ocaml
#### coq
#### metcoq
#### haskell,
#### terraform
#### Cloudformation
#### Bash/Shellcheck 
#### LLVM/IR
#### ASM
#### GNU/mes
#### Guile Schema
#### Nix
#### Dockerfile
#### Python
#### Mojo

### Theories
#### Category Thoery
#### Group Thoery
#### HOTT
Univalent foundations
Unimath
Paths as proofs. 
Rewrite transformation from one form to antother.
lean4 <-> coq
8 level of abstraction. 
goedel one level number. 

#### Ast
#### Bott periodicty
#### Vector Bundles
#### Lie Groups
#### Memes
#### Semiotics
#### Programming Language

#### Open Source/Open Knowledge
#### Biosemiotics
#### Incompleteness
#### Representation
#### Complexity

#### Self hosting
#### Reproducible
#### Secure
#### Audited
#### Declarative
#### Emergent
#### Omni Combinator

### low value :
1. pay for promotion.
give out coins.
calls 
bots telegram
twitter
pay for meetings.
going to conferences
buying dinner.
marketing - p2p - grass roots.

2. communication, style. loner.
ideosycretic. 
old - offline. rotary phone. zk84 sinclar. 1984. 
cassette tapes.
trending, copying. 
old school.
germany - sceptical.

3. old school ai
prolog, lisp, 
compiler ai
emacs gnu

4. worldcom- 
fraud. 2003,2007, 
sick hype.
open honest

5. loyal to holder. 
long term. months and year.

6. value:
sell consulting time via our token, consult and help the holder,
services would be provided to holders.
longer you hold the more credits you earn , sell, swap or use those credits.

7. hosting providers 
rent ip and ports at market rate. 
mine tokens into existence over time. 

eventually we swap tokens for real solana on the rollup. 

## Systems

### pump fun
### open sea
nft marketplaces
### githosting.
### telegram, discord, x,
### matrix, mastodon
#### BBS 
#### PDP 11, 8 bit, 128kb.
####  Browser + server + homelab p2p2 

### WIkipedia
### Wikidata
### Linux Project

### Bittensor
### Bitcoin
### Solana
### Solana- sidechains

#### distribute batch vectorizing large projects
reducing the transfer of data between node

## Ideas

### Value - free tier (aws), compress solana, 

Other memescoins, provide service. 
hosting providers will earn coins credits. run gpu. 
your free tier groq, 
social credit score. 

### Artists create and swap value as solana tokens.
Instead of revealing the art they produce zkp of the value, like a blind bag.
public reduced quality, short clips or description.
human + ai art critique that you can trust. 
license usage of custom models with quality control. 
Consumers can share if they want, or keep for themselves.

### code quality
vibe code generation with quality. 

human first, ai second. 
no ai government, dao. Ai support and help the decision making process. 
quality controls. 

Game engine. 
dex - distributed excchnage. 
atom swaps - prisoner exachange.
This many tokens for that many. 

## Introspection idea.
mathlib -> lean4 -> json -> memes 

#### Like 
### Declarations are transactions, semiosis, biosemiosis
The gut feeling, this emergent thing, never finished, self modifiying system.
adjusting, modifying, moving foward. 

Interpretion of the data in the system will evolve. 

### Types are equilibrium between systems
### Memes are recorded as chats, issues, code, git commits, 
### Compilers compile and interpret language expressions.
### All the data can be stored in our solana sidechain on rocksb (what solana uses internally)
### Memes are lean4 expressions that when executed can produce text or images or other values, the evaluation.
binding of parmeters to other memes or contract addresess to compose memes.
### Solana sidechain can rollup into mainnet via compression, 
storing results immutably in public and referencing the results on mainnet via zkp



# Older stuff

See:
https://stackoverflow.com/questions/55912871/how-to-work-with-openssl-for-rust-within-a-windows-development-environment


```
   
   $env:VCPKG_ROOT="C:\Users\gentd\OneDrive\Documents\GitHub\vcpkg"
   vcpkg  install openssl 
   vcpkg.exe install openssl:x64-windows
   vcpkg.exe install openssl:x64-windows-static
   vcpkg.exe integrate install
   set VCPKGRS_DYNAMIC=1
  
   $env:OPENSSL_DIR="C:\Users\gentd\OneDrive\Documents\GitHub\vcpkg\installed\x64-windows-static"
   cargo build

or in bash
    export OPENSSL_DIR="/c/Users/gentd/OneDrive/Documents/GitHub/vcpkg/installed/x64-windows-static"
    
```



### Requirements
1. This template relies on Tailwind CSS to generate the stylesheet. 

Install the standalone Tailwind CLI - [https://tailwindcss.com/docs/installation/tailwind-cli](https://tailwindcss.com/docs/installation/tailwind-cli)
2. Install Dioxus cli from official website - [https://dioxuslabs.com/](https://dioxuslabs.com/)


### Running the dev server
1. Start the tailwind CLI within the Root of the directory
    ```sh
    tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
    ```
2. Start the Dioxus CLI
    ```sh
    dx serve
    ```

- Open the browser at default port http://localhost:8080 or the port described by Dioxus CLI in case port `8080` was already in use

- Sometimes there are warning in the browser console, use `dx check` command to find if there are fixes that need to be done.


### Plan

#### Client side interpretation.
#### Server side storage
The first memes are just free text stored as instructions in the blockchain
each one can be seen as a rust program that when executed produces some text or json or url
We can parameterize those. the programs can be updated by creating new versions of them
because we are on a sidechain we can rewrite or garbage collect them.

##### Import Git
we can use git submodules or git urls like package managers do,

###### Import Data via git
we can import telegram, discord, twitter, github, ai chat logs, files into the system.
each sidechain becomes its own meme database with all the transactions around a meme coin ecosystem.
see the time repo for example.

######## A tweet becomes a program or meme, 
an anchor is the code.

#### Forking all repos into our chain
#### Copying all data into our archives
#### Running all code on our chain
#### Proving all code to be valid.
#### Showing execution of all paths of the code
using dfa and parsers for the emoji language. 
each version of the meme state is a new version of the language and library.
we hope to construct that each path in the system is a unique word and a program at the same time that has a meaning in emojis.
#### Mathematical modeling of the structures (groups, hott, etc)
#### Using of AI outside the system, storing results on the chain.
##### AI via client side inferences
###### looking at Invoke AI for example
###### looking at Kobold AI for example
###### calling typescript from rust wasm dioxus (calling eliza?)
#### Tracing the AI inference into the model.


#### Convert this source to json
Createing json  using https://github.com/meta-introspector/syn-serde-rust.git

 ```
    cd .\syn-serde-rust\
    cargo build
    cd .\examples\rust2emoji\
    cargo build
    cargo run ..\..\..\solfunmeme-dioxus\
```

#### Reading in source of code via reflection. splitting into chunks. saving in the blockchain.
#### interpreting the functions as json or emojis
#### linking functions together creating systems as groups of contracts.
#### embedding contrats as vectors


## Multiple visualization
### ai convergence on models
#### PCA
#### Threading of dimensions
#### Area or surface of points of the spaces.
#### connecting solana code to memes to llms
#### read git history
sample source code into rust
be able to read git repor (store older git objects or pack files into exec.


# More ideas
game engine 
simulation engine
vector of code 
vector -> code 
make each possible value a valid program, no invalid program states possible.
encode program state into a vector.


solana 

# remove 
unimplemented
"#FIXME"

# code coverage
```
RUSTFLAGS="-C instrument-coverage" cargo build
$env:RUSTFLAGS="-C instrument-coverage"
set RUSTFLAGS="-C instrument-coverage" 
```


cargo tarpauline
cargo llvm-cov
