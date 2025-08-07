<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { open } from '@tauri-apps/plugin-dialog';

  interface BuildNode {
    id: string;
    name: string;
    type: 'remote' | 'local-docker' | 'local-vm';
    address?: string;
    port?: number;
    username?: string;
    status: 'online' | 'offline' | 'connecting' | 'installing' | 'starting' | 'stopping';
    capabilities: string[];
    last_seen?: string;
    vm_config?: {
      platform: string;
      memory: number;
      cpu_cores: number;
      vm_type: 'docker' | 'wsl' | 'kvm' | 'qemu' | 'hyperv' | 'macos-vm';
    };
  }

  let nodes: BuildNode[] = [];
  let showNodeSelector = false;
  let nodeSelectionType: 'existing' | 'remote' | 'new-local' = 'existing';
  let isLoading = false;
  let errorMessage = '';
  let successMessage = '';
  let editingNode: BuildNode | null = null;

  // Form fields
  let nodeName = '';
  let nodeAddress = '';
  let nodePort = 22;
  let nodeUsername = '';
  let authType: 'password' | 'key' = 'key';
  let nodePassword = '';
  let sshKeyPath = '';
  let localNodeName = '';
  let localNodePlatform = 'ubuntu-22.04';
  let localNodeMemory = 4096;
  let localNodeCpuCores = 2;
  let virtualizationTech = 'auto';
  let selectedCapabilities = new Set(['build']);
  let selectedLanguages = new Set(['rust', 'node', 'python']);

  let detectedVirtualization: string[] = [];
  let systemInfo: any = {};

  async function loadNodes() {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      // Load both local and remote nodes
      const remoteNodes = await invoke('list_remote_nodes');
      const localNodes = await invoke('list_build_nodes');
      
      nodes = [...remoteNodes, ...localNodes];
    } catch (error) {
      console.error('Failed to load nodes:', error);
      // Mock data for development
      nodes = [
        {
          id: '1',
          name: 'Local Docker Node',
          type: 'local-docker',
          status: 'online',
          capabilities: ['build'],
          vm_config: {
            platform: 'ubuntu-22.04',
            memory: 4096,
            cpu_cores: 2,
            vm_type: 'docker'
          }
        },
        {
          id: '2', 
          name: 'Remote Build Server',
          type: 'remote',
          address: '192.168.1.100',
          port: 22,
          username: 'builder',
          status: 'online',
          capabilities: ['build', 'test'],
          last_seen: '2024-01-15T09:15:00Z'
        }
      ];
    }
  }

  async function detectVirtualization() {
    try {
      detectedVirtualization = await detectVirtualizationSupport();
      systemInfo = await getSystemInfo();
    } catch (e) {
      // Mock detection
      detectedVirtualization = ['docker', 'kvm', 'qemu'];
      systemInfo = { os: 'linux', arch: 'x86_64', memory: 16384 };
    }
  }

  async function detectVirtualizationSupport() {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke('detect_virtualization_support');
    } catch (error) {
      console.error('Failed to detect virtualization support:', error);
      return [];
    }
  }

  async function getSystemInfo() {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke('get_system_info');
    } catch (error) {
      console.error('Failed to get system info:', error);
      return {
        os: 'unknown',
        arch: 'unknown',
        memory: 0,
        cpuCores: 0,
        virtualizationSupport: {}
      };
    }
  }

  async function createLocalNode() {
    if (!localNodeName) {
      errorMessage = 'Please provide a node name';
      return;
    }

    isLoading = true;
    errorMessage = '';
    
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const nodeConfig = {
        name: localNodeName,
        platform: localNodePlatform,
        memory: parseInt(localNodeMemory.toString()),
        cpu_cores: parseInt(localNodeCpuCores.toString()),
        virtualization: virtualizationTech,
        capabilities: Array.from(selectedCapabilities),
        languages: Array.from(selectedLanguages)
      };
      
      console.log('Creating local node with config:', nodeConfig);
      
      const nodeId = await invoke('create_local_node', { config: nodeConfig });
      
      successMessage = `Local node "${localNodeName}" created successfully! ID: ${nodeId}`;
      resetForm();
      await loadNodes();
    } catch (e) {
      errorMessage = `Failed to create local node: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  async function startNode(nodeId: string) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('start_node', { id: nodeId });
      successMessage = 'Node started successfully';
      await loadNodes();
    } catch (error) {
      errorMessage = `Failed to start node: ${error}`;
    }
  }

  async function stopNode(nodeId: string) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('stop_node', { id: nodeId });
      successMessage = 'Node stopped successfully';
      await loadNodes();
    } catch (error) {
      errorMessage = `Failed to stop node: ${error}`;
    }
  }

  async function removeNode(nodeId: string) {
    if (!confirm('Are you sure you want to remove this node?')) {
      return;
    }
    
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('remove_node', { id: nodeId });
      successMessage = 'Node removed successfully';
      await loadNodes();
    } catch (error) {
      errorMessage = `Failed to remove node: ${error}`;
    }
  }

  async function scanLocalNodes() {
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    const discoveredNodes = await invoke('scan_local_nodes');
    console.log('Discovered local nodes:', discoveredNodes);
    successMessage = `Found ${discoveredNodes.length} local nodes`;
    await loadNodes();
  } catch (error) {
    errorMessage = `Failed to scan local nodes: ${error}`;
  }
}

// Edit node functionality
function editNode(node: BuildNode) {
    editingNode = node;
    
    // Populate form with node data
    nodeName = node.name;
    if (node.type === 'remote') {
      nodeAddress = node.address || '';
      nodePort = node.port || 22;
      nodeUsername = node.username || '';
    } else if (node.type === 'local-docker' || node.type === 'local-vm') {
      localNodeName = node.name;
      if (node.vm_config) {
        localNodePlatform = node.vm_config.platform;
        localNodeMemory = node.vm_config.memory;
        localNodeCpuCores = node.vm_config.cpu_cores;
      }
    }
    
    // Set the appropriate tab
    if (node.type === 'remote') {
      nodeSelectionType = 'remote';
    } else {
      nodeSelectionType = 'new-local';
    }
    
    showNodeSelector = true;
  }

  async function updateNode() {
    if (!editingNode) return;
    
    isLoading = true;
    errorMessage = '';
    
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      
      if (editingNode.type === 'remote') {
        // Update remote node
        await invoke('update_remote_node', {
          id: editingNode.id,
          name: nodeName,
          address: nodeAddress,
          port: nodePort,
          username: nodeUsername
        });
        successMessage = `Remote node "${nodeName}" updated successfully`;
      } else {
        // For local nodes, we'll need to recreate them
        // This is a simplified approach - in a real app you might want to preserve data
        await invoke('remove_node', { id: editingNode.id });
        
        const nodeConfig = {
          name: localNodeName,
          platform: localNodePlatform,
          memory: parseInt(localNodeMemory.toString()),
          cpu_cores: parseInt(localNodeCpuCores.toString()),
          virtualization: virtualizationTech,
          capabilities: Array.from(selectedCapabilities),
          languages: Array.from(selectedLanguages)
        };
        
        const nodeId = await invoke('create_local_node', { config: nodeConfig });
        successMessage = `Local node "${localNodeName}" updated successfully! ID: ${nodeId}`;
      }
      
      resetForm();
      await loadNodes();
    } catch (e) {
      errorMessage = `Failed to update node: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  function resetForm() {
    showNodeSelector = false;
    nodeSelectionType = 'existing';
    nodeName = '';
    localNodeName = '';
    errorMessage = '';
  }

  function getNodeTypeIcon(node: BuildNode): string {
    switch (node.type) {
      case 'remote': return '🌐';
      case 'local-docker': return '🐳';
      case 'local-vm': return '💻';
      default: return '🖥️';
    }
  }

  function getNodeTypeName(nodeType: string): string {
    switch (nodeType) {
      case 'remote': return 'Remote';
      case 'local-docker': return 'Local Docker';
      case 'local-vm': return 'Local VM';
      default: return 'Unknown';
    }
  }

  onMount(() => {
    loadNodes();
    detectVirtualization();
  });
</script>

<div class="node-manager">
  <div class="node-header">
    <h3>Build Nodes</h3>
    <button class="add-node-btn" on:click={() => showNodeSelector = !showNodeSelector}>
      {showNodeSelector ? '✖ Cancel' : '+ Add Node'}
    </button>
  </div>

  {#if showNodeSelector}
    <div class="node-selector-modal">
      <h3>{editingNode ? 'Edit Node' : 'Add New Node'}</h3>
      
      {#if !editingNode}
        <div class="node-type-selector">
          <button 
            class="type-btn {nodeSelectionType === 'existing' ? 'active' : ''}"
            on:click={() => nodeSelectionType = 'existing'}
          >
            Use Existing
          </button>
          <button 
            class="type-btn {nodeSelectionType === 'remote' ? 'active' : ''}"
            on:click={() => nodeSelectionType = 'remote'}
          >
            Add Remote
          </button>
          <button 
            class="type-btn {nodeSelectionType === 'new-local' ? 'active' : ''}"
            on:click={() => nodeSelectionType = 'new-local'}
          >
            Install Local
          </button>
        </div>
      {/if}

        {#if nodeSelectionType === 'existing'}
          <div class="existing-nodes-section">
            <p class="section-description">
              Connect to an already running build node or container on your system.
            </p>
            
            <div class="detection-info">
              <h5>Detected Virtualization Support</h5>
              <div class="virtualization-grid">
                {#each detectedVirtualization as tech}
                  <div class="virtualization-item">
                    {tech === 'docker' ? '🐳' : tech === 'kvm' ? '⚡' : '💻'} {tech.toUpperCase()}
                  </div>
                {/each}
              </div>
            </div>

              <div class="scan-section">
                <button class="scan-btn" on:click={scanLocalNodes}>
                  🔍 Scan for Local Nodes
                </button>
                <p class="help-text">Automatically detect running Docker containers and VMs</p>
              </div>
            </div>

          {:else if nodeSelectionType === 'new-local'}
            <div class="local-node-form">
              <h5>Create New Local Build Node</h5>
              
              <div class="form-grid">
                <div class="form-group">
                  <label for="localNodeName">Node Name:</label>
                  <input 
                    id="localNodeName"
                    type="text" 
                    bind:value={localNodeName} 
                    placeholder="Ubuntu Build Node" 
                  />
                </div>
                
                <div class="form-group">
                  <label for="virtualizationTech">Virtualization:</label>
                  <select id="virtualizationTech" bind:value={virtualizationTech}>
                    <option value="auto">Auto-detect Best</option>
                    <option value="docker">🐳 Docker</option>
                    <option value="wsl">🪟 WSL (Windows)</option>
                    <option value="kvm">⚡ KVM (Linux)</option>
                    <option value="qemu">🔧 QEMU</option>
                    <option value="macos-vm">🍎 macOS VM</option>
                  </select>
                </div>
                
                <div class="form-group">
                  <label for="localNodePlatform">Platform:</label>
                  <select id="localNodePlatform" bind:value={localNodePlatform}>
                    <option value="ubuntu-22.04">Ubuntu 22.04</option>
                    <option value="ubuntu-20.04">Ubuntu 20.04</option>
                    <option value="debian-11">Debian 11</option>
                    <option value="windows-server-2022">Windows Server 2022</option>
                    <option value="alpine-3.18">Alpine 3.18</option>
                  </select>
                </div>
              </div>

              <div class="specs-section">
                <h6>Resource Allocation</h6>
                <div class="specs-grid">
                  <div class="form-group">
                    <label for="localNodeMemory">Memory (MB):</label>
                    <input 
                      id="localNodeMemory"
                      type="number" 
                      bind:value={localNodeMemory} 
                      min="512" 
                      max="16384"
                      step="512"
                    />
                  </div>
                  
                  <div class="form-group">
                    <label for="localNodeCpuCores">CPU Cores:</label>
                    <input 
                      id="localNodeCpuCores"
                      type="number" 
                      bind:value={localNodeCpuCores} 
                      min="1" 
                      max="16"
                    />
                  </div>
                </div>
              </div>
            </div>
          {/if}

          <div class="form-actions">
            <button class="cancel-btn" on:click={resetForm}>Cancel</button>
            {#if nodeSelectionType === 'new-local'}
              <button 
              class="create-btn" 
              on:click={editingNode ? updateNode : createLocalNode} 
              disabled={isLoading || (editingNode ? false : !localNodeName)}
            >
              {isLoading ? (editingNode ? 'Updating...' : 'Creating...') : (editingNode ? 'Update Node' : 'Create Local Node')}
            </button>
            {/if}
          </div>
        </div>
      {/if}
    
    <div class="node-list-container">
        {#if nodes.length === 0}
          <p class="empty-state">No build nodes configured. Add a node to get started.</p>
        {:else}
          <ul class="node-list">
            {#each nodes as node}
              <li class="node-item">
                <div class="node-info">
                  <div class="node-header-info">
                    <span class="node-icon">{getNodeTypeIcon(node)}</span>
                    <div class="node-details">
                      <div class="node-name">{node.name}</div>
                      <div class="node-type">{getNodeTypeName(node.type)}</div>
                      {#if node.type === 'remote' && node.address}
                        <div class="node-address">{node.address}:{node.port}</div>
                      {:else if node.type.startsWith('local-') && node.vm_config}
                        <div class="node-platform">{node.vm_config.platform}</div>
                        <div class="node-specs">{node.vm_config.memory}MB RAM • {node.vm_config.cpu_cores} CPU</div>
                      {/if}
                      {#if node.capabilities && node.capabilities.length > 0}
                        <div class="node-capabilities">
                          {#each node.capabilities as capability}
                            <span class="capability-tag">{capability}</span>
                          {/each}
                        </div>
                      {/if}
                    </div>
                  </div>
                </div>
                
                <div class="node-actions">
                  <button class="action-btn edit" on:click={() => editNode(node)} title="Edit Node">
                    ✏️
                  </button>
                  
                  {#if node.status === 'offline'}
                    <button class="action-btn start" on:click={() => startNode(node.id)} title="Start Node">
                      ▶
                    </button>
                  {:else if node.status === 'online'}
                    <button class="action-btn stop" on:click={() => stopNode(node.id)} title="Stop Node">
                      ⏹
                    </button>
                  {/if}
                  
                  <button class="action-btn remove" on:click={() => removeNode(node.id)} title="Remove Node">
                    🗑
                  </button>
                </div>
              </li>
            {/each}
          </ul>
        {/if}
      </div>
</div>

<style>
  .node-manager {
    padding: 1.5rem;
    background-color: var(--primary-bg);
    min-height: 100vh;
  }

  .node-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  .node-header h3 {
    margin: 0;
    color: var(--text-primary);
    font-size: 1.5rem;
    font-weight: 600;
  }

  .add-node-btn {
    background-color: var(--primary-accent);
    color: var(--primary-bg);
    border: none;
    padding: 0.5rem 1rem;
    border-radius: var(--border-radius);
    font-weight: 500;
    cursor: pointer;
    transition: var(--transition);
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .add-node-btn:hover {
    background-color: var(--secondary-accent);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(100, 255, 218, 0.3);
  }

  .node-selector-modal {
    background-color: var(--secondary-bg);
    border-radius: var(--border-radius);
    border: 1px solid var(--border-color);
    padding: 1.5rem;
    margin-bottom: 2rem;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
  }

  .node-selector-modal h3 {
    margin-top: 0;
    color: var(--text-primary);
    font-size: 1.25rem;
  }

  .node-type-selector {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
  }

  .type-btn {
    flex: 1;
    min-width: 120px;
    padding: 0.75rem;
    border: 1px solid var(--border-color);
    background-color: var(--surface-bg);
    color: var(--text-primary);
    border-radius: var(--border-radius);
    cursor: pointer;
    transition: var(--transition);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    font-weight: 500;
  }

  .type-btn:hover {
    border-color: var(--primary-accent);
    background-color: rgba(100, 255, 218, 0.1);
  }

  .type-btn.active {
    border-color: var(--primary-accent);
    background-color: var(--primary-accent);
    color: var(--primary-bg);
  }

  .type-btn .tab-icon {
    font-size: 1.25rem;
  }

  .type-btn .tab-text {
    font-size: 0.85rem;
  }

  .section-description {
    color: var(--text-secondary);
    margin-bottom: 1rem;
    font-size: 0.9rem;
    line-height: 1.5;
  }

  .detection-info {
    background-color: var(--surface-bg);
    border-radius: var(--border-radius);
    padding: 1rem;
    margin-bottom: 1rem;
    border: 1px solid var(--border-color);
  }

  .detection-info h5 {
    margin-top: 0;
    color: var(--text-primary);
    font-size: 1rem;
  }

  .virtualization-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    gap: 0.75rem;
  }

  .virtualization-item {
    background-color: var(--tertiary-bg);
    padding: 0.5rem;
    border-radius: var(--border-radius);
    text-align: center;
    font-size: 0.85rem;
    font-weight: 500;
  }

  .existing-nodes-section, .remote-node-form, .local-node-form {
    background-color: var(--surface-bg);
    border-radius: var(--border-radius);
    padding: 1.5rem;
    border: 1px solid var(--border-color);
  }

  .existing-nodes-section h4, .remote-node-form h4, .local-node-form h4 {
    margin-top: 0;
    color: var(--text-primary);
  }

  .form-group {
    margin-bottom: 1rem;
  }

  .form-group label {
    display: block;
    margin-bottom: 0.25rem;
    color: var(--text-primary);
    font-weight: 500;
  }

  .form-group input, .form-group select, .form-group textarea {
    width: 100%;
    padding: 0.5rem;
    border-radius: var(--border-radius);
    border: 1px solid var(--border-color);
    background-color: var(--secondary-bg);
    color: var(--text-primary);
    font-family: inherit;
  }

  .form-group input:focus, .form-group select:focus, .form-group textarea:focus {
    outline: none;
    border-color: var(--primary-accent);
    box-shadow: 0 0 0 2px rgba(100, 255, 218, 0.2);
  }

  .checkbox-group {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    margin-top: 0.5rem;
  }

  .checkbox-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background-color: var(--tertiary-bg);
    padding: 0.5rem 0.75rem;
    border-radius: var(--border-radius);
    font-size: 0.85rem;
  }

  .checkbox-item input {
    width: auto;
  }

  .specs-section {
    background-color: var(--tertiary-bg);
    border-radius: var(--border-radius);
    padding: 1rem;
    margin-top: 1rem;
  }

  .specs-section h6 {
    margin-top: 0;
    color: var(--text-primary);
    font-size: 0.9rem;
  }

  .specs-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 1rem;
  }

  .form-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
    margin-top: 1.5rem;
  }

  .add-btn, .scan-btn, .cancel-btn {
    padding: 0.5rem 1rem;
    border-radius: var(--border-radius);
    font-weight: 500;
    border: none;
    cursor: pointer;
    transition: var(--transition);
  }

  .add-btn {
    color: var(--primary-bg);
    background-color: var(--primary-accent);
  }

  .add-btn:hover:not(:disabled) {
    background-color: var(--secondary-accent);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(100, 255, 218, 0.3);
  }

  .add-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .scan-btn {
    color: var(--text-primary);
    background-color: var(--surface-bg);
    border: 1px solid var(--border-color);
  }

  .scan-btn:hover {
    background-color: var(--secondary-bg);
  }

  .cancel-btn {
    color: var(--text-secondary);
    background-color: var(--tertiary-bg);
    border: 1px solid var(--border-color);
  }

  .cancel-btn:hover {
    background-color: var(--surface-bg);
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
  }

  .empty-subtext {
    font-size: 0.9rem;
    margin: 0;
    font-style: italic;
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

  .node-header-info {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 0.5rem;
  }

  .node-icon {
    font-size: 1.5rem;
  }

  .node-details {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .node-name {
    font-weight: 600;
    color: var(--text-primary);
    font-size: 1rem;
  }

  .node-type {
    background-color: var(--tertiary-bg);
    color: var(--text-secondary);
    padding: 0.25rem 0.5rem;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .node-address, .node-platform, .node-specs {
    font-family: 'Fira Code', monospace;
    font-size: 0.8rem;
    color: var(--text-secondary);
  }

  .node-capabilities {
    display: flex;
    gap: 0.25rem;
    flex-wrap: wrap;
    margin-top: 0.25rem;
  }

  .capability-tag {
    background-color: var(--tertiary-bg);
    color: var(--text-secondary);
    padding: 0.15rem 0.5rem;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .node-actions {
    display: flex;
    gap: 0.5rem;
  }

  .control-btn {
    padding: 0.25rem 0.5rem;
    border-radius: var(--border-radius);
    border: 1px solid var(--border-color);
    background-color: var(--surface-bg);
    color: var(--text-primary);
    cursor: pointer;
    transition: var(--transition);
    font-size: 0.8rem;
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .control-btn:hover:not(:disabled) {
    border-color: var(--primary-accent);
    background-color: rgba(100, 255, 218, 0.1);
  }

  .control-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .start-btn {
    color: var(--success-color);
    border-color: var(--success-color);
  }

  .stop-btn {
    color: var(--warning-color);
    border-color: var(--warning-color);
  }

  .remove-btn {
    color: var(--error-color);
    border-color: var(--error-color);
  }

  .edit-btn {
    color: var(--primary-accent);
    border-color: var(--primary-accent);
  }

  .success-message {
    color: var(--success-color);
    background-color: rgba(85, 255, 127, 0.1);
    padding: 0.75rem;
    border-radius: var(--border-radius);
    border: 1px solid var(--success-color);
    margin-bottom: 1rem;
    font-size: 0.9rem;
  }

  .error-message {
    color: var(--error-color);
    background-color: rgba(255, 85, 85, 0.1);
    padding: 0.75rem;
    border-radius: var(--border-radius);
    border: 1px solid var(--error-color);
    margin-bottom: 1rem;
    font-size: 0.9rem;
  }

  .form-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
    margin-top: 1.5rem;
  }
</style>
