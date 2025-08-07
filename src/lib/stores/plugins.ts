import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface PluginInfo {
  name: string;
  author: string;
  version: string;
  description: string;
  path: string;
}

export const plugins = writable<PluginInfo[]>([]);

export async function loadPlugins(pluginDir: string) {
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    const result = await invoke<PluginInfo[]>('list_plugins', { pluginDir });
    plugins.set(result);
  } catch (error) {
    console.error('Failed to load plugins:', error);
    plugins.set([]);
  }
}

// Add a plugin by providing its file path
export async function addPlugin(pluginPath: string) {
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    await invoke('add_plugin', { path: pluginPath });
    // Reload plugins after addition
    await loadPlugins('../../plugins');
  } catch (error) {
    console.error('Failed to add plugin:', error);
  }
}

// Remove a plugin by name
export async function removePlugin(name: string) {
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    await invoke('remove_plugin', { name });
    await loadPlugins('../../plugins');
  } catch (error) {
    console.error('Failed to remove plugin:', error);
  }
}
