<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { writable } from 'svelte/store';

  interface BuildRecord {
    id: string;
    project: string;
    platform: string;
    status: 'success' | 'failed' | 'running';
    timestamp: string;
    duration?: number;
    output_path?: string;
    error_message?: string;
  }

  let buildHistory = writable<BuildRecord[]>([]);
  let isLoading = false;

  async function loadBuildHistory() {
    isLoading = true;
    try {
      // Mock data for now - in a real implementation this would come from the backend
      const mockHistory: BuildRecord[] = [
        {
          id: '1',
          project: 'my-rust-app',
          platform: 'linux',
          status: 'success',
          timestamp: '2024-01-15T10:30:00Z',
          duration: 45,
          output_path: './builds/linux/2024-01-15_10-30/my-rust-app'
        },
        {
          id: '2',
          project: 'web-frontend',
          platform: 'web',
          status: 'failed',
          timestamp: '2024-01-15T09:15:00Z',
          duration: 12,
          error_message: 'TypeScript compilation failed'
        },
        {
          id: '3',
          project: 'mobile-app',
          platform: 'android',
          status: 'running',
          timestamp: '2024-01-15T11:00:00Z'
        }
      ];
      buildHistory.set(mockHistory);
    } catch (e) {
      console.error('Failed to load build history:', e);
    } finally {
      isLoading = false;
    }
  }

  function formatTimestamp(timestamp: string): string {
    return new Date(timestamp).toLocaleString();
  }

  function formatDuration(duration?: number): string {
    if (!duration) return 'N/A';
    const minutes = Math.floor(duration / 60);
    const seconds = duration % 60;
    return minutes > 0 ? `${minutes}m ${seconds}s` : `${seconds}s`;
  }

  function getStatusIcon(status: string): string {
    switch (status) {
      case 'success': return '✅';
      case 'failed': return '❌';
      case 'running': return '⏳';
      default: return '❓';
    }
  }

  onMount(loadBuildHistory);
</script>

<div class="build-history">
  <h3>Build History</h3>
  
  <div class="history-actions">
    <button class="secondary" on:click={loadBuildHistory} disabled={isLoading}>
      {isLoading ? 'Loading...' : 'Refresh'}
    </button>
    <button class="secondary">Clear History</button>
  </div>
  
  <div class="history-list-container">
    {#if isLoading}
      <div class="loading-state">
        <p>Loading build history...</p>
      </div>
    {:else if $buildHistory.length === 0}
      <p class="empty-state">No build history available. Start building projects to see history here.</p>
    {:else}
      <ul class="history-list">
        {#each $buildHistory as build}
          <li class="history-item">
            <div class="build-info">
              <div class="build-header">
                <span class="build-project">{build.project}</span>
                <span class="build-platform">{build.platform}</span>
              </div>
              <div class="build-details">
                <span class="build-timestamp">{formatTimestamp(build.timestamp)}</span>
                <span class="build-duration">{formatDuration(build.duration)}</span>
              </div>
              {#if build.output_path}
                <div class="build-output">
                  <span class="output-label">Output:</span>
                  <code class="output-path">{build.output_path}</code>
                </div>
              {/if}
              {#if build.error_message}
                <div class="build-error">
                  <span class="error-label">Error:</span>
                  <span class="error-message">{build.error_message}</span>
                </div>
              {/if}
            </div>
            <div class="build-status {build.status}">
              <span class="status-icon">{getStatusIcon(build.status)}</span>
              <span class="status-text">{build.status}</span>
            </div>
            <div class="build-actions">
              <button class="icon-button" title="View Details">📋</button>
              <button class="icon-button" title="Rebuild">🔄</button>
              <button class="icon-button danger" title="Delete">🗑️</button>
            </div>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<style>
  .build-history {
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

  .history-actions {
    display: flex;
    gap: 1rem;
  }

  .history-list-container {
    background-color: var(--secondary-bg);
    border-radius: var(--border-radius);
    border: 1px solid var(--border-color);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  }

  .loading-state, .empty-state {
    padding: 2rem;
    text-align: center;
    color: var(--text-secondary);
    margin: 0;
  }

  .history-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .history-item {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 1.25rem;
    background-color: var(--surface-bg);
    border-radius: var(--border-radius);
    border: 1px solid var(--border-color);
    transition: var(--transition);
  }

  .history-item:hover {
    border-color: var(--primary-accent);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .build-info {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    flex: 1;
  }

  .build-header {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .build-project {
    font-weight: 600;
    color: var(--text-primary);
    font-size: 1.1rem;
  }

  .build-platform {
    background-color: var(--tertiary-bg);
    color: var(--text-secondary);
    padding: 0.25rem 0.75rem;
    border-radius: 20px;
    font-size: 0.85rem;
    font-weight: 500;
  }

  .build-details {
    display: flex;
    gap: 1.5rem;
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

  .build-output, .build-error {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.9rem;
  }

  .output-label, .error-label {
    font-weight: 500;
    color: var(--text-secondary);
  }

  .output-path {
    font-family: 'Fira Code', monospace;
    background-color: var(--tertiary-bg);
    padding: 0.2em 0.4em;
    border-radius: var(--border-radius);
    font-size: 0.85em;
  }

  .error-message {
    color: var(--error-color);
    font-style: italic;
  }

  .build-status {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 500;
    padding: 0.5rem 1rem;
    border-radius: 20px;
    background-color: var(--tertiary-bg);
    min-width: 100px;
    justify-content: center;
  }

  .build-status.success {
    color: var(--success-color);
    background-color: rgba(100, 255, 218, 0.1);
  }

  .build-status.failed {
    color: var(--error-color);
    background-color: rgba(255, 85, 85, 0.1);
  }

  .build-status.running {
    color: var(--warning-color);
    background-color: rgba(255, 193, 7, 0.1);
  }

  .status-icon {
    font-size: 1.1rem;
  }

  .build-actions {
    display: flex;
    gap: 0.5rem;
    margin-left: 1rem;
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
