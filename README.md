# Max1161x

Libreria per la gestione di adc [max1161x](https://www.analog.com/media/en/technical-documentation/data-sheets/MAX11612-MAX11617.pdf).

Gli adc dovranno poter essere gestiti tramite un multiplexer, ad esempio [tca9543a](https://www.ti.com/lit/ds/symlink/tca9543a.pdf).

## Schema della struttura fisica

Schema della struttura fiscica della board per cui questa libreria viene progettata.

ESP -i2c-> tca9543a -i2c--> max11617 -> 12 sensori temperatura
                        --> max11617 -> 12 sensori temperatura

## Schema logico della libreria

fw  -> tca9543a -> max1161x
                -> max1161x

L'utente che scrive il fw utilizzando questa libreria può scegliere se utilizzare l'adc in maniera standalone oppure utilizzare l'astrazione fornita dal multiplexer per gestire più sensori contemporaneamente.

> [!TIP]
>
> La classe che gesisce il multiplexer dovrà comportarsi come una sorta di facade nei confronti degli adc.

## Struttura attule libreria

- /src/max_messages: contiene la definizione astratta dei messaggi per max1161x
    - config_byte: byte di configurazione
    - setup_byte: byte di setup
- /src/max_driver: driver per adc della famiglia max1161x, usa i generics di /src/maxvariants per specializzarsi in base alla variente hardware. Si tratta di un wrapper generico, simile al pattern facade
- /src/max_variants: varianti hardware della famiglia max1161x
- /src/mux_govenor: classe facade creata con pattern builder che permette di gestire più adc max1161x con tramite un unico oggetto governor

## TODO

- /src/maxdriver: correggere funzione di conversine da valore adc a temperatura
- /src/maxdriver: terminare il codice del costruttore
- aggiungere supporto a multiplexer [tca9543a](https://www.ti.com/lit/ds/symlink/tca9543a.pdf) tramite muxgovernor
- testare irl

