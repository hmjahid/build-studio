<script lang="ts">
  import { buildConfig } from '../stores/buildconfig';
  import type { BuildStudioConfig } from '../stores/buildconfig';
  import { onMount, afterUpdate } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  export let configPath: string;

  async function loadConfig() {
    if (configPath) {
      try {
        const result: BuildStudioConfig = await invoke('read_config', { path: configPath });
        buildConfig.set(result);
      } catch (e) {
        buildConfig.set(null);
        console.error('Failed to read config:', e);
      }
    }
  }

  onMount(loadConfig);
  afterUpdate(() => {
    loadConfig();
  });
</script>

<div class="build-config-viewer">
  <h4>Build Configuration</h4>
  {#if $buildConfig}
    <div class="config-details">
      <h5>Build Targets</h5>
      <ul class="build-list">
        {#each $buildConfig.builds as build}
          <li class="build-item">
            <span class="name">{build.name}</span>
            <span class="platform">{build.platform}</span>
            <span class="language">{build.language}</span>
            <code class="command">{build.command}</code>
            {#if build.container}
              <small class="container">Container: {build.container}</small>
            {/if}
          </li>
        {/each}
      </ul>

      {#if $buildConfig.package}
        <h5>Packaging</h5>
        <pre class="package-config">{JSON.stringify($buildConfig.package, null, 2)}</pre>
      {/if}
    </div>
  {:else}
    <p>No build configuration loaded or found.</p>
  {/if}
</div>

<style>
  .build-config-viewer {
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

  h5 {
    margin: 0 0 0.75rem 0;
    font-size: 1.1rem;
    font-weight: 500;
    color: var(--text-primary);
  }

  .config-details {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .build-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .build-item {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    gap: 1rem;
    padding: 1.25rem;
    background-color: var(--secondary-bg);
    border-radius: var(--border-radius);
    border: 1px solid var(--border-color);
    transition: var(--transition);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  }

  .build-item:hover {
    border-color: var(--primary-accent);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .name {
    font-weight: 600;
    grid-column: 1 / 2;
    color: var(--text-primary);
  }

  .platform, .language {
    font-style: italic;
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .platform::before, .language::before {
    content: '';
    display: inline-block;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: var(--primary-accent);
  }

  .command {
    grid-column: 1 / -1;
    background-color: var(--surface-bg);
    padding: 0.75rem;
    border-radius: var(--border-radius);
    font-family: 'Fira Code', monospace;
    font-size: 0.9rem;
    color: var(--text-primary);
    border: 1px solid var(--border-color);
    overflow-x: auto;
  }

  .container {
    grid-column: 1 / -1;
    color: var(--text-secondary);
    font-size: 0.9rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .container::before {
    content: '🐳';
  }

  .package-config {
    background-color: var(--surface-bg);
    padding: 1.25rem;
    border-radius: var(--border-radius);
    white-space: pre-wrap;
    font-family: 'Fira Code', monospace;
    font-size: 0.9rem;
    color: var(--text-primary);
    border: 1px solid var(--border-color);
    max-height: 300px;
    overflow-y: auto;
  }
</style>