
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { writable } from 'svelte/store';
  import { open } from '@tauri-apps/plugin-dialog';

  interface RemoteNode {
    id: string;
    name: string;
    address: string;
    port: number;
    username: string;
    auth_type: 'password' | 'key';
    key_path?: string;
    status: 'online' | 'offline' | 'connecting';
    capabilities: string[];
    last_seen?: string;
  }

  let nodes = writable<RemoteNode[]>([]);
  let showAddNode = false;
  let isLoading = false;
  let errorMessage = '';
  let testingConnection = false;

  // Form fields
  let nodeName = '';
  let nodeAddress = '';
  let nodePort = 22;
  let nodeUsername = '';
  let authType: 'password' | 'key' = 'key';
  let nodePassword = '';
  let sshKeyPath = '';
  let nodeCapabilities = ['build', 'test', 'package'];
  let selectedCapabilities = new Set(['build']);

  async function loadNodes() {
    try {
      const result = await invoke('list_remote_nodes');
      nodes.set(result as RemoteNode[]);
    } catch (e) {
      console.error('Failed to load remote nodes:', e);
      // Set mock data for demonstration
      nodes.set([
        {
          id: '1',
          name: 'Build Server 1',
          address: '192.168.1.100',
          port: 22,
          username: 'builder',
          auth_type: 'key',
          key_path: '/home/user/.ssh/id_rsa',
          status: 'online',
          capabilities: ['build', 'test'],
          last_seen: '2024-01-15T10:30:00Z'
        },
        {
          id: '2',
          name: 'CI Server',
          address: 'ci.example.com',
          port: 2222,
          username: 'ci-user',
          auth_type: 'password',
          status: 'offline',
          capabilities: ['build', 'test', 'package'],
          last_seen: '2024-01-14T15:20:00Z'
        }
      ]);
    }
  }

  async function selectSSHKey() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'SSH Keys',
          extensions: ['', 'pem', 'key', 'pub']
        }],
        title: 'Select SSH Private Key'
      });
      
      if (selected && typeof selected === 'string') {
        sshKeyPath = selected;
      }
    } catch (e) {
      console.error('Failed to select SSH key:', e);
    }
  }

  async function testConnection() {
    if (!nodeAddress || !nodeUsername) {
      errorMessage = 'Please provide address and username';
      return;
    }

    testingConnection = true;
    errorMessage = '';
    
    try {
      const connectionTest = await invoke('test_ssh_connection', {
        address: nodeAddress,
        port: nodePort,
        username: nodeUsername,
        authType,
        password: authType === 'password' ? nodePassword : undefined,
        keyPath: authType === 'key' ? sshKeyPath : undefined
      });
      
      if (connectionTest) {
        errorMessage = '';
        // Show success message
        const successMsg = document.createElement('div');
        successMsg.className = 'success-message';
        successMsg.textContent = 'Connection successful!';
        document.querySelector('.add-node-modal')?.appendChild(successMsg);
        setTimeout(() => successMsg.remove(), 3000);
      }
    } catch (e) {
      errorMessage = `Connection failed: ${e}`;
    } finally {
      testingConnection = false;
    }
  }

  async function addNode() {
    if (!nodeName || !nodeAddress || !nodeUsername) {
      errorMessage = 'Please fill in all required fields';
      return;
    }

    if (authType === 'key' && !sshKeyPath) {
      errorMessage = 'Please select an SSH key file';
      return;
    }

    if (authType === 'password' && !nodePassword) {
      errorMessage = 'Please provide a password';
      return;
    }

    isLoading = true;
    errorMessage = '';
    
    try {
      const newNode = {
        name: nodeName,
        address: nodeAddress,
        port: nodePort,
        username: nodeUsername,
        auth_type: authType,
        password: authType === 'password' ? nodePassword : undefined,
        key_path: authType === 'key' ? sshKeyPath : undefined,
        capabilities: Array.from(selectedCapabilities)
      };

      await invoke('add_remote_node', newNode);
      await loadNodes();
      resetForm();
    } catch (e) {
      errorMessage = `Failed to add node: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  async function removeNode(nodeId: string, nodeName: string) {
    if (confirm(`Are you sure you want to remove node "${nodeName}"?`)) {
      try {
        await invoke('remove_remote_node', { id: nodeId });
        await loadNodes();
      } catch (e) {
        console.error('Failed to remove node:', e);
      }
    }
  }

  async function pingNode(nodeId: string) {
    try {
      await invoke('ping_remote_node', { id: nodeId });
      await loadNodes(); // Refresh to get updated status
    } catch (e) {
      console.error('Failed to ping node:', e);
    }
  }

  function resetForm() {
    showAddNode = false;
    nodeName = '';
    nodeAddress = '';
    nodePort = 22;
    nodeUsername = '';
    authType = 'key';
    nodePassword = '';
    sshKeyPath = '';
    selectedCapabilities = new Set(['build']);
    errorMessage = '';
  }

  function toggleCapability(capability: string) {
    if (selectedCapabilities.has(capability)) {
      selectedCapabilities.delete(capability);
    } else {
      selectedCapabilities.add(capability);
    }
    selectedCapabilities = selectedCapabilities; // Trigger reactivity
  }

  function formatLastSeen(timestamp?: string): string {
    if (!timestamp) return 'Never';
    return new Date(timestamp).toLocaleString();
  }

  onMount(loadNodes);
</script>

<div class="remote-node-manager">
  <div class="node-header">
    <h3>Remote Build Nodes</h3>
    <button class="add-node-btn" on:click={() => showAddNode = !showAddNode}>
      {showAddNode ? '✖ Cancel' : '+ Add SSH Node'}
    </button>
  </div>

  {#if showAddNode}
    <div class="add-node-modal">
      <h4>Add SSH Remote Node</h4>
      
      <div class="form-grid">
        <div class="form-group">
          <label for="nodeName">Node Name:</label>
          <input 
            id="nodeName"
            type="text" 
            bind:value={nodeName} 
            placeholder="Build Server 1" 
          />
        </div>
        
        <div class="form-group">
          <label for="nodeAddress">SSH Address:</label>
          <input 
            id="nodeAddress"
            type="text" 
            bind:value={nodeAddress} 
            placeholder="192.168.1.100 or server.example.com" 
          />
        </div>
        
        <div class="form-group">
          <label for="nodePort">SSH Port:</label>
          <input 
            id="nodePort"
            type="number" 
            bind:value={nodePort} 
            placeholder="22" 
            min="1" 
            max="65535"
          />
        </div>
        
        <div class="form-group">
          <label for="nodeUsername">Username:</label>
          <input 
            id="nodeUsername"
            type="text" 
            bind:value={nodeUsername} 
            placeholder="ubuntu, root, builder" 
          />
        </div>
      </div>

      <div class="auth-section">
        <h5>Authentication Method</h5>
        <div class="auth-tabs">
          <button 
            class="auth-tab {authType === 'key' ? 'active' : ''}"
            on:click={() => authType = 'key'}
          >
            🔑 SSH Key
          </button>
          <button 
            class="auth-tab {authType === 'password' ? 'active' : ''}"
            on:click={() => authType = 'password'}
          >
            🔒 Password
          </button>
        </div>

        {#if authType === 'key'}
          <div class="form-group">
            <label for="sshKeyPath">SSH Private Key:</label>
            <div class="path-input-group">
              <input 
                id="sshKeyPath"
                type="text" 
                bind:value={sshKeyPath} 
                placeholder="Select SSH private key file" 
                readonly
              />
              <button class="browse-btn" on:click={selectSSHKey}>Browse</button>
            </div>
            <small class="help-text">Select your SSH private key file (e.g., ~/.ssh/id_rsa)</small>
          </div>
        {:else}
          <div class="form-group">
            <label for="nodePassword">Password:</label>
            <input 
              id="nodePassword"
              type="password" 
              bind:value={nodePassword} 
              placeholder="Enter SSH password" 
            />
            <small class="help-text">SSH key authentication is recommended for security</small>
          </div>
        {/if}
      </div>

      <div class="capabilities-section">
        <h5>Node Capabilities</h5>
        <div class="capabilities-grid">
          {#each nodeCapabilities as capability}
            <label class="capability-checkbox">
              <input 
                type="checkbox" 
                checked={selectedCapabilities.has(capability)}
                on:change={() => toggleCapability(capability)}
              />
              <span class="checkmark"></span>
              {capability.charAt(0).toUpperCase() + capability.slice(1)}
            </label>
          {/each}
        </div>
      </div>

      {#if errorMessage}
        <div class="error-message">{errorMessage}</div>
      {/if}

      <div class="form-actions">
        <button class="test-btn" on:click={testConnection} disabled={testingConnection || !nodeAddress || !nodeUsername}>
          {testingConnection ? 'Testing...' : '🔍 Test Connection'}
        </button>
        <button class="cancel-btn" on:click={resetForm}>Cancel</button>
        <button class="add-btn" on:click={addNode} disabled={isLoading || !nodeName || !nodeAddress || !nodeUsername}>
          {isLoading ? 'Adding...' : 'Add Node'}
        </button>
      </div>
    </div>
  {/if}
  
  <div class="node-list-container">
    {#if $nodes.length === 0}
      <p class="empty-state">No remote nodes configured. Add a new SSH node to get started.</p>
    {:else}
      <ul class="node-list">
        {#each $nodes as node}
          <li class="node-item">
            <div class="node-info">
              <div class="node-header-info">
                <span class="node-name">{node.name}</span>
                <span class="node-address">{node.username}@{node.address}:{node.port}</span>
              </div>
              <div class="node-details">
                <span class="node-capabilities">
                  {#each node.capabilities as capability}
                    <span class="capability-tag">{capability}</span>
                  {/each}
                </span>
                <span class="node-last-seen">Last seen: {formatLastSeen(node.last_seen)}</span>
              </div>
            </div>
            <div class="node-status {node.status.toLowerCase()}">
              <span class="status-indicator"></span>
              {node.status}
            </div>
            <div class="node-actions">
              <button class="icon-button" on:click={() => pingNode(node.id)} title="Ping Node">📡</button>
              <button class="icon-button" title="Edit Node">✏️</button>
              <button class="icon-button danger" on:click={() => removeNode(node.id, node.name)} title="Remove Node">🗑️</button>
            </div>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<style>
  .remote-node-manager {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .node-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid var(--border-color);
  }

  h3 {
    margin: 0;
    font-size: 1.3rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .add-node-btn {
    padding: 0.5em 1em;
    font-size: 0.9em;
    font-weight: 500;
    color: var(--primary-bg);
    background-color: var(--primary-accent);
    border: none;
    border-radius: var(--border-radius);
    cursor: pointer;
    transition: var(--transition);
  }

  .add-node-btn:hover {
    background-color: var(--secondary-accent);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(100, 255, 218, 0.3);
  }

  .add-node-modal {
    background-color: var(--secondary-bg);
    border-radius: var(--border-radius);
    padding: 1.5rem;
    border: 1px solid var(--border-color);
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
    margin-bottom: 1.5rem;
  }

  .add-node-modal h4 {
    margin: 0 0 1.5rem 0;
    font-size: 1.2rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .form-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .auth-section, .capabilities-section {
    margin-bottom: 1.5rem;
  }

  .auth-section h5, .capabilities-section h5 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .auth-tabs {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .auth-tab {
    padding: 0.5em 1em;
    font-size: 0.85em;
    font-weight: 500;
    background-color: var(--tertiary-bg);
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius);
    cursor: pointer;
    transition: var(--transition);
  }

  .auth-tab:hover {
    border-color: var(--primary-accent);
    color: var(--text-primary);
  }

  .auth-tab.active {
    background-color: var(--primary-accent);
    color: var(--primary-bg);
    border-color: var(--primary-accent);
  }

  .capabilities-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    gap: 0.75rem;
  }

  .capability-checkbox {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    padding: 0.5rem;
    border-radius: var(--border-radius);
    transition: var(--transition);
  }

  .capability-checkbox:hover {
    background-color: var(--tertiary-bg);
  }

  .capability-checkbox input[type="checkbox"] {
    display: none;
  }

  .checkmark {
    width: 18px;
    height: 18px;
    border: 2px solid var(--border-color);
    border-radius: 3px;
    position: relative;
    transition: var(--transition);
  }

  .capability-checkbox input[type="checkbox"]:checked + .checkmark {
    background-color: var(--primary-accent);
    border-color: var(--primary-accent);
  }

  .capability-checkbox input[type="checkbox"]:checked + .checkmark::after {
    content: '✓';
    position: absolute;
    top: -2px;
    left: 2px;
    color: var(--primary-bg);
    font-size: 12px;
    font-weight: bold;
  }

  .help-text {
    font-size: 0.8rem;
    color: var(--text-secondary);
    margin-top: 0.25rem;
    font-style: italic;
  }

  .test-btn {
    color: var(--text-primary);
    background-color: var(--surface-bg);
    border: 1px solid var(--border-color);
  }

  .test-btn:hover:not(:disabled) {
    border-color: var(--primary-accent);
    background-color: rgba(100, 255, 218, 0.1);
  }

  .node-header-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .node-name {
    font-weight: 600;
    color: var(--text-primary);
    font-size: 1rem;
  }

  .node-address {
    font-family: 'Fira Code', monospace;
    font-size: 0.85em;
    color: var(--text-secondary);
  }

  .node-details {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-top: 0.5rem;
  }

  .node-capabilities {
    display: flex;
    gap: 0.25rem;
    flex-wrap: wrap;
  }

  .capability-tag {
    background-color: var(--tertiary-bg);
    color: var(--text-secondary);
    padding: 0.15rem 0.5rem;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .node-last-seen {
    font-size: 0.8rem;
    color: var(--text-secondary);
    font-style: italic;
  }

  .node-list-container {
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

  .node-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .node-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background-color: var(--surface-bg);
    border-radius: var(--border-radius);
    border: 1px solid var(--border-color);
    transition: var(--transition);
  }

  .node-item:hover {
    border-color: var(--primary-accent);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .node-info {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .node-id {
    font-weight: 600;
    color: var(--text-primary);
  }

  .node-address {
    font-family: 'Fira Code', monospace;
    font-size: 0.85em;
    color: var(--text-secondary);
  }

  .node-status {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 500;
    padding: 0.25rem 0.75rem;
    border-radius: 20px;
    background-color: var(--tertiary-bg);
  }

  .node-status.online {
    color: var(--success-color);
    background-color: rgba(100, 255, 218, 0.1);
  }

  .node-status.offline {
    color: var(--error-color);
    background-color: rgba(255, 85, 85, 0.1);
  }

  .status-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: currentColor;
  }

  .node-actions {
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
