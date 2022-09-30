fn main() {
 
    println!("Ingrese el primer numero: ");
    let mut numero_1 = String::new();
    std::io::stdin().read_line(&mut numero_1).unwrap();
    let numero1_int: i32 = numero_1.trim().parse().unwrap();
    
    println!("Ingrese el segundo numero: ");
    let mut numero_2 = String::new();
    std::io::stdin().read_line(&mut numero_2).unwrap();
    let numero2_int: i32 = numero_2.trim().parse().unwrap();    

    let suma = numero1_int + numero2_int;
    
    loop {
        //mostrar los dos numeros en consola
        println!("El primer numero es: {}, el segundo numero es: {}, ¿Cuál es la suma?", numero_1, numero_2);
        
        //Obtener del usuario el numero que represnta la suma
        let mut suma_usuario = String::new();
        std::io::stdin().read_line(&mut suma_usuario).unwrap();
        let suma_usuario_int: i32 = suma_usuario.trim().parse().unwrap();
        
        
        if suma_usuario_int == suma {
            println!("La suma es correcta C:");
            break
        } else {
            println!("La suma es incorrecta :C. Respuesta correcta: {}", suma)
        }
    }
}

