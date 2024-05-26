import type { MapSet } from "$lib/map";
import { writable, type Writable } from "svelte/store";

export const placenameDatasetStore = writable('');

export const mapSetStore: Writable<MapSet | undefined> = writable(undefined);

export class InitialSettings {
    seed: number | undefined;
    view3D: boolean | undefined;
    darkMode: boolean | undefined;
}

export const initialSettingsStore = writable(new InitialSettings());