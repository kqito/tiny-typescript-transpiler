# Tiny-Typescript-Transpiler
Tiny-Typescript-Transpiler is a simple learning library for transpilers like swc, excluding binder and checker.

> [!IMPORTANT]
> This library is only for learning the overview of transpiler.

## Concept
This library is designed as a simplified learning tool for understanding how transpilers like swc work. It does not include the complexity of binders and checkers.

## Supported Syntax
Currently, this library supports the following syntax:
- CallExpression
  - Example: `fn();`
- Function Declaration
  - Example: `function fn () { var num: number = 100; const hi = "hello"; }`
  - Example: `function alertMessage (message: string) { alert(message); }`
- Type Alias Declaration
  - Example: `type str = string;`
- Variable Declaration
  - Example: `const hi: str = "hello";`

## Usage
To use the Tiny-Typescript-Transpiler, use the following command:

```bash
ttt [OPTIONS] <FILE_PATH>
```

### Arguments
- <FILE_PATH>: The path to the file you want to transpile.

### Options
- -d, --debug: Enable debug mode.
- -h, --help: Print help information.

## Future Syntax Implementations (TODO)
The following are some of the syntax that we plan to support in the future:

- [ ] BinaryExpression: 1 + 1 or foo - bar
- [ ] IfStatement: if, else if, and else statements
- [ ] ForStatement: for loop
- [ ] WhileStatement: while loop
- [ ] ArrayLiteralExpression: [1, 2, 3]
- [ ] ObjectLiteralExpression: { foo: 'bar' }
- [ ] ClassDeclaration: Class definitions
- [ ] InterfaceDeclaration: Interface definitions
- [ ] EnumDeclaration: Enum definitions
- [ ] ModuleDeclaration: Module definitions

## License
[MIT © kqito](./LICENSE)
