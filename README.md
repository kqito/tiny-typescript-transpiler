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

- [ ] **BinaryExpression**: Support for binary operations like 1 + 1 or foo - bar.
- [ ] **UnaryExpression**: Support for unary operations like !true or -1.
- [ ] **IfStatement**: Support for if, else if, and else statements.
- [ ] **SwitchStatement**: Support for switch statements.
- [ ] **ForStatement**: Support for for loop.
- [ ] **WhileStatement**: Support for while loop.
- [ ] **DoWhileStatement**: Support for do while loop.
- [ ] **ArrayLiteralExpression**: Support for array literals like [1, 2, 3].
- [ ] **ObjectLiteralExpression**: Support for object literals like { foo: 'bar' }.
- [ ] **ClassDeclaration**: Support for class definitions.
- [ ] **InterfaceDeclaration**: Support for interface definitions.
- [ ] **EnumDeclaration**: Support for enum definitions.
- [ ] **ModuleDeclaration**: Support for module definitions.
- [ ] **ImportDeclaration**: Support for import statements.
- [ ] **ExportDeclaration**: Support for export statements.
- [ ] **ArrowFunction**: Support for arrow functions.
- [ ] **TemplateLiteral**: Support for template literals.
- [ ] **OptionalChaining**: Support for optional chaining.
- [ ] **NullishCoalescing**: Support for nullish coalescing.

## License
[MIT © kqito](./LICENSE)
