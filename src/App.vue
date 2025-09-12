<template>
  <div id="app">
    <router-view />
  </div>
</template>

<script>
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'App',
})
</script>

<style>
/* Global styles */
* {
  box-sizing: border-box;
}

html {
  height: 100%;
}

body {
  margin: 0;
  padding: 0;
  height: 100%;
  font-family:
    -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu',
    'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  background-color: var(--background-color, #ffffff);
  color: var(--text-color, #1d1d1f);
}

#app {
  min-height: 100vh;
  width: 100%;
}

/* CSS Custom Properties for theming */
:root {
  --background-color: #ffffff;
  --text-color: #1d1d1f;
  --secondary-text: #86868b;
  --card-background: #ffffff;
  --border-color: #d2d2d7;
  --hover-border: #007aff;
  --input-background: #ffffff;
  --hover-background: #f5f5f7;
  --primary-color: #007aff;
  --primary-hover: #0056cc;
  --primary-active: #004499;
  --danger-color: #ff3b30;
  --danger-hover: #d70015;
  --danger-active: #a20010;
  --success-color: #30d158;
  --warning-color: #ff9500;
  --error-background: #fff5f5;
  --error-color: #dc2626;
  --error-border: #fecaca;
}

/* Dark mode support */
@media (prefers-color-scheme: dark) {
  :root {
    --background-color: #000000;
    --text-color: #f2f2f7;
    --secondary-text: #8e8e93;
    --card-background: #1c1c1e;
    --border-color: #38383a;
    --hover-border: #0a84ff;
    --input-background: #1c1c1e;
    --hover-background: rgba(255, 255, 255, 0.1);
    --primary-color: #0a84ff;
    --primary-hover: #409cff;
    --primary-active: #66b3ff;
    --danger-color: #ff453a;
    --danger-hover: #ff6961;
    --danger-active: #ff8a80;
    --success-color: #32d74b;
    --warning-color: #ff9f0a;
    --error-background: #2c1810;
    --error-color: #ff453a;
    --error-border: #ff453a;
  }
}

/* Focus styles for accessibility */
*:focus {
  outline: 2px solid var(--primary-color);
  outline-offset: 2px;
}

button:focus,
input:focus,
select:focus,
textarea:focus {
  outline: 2px solid var(--primary-color);
  outline-offset: 2px;
}

/* Remove focus outline for mouse users */
*:focus:not(:focus-visible) {
  outline: none;
}

/* High contrast mode support */
@media (prefers-contrast: high) {
  :root {
    --border-color: #000000;
    --hover-border: #0000ff;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      --border-color: #ffffff;
      --hover-border: #66b3ff;
    }
  }
}

/* Reduced motion support */
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }
}

/* Scrollbar styling for webkit browsers */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: var(--card-background);
}

::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--secondary-text);
}

/* Selection styling */
::selection {
  background-color: var(--primary-color);
  color: white;
}

::-moz-selection {
  background-color: var(--primary-color);
  color: white;
}
</style>
