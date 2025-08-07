<script lang="ts">
  import { onMount, afterUpdate } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { writable } from 'svelte/store';

  export let configPath: string;
  let content = writable('');
  let saveStatus = writable('');

  async function loadContent() {
    if (configPath) {
      try {
        const result = await invoke<string>('read_text_file', { path: configPath });
        content.set(result);
      } catch (e) {
        console.error('Failed to read config file:', e);
        content.set(`Failed to load ${configPath}`);
      }
    }
  }

  async function saveConfig() {
    try {
      await invoke('write_text_file', { path: configPath, content: $content });
      saveStatus.set('Saved successfully!');
    } catch (e) {
      saveStatus.set(`Error: ${e}`);
    }
    setTimeout(() => saveStatus.set(''), 2000);
  }

  onMount(loadContent);
  afterUpdate(() => {
    // This is a simple way to reload if the configPath changes.
    // A more robust solution might use a reactive statement: `$: if (configPath) loadContent();`
    loadContent();
  });

</script>

<div class="config-editor">
  <h4>Configuration Editor</h4>
  <textarea bind:value={$content} placeholder="Enter your configuration here..."></textarea>
  <div class="editor-footer">
    <button on:click={saveConfig}>Save Configuration</button>
    {#if $saveStatus}
      <span class="save-status {($saveStatus.includes('Error') || $saveStatus.includes('error')) ? 'error' : 'success'}">{$saveStatus}</span>
    {/if}
  </div>
</div>

<style>
  .config-editor {
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

  textarea {
    width: 100%;
    height: 400px;
    box-sizing: border-box;
    background-color: var(--surface-bg);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius);
    padding: 1.25rem;
    font-family: 'Fira Code', monospace;
    font-size: 0.95em;
    transition: var(--transition);
    resize: vertical;
    min-height: 200px;
    max-height: 600px;
  }

  textarea:focus {
    outline: none;
    border-color: var(--primary-accent);
    box-shadow: 0 0 0 2px rgba(100, 255, 218, 0.2);
  }

  .editor-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: 0.5rem;
    border-top: 1px solid var(--border-color);
  }

  .save-status {
    font-style: italic;
    color: var(--text-secondary);
    font-size: 0.9rem;
  }

  .save-status.success {
    color: var(--success-color);
  }

  .save-status.error {
    color: var(--error-color);
  }
</style>