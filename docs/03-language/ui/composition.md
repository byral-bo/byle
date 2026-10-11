# Patrones de Composición

**Status:** Draft v1.0
**Date:** 2026-07-10
**Documentación relacionada:**
- [ui-syntax.md](../syntax/ui-syntax.md) — Sintaxis .byx (slots, componentes)
- [state-events.md](state-events.md) — Estado y eventos
- [context-patterns.md](context-patterns.md) — Contexto

---

## 1. Filosofía

Byle favorece **composición sobre herencia**. Los componentes se combinan,
no se extienden. Esto da más flexibilidad, mejor tipado, y menos acoplamiento.

```
Herencia:           Composición:
Button               Button
  └── PrimaryButton    └── style=Primary
  └── SecondaryBtn     └── style=Secondary
  └── DangerBtn        └── style=Danger
```

---

## 2. Layout Persistente + Slot (Routing)

El patrón `<layout>` + `<slot />` permite crear wrappers persistentes
que NO se re-renderizan al navegar entre rutas.

### 2.1 Layout con slot

```byx
# layouts/main.byx
<layout>
    <navbar title="Mi App" />
    <hstack>
        <sidebar />
        <main>
            <slot />    ← contenido de la ruta activa
        </main>
    </hstack>
    <footer />
</layout>
```

```byx
# layouts/blank.byx — sin navbar, sidebar ni footer
<layout>
    <slot />
</layout>
```

### 2.2 Cómo funciona

Cuando el usuario navega de `/` a `/login`:

```
Antes:                      Después:
<main_layout>               <main_layout>        ← PERSISTE (no se recrea)
  <navbar />                  <navbar />         ← PERSISTE
  <vstack>                    <vstack>            ← PERSISTE
    <sidebar />                 <sidebar />       ← PERSISTE
    <main>                      <main>            ← PERSISTE
      <slot>                      <slot>
        <home_view />    →         <login_view />  ← SOLO ESTO CAMBIA
      </slot>                     </slot>
    </main>                     </main>
  </vstack>                   </vstack>
  <footer />                  <footer />          ← PERSISTE
</main_layout>              </main_layout>
```

### 2.3 Uso en router

```byx
<router>
    <route path="/" component=home layout=main />
    <route path="/login" component=login layout=blank />
    <route path="/dashboard" component=dashboard layout=main />
</router>
```

El layout se monta UNA VEZ. Solo el `<slot />` se re-renderiza al navegar.

---

## 3. Slots Básicos

### 2.2 Slots nombrados

```byx
# card.byx
<view>
    <slot name="header" />
    <slot name="content" />
    <slot name="footer" />
</view>
```

```byx
<card>
    <header>
        <text value="Título" />
    </header>
    <content>
        <text value="Cuerpo" />
    </content>
    <footer>
        <button text="Cerrar" />
    </footer>
</card>
```

### 2.3 Slot con fallback

```byx
<slot name="header">
    <text value="Header por defecto" />
</slot>
```

Si no se provee slot `header`, se muestra el fallback.

---

## 3. Render Props (Slot con Datos)

El slot puede recibir datos del componente padre:

```byx
# list.byx — componente que renderiza items
@(
    items: {T}
    # Slot "item" recibe cada item como data
)

<view>
    @for(item in items):
        <slot name="item" data=@item />
</view>
```

```byx
<!-- Uso -->
<List items=@products>
    <item>
        <card>
            <text value=@data.name />
            <text value=@data.price.to_str() />
        </card>
    </item>
</List>
```

### 3.1 Múltiples render props

```byx
# data_table.byx
<view>
    <slot name="header" data=@columns />
    <slot name="row" data=@row />
    <slot name="empty">
        <text value="No hay datos" />
    </slot>
</view>
```

```byx
<data_table columns=@cols rows=@rows>
    <header>
        <hstack>
            @for(col in data):
                <text value=col.label />
        </hstack>
    </header>
    <hstack>
        <text value=data.name />
        <text value=data.email />
    </hstack>
</data_table>
```

---

## 4. Forwarding Refs

Un componente puede exponer una referencia a un elemento interno:

```byx
# input_wrapper.byx
@(
    inner_ref: ^&ElementRef  # el padre recibe esta ref
)

<view>
    <text_field ref=@inner_ref placeholder="Escribe..." />
</view>
```

```byx
@(
    input_ref: ElementRef
    fn focus_input():
        input_ref.focus()
)

<vstack>
    <input_wrapper ref=@input_ref />
    <button text="Focus" click=@focus_input />
</vstack>
```

### 4.1 Forward con múltiples refs

```byx
# form_group.byx
@(
    refs: {
        "name": ^ElementRef,
        "email": ^ElementRef,
        "password": ^ElementRef,
    }
)

<view>
    <text_field ref=@refs.name label="Nombre" />
    <text_field ref=@refs.email label="Email" />
    <password_field ref=@refs.password label="Contraseña" />
</view>
```

---

## 5. Higher-Order Components (HOC)

Un HOC es una función que toma un componente y retorna uno nuevo con
funcionalidad extra:

```byle
fn with_auth<T>(component: type) -> type:
    # Retorna un nuevo componente que verifica auth
    return fn(props: T) View:
        @(
            auth = use_context(AuthContext)
        )
        @if auth.is_authenticated:
            <component ...props>
                <slot />
            </component>
        @else:
            <redirect to="/login" />
```

```byx
@with_auth
<admin_panel user_id=@uid />
```

### 5.1 HOC con configuración

```byle
fn with_logging<T>(component: type, name: str) -> type:
    return fn(props: T) View:
        @(
            fn on_mounted():
                log_component_mount(name)
            fn on_unmounted():
                log_component_unmount(name)
        )
        <component ...props>
            <slot />
        </component>
```

---

## 6. Component Composition Patterns

### 6.1 Compound Components

Múltiples componentes que funcionan juntos, compartiendo estado implícito:

```byx
# tabs.byx — compound component
@(
    active_tab: ^str = ""
    context TabsContext:
        active: str = active_tab
        fn select(tab: str):
            active_tab = tab
)

<view>
    <slot name="tabs" />
    <slot name="panels" />
</view>
```

```byx
<tabs>
    <tabs_header>
        <tab name="info" label="Información" />
        <tab name="config" label="Configuración" />
    </tabs_header>
    <tab_panel name="info">
        <text value="Información del usuario" />
    </tab_panel>
    <tab_panel name="config">
        <text value="Configuración" />
    </tab_panel>
</tabs>
```

### 6.2 Layout Components

Componentes que solo definen estructura:

```byx
# two_panel.byx
<view>
    <slot name="sidebar" />
    <slot name="main" />
</view>
```

```byx
<two_panel>
    <sidebar>
        <nav_menu />
    </sidebar>
    <main>
        <dashboard />
    </main>
</two_panel>
```

### 6.3 Delegation Pattern

```byx
# confirm_dialog.byx
@(
    on_confirm: ^&(fn ())
    on_cancel: ^&(fn ())
)

<modal>
    <text value="¿Estás seguro?" />
    <hstack>
        <button text="Sí" click=@on_confirm />
        <button text="No" click=@on_cancel />
    </hstack>
</modal>
```

---

## 7. Composición vs Herencia

| Aspecto | Composición | Herencia |
|---------|:-----------:|:--------:|
| Acoplamiento | Bajo | Alto |
| Reutilización | Horizontal | Vertical |
| Tipado | Fuerte + flexible | Fijo |
| Testing | Fácil (aislado) | Difícil (base+derivado) |
| Cambios | Locales | En cascada |

**En Byle:** Todo componente es `final` por defecto. No hay herencia de
componentes. Solo composición.

---

## 8. Patrón Provider/Consumer

Separa la lógica de estado de la presentación:

```byx
# counter_provider.byx — solo estado
@(
    count: ^i32 = 0
    context CounterContext:
        count: i32 = count
        increment: fn () = fn(): count = count + 1
        decrement: fn () = fn(): count = count - 1
        reset: fn () = fn(): count = 0
)

<slot />
```

```byx
# counter_display.byx — solo UI
@(
    ctx = use_context(CounterContext)
)

<view>
    <text value=ctx.count.to_str() />
    <button text="+" click=@ctx.increment />
    <button text="-" click=@ctx.decrement />
    <button text="Reset" click=@ctx.reset />
</view>
```

---

## 9. Composición con Estilos

```byx
# styled_button.byx — componente que compone estilo + comportamiento
@(
    variant: style  # Primary | Secondary | Danger
    on_click: ^&(fn ())
)

<button style=@variant click=@on_click>
    <slot />
</button>
```

```byx
<styled_button variant=Primary click=@save>
    <icon name="save" />
    <text value="Guardar" />
</styled_button>
```

---

## 10. Buenas Prácticas

| Práctica | Descripción |
|----------|-------------|
| **Slots sobre props** | Si el hijo necesita renderizar markup, usa slots |
| **Un propósito** | Cada componente hace una cosa |
| **Props planas** | Preferir props atómicas a objetos complejos |
| **Composición profunda** | 2-3 niveles de composición es ideal |
| **Nombres descriptivos** | `header`, `sidebar`, `main` no `slot1`, `slot2` |
| **Sin herencia** | Componentes final, composición para variantes |

---

## 11. Referencias

- [ui-syntax.md](../syntax/ui-syntax.md) — Sintaxis .byx
- [state-events.md](state-events.md) — Estado y eventos
- [context-patterns.md](context-patterns.md) — Provider/Consumer
