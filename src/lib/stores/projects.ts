import { writable } from 'svelte/store';

export interface Project {
  name: string;
  path: string;
  config_path: string;
  last_build?: string | null;
  last_status?: string | null;
}

export const projects = writable<Project[]>([]);
