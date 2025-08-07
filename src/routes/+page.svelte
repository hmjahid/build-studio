
<script lang="ts">
  import ProjectManager from '../lib/components/ProjectManager.svelte';
  import BuildConfigViewer from '../lib/components/BuildConfigViewer.svelte';
  import { projects } from '../lib/stores/projects';
  import type { Project } from '../lib/stores/projects';
  import { onMount } from 'svelte';
  
  import PluginManager from '../lib/components/PluginManager.svelte';
import NodeManager from '../lib/components/NodeManager.svelte';
  import ConfigEditor from '../lib/components/ConfigEditor.svelte';
  import CiCdManager from '../lib/components/CiCdManager.svelte';
  import PackagingManager from '../lib/components/PackagingManager.svelte';
  import Settings from '../lib/components/Settings.svelte';
  import About from '../lib/components/About.svelte';
  import { writable } from 'svelte/store';
  // Direct Tauri invoke is avoided; safe wrappers are used in runBuild
  import { buildConfig } from '../lib/stores/buildconfig';
  import type { BuildStudioConfig } from '../lib/stores/buildconfig';

  let selectedProject: Project | null = null;
  let selectedBuildCmd: string | null = null;
  let buildLogs = writable<string[]>([]);
  let buildStatus = writable<string | null>(null);
  let activeTab = 'build';
  // New flags to control Settings and About pages outside of project view
  let showSettings = false;
  let showAbout = false;
  let showPlugins = false;
  let showProjects = false;
  let showBuilds = false;
  let showPackages = false;
  let showNodes = false;
  // Track which top menu item is currently active for UI highlighting
  let activeMenu = 'dashboard';

  $: projectList = $projects;

  function selectProject(project: Project) {
    selectedProject = project;
    selectedBuildCmd = null;
    // Reset page flags when a project is selected
    showSettings = false;
    showAbout = false;
    showPlugins = false;
  }

  function isTauri(): boolean {
    return typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__;
  }

  async function runBuild(command: string, cwd: string) {
    buildLogs.set([]);
    buildStatus.set('running');
    if (isTauri()) {
      try {
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('run_build', { command, cwd });
        buildStatus.set('success');
      } catch (e) {
        buildStatus.set('error');
        buildLogs.update(l => [...l, String(e)]);
      }
    } else {
      console.warn('runBuild called in non‑Tauri environment (mock)');
      // Mock success for development without Tauri
      buildStatus.set('success');
    }
  }

  function switchTab(tab: string) {
    activeTab = tab;
    const tabContent = document.querySelector('.tab-content');
    if (tabContent) {
      const sections = tabContent.querySelectorAll(':scope > div');
      sections.forEach(section => {
        (section as HTMLElement).style.display = 'none';
      });
      const activeSection = tabContent.querySelector(`.${tab}-section`);
      if (activeSection) {
        (activeSection as HTMLElement).style.display = 'block';
      }
    }

    const tabButtons = document.querySelectorAll('.tab-button');
    tabButtons.forEach(button => {
      button.classList.remove('active');
    });
    const activeButton = document.querySelector(`.tab-button[on\:click="() => switchTab('${tab}')"]`);
    if (activeButton) {
      activeButton.classList.add('active');
    }
  }

  // Handlers for header navigation to Settings and About pages
  function openDashboard() {
    showSettings = false;
    showAbout = false;
    showPlugins = false;
    showProjects = false;
    showBuilds = false;
    showPackages = false;
    showNodes = false;
    selectedProject = null;
    activeMenu = 'dashboard';
  }

  // Navigation handlers for other top menu items
  function openProjects() {
    showProjects = true;
    showBuilds = false;
    showPackages = false;
    showSettings = false;
    showAbout = false;
    showPlugins = false;
    showNodes = false;
    selectedProject = null;
    activeMenu = 'projects';
  }

  function openBuilds() {
    showProjects = false;
    showBuilds = true;
    showPackages = false;
    showSettings = false;
    showAbout = false;
    showPlugins = false;
    showNodes = false;
    selectedProject = null;
    activeMenu = 'builds';
  }

  function openPackages() {
    showProjects = false;
    showBuilds = false;
    showPackages = true;
    showSettings = false;
    showAbout = false;
    showPlugins = false;
    showNodes = false;
    selectedProject = null;
    activeMenu = 'packages';
  }

  function openSettings() {
    showSettings = true;
    showAbout = false;
    showPlugins = false;
    showNodes = false;
    selectedProject = null;
    activeMenu = 'settings';
  }

  function openAbout() {
    showAbout = true;
    showSettings = false;
    showPlugins = false;
    showNodes = false;
    selectedProject = null;
    activeMenu = 'about';
  }

  function openPlugins() {
    showPlugins = true;
    showSettings = false;
    showAbout = false;
    showNodes = false;
    selectedProject = null;
    activeMenu = 'plugins';
  }

  function openNodes() {
    showNodes = true;
    showSettings = false;
    showAbout = false;
    showPlugins = false;
    selectedProject = null;
    activeMenu = 'nodes';
  }
</script>


<div class="app-layout">
  <header class="app-header">
    <h1>Build Studio</h1>
    <nav class="app-header-nav">
      <a href="#dashboard" class:active={activeMenu === 'dashboard'} on:click|preventDefault={openDashboard}>Dashboard</a>
       <a href="#projects" class:active={activeMenu === 'projects'} on:click|preventDefault={openProjects}>Projects</a>
       <a href="#builds" class:active={activeMenu === 'builds'} on:click|preventDefault={openBuilds}>Builds</a>
       <a href="#packages" class:active={activeMenu === 'packages'} on:click|preventDefault={openPackages}>Packages</a>
      <a href="#settings" on:click|preventDefault={openSettings} class:active={activeMenu === 'settings'}>Settings</a>
      <a href="#about" on:click|preventDefault={openAbout} class:active={activeMenu === 'about'}>About</a>
      <a href="#plugins" on:click|preventDefault={openPlugins} class:active={activeMenu === 'plugins'}>Plugins</a>
      <a href="#nodes" on:click|preventDefault={openNodes} class:active={activeMenu === 'nodes'}>Nodes</a>
    </nav>
  </header>

  <div class="main-content">
    <div class="sidebar">
      <ProjectManager on:selectProject={(e) => selectProject(e.detail)} />
    </div>

    <main class="main-area">
      {#if showSettings}
        <Settings />
      {:else if showAbout}
        <About />
      {:else if showPlugins}
        <PluginManager />
      {:else if showNodes}
        <NodeManager />
      {:else if showProjects}
        <div class="placeholder-section"><h2>Projects Overview</h2><p>Use the sidebar to manage your projects.</p></div>
      {:else if showBuilds}
        <div class="placeholder-section"><h2>Builds Overview</h2><p>Build history and controls will appear here.</p></div>
      {:else if showPackages}
        <div class="placeholder-section"><h2>Packages Overview</h2><p>Package management UI will appear here.</p></div>
      {:else if selectedProject}
        <div class="project-details">
          <div class="project-header">
            <h2>{selectedProject.name}</h2>
            <span class="path">Path: {selectedProject.path}</span>
          </div>

          <div class="tabs">
            <button class="tab-button active" on:click={() => switchTab('build')}>Build</button>
            <button class="tab-button" on:click={() => switchTab('config')}>Configuration</button>
            <button class="tab-button" on:click={() => switchTab('packaging')}>Packaging</button>
            <button class="tab-button" on:click={() => switchTab('cicd')}>CI/CD</button>
            <button class="tab-button" on:click={() => switchTab('remote')}>Remote Nodes</button>
            
            
          </div>

          <div class="tab-content">
            <div class="build-section">
              <div class="build-controls">
                <h3>Run a Build</h3>
                {#if $buildConfig && $buildConfig.builds && $buildConfig.builds.length > 0}
                  <select bind:value={selectedBuildCmd}>
                    <option value={null}>Select build...</option>
                    {#each $buildConfig.builds as build}
                      <option value={build.command}>{build.name} ({build.platform})</option>
                    {/each}
                  </select>
                  <button
                    disabled={!selectedBuildCmd}
                    on:click={() => {
                      if (selectedBuildCmd && selectedProject) {
                        runBuild(selectedBuildCmd, selectedProject.path);
                      }
                    }}>
                    Run Build
                  </button>
                {:else}
                  <p>No builds defined in config.</p>
                {/if}
              </div>
              
            </div>

            <div class="config-section" style="display: none;">
              <BuildConfigViewer configPath={selectedProject.config_path} />
              <ConfigEditor configPath={selectedProject.config_path} />
            </div>

            <div class="packaging-section" style="display: none;">
              <PackagingManager />
            </div>

            <div class="cicd-section" style="display: none;">
              <CiCdManager />
            </div>

            <div class="remote-section" style="display: none;">
              <NodeManager />
            </div>

            
            
            
          </div>
        </div>
      {:else}
        <div class="no-project-selected">
          <p>Select a project from the sidebar to get started.</p>
        </div>
      {/if}
    </main>
  </div>
</div>

<style>
  :root {
    /* Enterprise-grade color palette */
    --primary-bg: #0a192f;        /* Deep navy blue */
    --secondary-bg: #112240;      /* Slightly lighter navy */
    --tertiary-bg: #233554;       /* Accent navy */
    --surface-bg: #1e3a5f;        /* Surface color */
    --text-primary: #e6f1ff;      /* Primary text */
    --text-secondary: #8892b0;    /* Secondary text */
    --primary-accent: #64ffda;    /* Accent teal */
    --accent-hover: #4cdcca;      /* Hover accent */
    --border-color: #233554;      /* Border color */
    --success-color: #64ffda;     /* Success green */
    --warning-color: #f7d353;     /* Warning yellow */
    --error-color: #ff5555;       /* Error red */
    --font-family: 'Inter', 'Segoe UI', system-ui, -apple-system, sans-serif;
    --border-radius: 6px;
    --transition: all 0.25s cubic-bezier(0.645, 0.045, 0.355, 1);
  }

  .app-layout {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background-color: var(--primary-bg);
    color: var(--text-color);
    font-family: var(--font-family);
  }

  .app-header {
    padding: 0.75rem 1.5rem;
    background-color: var(--secondary-bg);
    border-bottom: 1px solid var(--border-color);
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
  }

  .app-header h1 {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 600;
    background: linear-gradient(90deg, var(--primary-accent), #4cc9f0);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .app-header-nav {
    display: flex;
    gap: 1.5rem;
  }

  .app-header-nav a {
    color: var(--text-secondary);
    text-decoration: none;
    font-size: 0.9rem;
    font-weight: 500;
    transition: var(--transition);
    padding: 0.25rem 0.5rem;
    border-radius: var(--border-radius);
  }

  .app-header-nav a:hover,
  .app-header-nav a.active {
    color: var(--primary-accent);
    background-color: rgba(100, 255, 218, 0.1);
  }

  .main-content {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .sidebar {
    width: 280px;
    background-color: var(--secondary-bg);
    border-right: 1px solid var(--border-color);
    padding: 1.5rem 1rem;
    overflow-y: auto;
    box-shadow: 2px 0 10px rgba(0, 0, 0, 0.1);
  }

  .main-area {
    flex: 1;
    padding: 2rem;
    overflow-y: auto;
    background-color: var(--primary-bg);
  }

  .project-details {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .project-header {
    padding-bottom: 1.5rem;
    border-bottom: 1px solid var(--border-color);
    margin-bottom: 1.5rem;
  }

  .project-header h2 {
    margin: 0 0 0.5rem 0;
    font-size: 2rem;
    color: var(--text-primary);
  }

  .path {
    font-size: 0.95rem;
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }



  .tabs {
    display: flex;
    gap: 0.25rem;
    border-bottom: 1px solid var(--border-color);
    margin-bottom: 1.5rem;
  }

  .tab-button {
    padding: 0.75rem 1.5rem;
    background: var(--secondary-bg);
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 0.95rem;
    font-weight: 500;
    border-radius: var(--border-radius) var(--border-radius) 0 0;
    transition: var(--transition);
    position: relative;
  }

  .tab-button.active {
    background: var(--surface-bg);
    color: var(--primary-accent);
  }

  .tab-button.active::after {
    content: '';
    position: absolute;
    bottom: -1px;
    left: 0;
    right: 0;
    height: 2px;
    background: var(--primary-accent);
  }

  .tab-button:hover:not(.active) {
    background: var(--tertiary-bg);
    color: var(--text-primary);
  }

  .tab-content > div {
    padding-top: 1.5rem;
  }

  .build-controls {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  button {
    cursor: pointer;
    background-color: var(--primary-accent);
    color: var(--primary-bg);
    font-weight: 600;
    border: none;
    padding: 0.7em 1.4em;
  }

  button:hover:not(:disabled) {
    background-color: var(--accent-hover);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(100, 255, 218, 0.2);
  }

  button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none;
    box-shadow: none;
  }

  .no-project-selected {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
    font-size: 1.2rem;
    color: #888;
  }
</style>