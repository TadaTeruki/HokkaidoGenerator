import type { MapSet } from "$lib/map";
import { writable, type Writable } from "svelte/store";

export const placenameDatasetStore = writable('');

export const mapSetStore: Writable<MapSet | undefined> = writable(undefined);

export const initialSeedStore: Writable<number | undefined> = writable(undefined);