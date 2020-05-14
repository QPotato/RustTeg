# Idea

## Datos estaticos

- Lista de paises
- Relaciones de fronteras
- conjuntos de paises en un continente
- Lista de objetivos
- Condiciones de cada objetivo
- Conjunto de jugadores
- lista de tarjetas
  - cobradas o no

## Estado

- Color de cada país
- Cantidad de tropas de cada país
- Cantidad de paises de cada jugador
- Objetivo de cada jugador
- ronda de turnos
- turno actual
- modo de turno
- etapa de turno
- tarjetas de cada jugador
- numero de canjes realizado por un jugador

## Mecanicas

- distribucion inicial de paises
- orden alaetorio inicial de ronda
- rotacion de turno
- colocacion de tropas
- Enfrentamiento, dados.
- Conquista, paso de tropas
- movimiento de tropas
- sacar tarjeta (3 canjes -> dos conquistas)
- cobrar tarjeta
- cambiar tarjetas
- jugador eliminado pasa tarjetas a eliminante

## Implementacion

- Ocupacion: (Jugador, usize)
- Player
- objetivo: una struct que con new() agarre uno del pool

## Path

1. MVP: browser texto plano, tabla (pais, numero de color), atacar con droplist y boton. conquistar cambia. Estado inicial predefinido.
2. Agregar reagrupar, sacar tarjeta y cobrar.
3. Agregar turno de agregar tropas, canjes, dos conquistas con tres canjes.
4. Agregar objetivo, checkeo y cartel de victoria
5. Agregar multijugador local
6. Distribucion inicial de paises, reparto de objetivos, paso de tarjetas en victoria
7. Crear server y armar multijugador a distancia.
8. Interfaz grafica linda
9. Sonidos
10. Salas
11. IA


### MVP

- / Releer rust-wasm
- / GameState tiene que ser serde
- / JavaScript que toma el gamestate y arma un html
- / estado inicial
- Server con rocket que inicie un juego y envie mapa contra request
- Cliente que pida el mapa y lo muestre
- Input -> Cambio de estado. Exportar funciones para cada accion posible de turno que toman un game state, lo cambian y lo devuelven

## Organizacion

- crate rusteg: estructuras generales
- crate rusteg-server: servidor http que recibe un Action y contesta con un Result
- crate rusteg-client: libreria para js que envia Actions al server y recibe resultados

y como carajo recibis updates?

- longpolling: sucia pero segura. Te ahorras inconsistencias porque siempre sobreescribis todo y tenes el estado reciente y completo.
- socket: no se ni como se usan, pero ponele que te conectas, abris un socket y el server te pushea siempre que pasa algo. Hay que aprender cosas raras, me suena a que puede haber inconsistencias, la logica es mas compleja.
