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
- sacar tarjeta
- cobrar tarjeta
- cambiar tarjetas

## Implementacion

- Ocupacion: (Jugador, usize)
- Mapa: HashMap de pais en Ocupacion
- Player
- objetivo: una struct que con new() agarre uno del pool