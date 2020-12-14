MMC Compiler

3AC generator for the following tuple:

Source Language : mmc
Implementation Language : Rust
Target Language: MIPS

## Build

```
# ./cargo build
```

## Play with REPL


## Code Gen

Our code generator goes through two separate stages: generating three-address
code, and generating real assembly from that. Our three-address code generator
("IntermediateGenerator.java") seems to be working great! It outputs code that
is already very close to the assembly we want.

Interesting design decisions include the fact that we decided to have two stages
to our generator: three-address code, then assembly code for MIPS. Additionally,
our compiler has the ability to print to STDOUT instead of a file (if no output
file is specified). Also, all variables and temps are maintained DIRECTLY ON THE
STACK, making register management very easy ;)

The evaluator consists of the following pipeline:
``````
   Representation    Stage        Important classes
   --------------    -----        -----------------

      source code                  String
          |
          |
          o--------- lexer         mmcrust.lexer.{Lexer,LookaheadLexer}
          |
          v
       tokens                      mmcrust.lexer.Token
          |
          |
          o--------- parser        mmcrust.parser.Parser
          |
          v
  abstract syntax tree             mmcrust.ast.{Expression, Statement}
          |
          |
          o--------- type checker  mmcrust.types.{Type, TypeChecker}, mmcrust.env.{StaticEnvironment, Binding}
          |
          v
    typed syntax tree              mmcrust.types.{TypedExpression, TypedStatement}
          |
          |
          o--------- optimizer     mmcrust.optimizer.ASTOptimizer
          |
          v
    typed syntax tree              (same as above)
          |
          |
          o--------- translator    mmcrust.translator.Translator
          |
          v          
      IR opcodes                   mmcrust.translator.{IR, BasicBlock}
          |
          |
          o--------- peephole      mmcrust.optimizer.peepholeOptimize
          |          optimization
          |  
          v          
      IR opcodes                   (same as above)
          |
          |
          o------- IR translation  mmcrust.translator.TranslateIRToOpCodes
          |  
          v          
    stack vm opcodes               mmcrust.vm.{OpCode, CodeSegment}
          |
          |
          o--------- evaluator     mmcrust.vm.{Evaluator, ThreadState, DataSegment}
          |
          v
        values                     mmcrust.objects.Value
``````
## Features
It supports the following commands

- PRINT
- LET
- INPUT
- GOTO
- DO LOOP
- DO WHILE LOOP
- FOR LOOP (with and without step size)
- If THEN construct
- If ELSE construct
