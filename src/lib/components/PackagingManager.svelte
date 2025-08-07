
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { buildConfig } from '../stores/buildconfig';
  import type { BuildStudioConfig } from '../stores/buildconfig';
  import { get } from 'svelte/store';

  let packageName = 'my-app';
  let packageVersion = '1.0.0';
  let packageType = 'deb';
  let packageDependencies = 'libssl,curl';
  let sourceDir = '';
  let outputDir = './packages';
  let packageResult = '';
  let packageError = '';

  async function createPackage() {
    packageResult = '';
    packageError = '';
    
    try {
      // Get the current build config to determine source directory
      const config: BuildStudioConfig | null = get(buildConfig);
      const sourcePath = sourceDir || (config && config.builds.length > 0 ? config.builds[0].command : './build');
      
      const result = await invoke('create_package', {
        config: {
          name: packageName,
          version: packageVersion,
          package_type: packageType,
          dependencies: packageDependencies.split(',').map(dep => dep.trim()).filter(dep => dep),
          source_dir: sourcePath,
          output_dir: outputDir,
        },
      });
      packageResult = result as string;
    } catch (e) {
      packageError = String(e);
    }
  }
</script>

<div class="packaging-manager">
  <h3>Packaging</h3>
  
  <div class="form-grid">
    <div class="form-group">
      <label for="packageName">Package Name:</label>
      <input id="packageName" type="text" bind:value={packageName} placeholder="my-application" />
    </div>
    
    <div class="form-group">
      <label for="packageVersion">Version:</label>
      <input id="packageVersion" type="text" bind:value={packageVersion} placeholder="1.0.0" />
    </div>
    
    <div class="form-group">
      <label for="packageType">Package Type:</label>
      <select id="packageType" bind:value={packageType}>
        <option value="deb">DEB (Debian/Ubuntu)</option>
        <option value="rpm">RPM (Red Hat/Fedora)</option>
        <option value="appimage">AppImage (Linux)</option>
        <option value="msi">MSI (Windows)</option>
        <option value="dmg">DMG (macOS)</option>
        <option value="apk">APK (Android)</option>
        <option value="ipa">IPA (iOS)</option>
      </select>
    </div>
    
    <div class="form-group">
      <label for="sourceDir">Source Directory:</label>
      <input id="sourceDir" type="text" bind:value={sourceDir} placeholder="./build or from config" />
    </div>
    
    <div class="form-group">
      <label for="outputDir">Output Directory:</label>
      <input id="outputDir" type="text" bind:value={outputDir} placeholder="./packages" />
    </div>
    
    <div class="form-group">
      <label for="packageDependencies">Dependencies (comma separated):</label>
      <input id="packageDependencies" type="text" bind:value={packageDependencies} placeholder="libssl, curl, python3" />
    </div>
  </div>
  
  <div class="button-group">
    <button on:click={createPackage}>Create Package</button>
    <button class="secondary">Load from Config</button>
  </div>
  
  {#if packageResult}
    <div class="success-message">{packageResult}</div>
  {/if}
  
  {#if packageError}
    <div class="error-message">{packageError}</div>
  {/if}
</div>

<style>
  .packaging-manager {
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

  .form-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  
  .form-group label {
    display: block;
    font-weight: 500;
    color: var(--text-primary);
    font-size: 0.95rem;
  }
  
  .form-group input, .form-group select {
    width: 100%;
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
  
  .form-group input:focus, .form-group select:focus {
    border-color: var(--primary-accent);
    box-shadow: 0 0 0 2px rgba(100, 255, 218, 0.2);
  }
  
  .form-group input::placeholder {
    color: var(--text-secondary);
  }
  
  .button-group {
    display: flex;
    gap: 1rem;
    margin-top: 1rem;
  }
  
  .success-message {
    color: var(--success-color);
    margin-top: 1rem;
    padding: 1rem;
    background-color: rgba(100, 255, 218, 0.1);
    border-radius: var(--border-radius);
    border: 1px solid var(--success-color);
  }
  
  .error-message {
    color: var(--error-color);
    margin-top: 1rem;
    padding: 1rem;
    background-color: rgba(255, 85, 85, 0.1);
    border-radius: var(--border-radius);
    border: 1px solid var(--error-color);
  }
</style>
