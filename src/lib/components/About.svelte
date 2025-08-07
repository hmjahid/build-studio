<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let appVersion = '1.0.0';
  let buildDate = new Date().toLocaleDateString();

  onMount(async () => {
    try {
      // Get app version from Tauri
      const version = await invoke('get_app_version');
      if (version) appVersion = version;
    } catch (error) {
      console.log('Could not get app version:', error);
    }
  });
</script>

<div class="about-container">
  <div class="about-header">
    <div class="app-logo">
      <div class="logo-icon">🏗️</div>
      <h1>Build Studio</h1>
      <p class="version">Version {appVersion}</p>
    </div>
  </div>

  <div class="about-content">
    <section class="app-info">
      <h2>About Build Studio</h2>
      <p class="description">
        Build Studio is an advanced, extensible desktop and CLI application for configuring, 
        managing, and executing multi-platform software builds from a unified interface. 
        Built with modern technologies including Tauri, SvelteKit, and Rust.
      </p>
      
      <div class="features-grid">
        <div class="feature-item">
          <div class="feature-icon">🚀</div>
          <h3>Multi-Platform Builds</h3>
          <p>Support for Linux, Windows, macOS, Android, iOS, and WebAssembly</p>
        </div>
        
        <div class="feature-item">
          <div class="feature-icon">🐳</div>
          <h3>Local & Remote Nodes</h3>
          <p>Docker containers, VMs, and SSH remote build environments</p>
        </div>
        
        <div class="feature-item">
          <div class="feature-icon">🔧</div>
          <h3>Multi-Language Support</h3>
          <p>C/C++, Python, Java, Rust, Go, Node.js, and more</p>
        </div>
        
        <div class="feature-item">
          <div class="feature-icon">🔌</div>
          <h3>Plugin System</h3>
          <p>Extensible architecture with Python and JavaScript plugins</p>
        </div>
        
        <div class="feature-item">
          <div class="feature-icon">📦</div>
          <h3>Package Management</h3>
          <p>Create packages for multiple platforms and distributions</p>
        </div>
        
        <div class="feature-item">
          <div class="feature-icon">🔄</div>
          <h3>CI/CD Integration</h3>
          <p>GitHub, GitLab webhooks and automated build triggers</p>
        </div>
      </div>
    </section>

    <section class="developer-info">
      <h2>Developer</h2>
      <div class="developer-card">
        <div class="developer-avatar">
          <div class="avatar-placeholder">MJH</div>
        </div>
        <div class="developer-details">
          <h3>Md Jahid Hasan</h3>
          <p class="developer-title">Full Stack Developer & Software Engineer</p>
          
          <div class="contact-links">
            <a href="mailto:mdjahidhasan919@gmail.com" class="contact-link email">
              <span class="contact-icon">📧</span>
              mdjahidhasan919@gmail.com
            </a>
            
            <a href="https://github.com/hmjahid/" target="_blank" class="contact-link github">
              <span class="contact-icon">🐙</span>
              GitHub: hmjahid
            </a>
            
            <a href="https://www.linkedin.com/in/hmjahid/" target="_blank" class="contact-link linkedin">
              <span class="contact-icon">💼</span>
              LinkedIn: hmjahid
            </a>
            
            <a href="https://hmjahid.netlify.app/" target="_blank" class="contact-link website">
              <span class="contact-icon">🌐</span>
              hmjahid.netlify.app
            </a>
          </div>
        </div>
      </div>
    </section>

    <section class="tech-info">
      <h2>Technology Stack</h2>
      <div class="tech-grid">
        <div class="tech-item">
          <span class="tech-name">Frontend</span>
          <span class="tech-value">SvelteKit + TypeScript</span>
        </div>
        <div class="tech-item">
          <span class="tech-name">Backend</span>
          <span class="tech-value">Rust + Tauri</span>
        </div>
        <div class="tech-item">
          <span class="tech-name">Build Date</span>
          <span class="tech-value">{buildDate}</span>
        </div>
        <div class="tech-item">
          <span class="tech-name">License</span>
          <span class="tech-value">MIT License</span>
        </div>
      </div>
    </section>

    <section class="acknowledgments">
      <h2>Acknowledgments</h2>
      <p>
        Built with ❤️ using open-source technologies. Special thanks to the Tauri, 
        Svelte, and Rust communities for their amazing tools and documentation.
      </p>
    </section>
  </div>
</div>

<style>
  .about-container {
    padding: 2rem;
    max-width: 1000px;
    margin: 0 auto;
    color: var(--text-primary);
  }

  .about-header {
    text-align: center;
    margin-bottom: 3rem;
    padding-bottom: 2rem;
    border-bottom: 1px solid var(--border-color);
  }

  .app-logo {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  .logo-icon {
    font-size: 4rem;
    background: linear-gradient(135deg, var(--primary-accent), var(--secondary-accent));
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .app-logo h1 {
    font-size: 2.5rem;
    font-weight: 700;
    margin: 0;
    background: linear-gradient(135deg, var(--primary-accent), var(--secondary-accent));
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .version {
    font-size: 1.1rem;
    color: var(--text-secondary);
    margin: 0;
  }

  .about-content {
    display: flex;
    flex-direction: column;
    gap: 3rem;
  }

  section {
    background-color: var(--secondary-bg);
    border-radius: var(--border-radius);
    padding: 2rem;
    border: 1px solid var(--border-color);
  }

  section h2 {
    font-size: 1.5rem;
    font-weight: 600;
    margin: 0 0 1.5rem 0;
    color: var(--primary-accent);
  }

  .description {
    font-size: 1.1rem;
    line-height: 1.6;
    color: var(--text-secondary);
    margin-bottom: 2rem;
  }

  .features-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
  }

  .feature-item {
    background-color: var(--tertiary-bg);
    padding: 1.5rem;
    border-radius: var(--border-radius);
    border: 1px solid var(--border-color);
    transition: var(--transition);
  }

  .feature-item:hover {
    border-color: var(--primary-accent);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(100, 255, 218, 0.1);
  }

  .feature-icon {
    font-size: 2rem;
    margin-bottom: 1rem;
  }

  .feature-item h3 {
    font-size: 1.2rem;
    font-weight: 600;
    margin: 0 0 0.5rem 0;
    color: var(--text-primary);
  }

  .feature-item p {
    font-size: 0.9rem;
    color: var(--text-secondary);
    margin: 0;
    line-height: 1.4;
  }

  .developer-card {
    display: flex;
    gap: 2rem;
    align-items: flex-start;
  }

  .developer-avatar {
    flex-shrink: 0;
  }

  .avatar-placeholder {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    background: linear-gradient(135deg, var(--primary-accent), var(--secondary-accent));
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--primary-bg);
  }

  .developer-details h3 {
    font-size: 1.4rem;
    font-weight: 600;
    margin: 0 0 0.5rem 0;
    color: var(--text-primary);
  }

  .developer-title {
    font-size: 1rem;
    color: var(--text-secondary);
    margin: 0 0 1.5rem 0;
  }

  .contact-links {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .contact-link {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 1rem;
    background-color: var(--tertiary-bg);
    border-radius: var(--border-radius);
    text-decoration: none;
    color: var(--text-primary);
    border: 1px solid var(--border-color);
    transition: var(--transition);
    font-size: 0.9rem;
  }

  .contact-link:hover {
    border-color: var(--primary-accent);
    background-color: rgba(100, 255, 218, 0.1);
    transform: translateX(4px);
  }

  .contact-icon {
    font-size: 1.1rem;
  }

  .tech-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
  }

  .tech-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background-color: var(--tertiary-bg);
    border-radius: var(--border-radius);
    border: 1px solid var(--border-color);
  }

  .tech-name {
    font-weight: 600;
    color: var(--text-primary);
  }

  .tech-value {
    color: var(--text-secondary);
    font-family: 'Fira Code', monospace;
    font-size: 0.9rem;
  }

  .acknowledgments p {
    font-size: 1rem;
    line-height: 1.6;
    color: var(--text-secondary);
    margin: 0;
    text-align: center;
  }

  @media (max-width: 768px) {
    .about-container {
      padding: 1rem;
    }

    .developer-card {
      flex-direction: column;
      text-align: center;
    }

    .features-grid {
      grid-template-columns: 1fr;
    }

    .tech-grid {
      grid-template-columns: 1fr;
    }

    .contact-links {
      align-items: center;
    }
  }
</style>
