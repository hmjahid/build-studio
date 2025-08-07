
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { writable } from 'svelte/store';

  let webhooks = writable<any[]>([]);

  async function loadWebhooks() {
    webhooks.set(await invoke('list_webhooks'));
  }

  async function addWebhook() {
    // TODO: Implement a form to add new webhooks
    console.log('Adding a new webhook...');
  }

  onMount(loadWebhooks);
</script>

<div class="cicd-manager">
  <h3>CI/CD Webhooks</h3>
  
  <div class="webhook-actions">
    <button class="primary" on:click={addWebhook}>Add Webhook</button>
    <button class="secondary" on:click={loadWebhooks}>Refresh</button>
  </div>
  
  <div class="webhook-list-container">
    {#if $webhooks.length === 0}
      <p class="empty-state">No webhooks configured. Add a new webhook to get started.</p>
    {:else}
      <ul class="webhook-list">
        {#each $webhooks as webhook}
          <li class="webhook-item">
            <div class="webhook-info">
              <span class="webhook-id">{webhook.id}</span>
              <span class="webhook-url">{webhook.url}</span>
            </div>
            <div class="webhook-actions">
              <button class="icon-button">✏️</button>
              <button class="icon-button danger">🗑️</button>
            </div>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<style>
  .cicd-manager {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  h3 {
    margin: 0;
    font-size: 1.3rem;
    font-weight: 600;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid var(--border-color);
    color: var(--text-primary);
  }

  .webhook-actions {
    display: flex;
    gap: 1rem;
  }

  .webhook-list-container {
    background-color: var(--secondary-bg);
    border-radius: var(--border-radius);
    border: 1px solid var(--border-color);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  }

  .empty-state {
    padding: 2rem;
    text-align: center;
    color: var(--text-secondary);
    margin: 0;
  }

  .webhook-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .webhook-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background-color: var(--surface-bg);
    border-radius: var(--border-radius);
    border: 1px solid var(--border-color);
    transition: var(--transition);
  }

  .webhook-item:hover {
    border-color: var(--primary-accent);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .webhook-info {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .webhook-id {
    font-weight: 600;
    color: var(--text-primary);
  }

  .webhook-url {
    font-family: 'Fira Code', monospace;
    font-size: 0.85em;
    color: var(--text-secondary);
  }

  .webhook-actions {
    display: flex;
    gap: 0.5rem;
  }

  .icon-button {
    background: none;
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius);
    padding: 0.5rem;
    cursor: pointer;
    transition: var(--transition);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
  }

  .icon-button:hover {
    border-color: var(--primary-accent);
    background-color: rgba(100, 255, 218, 0.1);
  }

  .icon-button.danger:hover {
    border-color: var(--error-color);
    background-color: rgba(255, 85, 85, 0.1);
  }
</style>
