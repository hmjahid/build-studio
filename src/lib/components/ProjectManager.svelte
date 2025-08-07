<script lang="ts">
  import { onMount, createEventDispatcher } from 'svelte';
  import { projects } from '../stores/projects';
  import type { Project } from '../stores/projects';
  // Dynamic import of Tauri invoke will be performed in safeInvoke when needed
  // open will be dynamically imported in safeOpen to avoid Tauri errors in non‑Tauri environments

  const dispatch = createEventDispatcher();

  let showAddProject = false;
  let projectSource = 'local'; // 'local', 'github', 'gitlab', 'bitbucket'
  let newProjectName = '';
  let newProjectPath = '';
  let repositoryUrl = '';
  let cloneDirectory = '';
  let isLoading = false;
  let errorMessage = '';

  function isTauri(): boolean {
    return typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__;
  }

  // Safe wrapper for Tauri invoke commands
  async function safeInvoke<T>(cmd: string, args: any): Promise<T | null> {
    if (isTauri()) {
      try {
        const { invoke } = await import('@tauri-apps/api/core');
        // eslint-disable-next-line @typescript-eslint/no-unsafe-return
        return await invoke(cmd, args);
      } catch (e) {
        console.error(`Tauri invoke error (${cmd}):`, e);
        return null;
      }
    }
    // Non‑Tauri environment: return null or mock data as needed
    return null;
  }

  // Safe wrapper for Tauri dialog open (dynamic import)
  // Fallback to HTML file input in non-Tauri environments
  async function safeOpen(options: any): Promise<string | null> {
    if (isTauri()) {
      try {
        const { open } = await import('@tauri-apps/plugin-dialog');
        // eslint-disable-next-line @typescript-eslint/no-unsafe-return
        return await open(options);
      } catch (e) {
        console.error('Tauri open error:', e);
        return null;
      }
    }
    
    // Non-Tauri environment: use HTML file input as fallback
    console.warn('safeOpen called in non-Tauri environment, using HTML file input fallback');
    
    return new Promise((resolve) => {
      // Create a temporary file input element
      const input = document.createElement('input');
      input.type = 'file';
      input.webkitdirectory = options.directory || false;
      input.multiple = options.multiple || false;
      
      // Handle file selection
      input.onchange = () => {
        if (input.files && input.files.length > 0) {
          if (options.directory) {
            // For directory selection, we'll use the first file's path
            // Note: Browser security restrictions prevent getting the actual directory path
            // This is a limitation of the web platform
            const path = input.files[0].webkitRelativePath || input.files[0].name;
            const directoryPath = path.split('/').slice(0, -1).join('/') || path.split('\\').slice(0, -1).join('\\') || '';
            resolve(directoryPath);
          } else {
            // For file selection, return the first file's name
            resolve(input.files[0].name);
          }
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
  }



  async function fetchProjects() {
    let result: Project[] = [];
    const data = await safeInvoke('list_projects', {});
    if (data) {
      result = data;
    } else {
      // Mock data when running in non-Tauri environment
      result = [];
    }
    projects.set(result);
  }

  async function selectFolder() {
    console.log('selectFolder called');
    try {
      const selected = await safeOpen({
        directory: true,
        multiple: false,
        title: 'Select Project Folder'
      });
      
      console.log('safeOpen result:', selected);
      
      if (selected && typeof selected === 'string') {
        newProjectPath = selected;
        // Auto-generate project name from folder name
        if (!newProjectName) {
          const folderName = selected.split('/').pop() || selected.split('\\').pop() || '';
          newProjectName = folderName;
        }
        console.log('Project path set to:', selected);
        console.log('Project name set to:', newProjectName);
      } else {
        console.log('No folder selected or invalid selection');
      }
    } catch (e) {
      console.error('Failed to select folder:', e);
      errorMessage = `Failed to select folder: ${e}`;
    }
  }

  async function selectCloneDirectory() {
    try {
      const selected = await safeOpen({
        directory: true,
        multiple: false,
        title: 'Select Directory to Clone Repository'
      });
      
      if (selected && typeof selected === 'string') {
        cloneDirectory = selected;
      }
    } catch (e) {
      console.error('Failed to select clone directory:', e);
      errorMessage = `Failed to select clone directory: ${e}`;
    }
  }

  async function cloneRepository() {
    if (!repositoryUrl || !newProjectName) {
      errorMessage = 'Please provide repository URL and project name';
      return;
    }

    isLoading = true;
    errorMessage = '';
    
    try {
      // First select clone directory if not specified
      if (!cloneDirectory) {
        const selected = await safeOpen({
          directory: true,
          multiple: false,
          title: 'Select Directory to Clone Repository'
        });
        
        if (!selected || typeof selected !== 'string') {
          isLoading = false;
          return;
        }
        cloneDirectory = selected;
      }

      // Clone the repository (this would need backend implementation)
      let projectPath;
      const data = await safeInvoke('clone_repository', {
        url: repositoryUrl,
        name: newProjectName,
        destination: cloneDirectory
      });
      if (data) {
        projectPath = data;
      } else {
        // Mock path for non-Tauri environment
        projectPath = '/mock/path/' + newProjectName;
      }
      
      newProjectPath = projectPath as string;
      await addProject();
      resetForm();
    } catch (e) {
      errorMessage = `Failed to clone repository: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  async function addProject() {
    if (!newProjectName || !newProjectPath) {
      errorMessage = 'Please provide project name and path';
      return;
    }
    
    isLoading = true;
    errorMessage = '';
    
    try {
      const data = await safeInvoke('add_project', { name: newProjectName, path: newProjectPath });
      if (data) {
        console.log('add_project result:', data);
        await fetchProjects();
        resetForm();
      } else {
        // Assume success in mock environment
        console.log('add_project result: true');
        await fetchProjects();
        resetForm();
      }
    } catch (e) {
      errorMessage = `Error adding project: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  function resetForm() {
    showAddProject = false;
    newProjectName = '';
    newProjectPath = '';
    repositoryUrl = '';
    cloneDirectory = '';
    projectSource = 'local';
    errorMessage = '';
  }

  async function removeProject(name: string) {
    if (confirm(`Are you sure you want to remove project "${name}"?`)) {
      const data = await safeInvoke('remove_project', { name });
      if (data) {
        fetchProjects();
      } else {
        // Assume success in mock environment
        fetchProjects();
      }
    }
  }

  function selectProject(project: Project) {
    dispatch('selectProject', project);
  }

  function getProjectIcon(project: Project): string {
    // Determine icon based on project type or path
    const path = project.path.toLowerCase();
    if (path.includes('rust') || path.includes('cargo')) return '🦀';
    if (path.includes('node') || path.includes('package.json')) return '📦';
    if (path.includes('python') || path.includes('requirements.txt')) return '🐍';
    if (path.includes('java') || path.includes('maven') || path.includes('gradle')) return '☕';
    if (path.includes('go') || path.includes('go.mod')) return '🐹';
    if (path.includes('cpp') || path.includes('cmake')) return '⚡';
    return '📁';
  }

  onMount(fetchProjects);
</script>

<div class="project-manager">
  <div class="project-header">
    <h3>Projects</h3>
    <button class="add-project-btn" on:click={() => showAddProject = !showAddProject}>
      {showAddProject ? '✖ Cancel' : '+ Add Project'}
    </button>
  </div>

  {#if showAddProject}
    <div class="add-project-modal">
      <div class="project-source-selector">
        <h4>Add New Project</h4>
        <div class="source-tabs">
          <button 
            class="source-tab {projectSource === 'local' ? 'active' : ''}"
            on:click={() => projectSource = 'local'}
          >
            <span class="tab-icon">📁</span>
            <span class="tab-text">Local Folder</span>
          </button>
          <button 
            class="source-tab {projectSource === 'github' ? 'active' : ''}"
            on:click={() => projectSource = 'github'}
          >
            <span class="tab-icon">🐙</span>
            <span class="tab-text">GitHub</span>
          </button>
          <button 
            class="source-tab {projectSource === 'gitlab' ? 'active' : ''}"
            on:click={() => projectSource = 'gitlab'}
          >
            <span class="tab-icon">🦊</span>
            <span class="tab-text">GitLab</span>
          </button>
          <button 
            class="source-tab {projectSource === 'bitbucket' ? 'active' : ''}"
            on:click={() => projectSource = 'bitbucket'}
          >
            <span class="tab-icon">🪣</span>
            <span class="tab-text">Bitbucket</span>
          </button>
        </div>
      </div>

      {#if projectSource === 'local'}
        <div class="local-project-form">
          <div class="form-group">
            <label for="projectName">Project Name:</label>
            <input 
              id="projectName"
              type="text" 
              bind:value={newProjectName} 
              placeholder="Enter project name" 
            />
          </div>
          
          <div class="form-group">
            <label for="projectPath">Project Path:</label>
            <input 
              id="projectPath"
              type="text" 
              bind:value={newProjectPath} 
              placeholder="Select project folder" 
              readonly
            />
            <button type="button" class="browse-btn" on:click={selectFolder}>Browse</button>
          </div>
        </div>
      {:else}
        <div class="repo-project-form">
          <div class="form-group">
            <label for="repoUrl">Repository URL:</label>
            <input 
              id="repoUrl"
              type="text" 
              bind:value={repositoryUrl} 
              placeholder="https://github.com/user/repo.git" 
            />
          </div>
          
          <div class="form-group">
            <label for="projectName">Project Name:</label>
            <input 
              id="projectName"
              type="text" 
              bind:value={newProjectName} 
              placeholder="Enter project name" 
            />
          </div>
          
          <div class="form-group">
            <label for="cloneDir">Clone Directory:</label>
            <input 
              id="cloneDir"
              type="text" 
              bind:value={cloneDirectory} 
              placeholder="Select directory to clone into" 
              readonly
            />
            <button type="button" class="browse-btn" on:click={selectCloneDirectory}>Browse</button>
          </div>
        </div>
      {/if}

      {#if errorMessage}
        <div class="error-message">{errorMessage}</div>
      {/if}

      <div class="form-actions">
        <button class="cancel-btn" on:click={resetForm}>Cancel</button>
        {#if projectSource === 'local'}
          <button class="add-btn" on:click={addProject} disabled={isLoading || !newProjectName || !newProjectPath}>
            {isLoading ? 'Adding...' : 'Add Project'}
          </button>
        {:else}
          <button class="add-btn" on:click={cloneRepository} disabled={isLoading || !repositoryUrl || !newProjectName}>
            {isLoading ? 'Cloning...' : 'Clone & Add'}
          </button>
        {/if}
      </div>
    </div>
  {/if}

  <div class="project-list-container">
    {#if $projects.length === 0}
      <p class="empty-state">No projects added yet. Click "Add Project" to get started.</p>
    {:else}
      <ul class="project-list">
        {#each $projects as project}
          <li class="project-item">
            <button on:click={() => selectProject(project)} class="project-item-btn">
              <span class="project-icon">{getProjectIcon(project)}</span>
              <div class="project-info">
                <span class="project-name">{project.name}</span>
                <span class="project-path">{project.path}</span>
              </div>
            </button>
            <button class="remove-btn" on:click|stopPropagation={() => removeProject(project.name)} title="Remove Project">
              🗑️
            </button>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<style>
  .project-manager {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .project-header {
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

  .add-project-btn {
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

  .add-project-btn:hover {
    background-color: var(--secondary-accent);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(100, 255, 218, 0.3);
  }

  .add-project-modal {
    background-color: var(--secondary-bg);
    border-radius: var(--border-radius);
    padding: 1.5rem;
    border: 1px solid var(--border-color);
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  }

  .project-source-selector h4 {
    margin: 0 0 1rem 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .source-tabs {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    gap: 0.75rem;
    margin-bottom: 1.5rem;
  }

  .source-tab {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 1rem 0.5rem;
    font-size: 0.85em;
    font-weight: 500;
    background-color: var(--tertiary-bg);
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius);
    cursor: pointer;
    transition: var(--transition);
    justify-content: center;
    min-height: 80px;
  }

  .tab-icon {
    font-size: 1.5rem;
    line-height: 1;
  }

  .tab-text {
    font-size: 0.85rem;
    text-align: center;
    line-height: 1.3;
    white-space: normal;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100%;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
  }

  .source-tab:hover {
    border-color: var(--primary-accent);
    color: var(--text-primary);
  }

  .source-tab.active {
    background-color: var(--primary-accent);
    color: var(--primary-bg);
    border-color: var(--primary-accent);
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .form-group label {
    font-weight: 500;
    color: var(--text-primary);
    font-size: 0.95rem;
  }

  .form-group input {
    padding: 0.75em 1em;
    font-size: 0.95em;
    font-family: inherit;
    color: var(--text-primary);
    background-color: var(--tertiary-bg);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius);
    outline: none;
    transition: var(--transition);
  }

  .form-group input:focus {
    border-color: var(--primary-accent);
    box-shadow: 0 0 0 2px rgba(100, 255, 218, 0.2);
  }

  .form-group input::placeholder {
    color: var(--text-secondary);
  }

  .browse-btn {
    padding: 0.75em 1em;
    font-size: 0.9em;
    font-weight: 500;
    color: var(--text-primary);
    background-color: var(--surface-bg);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius);
    cursor: pointer;
    transition: var(--transition);
    white-space: nowrap;
    align-self: flex-start;
    margin-top: 0.5rem;
  }

  .browse-btn:hover {
    border-color: var(--primary-accent);
    background-color: rgba(100, 255, 218, 0.1);
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

  .cancel-btn, .add-btn {
    padding: 0.75em 1.5em;
    font-size: 0.95em;
    font-weight: 500;
    border: none;
    border-radius: var(--border-radius);
    cursor: pointer;
    transition: var(--transition);
  }

  .cancel-btn {
    color: var(--text-secondary);
    background-color: var(--tertiary-bg);
    border: 1px solid var(--border-color);
  }

  .cancel-btn:hover {
    color: var(--text-primary);
    border-color: var(--primary-accent);
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

  .project-list-container {
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
    font-style: italic;
  }

  .project-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .project-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    background-color: var(--surface-bg);
    border-radius: var(--border-radius);
    border: 1px solid var(--border-color);
    transition: var(--transition);
  }

  .project-item:hover {
    border-color: var(--primary-accent);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .project-item-btn {
    flex: 1;
    background: none;
    border: none;
    text-align: left;
    cursor: pointer;
    padding: 0;
    transition: var(--transition);
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .project-icon {
    font-size: 1.5rem;
    flex-shrink: 0;
  }

  .project-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    min-width: 0;
  }

  .project-name {
    font-weight: 600;
    color: var(--text-primary);
    font-size: 1rem;
  }

  .project-path {
    font-family: 'Fira Code', monospace;
    font-size: 0.8rem;
    color: var(--text-secondary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .project-item-btn:hover .project-name {
    color: var(--primary-accent);
  }

  .remove-btn {
    background: none;
    border: 1px solid var(--border-color);
    color: var(--text-secondary);
    cursor: pointer;
    padding: 0.5rem;
    border-radius: var(--border-radius);
    transition: var(--transition);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    font-size: 1rem;
  }

  .remove-btn:hover {
    border-color: var(--error-color);
    background-color: rgba(255, 85, 85, 0.1);
    color: var(--error-color);
  }
</style>