// One registry for the native menu (src-tauri/src/menu.rs uses the same ids and owns the shortcuts on macOS),
// the ⌘K palette and Settings › Shortcuts. Svelte 5 runes; adapt imports to your app.
import { app, go, updateSettings } from "./app.svelte";
import { openItem } from "./dialogs.svelte";
import { undo } from "./undo.svelte";
import { keyLabel } from "./keys";

export interface Command { id: string; label: string; group: string; icon: string; keys: string; run: () => unknown }

export const palette = $state({ open: false, text: "" });
export const openPalette = (text = "") => Object.assign(palette, { open: true, text });

// Collapsed to icons by choice (remembered on this computer); narrow windows collapse it on their own.
export const sidebar = $state({ collapsed: localStorage.getItem("sidebar-collapsed") === "1" });
export function toggleSidebar() {
  sidebar.collapsed = !sidebar.collapsed;
  localStorage.setItem("sidebar-collapsed", sidebar.collapsed ? "1" : "0");
}

const isDark = () => app.settings?.dark ?? matchMedia("(prefers-color-scheme: dark)").matches;

export const COMMANDS: Command[] = [
  { id: "go-home", label: "Ir a Inicio", group: "Navegar", icon: "home", keys: "⌘1", run: () => go("inicio") },
  { id: "go-list", label: "Ir a <pantalla 2>", group: "Navegar", icon: "list", keys: "⌘2", run: () => go("lista") },
  // An optional module: off until enabled in Settings, then the command opens that switch instead.
  { id: "go-module", label: "Ir a <módulo>", group: "Navegar", icon: "card", keys: "", run: () => go(app.settings?.module ? "modulo" : "ajustes/modulo") },
  { id: "go-settings", label: "Abrir Ajustes", group: "Navegar", icon: "gear", keys: "⌘,", run: () => go("ajustes") },
  { id: "new-item", label: "Nuevo <elemento>", group: "Acciones", icon: "plus", keys: "⌘N", run: () => openItem() },
  { id: "search", label: "Buscar <elementos>", group: "Editar", icon: "search", keys: "⌘F", run: () => { go("lista"); setTimeout(() => document.querySelector<HTMLInputElement>("#main input[type=search]")?.focus()); } },
  { id: "undo", label: "Deshacer el último cambio", group: "Editar", icon: "undo", keys: "⌘Z", run: () => undo() },
  { id: "sidebar", label: "Mostrar u ocultar la barra lateral", group: "Ver", icon: "sidebar", keys: "⌘S", run: () => toggleSidebar() },
  { id: "theme", label: "Cambiar entre claro y oscuro", group: "Ver", icon: "moon", keys: "⇧⌘L", run: () => updateSettings({ dark: !isDark() }) },
  { id: "palette", label: "Paleta de comandos", group: "Ventana", icon: "command", keys: "⌘K", run: () => openPalette() },
];

export function runCommand(id: string) {
  const command = COMMANDS.find(c => c.id === id);
  if (!command) return;
  // A menu shortcut while a dialog is open would stack windows; the palette is the exception.
  const open = document.querySelector<HTMLDialogElement>("dialog[open]");
  if (open && id !== "palette") return;
  open?.close();
  void command.run();
}

// Settings › Shortcuts: generated, never written by hand.
export const SHORTCUTS: [string, string][] = [
  ...COMMANDS.filter(c => c.keys).map(c => [c.label, keyLabel(c.keys)] as [string, string]),
  ["Guardar en un campo o diálogo", keyLabel("↩")],
  ["Cerrar un diálogo o cancelar una edición", "esc"],
];

/* App.svelte (root component):
  import { listen } from "@tauri-apps/api/event";
  import { comboOf, isMac } from "./lib/keys";
  // Windows/Linux: shortcuts run from here; on Mac the native menu owns them.
  let lastKey = { id: "", at: 0 };
  $effect(() => {
    // Skips a menu event the keydown below already ran, in case both fire.
    const off = listen<string>("menu", e => { if (e.payload !== lastKey.id || Date.now() - lastKey.at > 500) runCommand(e.payload); });
    return () => { off.then(f => f()); };
  });
  function keydown(e: KeyboardEvent) {
    const combo = isMac ? null : comboOf(e);
    const command = combo && combo !== "⌘Z" ? COMMANDS.find(c => c.keys === combo) : undefined;
    if (command) { e.preventDefault(); lastKey = { id: command.id, at: Date.now() }; runCommand(command.id); return; }
    const typing = e.target instanceof Element && e.target.matches("input, textarea, select");
    if ((e.metaKey || e.ctrlKey) && !e.shiftKey && e.key.toLowerCase() === "z" && !typing && !document.querySelector("dialog[open]")) {
      e.preventDefault(); void undo();
    }
  }
  <svelte:window onkeydown={keydown} />
*/
