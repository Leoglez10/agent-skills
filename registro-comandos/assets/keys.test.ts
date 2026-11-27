// Run: node --test keys.test.ts   (Node 22.18+ runs TypeScript directly; no dependencies)
import { test } from "node:test";
import assert from "node:assert/strict";
import { comboOf, keyLabel } from "./keys.ts";

const press = (key: string, mods: Partial<{ ctrlKey: boolean; metaKey: boolean; shiftKey: boolean; altKey: boolean }> = {}) =>
  ({ key, ctrlKey: false, metaKey: false, shiftKey: false, altKey: false, ...mods });

test("keyLabel keeps Mac symbols on a Mac", () => assert.equal(keyLabel("⇧⌘N", true), "⇧⌘N"));
test("keyLabel writes Ctrl first elsewhere", () => assert.equal(keyLabel("⇧⌘N", false), "Ctrl+Shift+N"));
test("keyLabel translates Alt and leaves plain keys alone", () => {
  assert.equal(keyLabel("⌥⇧F", false), "Alt+Shift+F");
  assert.equal(keyLabel("↩", false), "↩");
  assert.equal(keyLabel("esc", false), "esc");
});
test("comboOf maps Ctrl presses to registry strings", () => {
  assert.equal(comboOf(press("k", { ctrlKey: true })), "⌘K");
  assert.equal(comboOf(press("N", { ctrlKey: true, shiftKey: true })), "⇧⌘N");
});
test("comboOf ignores presses without Ctrl, with Meta or Alt, and named keys", () => {
  assert.equal(comboOf(press("k")), null);
  assert.equal(comboOf(press("k", { ctrlKey: true, metaKey: true })), null);
  assert.equal(comboOf(press("k", { ctrlKey: true, altKey: true })), null);
  assert.equal(comboOf(press("Enter", { ctrlKey: true })), null);
});
