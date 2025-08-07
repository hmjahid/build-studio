<script lang="ts">
  import { onMount } from 'svelte';
  import { plugins, loadPlugins, addPlugin, removePlugin } from '../stores/plugins';
  import type { PluginInfo } from '../stores/plugins';

  // In a real app, this might be configurable
  const PLUGIN_DIR = '../../plugins';
  let newPluginPath = '';
  let isAddingPlugin = false;
  let pluginError = '';

  onMount(async () => {
    await loadPlugins(PLUGIN_DIR);
  });

  function isTauri(): boolean {
    return typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__;
  }

  async function selectPluginFile() {
    if (isTauri()) {
      try {
        const { open } = await import('@tauri-apps/plugin-dialog');
        const selected = await open({
          multiple: false,
          filters: [{
            name: 'Plugin Files',
            extensions: ['js', 'ts', 'wasm']
          }]
        });
        
        if (selected && typeof selected === 'string') {
          newPluginPath = selected;
        }
      } catch (e) {
        console.error('Failed to select plugin file:', e);
        pluginError = `Failed to select plugin file: ${e}`;
      }
    } else {
      // Non-Tauri environment: use HTML file input as fallback
      console.warn('File selection not available in non-Tauri environment, using HTML file input fallback');
      
      try {
        const selected = await new Promise<string | null>((resolve) => {
          // Create a temporary file input element
          const input = document.createElement('input');
          input.type = 'file';
          input.accept = '.js,.ts,.wasm';
          
          // Handle file selection
          input.onchange = () => {
            if (input.files && input.files.length > 0) {
              resolve(input.files[0].name);
            } else {
              resolve(null);
            }
            
            // Clean up
            document.body.removeChild(input);
          };
          
          // Handle cancel
          input.oncancel = () => {
            document.body.removeChild(input);
            resolve(null);
          };
          
          // Handle errors
          input.onerror = (e) => {
            console.error('File input error:', e);
            document.body.removeChild(input);
            resolve(null);
          };
          
          // Hide the input element
          input.style.display = 'none';
          
          // Add to document and trigger click
          document.body.appendChild(input);
          input.click();
        });
        
        if (selected) {
          newPluginPath = selected;
        }
      } catch (e) {
        console.error('Failed to select plugin file:', e);
        pluginError = `Failed to select plugin file: ${e}`;
      }
    }
  }

  async function handleAddPlugin() {
    if (!newPluginPath) {
      pluginError = 'Please select a plugin file';
      return;
    }
    
    isAddingPlugin = true;
    pluginError = '';
    
    try {
      await addPlugin(newPluginPath);
      newPluginPath = '';
    } catch (e) {
      pluginError = `Failed to add plugin: ${e}`;
    } finally {
      isAddingPlugin = false;
    }
  }

  async function handleRemovePlugin(name: string) {
    if (!confirm(`Are you sure you want to remove plugin "${name}"?`)) {
      return;
    }
    
    try {
      await removePlugin(name);
    } catch (e) {
      pluginError = `Failed to remove plugin: ${e}`;
    }
  }
</script>

<div class="plugin-manager">
  <h4>Plugin Management</h4>
  
  <div class="plugin-actions">
    <div class="add-plugin-form">
      <div class="form-group">
        <label for="pluginPath">Plugin File:</label>
        <input 
          id="pluginPath"
          type="text" 
          bind:value={newPluginPath} 
          placeholder="Select plugin file" 
          readonly
        />
        <button type="button" class="browse-btn" on:click={selectPluginFile}>Browse</button>
      </div>
      <button 
        class="add-plugin-btn" 
        on:click={handleAddPlugin} 
        disabled={isAddingPlugin || !newPluginPath}
      >
        {isAddingPlugin ? 'Adding...' : 'Add Plugin'}
      </button>
    </div>
    
    {#if pluginError}
      <div class="error-message">{pluginError}</div>
    {/if}
  </div>
  
  <div class="plugin-list-container">
    {#if $plugins.length === 0}
      <p>No plugins loaded. Place plugins in the <code>{PLUGIN_DIR}</code> directory.</p>
    {:else}
      <ul class="plugin-list">
        {#each $plugins as plugin}
          <li class="plugin-item">
            <span class="name">{plugin.name}</span>
            <span class="kind">{plugin.kind}</span>
            <span class="path">{plugin.path}</span>
            <button 
              class="remove-btn" 
              on:click={() => handleRemovePlugin(plugin.name)}
              title="Remove Plugin"
            >
              Remove
            </button>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<style>
  .plugin-manager {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  h4 {
    margin: 0;
    font-size: 1.3rem;
    font-weight: 600;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid var(--border-color);
    color: var(--text-primary);
  }

  .plugin-list-container {
    background-color: var(--secondary-bg);
    border-radius: var(--border-radius);
    padding: 1.25rem;
    border: 1px solid var(--border-color);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  }

  .plugin-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .plugin-item {
    display: grid;
    grid-template-columns: 1fr 1fr auto;
    gap: 1rem;
    padding: 1rem;
    background-color: var(--surface-bg);
    border-radius: var(--border-radius);
    border: 1px solid var(--border-color);
    transition: var(--transition);
  }

  .plugin-item:hover {
    border-color: var(--primary-accent);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .name {
    font-weight: 600;
    color: var(--text-primary);
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .name::before {
    content: '🔌';
  }

  .kind {
    font-style: italic;
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .kind::before {
    content: '⚙️';
  }

  .path {
    font-family: 'Fira Code', monospace;
    font-size: 0.85em;
    color: var(--text-secondary);
    justify-self: end;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .path::before {
    content: '📁';
  }

  p {
    color: var(--text-secondary);
    margin: 0;
    line-height: 1.5;
  }

  code {
    background-color: var(--tertiary-bg);
    padding: 0.2em 0.4em;
    border-radius: var(--border-radius);
    font-family: 'Fira Code', monospace;
    font-size: 0.9em;
  }
</style>