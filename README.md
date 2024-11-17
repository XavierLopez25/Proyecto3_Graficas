# Sistema Solar en Rust

Este repositorio contiene un proyecto de simulación del sistema solar inspirado en el juego Warframe, utilizando Rust para renderizar gráficamente los cuerpos celestes, incluyendo efectos de shaders personalizados y modelos de anillos para los planetas gaseosos.

## Descripción del Proyecto

El sistema solar se representa mediante un renderer de software en Rust, donde cada planeta y el sol se muestran con efectos visuales únicos, simulados a través de shaders sin emplear texturas externas. La representación incluye detalles como atmósferas, superficies rocosas o gaseosas, anillos planetarios y otros fenómenos visuales.

## Sistema Solar de Warframe

![image](https://github.com/user-attachments/assets/8aa45cb6-602d-4ea6-9bba-8cc127054ba8)

## Videos de los planetas de Warframe en el simulador de Rust

#### Sol (no visible en Warframe, pero presente)

https://github.com/user-attachments/assets/113da9b7-7990-4178-b1e1-6f49560d9cfb

---

#### Tierra y Lua

https://github.com/user-attachments/assets/1f7691e3-7bd2-4bbe-87f0-7d00b3ef4edc

---

### Venus

https://github.com/user-attachments/assets/885c6598-99ae-4732-b466-5f3a6130aef5

---

### Mercurio

https://github.com/user-attachments/assets/d05398d2-8a85-4d55-9290-dc5b708e4521

---

### Marte y Phobos

https://github.com/user-attachments/assets/12be8740-3a87-46a0-b187-53403f682a99

---

### Jupiter

https://github.com/user-attachments/assets/4ad401c6-6e5f-4698-8014-61b6ef0a9b1d

---

### Saturno

https://github.com/user-attachments/assets/8212a97e-7b21-4873-b120-04f0ae0e2fde

---

### Urano

https://github.com/user-attachments/assets/5d4599c1-809b-4ee2-a4ca-f15907517a08

---

### Neptuno, Plutón, Eris y Sedna

https://github.com/user-attachments/assets/231716d5-f172-4bdc-af6d-40747c66ea19

# Sistema Solar Completo

https://github.com/user-attachments/assets/09978c88-d6aa-47f5-8041-b3acb7318533

# Sistema Solar Completo con movimiento de cámara por mouse/teclado

https://github.com/user-attachments/assets/1f11bd70-9503-4df2-b22e-27784d210f45

---

### Características

- Representación del Sol y varios planetas, cada uno con su propio conjunto de shaders y efectos visuales.
- Uso de modelos para los anillos de los planetas gaseosos, sin shaders para los anillos.
- Implementación de un sistema de movimiento y rotación para simular la órbita y rotación de los cuerpos celestes.
- Control de cámara interactivo para explorar el sistema solar.

### Planetas Incluidos

- **Sol**: Efectos de brillo y llamaradas simuladas con shaders.
- **Planetas rocosos y gaseosos**: Desde Mercurio hasta Sedna, cada uno con características únicas.
- **Anillos de Saturno y Urano**: Modelados con objetos específicos y no mediante shaders.

## Instalación y Uso

### Prerrequisitos

Asegúrate de tener Rust y Cargo instalados en tu sistema. Puedes instalarlos desde [la página oficial de Rust](https://www.rust-lang.org/tools/install).

### Ejecutar el Proyecto

1. Clona este repositorio:

   ```bash
   git clone https://github.com/XavierLopez25/Lab4_Graficas.git
   cd Lab4_Graficas
   ```

2. Compila y ejecuta el proyecto:
   ```bash
   cargo build --release
   cargo run --release
   ```

### Controles

- **Órbita de la cámara**: Usa las flechas `Izquierda` y `Derecha` para rotar horizontalmente, `W` y `S` para rotar verticalmente.
- **Movimiento de la cámara**: Usa `A` y `D` para mover la cámara a la izquierda y derecha, `Q` y `E` para mover hacia arriba y abajo.
- **Zoom**: Usa las flechas `Arriba` y `Abajo` para acercar y alejar.
- **Bird Eye View**: Presiona `B` para alternar entre la vista normal y la vista aérea.
- **Rotación con el mouse**: Mantén presionado el botón izquierdo del mouse y arrastra para rotar la cámara.
- **Zoom con el mouse**: Mantén presionado el botón derecho del mouse y arrastra hacia arriba o abajo para hacer zoom.
- **Paneo con el mouse**: Mantén presionado el botón central del mouse y arrastra para mover la cámara.
- **Salir**: Presiona `Esc` para cerrar la aplicación.

## Detalles Técnicos

- **Renderer**: Utiliza `minifb` para la ventana y el dibujo pixel por pixel.
- **Shaders**: Cada cuerpo celeste utiliza shaders escritos en Rust para definir su apariencia.
- **Modelos 3D**: Carga modelos de esferas y anillos usando `tobj`.

## Librerías Usadas

- `fastnoise-lite`: Para generar ruido en los shaders.
- `minifb`: Para la creación de ventanas y manejo de eventos.
- `nalgebra-glm`: Para cálculos matemáticos de gráficos.
- `rand`: Utilizado en la generación de algunas características aleatorias.
- `tobj`: Para cargar modelos 3D.
