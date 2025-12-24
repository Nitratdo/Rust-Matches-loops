fn main() {
// ============================================== Desafio 1 — match simples ==============================================
    let dia = 5;
    // Aqui você vai criar uma variavel de valor = 5;
    match dia {
        // Aqui a gente vai comparar o valor da variavel dia e dizer em qual dia da semana está;
        1 => println!("Domingo"),
        2 => println!("Segunda"),
        3 => println!("Terça"),
        4 => println!("Quarta"),
        5 => println!("Quinta"),
        6 => println!("Sexta"),
        7 => println!("Sabado"),
        _ => println!("Dia invalido"),
        // Aqui a gente vai descartar qualquer dia da semana que passe de 7 dias semanais
    };
// ============================================== Desafio 2 — match com intervalos ==============================================

    // Definimos o Valor da nota;
    let nota = 9;
    // Aqui vamos comparar se a foi ruim, média ou boa;
    let resultado = match nota {
        0..=4 => "Reprovado",
        5..=6 => "Recuperação",
        7..=10 => "Aprovado",
        _ => "Nota invalida", 
        // Aqui usamos esse _ como else;
    };
    // Por fim aqui a gente imprime o resultado
    println!("O resultado é : {}", resultado);

// ============================================== Desafio 3 — for ==============================================
    for i in 0..=10 {
        // Aqui a gente fez um for que vai de 0 a 10
        println!("i = {}", i);
    }
// ============================================== Desafio 4 — while ==============================================

    let mut contador = 5;

    while contador > 0 {
        println!("contador: {}", contador);
        contador -= 1;
    }
    println!("Chegou a zero!: {}", contador);
// ============================================== Desafio 5 — loop ==============================================


    for i in 0..=9 {
        println!("i : {}", i)
    };

    let resultado = loop {
        let y = 9;
        break y / 3;
    };
    println!("Parou em : {}", resultado);

}
