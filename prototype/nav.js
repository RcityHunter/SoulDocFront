// SoulDoc V5 - Shared Navigation & Utilities
(function() {
  const page = location.pathname.split('/').pop() || 'index.html';
  const sidebar = document.querySelector('.sidebar');

  function findNavSection(predicate) {
    if (!sidebar) return null;
    return Array.from(sidebar.querySelectorAll('.nav-section')).find(predicate) || null;
  }

  function ensureNavItem(section, config) {
    if (!section || section.querySelector(`[href="${config.href}"]`)) return;
    const link = document.createElement('a');
    link.className = 'nav-item';
    link.href = config.href;
    link.innerHTML = `<span class="nav-icon">${config.icon}</span>${config.label}`;
    const before = config.beforeHref ? section.querySelector(`[href="${config.beforeHref}"]`) : null;
    if (before) before.before(link);
    else section.appendChild(link);
  }

  function ensureWorkspaceSwitcher() {
    if (!sidebar || sidebar.querySelector('.workspace-switcher')) return;
    const brand = sidebar.querySelector('.sidebar-brand');
    if (!brand) return;

    const block = document.createElement('div');
    block.className = 'workspace-switcher';
    block.innerHTML = `
      <div class="workspace-switcher-list">
        <a class="workspace-switcher-item" href="profile.html">
          <div>
            <div class="workspace-switcher-item-title">个人工作区</div>
            <div class="workspace-switcher-item-meta">私有草稿、偏好与个人知识沉淀</div>
          </div>
          <span class="scope-chip personal">个人</span>
        </a>
        <a class="workspace-switcher-item" href="organization.html">
          <div>
            <div class="workspace-switcher-item-title">SoulDoc 团队</div>
            <div class="workspace-switcher-item-meta">成员、集合、空间、发布站点与审批流</div>
          </div>
          <span class="scope-chip org">组织</span>
        </a>
      </div>
    `;
    brand.after(block);
  }

  ensureWorkspaceSwitcher();

  const currentSpaceSection = findNavSection(section => section.querySelector('[href="docs.html"]'));
  ensureNavItem(currentSpaceSection, {
    href: 'templates.html',
    icon: '📐',
    label: '模板中心',
    beforeHref: 'editor.html'
  });

  const smartSection = findNavSection(section => section.querySelector('[href="language.html"]'));
  ensureNavItem(smartSection, {
    href: 'ai-tools.html',
    icon: '🧩',
    label: 'AI 工具配置',
    beforeHref: 'language.html'
  });

  const platformSection = findNavSection(section => section.querySelector('[href="developer.html"]'));
  ensureNavItem(platformSection, {
    href: 'workspace.html',
    icon: '🏛️',
    label: '发布站点',
    beforeHref: 'developer.html'
  });
  ensureNavItem(platformSection, {
    href: 'git-sync.html',
    icon: '🔀',
    label: 'GitHub 同步',
    beforeHref: 'developer.html'
  });

  // Set active nav after entries are normalized.
  document.querySelectorAll('.nav-item').forEach(el => {
    const href = el.getAttribute('href');
    if (href === page) el.classList.add('active');
  });

  // Toast utility
  window.showToast = function(msg, type = 'default', duration = 3000) {
    let container = document.querySelector('.toast-container');
    if (!container) {
      container = document.createElement('div');
      container.className = 'toast-container';
      document.body.appendChild(container);
    }
    const toast = document.createElement('div');
    toast.className = 'toast ' + type;
    toast.innerHTML = `<span>${msg}</span>`;
    container.appendChild(toast);
    setTimeout(() => toast.remove(), duration);
  };

  // Modal utility
  window.openModal = function(id) {
    const el = document.getElementById(id);
    if (el) { el.classList.add('open'); document.body.style.overflow = 'hidden'; }
  };
  window.closeModal = function(id) {
    const el = document.getElementById(id);
    if (el) { el.classList.remove('open'); document.body.style.overflow = ''; }
  };

  // Drawer utility
  window.openDrawer = function(id) {
    const el = document.getElementById(id);
    if (el) { el.classList.add('open'); }
  };
  window.closeDrawer = function(id) {
    const el = document.getElementById(id);
    if (el) { el.classList.remove('open'); }
  };

  // Toggle utility
  document.querySelectorAll('.toggle').forEach(el => {
    el.addEventListener('click', () => el.classList.toggle('on'));
  });

  // Tab utility
  document.querySelectorAll('.tabs').forEach(tabGroup => {
    tabGroup.querySelectorAll('.tab').forEach(tab => {
      tab.addEventListener('click', () => {
        const target = tab.dataset.tab;
        tabGroup.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
        tab.classList.add('active');
        if (target) {
          document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
          const content = document.getElementById(target);
          if (content) content.classList.add('active');
        }
      });
    });
  });

  // Close modals on overlay click
  document.querySelectorAll('.modal-overlay').forEach(overlay => {
    overlay.addEventListener('click', e => {
      if (e.target === overlay) {
        overlay.classList.remove('open');
        document.body.style.overflow = '';
      }
    });
  });

  // Close drawers on overlay click
  document.querySelectorAll('.drawer-overlay').forEach(overlay => {
    overlay.addEventListener('click', e => {
      if (e.target === overlay) overlay.classList.remove('open');
    });
  });

  // Prevent placeholder anchors from jumping to top in prototypes.
  document.querySelectorAll('a[href="#"]').forEach(link => {
    link.addEventListener('click', e => e.preventDefault());
  });
})();
