# Lab-4-Carga-de-modelos

El programa:

- Carga un modelo `.obj`.
- Lee manualmente los vértices y las caras.
- Obtiene los tres índices de cada cara.
- Usa esos índices para recuperar los tres vértices.
- Convierte los vértices a coordenadas de pantalla.
- Dibuja cada cara mediante una función propia de rasterización de triángulos.
- Utiliza SDL2 únicamente para crear la ventana y mostrar el framebuffer.

## Ejecutar

Desde la raíz del proyecto:

```bash
cargo run
```

Presiona `Esc` o cierra la ventana para salir.

## Estructura

```text
src/
├── main.rs
├── color.rs
├── framebuffer.rs
├── triangle.rs
└── obj.rs

assets/
└── model.obj
```

## Evidencia
