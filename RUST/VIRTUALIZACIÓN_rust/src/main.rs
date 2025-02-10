//importar modulos de std los includes
use std::io::stdin;
//use std::string;
use std::thread::sleep;
use std::time;

// Funcion lectura de potenciometro
fn lectura_pot() -> u16 {
    let mut valor= String::new(); //variable par almacenar el valor ingresado
    //let mut valor = 0;

    println!("Ingresa el valor del potenciometro:");

    stdin().read_line(&mut valor).expect("Error al leer el valor"); //Para leer la entrada desde la terminal y sino lee para alertar de un error

    match valor.trim().parse::<u16>() { //match para comprobar Ok o error     trim delvuelve un segmento de cadena no espacios iniciales y finales eliminados    parse para saber que tipo de valor esta intentando analizar
        Ok(n) => n,
        Err(_) => {
            println!("Valor no valido");
            return 0;
        }

    }
}

fn parpadeo_led() {
    for _ in 0..5{
        println!("LED ON\r"); // \r para sobreiscribir en la misma linea
        sleep(time::Duration::from_millis(500));
        println!("LED OFF\r");
        sleep(time::Duration::from_millis(500));

    }
    println!("\nParpadeo OFF");

}
fn main() {
    
    loop {
        let valor_potenciometro = lectura_pot();
    
    if valor_potenciometro > 10 && valor_potenciometro < 20 {
        //Enviar mensaje de peligro y LED parpadeo
        println!("\nKONTUZ!! PELIGRO!!");
        parpadeo_led();
        
        
    }
    else if valor_potenciometro == 0 {
        break;
    }
    else {
        for _ in 0..5{
            println!("Freno OK");
            sleep(time::Duration::from_millis(500));
        }
        println!("\nValor Potenciometro: {}", valor_potenciometro);
    }

    } 
    

}
