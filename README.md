# winreg-rs
writen by rust and [winreg](https://docs.rs/winreg/0.50.0/winreg/enums/index.html)

## install

yarn add @camol/winreg-rs

## usage

### getKeyValue(key: string, name: string): string | null
```
const { JsRegistry, HKLM } = require("@camol/winreg-rs");

const registry = new JsRegistry(HKLM);

const value = registry.getKeyValue("Software\\Classes\\.docx","")
console.log({value})
```

### getValues(key: string): string[] | null
```
const { JsRegistry, HKLM } = require("@camol/winreg-rs");

const registry = new JsRegistry(HKLM);

const values = registry.getValues("Software\\Classes\\.docx")
console.log({values})
```

### getKeys(key: string): string[] | null
```
const { JsRegistry, HKLM } = require("@camol/winreg-rs");

const registry = new JsRegistry(HKLM);

const keys = registry.getKeys("Software\\Classes\\.docx")
console.log({keys})
```