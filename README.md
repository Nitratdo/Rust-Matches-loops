# 🦀 Mini Projeto em Rust — match e Estruturas de Repetição

Este mini projeto em **Rust** apresenta os principais conceitos de **controle de fluxo**, utilizando `match`, intervalos em `match`, e estruturas de repetição como `for`, `while` e `loop`.

O código é dividido em desafios práticos e comentados, ideal para quem está aprendendo Rust e deseja entender como tomar decisões e repetir ações no programa.

---

## 📌 Objetivos do Projeto

* Aprender a usar o `match` em Rust
* Trabalhar com padrões e intervalos (`..=`)
* Utilizar o caractere `_` como caso padrão
* Conhecer os loops `for`, `while` e `loop`
* Entender como interromper loops com `break`

---

## 📄 Código Fonte

```rust
fn main() {
    // ================= Desafio 1 — match simples =================
    let dia = 5;

    match dia {
        1 => println!("Domingo"),
        2 => println!("Segunda"),
        3 => println!("Terça"),
        4 => println!("Quarta"),
        5 => println!("Quinta"),
        6 => println!("Sexta"),
        7 => println!("Sabado"),
        _ => println!("Dia invalido"),
    };

    // ================= Desafio 2 — match com intervalos =================
    let nota = 9;

    let resultado = match nota {
        0..=4 => "Reprovado",
        4..=5 => "Recuperação",
        6..=10 => "Aprovado",
        _ => "Nota invalida",
    };

    println!("O resultado é : {}", resultado);

    // ================= Desafio 3 — for =================
    for i in 0..=10 {
        println!("i = {}", i);
    }

    // ================= Desafio 4 — while =================
    let mut contador = 5;

    while contador < 0 {
        println!("contador: {}", contador);
        contador -= 1;
    }

    println!("Chegou a zero!: {}", contador);

    // ================= Desafio 5 — loop =================
    for i in 0..=9 {
        println!("i : {}", i)
    };

    let resultado = loop {
        let y = 9;
        break y / 3;
    };

    println!("Parou em : {}", resultado);
}
```

---

## 🧠 Explicação dos Desafios

### 🔹 Desafio 1 — `match` simples

* Compara o valor da variável `dia`
* Associa números aos dias da semana
* Usa `_` como caso padrão (equivalente ao `else`)

---

### 🔹 Desafio 2 — `match` com intervalos

* Utiliza intervalos numéricos (`0..=4`, `6..=10`)
* Classifica a nota como reprovado, recuperação ou aprovado
* Demonstra o uso do `match` como expressão

---

### 🔹 Desafio 3 — `for`

* Loop que percorre um intervalo definido
* Executa de 0 até 10 (inclusive)

---

### 🔹 Desafio 4 — `while`

* Loop baseado em condição
* Utiliza variável mutável
* Demonstra controle manual do contador

> Observação: neste exemplo, a condição do `while` não é satisfeita, então o loop não executa.

---

### 🔹 Desafio 5 — `loop`

* Loop infinito controlado por `break`
* Retorna um valor ao ser interrompido
* Mostra que `loop` também pode ser uma expressão

---

## ▶️ Como Executar o Projeto

1. Verifique se o Rust está instalado:

   ```bash
   rustc --version
   ```

2. Crie um novo projeto:

   ```bash
   cargo new controle_fluxo_rust
   ```

3. Substitua o conteúdo do arquivo `src/main.rs` pelo código acima

4. Execute:

   ```bash
   cargo run
   ```

---

## 🖨️ Saída Esperada (Exemplo)

```
Quinta
O resultado é : Aprovado
i = 0
i = 1
...
i = 10
Chegou a zero!: 5
i : 0
i : 1
...
i : 9
Parou em : 3
```

---

## 🚀 Próximos Passos

* Combinar `match` com enums
* Usar `break` e `continue` em loops
* Criar funções que utilizam `match`
* Explorar `while let`

---

## 📚 Conclusão

Este mini projeto mostra como o **controle de fluxo em Rust** é poderoso e expressivo, permitindo decisões claras e loops flexíveis, além de tratar `match` e loops como expressões que retornam valores.

Excelente prática para consolidar a base da linguagem! 🦀🚀
