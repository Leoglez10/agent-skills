// Which dialog is open and with what. Dialog components watch this and call showModal().
// One state object per dialog; the same dialog creates (row null) and edits (row given).
import type { Item } from "./api"; // replace with your domain types

export const itemDialog = $state({ open: false, row: null as Item | null, draft: null as Partial<Item> | null });
export function openItem(row: Item | null = null, draft: Partial<Item> | null = null) {
  Object.assign(itemDialog, { open: true, row, draft });
}

/* ItemDialog.svelte (pattern):
<script lang="ts">
  import { itemDialog } from "../lib/dialogs.svelte";
  let dialog: HTMLDialogElement;
  $effect(() => { if (itemDialog.open && !dialog.open) dialog.showModal(); });
  const close = () => { dialog.close(); };
</script>
<dialog bind:this={dialog} onclose={() => (itemDialog.open = false)} aria-labelledby="item-title">
  <form method="dialog" onsubmit={save}>
    <header class="dlg-h"><h2 id="item-title">{itemDialog.row ? "Editar" : "Nuevo"} …</h2>
      <button class="btn ghost icon" type="button" onclick={close} aria-label="Cerrar">×</button></header>
    <div class="dlg-b">…fields…</div>
    <footer class="dlg-f"><button class="btn" type="button" onclick={close}>Cancelar</button>
      <button class="btn primary">Guardar</button></footer>
  </form>
</dialog>
*/
