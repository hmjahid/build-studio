<script lang="ts">
  import { onMount } from 'svelte';
  import { writable } from 'svelte/store';

  // Theme store
  export const theme = writable('dark');

  let currentTheme = 'dark';
  let autoSave = true;
  let buildNotifications = true;
  let errorNotifications = true;
  let successNotifications = true;
  let maxConcurrentBuilds = 3;
  let buildTimeout = 30;
  let defaultBuildPath = './builds';
  let defaultPackagePath = './packages';
  let enableTelemetry = false;
  let enableAutoUpdates = true;

  const themes = [
    { id: 'dark', name: 'Dark Theme', description: 'Default dark theme with blue accents' },
    { id: 'light', name: 'Light Theme', description: 'Clean light theme for daytime use' },
    { id: 'midnight', name: 'Midnight Blue', description: 'Deep blue theme for night coding' },
    { id: 'forest', name: 'Forest Green', description: 'Nature-inspired green theme' },
    { id: 'sunset', name: 'Sunset Orange', description: 'Warm orange theme for creativity' }
  ];

  onMount(() => {
    // Load saved settings from localStorage
    loadSettings();
    // Apply the current theme
    applyTheme(currentTheme);
  });

  function loadSettings() {
    try {
      const savedTheme = localStorage.getItem('build-studio-theme');
      if (savedTheme) {
        currentTheme = savedTheme;
        theme.set(savedTheme);
      }

      const savedSettings = localStorage.getItem('build-studio-settings');
      if (savedSettings) {
        const settings = JSON.parse(savedSettings);
        autoSave = settings.autoSave ?? true;
        buildNotifications = settings.buildNotifications ?? true;
        errorNotifications = settings.errorNotifications ?? true;
        successNotifications = settings.successNotifications ?? true;
        maxConcurrentBuilds = settings.maxConcurrentBuilds ?? 3;
        buildTimeout = settings.buildTimeout ?? 30;
        defaultBuildPath = settings.defaultBuildPath ?? './builds';
        defaultPackagePath = settings.defaultPackagePath ?? './packages';
        enableTelemetry = settings.enableTelemetry ?? false;
        enableAutoUpdates = settings.enableAutoUpdates ?? true;
      }
    } catch (error) {
      console.error('Failed to load settings:', error);
    }
  }

  function saveSettings() {
    try {
      const settings = {
        autoSave,
        buildNotifications,
        errorNotifications,
        successNotifications,
        maxConcurrentBuilds,
        buildTimeout,
        defaultBuildPath,
        defaultPackagePath,
        enableTelemetry,
        enableAutoUpdates
      };
      localStorage.setItem('build-studio-settings', JSON.stringify(settings));
      localStorage.setItem('build-studio-theme', currentTheme);
    } catch (error) {
      console.error('Failed to save settings:', error);
    }
  }

  function changeTheme(themeId: string) {
    currentTheme = themeId;
    theme.set(themeId);
    applyTheme(themeId);
    saveSettings();
  }

  function applyTheme(themeId: string) {
    const root = document.documentElement;
    
    // Remove existing theme classes
    root.classList.remove('theme-dark', 'theme-light', 'theme-midnight', 'theme-forest', 'theme-sunset');
    
    // Add new theme class
    root.classList.add(`theme-${themeId}`);
    
    // Apply theme-specific CSS variables
    switch (themeId) {
      case 'light':
        root.style.setProperty('--primary-bg', '#ffffff');
        root.style.setProperty('--secondary-bg', '#f8f9fa');
        root.style.setProperty('--tertiary-bg', '#e9ecef');
        root.style.setProperty('--text-primary', '#212529');
        root.style.setProperty('--text-secondary', '#6c757d');
        root.style.setProperty('--border-color', '#dee2e6');
        root.style.setProperty('--primary-accent', '#0066cc');
        root.style.setProperty('--secondary-accent', '#0052a3');
        break;
      case 'midnight':
        root.style.setProperty('--primary-bg', '#0f1419');
        root.style.setProperty('--secondary-bg', '#1a1f29');
        root.style.setProperty('--tertiary-bg', '#252a35');
        root.style.setProperty('--text-primary', '#e6e6e6');
        root.style.setProperty('--text-secondary', '#b3b3b3');
        root.style.setProperty('--border-color', '#3d4450');
        root.style.setProperty('--primary-accent', '#4a9eff');
        root.style.setProperty('--secondary-accent', '#357abd');
        break;
      case 'forest':
        root.style.setProperty('--primary-bg', '#0d1b0d');
        root.style.setProperty('--secondary-bg', '#1a2e1a');
        root.style.setProperty('--tertiary-bg', '#264026');
        root.style.setProperty('--text-primary', '#e8f5e8');
        root.style.setProperty('--text-secondary', '#b8d4b8');
        root.style.setProperty('--border-color', '#3d5a3d');
        root.style.setProperty('--primary-accent', '#4ade80');
        root.style.setProperty('--secondary-accent', '#22c55e');
        break;
      case 'sunset':
        root.style.setProperty('--primary-bg', '#1a0f0a');
        root.style.setProperty('--secondary-bg', '#2d1b15');
        root.style.setProperty('--tertiary-bg', '#402820');
        root.style.setProperty('--text-primary', '#f5e8e0');
        root.style.setProperty('--text-secondary', '#d4b8a8');
        root.style.setProperty('--border-color', '#5a3d2d');
        root.style.setProperty('--primary-accent', '#f97316');
        root.style.setProperty('--secondary-accent', '#ea580c');
        break;
      default: // dark theme
        root.style.setProperty('--primary-bg', '#0a192f');
        root.style.setProperty('--secondary-bg', '#112240');
        root.style.setProperty('--tertiary-bg', '#1d2d50');
        root.style.setProperty('--text-primary', '#ccd6f6');
        root.style.setProperty('--text-secondary', '#8892b0');
        root.style.setProperty('--border-color', '#233554');
        root.style.setProperty('--primary-accent', '#64ffda');
        root.style.setProperty('--secondary-accent', '#4fd1c7');
        break;
    }
  }

  function resetToDefaults() {
    if (confirm('Are you sure you want to reset all settings to defaults?')) {
      currentTheme = 'dark';
      autoSave = true;
      buildNotifications = true;
      errorNotifications = true;
      successNotifications = true;
      maxConcurrentBuilds = 3;
      buildTimeout = 30;
      defaultBuildPath = './builds';
      defaultPackagePath = './packages';
      enableTelemetry = false;
      enableAutoUpdates = true;
      
      changeTheme('dark');
      saveSettings();
    }
  }

  // Auto-save settings when they change
  $: if (typeof window !== 'undefined') {
    saveSettings();
  }
</script>

<div class="settings-container">
  <div class="settings-header">
    <h2>Settings</h2>
    <p>Customize your Build Studio experience</p>
  </div>

  <div class="settings-content">
    <!-- Theme Settings -->
    <section class="settings-section">
      <h3>🎨 Appearance</h3>
      
      <div class="setting-group">
        <label class="setting-label" for="theme-selector">Theme</label>
        <p class="setting-description">Choose your preferred color theme</p>
        
        <div class="theme-grid">
          {#each themes as themeOption}
            <div 
              class="theme-option {currentTheme === themeOption.id ? 'active' : ''}"
              on:click={() => changeTheme(themeOption.id)}
              role="button"
              tabindex="0"
              on:keydown={(e) => e.key === 'Enter' && changeTheme(themeOption.id)}
            >
              <div class="theme-preview theme-{themeOption.id}"></div>
              <div class="theme-info">
                <span class="theme-name">{themeOption.name}</span>
                <span class="theme-desc">{themeOption.description}</span>
              </div>
              {#if currentTheme === themeOption.id}
                <div class="theme-check">✓</div>
              {/if}
            </div>
          {/each}
        </div>
      </div>
    </section>

    <!-- Build Settings -->
    <section class="settings-section">
      <h3>🔧 Build Configuration</h3>
      
      <div class="setting-group">
        <label class="setting-label" for="auto-save-toggle">
          <input type="checkbox" id="auto-save-toggle" bind:checked={autoSave} />
          Auto-save project configurations
        </label>
        <p class="setting-description">Automatically save changes to build configurations</p>
      </div>

      <div class="setting-group">
        <label class="setting-label" for="max-concurrent-builds">Max Concurrent Builds</label>
        <input 
          type="number" 
          id="max-concurrent-builds"
          bind:value={maxConcurrentBuilds} 
          min="1" 
          max="10" 
          class="number-input"
        />
      </div>

      <div class="setting-group">
        <label class="setting-label" for="build-timeout">Build Timeout (minutes)</label>
        <p class="setting-description">Maximum time to wait for a build to complete</p>
        <input 
          type="number" 
          id="build-timeout"
          bind:value={buildTimeout} 
          min="5" 
          max="120" 
          class="number-input"
        />
      </div>

      <div class="setting-group">
        <label class="setting-label" for="default-build-path">Default Build Output Path</label>
        <p class="setting-description">Default directory for build outputs</p>
        <input 
          type="text" 
          id="default-build-path"
          bind:value={defaultBuildPath} 
          class="text-input"
          placeholder="./builds"
        />
      </div>

      <div class="setting-group">
        <label class="setting-label" for="default-package-path">Default Package Output Path</label>
        <p class="setting-description">Default directory for package outputs</p>
        <input 
          type="text" 
          id="default-package-path"
          bind:value={defaultPackagePath} 
          class="text-input"
          placeholder="./packages"
        />
      </div>
    </section>

    <!-- Notification Settings -->
    <section class="settings-section">
      <h3>🔔 Notifications</h3>
      
      <div class="setting-group">
        <label class="setting-label" for="build-notifications">
          <input type="checkbox" id="build-notifications" bind:checked={buildNotifications} />
          Build notifications
        </label>
        <p class="setting-description">Show notifications for build events</p>
      </div>

      <div class="setting-group">
        <label class="setting-label" for="error-notifications">
          <input type="checkbox" id="error-notifications" bind:checked={errorNotifications} />
          Error notifications
        </label>
        <p class="setting-description">Show notifications for build errors</p>
      </div>

      <div class="setting-group">
        <label class="setting-label" for="success-notifications">
          <input type="checkbox" id="success-notifications" bind:checked={successNotifications} />
          Success notifications
        </label>
        <p class="setting-description">Show notifications for successful builds</p>
      </div>
    </section>

    <!-- Privacy & Updates -->
    <section class="settings-section">
      <h3>🔒 Privacy & Updates</h3>
      
      <div class="setting-group">
        <label class="setting-label">
          <input type="checkbox" bind:checked={enableAutoUpdates} />
          Automatic updates
        </label>
        <p class="setting-description">Automatically check for and install updates</p>
      </div>

      <div class="setting-group">
        <label class="setting-label">
          <input type="checkbox" bind:checked={enableTelemetry} />
          Anonymous usage analytics
        </label>
        <p class="setting-description">Help improve Build Studio by sharing anonymous usage data</p>
      </div>
    </section>

    <!-- Actions -->
    <section class="settings-section">
      <h3>⚙️ Actions</h3>
      
      <div class="setting-actions">
        <button class="action-button secondary" on:click={resetToDefaults}>
          Reset to Defaults
        </button>
        <button class="action-button primary" on:click={saveSettings}>
          Save Settings
        </button>
      </div>
    </section>
  </div>
</div>

<style>
  .settings-container {
    padding: 2rem;
    max-width: 900px;
    margin: 0 auto;
    color: var(--text-primary);
  }

  .settings-header {
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--border-color);
  }

  .settings-header h2 {
    font-size: 2rem;
    font-weight: 700;
    margin: 0 0 0.5rem 0;
    color: var(--primary-accent);
  }

  .settings-header p {
    font-size: 1.1rem;
    color: var(--text-secondary);
    margin: 0;
  }

  .settings-content {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .settings-section {
    background-color: var(--secondary-bg);
    border-radius: var(--border-radius);
    padding: 2rem;
    border: 1px solid var(--border-color);
  }

  .settings-section h3 {
    font-size: 1.3rem;
    font-weight: 600;
    margin: 0 0 1.5rem 0;
    color: var(--text-primary);
  }

  .setting-group {
    margin-bottom: 2rem;
  }

  .setting-group:last-child {
    margin-bottom: 0;
  }

  .setting-label {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 1rem;
    font-weight: 500;
    color: var(--text-primary);
    margin-bottom: 0.5rem;
    cursor: pointer;
  }

  .setting-label input[type="checkbox"] {
    width: 18px;
    height: 18px;
    accent-color: var(--primary-accent);
  }

  .setting-description {
    font-size: 0.9rem;
    color: var(--text-secondary);
    margin: 0 0 1rem 0;
    line-height: 1.4;
  }

  .theme-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 1rem;
  }

  .theme-option {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    background-color: var(--tertiary-bg);
    border: 2px solid var(--border-color);
    border-radius: var(--border-radius);
    cursor: pointer;
    transition: var(--transition);
    position: relative;
  }

  .theme-option:hover {
    border-color: var(--primary-accent);
    transform: translateY(-2px);
  }

  .theme-option.active {
    border-color: var(--primary-accent);
    background-color: rgba(100, 255, 218, 0.1);
  }

  .theme-preview {
    width: 40px;
    height: 40px;
    border-radius: 8px;
    border: 2px solid var(--border-color);
    flex-shrink: 0;
  }

  .theme-preview.theme-dark {
    background: linear-gradient(135deg, #0a192f, #64ffda);
  }

  .theme-preview.theme-light {
    background: linear-gradient(135deg, #ffffff, #0066cc);
  }

  .theme-preview.theme-midnight {
    background: linear-gradient(135deg, #0f1419, #4a9eff);
  }

  .theme-preview.theme-forest {
    background: linear-gradient(135deg, #0d1b0d, #4ade80);
  }

  .theme-preview.theme-sunset {
    background: linear-gradient(135deg, #1a0f0a, #f97316);
  }

  .theme-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    flex: 1;
  }

  .theme-name {
    font-weight: 600;
    color: var(--text-primary);
  }

  .theme-desc {
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .theme-check {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    color: var(--primary-accent);
    font-weight: bold;
    font-size: 1.2rem;
  }

  .number-input,
  .text-input {
    width: 100%;
    max-width: 200px;
    padding: 0.75rem;
    background-color: var(--tertiary-bg);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius);
    color: var(--text-primary);
    font-size: 0.9rem;
  }

  .number-input:focus,
  .text-input:focus {
    outline: none;
    border-color: var(--primary-accent);
    box-shadow: 0 0 0 2px rgba(100, 255, 218, 0.2);
  }

  .setting-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
  }

  .action-button {
    padding: 0.75rem 1.5rem;
    border-radius: var(--border-radius);
    font-weight: 500;
    cursor: pointer;
    transition: var(--transition);
    border: none;
    font-size: 0.9rem;
  }

  .action-button.primary {
    background-color: var(--primary-accent);
    color: var(--primary-bg);
  }

  .action-button.primary:hover {
    background-color: var(--secondary-accent);
    transform: translateY(-1px);
  }

  .action-button.secondary {
    background-color: var(--tertiary-bg);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
  }

  .action-button.secondary:hover {
    border-color: var(--primary-accent);
    background-color: rgba(100, 255, 218, 0.1);
  }

  @media (max-width: 768px) {
    .settings-container {
      padding: 1rem;
    }

    .theme-grid {
      grid-template-columns: 1fr;
    }

    .setting-actions {
      flex-direction: column;
    }

    .action-button {
      width: 100%;
    }
  }
</style>
