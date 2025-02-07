function parpadeo_led()
    for i = 1:5
        disp("LED ON");
        sleep(500); //500ms 0.5sg
        disp("LED OFF");
        sleep(500);
    end
    disp("Parpadeo OFF");
endfunction

function freno_de_emergencia()
    while %t //Bucle Infinito
        valor_potenciometro = input("Ingresa el valor del potenciometro:");
        
        if valor_potenciometro > 10 & valor_potenciometro < 20 then
            //Enviar mensaje de peligro y LED parpadeo
            disp("KONTUZ!! PELIGRO!!");
            parpadeo_led();
        elseif valor_potenciometro == 0
            break;
        else
            for i = 1:5
            disp("Freno OK");
            sleep(500); //500ms 0.5sg
        end
        printf("Valor Potenciometro: %d \n",valor_potenciometro)
    end
    end
endfunction

freno_de_emergencia();
