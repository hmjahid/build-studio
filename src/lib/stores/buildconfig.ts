import { writable } from 'svelte/store';

export interface BuildConfig {
  name: string;
  platform: string;
  language?: string;
  command: string;
  container?: string;
}

export interface PackageConfig {
  type?: string;
  name?: string;
  version?: string;
  dependencies?: string[];
}

export interface BuildStudioConfig {
  builds: BuildConfig[];
  package?: PackageConfig;
}

export const buildConfig = writable<BuildStudioConfig | null>(null);
