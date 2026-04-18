(function () {
  const STORAGE_KEY = 'souldoc.api.console.v1';

  const ENDPOINT_GROUPS = [
    {
      key: 'system',
      title: '系统与安装',
      endpoints: [
        'GET /health',
        'GET /install/status',
        'POST /install/bootstrap',
        'GET /system/config',
        'PUT /system/config',
        'GET /system/runtime',
        'GET /system/version'
      ]
    },
    {
      key: 'auth',
      title: '认证与用户',
      endpoints: [
        'POST /auth/login',
        'POST /auth/refresh',
        'POST /auth/logout',
        'GET /auth/me',
        'PUT /auth/me',
        'PUT /auth/password',
        'GET /auth/oauth/:provider/url',
        'POST /auth/oauth/:provider/callback',
        'GET /users',
        'POST /users',
        'GET /users/:id',
        'PUT /users/:id',
        'GET /users/search',
        'GET /users?type=human',
        'GET /users?type=ai',
        'GET /users/me/preferences',
        'PUT /users/me/preferences',
        'POST /users/:id/service-credentials',
        'GET /users/:id/service-credentials',
        'DELETE /service-credentials/:credentialId'
      ]
    },
    {
      key: 'scope',
      title: '工作区 / 组织 / 集合 / 空间',
      endpoints: [
        'GET /me/workspaces',
        'GET /personal-workspace',
        'PUT /personal-workspace/preferences',
        'GET /organizations',
        'POST /organizations',
        'GET /organizations/:orgId',
        'PUT /organizations/:orgId',
        'GET /organizations/:orgId/members',
        'POST /organizations/:orgId/members/invite',
        'PUT /organizations/:orgId/members/:memberId',
        'DELETE /organizations/:orgId/members/:memberId',
        'GET /sites',
        'POST /sites',
        'GET /sites/:siteId',
        'PUT /sites/:siteId',
        'DELETE /sites/:siteId',
        'GET /sites/:siteId/navigation',
        'PUT /sites/:siteId/navigation',
        'GET /sites/:siteId/sections',
        'POST /sites/:siteId/sections',
        'PUT /sites/:siteId/sections/:sectionId',
        'DELETE /sites/:siteId/sections/:sectionId',
        'GET /sites/:siteId/spaces',
        'POST /sites/:siteId/spaces/bind',
        'DELETE /sites/:siteId/spaces/:bindingId',
        'GET /collections',
        'POST /collections',
        'GET /collections/:collectionId',
        'PUT /collections/:collectionId',
        'DELETE /collections/:collectionId',
        'POST /collections/:collectionId/spaces/:spaceId/bind',
        'DELETE /collections/:collectionId/spaces/:spaceId/unbind',
        'GET /spaces',
        'POST /spaces',
        'GET /spaces/:spaceId',
        'PUT /spaces/:spaceId',
        'DELETE /spaces/:spaceId',
        'GET /spaces/:spaceId/overview',
        'GET /spaces/:spaceId/settings',
        'PUT /spaces/:spaceId/settings'
      ]
    },
    {
      key: 'members',
      title: '成员 / 角色 / 权限',
      endpoints: [
        'GET /organizations/:orgId/members',
        'POST /organizations/:orgId/members/invite',
        'PUT /organizations/:orgId/members/:memberId',
        'DELETE /organizations/:orgId/members/:memberId',
        'GET /spaces/:spaceId/members',
        'POST /spaces/:spaceId/members/invite',
        'PUT /spaces/:spaceId/members/:memberId',
        'DELETE /spaces/:spaceId/members/:memberId',
        'GET /spaces/:spaceId/invitations',
        'POST /invitations/:token/accept',
        'POST /invitations/:token/reject',
        'GET /spaces/:spaceId/roles',
        'POST /spaces/:spaceId/roles',
        'PUT /spaces/:spaceId/roles/:roleId',
        'DELETE /spaces/:spaceId/roles/:roleId',
        'GET /docs/:docId/permissions',
        'PUT /docs/:docId/permissions'
      ]
    },
    {
      key: 'docs',
      title: '文档 / 模板 / 多语言',
      endpoints: [
        'GET /spaces/:spaceId/docs/tree',
        'GET /spaces/:spaceId/docs',
        'POST /spaces/:spaceId/docs',
        'GET /docs/:docId',
        'PUT /docs/:docId',
        'PUT /docs/:docId/content',
        'POST /docs/:docId/copy',
        'GET /templates',
        'POST /templates',
        'GET /templates/:templateId',
        'PUT /templates/:templateId',
        'POST /templates/:templateId/apply',
        'POST /templates/:templateId/upgrade-preview',
        'POST /templates/:templateId/publish',
        'POST /docs/:docId/move',
        'DELETE /docs/:docId',
        'POST /docs/:docId/restore',
        'GET /doc-groups/:docGroupId',
        'GET /doc-groups/:docGroupId/locales',
        'POST /doc-groups/:docGroupId/locales',
        'GET /docs/:docId/localization-status',
        'PUT /docs/:docId/locale-meta',
        'POST /docs/:docId/locales/:locale/sync-from-source',
        'POST /docs/:docId/locales/:locale/request-review',
        'POST /docs/:docId/locales/:locale/publish'
      ]
    },
    {
      key: 'governance',
      title: '块 / 版本 / 变更请求',
      endpoints: [
        'GET /docs/:docId/blocks',
        'PUT /docs/:docId/blocks',
        'GET /blocks/:blockId',
        'PUT /blocks/:blockId',
        'DELETE /blocks/:blockId',
        'GET /docs/:docId/versions',
        'GET /versions/:versionId',
        'GET /versions/compare',
        'POST /docs/:docId/save-version',
        'POST /versions/:versionId/restore',
        'GET /docs/:docId/change-requests',
        'POST /docs/:docId/change-requests',
        'GET /change-requests/:requestId',
        'POST /change-requests/:requestId/comment',
        'POST /change-requests/:requestId/request-changes',
        'POST /change-requests/:requestId/approve',
        'POST /change-requests/:requestId/merge'
      ]
    },
    {
      key: 'resources',
      title: '标签 / 文件 / 评论 / 通知',
      endpoints: [
        'GET /spaces/:spaceId/tags',
        'POST /spaces/:spaceId/tags',
        'PUT /tags/:tagId',
        'DELETE /tags/:tagId',
        'POST /docs/:docId/tags',
        'DELETE /docs/:docId/tags/:tagId',
        'GET /spaces/:spaceId/files',
        'POST /spaces/:spaceId/files/upload',
        'GET /files/:fileId',
        'DELETE /files/:fileId',
        'POST /docs/:docId/files/:fileId/bind',
        'DELETE /docs/:docId/files/:fileId/unbind',
        'GET /docs/:docId/comments',
        'POST /docs/:docId/comments',
        'PUT /comments/:commentId',
        'DELETE /comments/:commentId',
        'POST /comments/:commentId/resolve',
        'GET /notifications',
        'POST /notifications/mark-read',
        'POST /notifications/mark-all-read',
        'GET /notifications/preferences',
        'PUT /notifications/preferences'
      ]
    },
    {
      key: 'ai',
      title: '搜索 / AI / 能力清单',
      endpoints: [
        'GET /search',
        'GET /search/suggest',
        'POST /search/semantic',
        'GET /search/hot',
        'GET /search/related?docId=...',
        'POST /ai/docs/:docId/summary',
        'POST /ai/docs/:docId/outline',
        'POST /ai/docs/:docId/tags',
        'POST /ai/docs/:docId/faq',
        'POST /ai/docs/:docId/rewrite',
        'POST /ai/docs/:docId/translate',
        'POST /ai/docs/:docId/review',
        'POST /ai/docs/:docId/seo-check',
        'POST /ai/docs/:docId/publish-readiness',
        'GET /ai/tasks',
        'GET /ai/tasks/:taskId',
        'POST /ai/tasks/:taskId/cancel',
        'GET /ai/tool-families',
        'GET /.well-known/capabilities.json',
        'GET /.well-known/skill-manifest.json',
        'POST /connectors/knowledge/query',
        'GET /connectors/docs/:docId/context',
        'POST /connectors/docs/:docId/suggestions'
      ]
    },
    {
      key: 'publish',
      title: 'SEO / 发布 / 公开阅读',
      endpoints: [
        'GET /docs/:docId/seo',
        'PUT /docs/:docId/seo',
        'GET /docs/:docId/seo/preview',
        'POST /seo/sitemap/generate',
        'GET /spaces/:spaceId/publish-targets',
        'POST /spaces/:spaceId/publish-targets',
        'PUT /publish-targets/:targetId',
        'DELETE /publish-targets/:targetId',
        'POST /publish-targets/:targetId/release',
        'GET /publish-targets/:targetId/releases',
        'GET /releases/:releaseId',
        'POST /releases/:releaseId/rollback',
        'GET /public/sites/:siteSlug',
        'GET /public/collections/:collectionSlug',
        'GET /public/spaces/:spaceSlug',
        'GET /public/docs/:slug',
        'GET /public/spaces/:spaceSlug/tags/:tagSlug',
        'GET /public/sitemap.xml',
        'GET /public/robots.txt'
      ]
    },
    {
      key: 'platform',
      title: 'GitHub 同步 / Webhook',
      endpoints: [
        'GET /spaces/:spaceId/git-sync',
        'POST /spaces/:spaceId/git-sync/connect',
        'POST /spaces/:spaceId/git-sync/sync',
        'GET /spaces/:spaceId/git-sync/previews',
        'GET /git-sync/previews/:previewId',
        'GET /git-sync/conflicts/:conflictId',
        'POST /git-sync/conflicts/:conflictId/resolve',
        'GET /webhooks',
        'POST /webhooks',
        'PUT /webhooks/:id',
        'DELETE /webhooks/:id',
        'POST /webhooks/:id/test',
        'GET /webhooks/:id/deliveries'
      ]
    }
  ];

  const endpointIndex = ENDPOINT_GROUPS.flatMap(function (group) {
    return group.endpoints.map(function (spec) {
      const separator = spec.indexOf(' ');
      const method = spec.slice(0, separator).trim();
      const path = spec.slice(separator + 1).trim();
      return {
        id: method + ' ' + path,
        groupKey: group.key,
        groupTitle: group.title,
        method: method,
        path: path,
        usePrefix: defaultUsePrefix(path)
      };
    });
  });

  function defaultUsePrefix(path) {
    return path !== '/health' && !path.startsWith('/public/') && !path.startsWith('/.well-known/');
  }

  function defaultQueryFor(path) {
    if (path === '/search') return '{\n  "q": "AI"\n}';
    if (path === '/versions/compare') return '{\n  "from": "v23",\n  "to": "v24"\n}';
    if (path === '/search/related?docId=...') return '{\n  "docId": "doc_ai_tools"\n}';
    return '';
  }

  function defaultBodyFor(method, path) {
    if (!['POST', 'PUT', 'PATCH', 'DELETE'].includes(method)) return '';

    const examples = {
      '/install/bootstrap': { site_name: 'SoulDoc', admin_email: 'admin@souldoc.io', admin_password: 'password123' },
      '/auth/login': { email: 'admin@souldoc.io', password: 'password123', remember_me: true },
      '/users': { display_name: 'Site Operator', email: 'agent@souldoc.io', user_type: 'ai', role: 'Editor' },
      '/users/:id/service-credentials': { label: 'Skill Credential' },
      '/organizations': { name: 'SoulDoc 团队', slug: 'souldoc' },
      '/sites': { organization_id: 'org_souldoc', name: 'docs.souldoc.io', slug: 'docs', domain: 'docs.souldoc.io' },
      '/sites/:siteId/sections': { name: '产品文档' },
      '/sites/:siteId/spaces/bind': { section_id: 'section_product', space_id: 'space_demo' },
      '/collections': { name: '平台文档', scope_id: 'org_souldoc' },
      '/spaces': { name: 'SoulDoc Demo', owner_scope_type: 'organization', owner_scope_id: 'org_souldoc', visibility: 'public' },
      '/spaces/:spaceId/settings': { visibility: 'public', default_locale: 'zh-CN' },
      '/spaces/:spaceId/docs': { title: '新文档', locale: 'zh-CN', parent_id: null },
      '/docs/:docId': { title: 'AI 工具配置' },
      '/docs/:docId/content': { content: '# 标题\\n\\n更新后的内容' },
      '/docs/:docId/copy': { target_space_id: 'space_demo' },
      '/templates': { name: 'PRD 模板', source_doc_id: 'doc_prd' },
      '/templates/:templateId/apply': { space_id: 'space_demo', title: '从模板创建文档' },
      '/templates/:templateId/upgrade-preview': { doc_id: 'doc_ai_tools' },
      '/templates/:templateId/publish': { status: 'published' },
      '/docs/:docId/move': { parent_id: 'doc_parent', sort_order: 10 },
      '/docs/:docId/restore': { restore_note: '恢复误删文档' },
      '/doc-groups/:docGroupId/locales': { locale: 'en-US' },
      '/docs/:docId/locale-meta': { default_locale: 'zh-CN', fallback_locale: 'en-US' },
      '/docs/:docId/locales/:locale/sync-from-source': { source_locale: 'zh-CN' },
      '/docs/:docId/locales/:locale/request-review': { reviewer_id: 'user_admin' },
      '/docs/:docId/locales/:locale/publish': { publish_target_id: 'target_docs_prod' },
      '/docs/:docId/blocks': { blocks: [] },
      '/blocks/:blockId': { content: '更新后的块内容' },
      '/docs/:docId/save-version': { message: '保存版本说明' },
      '/versions/:versionId/restore': { reason: '回滚到稳定版本' },
      '/docs/:docId/change-requests': { title: '更新 Skill 接入说明', reviewer_id: 'user_admin' },
      '/change-requests/:requestId/comment': { comment: '请补充站点治理流程' },
      '/change-requests/:requestId/request-changes': { reason: '需要补充风险说明' },
      '/change-requests/:requestId/approve': { note: '审核通过' },
      '/change-requests/:requestId/merge': { note: '合并到主分支' },
      '/spaces/:spaceId/tags': { name: 'AI', color: '#4f46e5' },
      '/tags/:tagId': { name: 'AI 治理', color: '#2563eb' },
      '/docs/:docId/tags': { tag_ids: ['tag_ai'] },
      '/spaces/:spaceId/files/upload': { file_name: 'logo.png' },
      '/docs/:docId/files/:fileId/bind': { position: 'body' },
      '/docs/:docId/comments': { content: '请补充 skill 注册流程' },
      '/comments/:commentId': { content: '已更新，请复查' },
      '/comments/:commentId/resolve': { resolved: true },
      '/notifications/mark-read': { ids: ['notif_1'] },
      '/notifications/mark-all-read': { scope: 'current_user' },
      '/notifications/preferences': { email: true, in_app: true },
      '/search/semantic': { query: 'AI tool families', top_k: 5 },
      '/ai/docs/:docId/summary': { locale: 'zh-CN' },
      '/ai/docs/:docId/outline': { locale: 'zh-CN' },
      '/ai/docs/:docId/tags': { locale: 'zh-CN' },
      '/ai/docs/:docId/faq': { locale: 'zh-CN' },
      '/ai/docs/:docId/rewrite': { tone: 'technical' },
      '/ai/docs/:docId/translate': { target_locale: 'en-US' },
      '/ai/docs/:docId/review': { rule_set: 'publishing' },
      '/ai/docs/:docId/seo-check': { target_site_id: 'site_docs' },
      '/ai/docs/:docId/publish-readiness': { publish_target_id: 'target_docs_prod' },
      '/ai/tasks/:taskId/cancel': { reason: 'manual_cancel' },
      '/connectors/knowledge/query': { query: 'skill 注册账号', scope_id: 'org_souldoc' },
      '/connectors/docs/:docId/suggestions': { objective: 'site_management' },
      '/docs/:docId/seo': { title: 'AI 工具配置', description: 'Skill 与站点治理方案' },
      '/seo/sitemap/generate': { site_id: 'site_docs' },
      '/spaces/:spaceId/publish-targets': { name: '生产站点', channel: 'public-site' },
      '/publish-targets/:targetId': { name: '生产站点', status: 'active' },
      '/publish-targets/:targetId/release': { version_id: 'v24', preview: true },
      '/releases/:releaseId/rollback': { reason: '验证失败，回滚' },
      '/spaces/:spaceId/git-sync/connect': { repository_url: 'https://github.com/example/docs', branch: 'main' },
      '/spaces/:spaceId/git-sync/sync': { direction: 'push' },
      '/git-sync/conflicts/:conflictId/resolve': { strategy: 'manual' },
      '/webhooks': { name: 'Publish Hook', event: 'document.published', target_url: 'https://example.com/webhook' },
      '/webhooks/:id': { enabled: true },
      '/webhooks/:id/test': { payload_preview: true }
    };

    const example = examples[path];
    if (example) {
      return JSON.stringify(example, null, 2);
    }

    return '{\n\n}';
  }

  function byId(id) {
    return document.getElementById(id);
  }

  function currentConfig() {
    return {
      baseUrl: byId('api-base-url') ? byId('api-base-url').value.trim() : '',
      prefix: byId('api-prefix') ? byId('api-prefix').value.trim() : '/api/v1',
      token: byId('api-bearer-token') ? byId('api-bearer-token').value.trim() : '',
      extraHeaders: byId('api-extra-headers') ? byId('api-extra-headers').value.trim() : ''
    };
  }

  function saveApiConsoleConfig() {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(currentConfig()));
    if (window.showToast) window.showToast('接口配置已保存', 'success');
  }

  function loadConfig() {
    let saved = null;
    try {
      saved = JSON.parse(localStorage.getItem(STORAGE_KEY) || 'null');
    } catch (_) {
      saved = null;
    }

    const config = Object.assign(
      {
        baseUrl: window.location.origin,
        prefix: '/api/v1',
        token: '',
        extraHeaders: ''
      },
      saved || {}
    );

    if (byId('api-base-url')) byId('api-base-url').value = config.baseUrl || window.location.origin;
    if (byId('api-prefix')) byId('api-prefix').value = config.prefix || '/api/v1';
    if (byId('api-bearer-token')) byId('api-bearer-token').value = config.token || '';
    if (byId('api-extra-headers')) byId('api-extra-headers').value = config.extraHeaders || '';
  }

  function normalizeOrigin(origin) {
    return (origin || window.location.origin).replace(/\/+$/, '');
  }

  function normalizePrefix(prefix) {
    const clean = (prefix || '').trim();
    if (!clean) return '';
    return clean.startsWith('/') ? clean.replace(/\/+$/, '') : '/' + clean.replace(/\/+$/, '');
  }

  function ensurePath(path) {
    if (/^https?:\/\//i.test(path)) return path;
    return path.startsWith('/') ? path : '/' + path;
  }

  function buildQueryString(raw) {
    const value = (raw || '').trim();
    if (!value) return '';
    if (value.startsWith('?')) return value;
    if (value.startsWith('{')) {
      const json = JSON.parse(value);
      const params = new URLSearchParams();
      Object.keys(json).forEach(function (key) {
        const item = json[key];
        if (Array.isArray(item)) {
          item.forEach(function (entry) {
            params.append(key, String(entry));
          });
        } else if (item !== undefined && item !== null && item !== '') {
          params.append(key, String(item));
        }
      });
      const built = params.toString();
      return built ? '?' + built : '';
    }
    return value.startsWith('&') ? '?' + value.slice(1) : '?' + value;
  }

  function resolveUrl() {
    const origin = normalizeOrigin(byId('api-base-url').value);
    const prefix = normalizePrefix(byId('api-prefix').value);
    const path = byId('api-request-path').value.trim();
    const usePrefix = !!byId('api-use-prefix').checked;

    if (!path) return origin;
    if (/^https?:\/\//i.test(path)) return path + buildQueryString(byId('api-request-query').value);

    const finalPath = ensurePath(path);
    const prefixPart = usePrefix ? prefix : '';
    return origin + prefixPart + finalPath + buildQueryString(byId('api-request-query').value);
  }

  function parseExtraHeaders(raw) {
    const value = (raw || '').trim();
    if (!value) return {};
    return JSON.parse(value);
  }

  function selectedEndpoint() {
    const id = byId('api-endpoint-select').value;
    return endpointIndex.find(function (item) { return item.id === id; }) || null;
  }

  function setSelectedEndpoint(endpoint) {
    if (!endpoint) return;
    byId('api-request-method').value = endpoint.method;
    byId('api-request-path').value = endpoint.path;
    byId('api-use-prefix').checked = endpoint.usePrefix;
    byId('api-request-query').value = defaultQueryFor(endpoint.path);
    byId('api-request-body').value = defaultBodyFor(endpoint.method, endpoint.path);
    byId('api-selected-badge').textContent = endpoint.groupTitle + ' · ' + endpoint.method;
  }

  function renderGroupSelect() {
    const groupSelect = byId('api-endpoint-group');
    groupSelect.innerHTML = '';

    const allOption = document.createElement('option');
    allOption.value = 'all';
    allOption.textContent = '全部分组';
    groupSelect.appendChild(allOption);

    ENDPOINT_GROUPS.forEach(function (group) {
      const option = document.createElement('option');
      option.value = group.key;
      option.textContent = group.title + ' (' + group.endpoints.length + ')';
      groupSelect.appendChild(option);
    });
  }

  function filteredEndpoints() {
    const group = byId('api-endpoint-group').value;
    const keyword = byId('api-endpoint-search').value.trim().toLowerCase();

    return endpointIndex.filter(function (endpoint) {
      const groupMatch = group === 'all' || endpoint.groupKey === group;
      const keywordMatch = !keyword ||
        endpoint.id.toLowerCase().includes(keyword) ||
        endpoint.groupTitle.toLowerCase().includes(keyword);
      return groupMatch && keywordMatch;
    });
  }

  function renderEndpointSelect() {
    const select = byId('api-endpoint-select');
    const endpoints = filteredEndpoints();
    select.innerHTML = '';

    endpoints.forEach(function (endpoint) {
      const option = document.createElement('option');
      option.value = endpoint.id;
      option.textContent = endpoint.id;
      select.appendChild(option);
    });

    if (endpoints.length) {
      select.value = endpoints[0].id;
      setSelectedEndpoint(endpoints[0]);
    } else {
      byId('api-selected-badge').textContent = '未匹配到接口';
    }

    byId('api-endpoint-count').textContent = endpointIndex.length + ' 个接口';
    renderCatalogSummary(endpoints);
  }

  function renderCatalogSummary(endpoints) {
    const holder = byId('api-endpoint-catalog');
    holder.innerHTML = '';

    const counts = {};
    endpoints.forEach(function (item) {
      counts[item.groupTitle] = (counts[item.groupTitle] || 0) + 1;
    });

    Object.keys(counts).forEach(function (title) {
      const row = document.createElement('div');
      row.className = 'api-catalog-item';
      row.innerHTML = '<strong>' + title + '</strong><span class="badge badge-gray">' + counts[title] + ' 个</span>';
      holder.appendChild(row);
    });

    if (!Object.keys(counts).length) {
      const row = document.createElement('div');
      row.className = 'api-catalog-item';
      row.innerHTML = '<strong>没有匹配到接口</strong><span class="badge badge-warning">0 个</span>';
      holder.appendChild(row);
    }
  }

  function resetApiConsoleRequest() {
    const endpoint = selectedEndpoint() || endpointIndex[0];
    if (endpoint) {
      setSelectedEndpoint(endpoint);
      byId('api-endpoint-select').value = endpoint.id;
    }
    setResponse('未请求', [{ label: '状态', value: '等待调用' }], '选择一个接口后可以直接发请求。支持修改请求方法、路径、Query 和 Body，也支持把任意路径改成你自己的接口。', 'gray');
  }

  function setResponse(statusText, metaItems, bodyText, badgeType) {
    const badge = byId('api-response-status');
    badge.className = 'badge badge-' + (badgeType || 'gray');
    badge.textContent = statusText;

    const meta = byId('api-response-meta');
    meta.innerHTML = '';
    metaItems.forEach(function (item) {
      const node = document.createElement('span');
      node.className = 'badge badge-gray';
      node.textContent = item.label + ': ' + item.value;
      meta.appendChild(node);
    });

    byId('api-response-body').textContent = bodyText;
  }

  async function sendApiConsoleRequest() {
    const method = byId('api-request-method').value.trim().toUpperCase();
    const url = resolveUrl();
    const token = byId('api-bearer-token').value.trim();
    const bodyRaw = byId('api-request-body').value.trim();
    const headers = Object.assign({ Accept: 'application/json' }, parseExtraHeaders(byId('api-extra-headers').value));
    const options = { method: method, headers: headers };

    if (token) {
      options.headers.Authorization = 'Bearer ' + token;
    }

    if (bodyRaw && !['GET', 'HEAD'].includes(method)) {
      options.headers['Content-Type'] = 'application/json';
      options.body = bodyRaw.startsWith('{') || bodyRaw.startsWith('[') ? JSON.stringify(JSON.parse(bodyRaw)) : bodyRaw;
    }

    const startedAt = performance.now();
    setResponse('请求中', [{ label: 'URL', value: url }], '正在发送请求...', 'info');

    try {
      const response = await fetch(url, options);
      const elapsed = Math.round(performance.now() - startedAt);
      const responseText = await response.text();
      let pretty = responseText;

      try {
        pretty = JSON.stringify(JSON.parse(responseText), null, 2);
      } catch (_) {
        pretty = responseText || '(空响应体)';
      }

      setResponse(
        response.status + ' ' + response.statusText,
        [
          { label: 'Method', value: method },
          { label: 'URL', value: url },
          { label: '耗时', value: elapsed + 'ms' }
        ],
        pretty,
        response.ok ? 'success' : 'danger'
      );

      if (window.showToast) {
        window.showToast(response.ok ? '请求成功' : '请求返回错误状态', response.ok ? 'success' : 'warning');
      }
    } catch (error) {
      setResponse(
        '请求失败',
        [
          { label: 'Method', value: method },
          { label: 'URL', value: url }
        ],
        error && error.stack ? error.stack : String(error),
        'danger'
      );
      if (window.showToast) window.showToast('请求失败，请检查地址、Token 或 CORS', 'error');
    }
  }

  async function loadApiConsoleManifests() {
    const origin = normalizeOrigin(byId('api-base-url').value);
    const targets = [
      origin + '/.well-known/capabilities.json',
      origin + '/.well-known/skill-manifest.json'
    ];

    try {
      const results = await Promise.all(targets.map(async function (url) {
        const response = await fetch(url);
        const text = await response.text();
        let parsed = text;
        try {
          parsed = JSON.stringify(JSON.parse(text), null, 2);
        } catch (_) {}
        return {
          url: url,
          status: response.status + ' ' + response.statusText,
          body: parsed
        };
      }));

      const rendered = results.map(function (item) {
        return '# ' + item.url + '\n' + item.status + '\n\n' + item.body;
      }).join('\n\n------------------------------\n\n');

      setResponse(
        'Manifest 已加载',
        [{ label: '数量', value: String(results.length) }],
        rendered,
        'success'
      );
      if (window.showToast) window.showToast('Manifest 已加载', 'success');
    } catch (error) {
      setResponse('Manifest 读取失败', [{ label: 'Origin', value: origin }], String(error), 'danger');
      if (window.showToast) window.showToast('Manifest 读取失败', 'error');
    }
  }

  function pingApiHealth() {
    byId('api-request-method').value = 'GET';
    byId('api-request-path').value = '/health';
    byId('api-request-query').value = '';
    byId('api-request-body').value = '';
    byId('api-use-prefix').checked = false;
    byId('api-selected-badge').textContent = '系统与安装 · GET';
    sendApiConsoleRequest();
  }

  async function copyApiConsoleCurl() {
    const method = byId('api-request-method').value.trim().toUpperCase();
    const url = resolveUrl();
    const token = byId('api-bearer-token').value.trim();
    const headers = Object.assign({ Accept: 'application/json' }, parseExtraHeaders(byId('api-extra-headers').value));
    const bodyRaw = byId('api-request-body').value.trim();
    const parts = ['curl -X ' + method, '"' + url + '"'];

    Object.keys(headers).forEach(function (key) {
      parts.push('-H "' + key + ': ' + String(headers[key]).replace(/"/g, '\\"') + '"');
    });

    if (token) {
      parts.push('-H "Authorization: Bearer ' + token.replace(/"/g, '\\"') + '"');
    }

    if (bodyRaw && !['GET', 'HEAD'].includes(method)) {
      parts.push('-H "Content-Type: application/json"');
      parts.push("--data '" + bodyRaw.replace(/'/g, "'\"'\"'") + "'");
    }

    const command = parts.join(' ');
    try {
      await navigator.clipboard.writeText(command);
      if (window.showToast) window.showToast('cURL 已复制', 'success');
    } catch (_) {
      setResponse('cURL 复制失败', [{ label: 'URL', value: url }], command, 'warning');
      if (window.showToast) window.showToast('无法直接复制，已把 cURL 输出到响应区', 'warning');
    }
  }

  function bindEvents() {
    byId('api-endpoint-group').addEventListener('change', renderEndpointSelect);
    byId('api-endpoint-search').addEventListener('input', renderEndpointSelect);
    byId('api-endpoint-select').addEventListener('change', function () {
      setSelectedEndpoint(selectedEndpoint());
    });
  }

  function init() {
    if (!byId('api-endpoint-group') || !byId('api-endpoint-select')) return;
    loadConfig();
    renderGroupSelect();
    bindEvents();
    renderEndpointSelect();
    resetApiConsoleRequest();
  }

  window.saveApiConsoleConfig = saveApiConsoleConfig;
  window.sendApiConsoleRequest = sendApiConsoleRequest;
  window.copyApiConsoleCurl = copyApiConsoleCurl;
  window.loadApiConsoleManifests = loadApiConsoleManifests;
  window.pingApiHealth = pingApiHealth;
  window.resetApiConsoleRequest = resetApiConsoleRequest;

  init();
})();
