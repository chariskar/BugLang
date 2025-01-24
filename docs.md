# Interpreter Basics

This document outlines the core features and syntax of the custom interpreter.

---

## 1. Variable Declaration

### Syntax:
```plaintext
variable <var_name> = <value>;
```

- Declares a variable and assigns an initial value.
- Supported types: `int`, `float`, `boolean`, and `string`.

### Example:
```plaintext
variable x = 10;
variable name = "Alice";
variable isReady = true;
```

### Errors:
- **Incorrect Syntax**: `Error: Incorrect variable declaration syntax.`

---

## 2. Variable Updates

### Syntax:
```plaintext
<var_name>++;
<var_name>--;
<var_name> += <value>;
<var_name> -= <value>;
```

- Supports increment, decrement, and basic arithmetic operations for `int` and `float`.

### Example:
```plaintext
x++;
y--;
z += 5;
w -= 2;
```

### Errors:
- **Undefined Variable**: `Error: Tried updating a non-existing variable.`
- **Invalid Type**: `Error: Invalid variable type.`

---

## 3. Print Statements

### Syntax:
```plaintext
print "<string>";
print <var_name>;
```

- Outputs string literals or variable values to the console.

### Example:
```plaintext
print "Hello, World!";
print x;
```

### Errors:
- **Undefined Variable**: `Error: Undefined variable '<var_name>'.`
- **Invalid Syntax**: `Error: invalid print syntax.`

---

## 4. If Statements

### Syntax:
```plaintext
if (<condition>) {
    // block of code
}
```

- Executes the block if the condition evaluates to `true`.

### Example:
```plaintext
if (x > 5) {
    print "x is greater than 5";
}
```

### Errors:
- **Invalid Condition**: `Error: Invalid condition.`

---

## 5. While Loops

### Syntax:
```plaintext
while (<condition>) {
    // block of code
}
```

- Executes the block repeatedly while the condition is `true`.

### Example:
```plaintext
while (x > 0) {
    print x;
    update x--;
}
```

### Errors:
- **Invalid Condition**: `Error: Invalid condition.`

---

## Example Program

```plaintext
variable x = 10;

while (x > 0) {
    print x;
    update x--;
}

print "Done!";
```

**Output**:
```plaintext
10
9
8
7
6
5
4
3
2
1
0
Done!
```

### Note: the only variables you can update rn are integers and it has to be done using: update varName++/--/+=/-=
