<template>
  <div class="debug-info">
    <h3>Debug Information</h3>
    <div class="debug-section">
      <h4>File Storage Location</h4>
      <p v-if="appDataPath">
        <strong>events.json saved to:</strong><br />
        <code>{{ appDataPath }}/events.json</code>
      </p>
      <p v-else-if="loading">Loading path...</p>
      <p v-else-if="error" class="error">{{ error }}</p>
      <button @click="getAppDataPath" :disabled="loading">
        {{ loading ? 'Loading...' : 'Get File Location' }}
      </button>
    </div>

    <div class="debug-section">
      <h4>Platform-specific locations:</h4>
      <ul class="location-list">
        <li>
          <strong>Windows:</strong>
          <code
            >C:\Users\{username}\AppData\Roaming\com.dlph.calendar-app\events.json</code
          >
        </li>
        <li>
          <strong>macOS:</strong>
          <code
            >~/Library/Application
            Support/com.dlph.calendar-app/events.json</code
          >
        </li>
        <li>
          <strong>Linux:</strong>
          <code>~/.local/share/com.dlph.calendar-app/events.json</code>
        </li>
      </ul>
    </div>

    <div class="debug-section">
      <h4>Quick Actions</h4>
      <button @click="openFileLocation" :disabled="!appDataPath">
        Open File Location
      </button>
      <button @click="copyPath" :disabled="!appDataPath">
        Copy Path to Clipboard
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const appDataPath = ref<string>('')
const loading = ref(false)
const error = ref<string>('')

const getAppDataPath = async () => {
  loading.value = true
  error.value = ''

  try {
    const path = await invoke<string>('get_app_data_path')
    appDataPath.value = path
  } catch (err) {
    error.value =
      err instanceof Error ? err.message : 'Failed to get app data path'
    console.error('Error getting app data path:', err)
  } finally {
    loading.value = false
  }
}

const openFileLocation = async () => {
  if (!appDataPath.value) return

  try {
    await invoke('open_file_location', { path: appDataPath.value })
  } catch (err) {
    console.error('Error opening file location:', err)
    // Fallback: try to open with system command
    try {
      const { Command } = await import('@tauri-apps/plugin-shell')
      const { platform } = await import('@tauri-apps/plugin-os')
      const osType = await platform()

      let cmd
      if (osType === 'windows') {
        cmd = Command.create('explorer', [appDataPath.value])
      } else if (osType === 'macos') {
        cmd = Command.create('open', [appDataPath.value])
      } else {
        cmd = Command.create('xdg-open', [appDataPath.value])
      }

      await cmd.execute()
    } catch (fallbackErr) {
      console.error('Fallback error:', fallbackErr)
      alert(
        `Cannot open file location automatically. Path: ${appDataPath.value}`
      )
    }
  }
}

const copyPath = async () => {
  if (!appDataPath.value) return

  try {
    const fullPath = `${appDataPath.value}/events.json`
    await navigator.clipboard.writeText(fullPath)
    alert('Path copied to clipboard!')
  } catch (err) {
    console.error('Error copying to clipboard:', err)
    // Fallback: select text for manual copy
    const textArea = document.createElement('textarea')
    textArea.value = `${appDataPath.value}/events.json`
    document.body.appendChild(textArea)
    textArea.select()
    document.execCommand('copy')
    document.body.removeChild(textArea)
    alert('Path copied to clipboard!')
  }
}

onMounted(() => {
  getAppDataPath()
})
</script>

<style>
.debug-info {
  margin: 20px 0;
  padding: 20px;
  border: 1px solid #ddd;
  border-radius: 8px;
  background-color: #f9f9f9;
  font-family: 'Courier New', monospace;
}

.debug-info h3 {
  margin-top: 0;
  color: #333;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
}

.debug-section {
  margin-bottom: 20px;
  padding-bottom: 15px;
  border-bottom: 1px solid #eee;
}

.debug-section:last-child {
  border-bottom: none;
  margin-bottom: 0;
}

.debug-section h4 {
  margin: 0 0 10px 0;
  color: #555;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
}

code {
  background-color: #f1f1f1;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 0.9em;
  word-break: break-all;
}

.location-list {
  list-style: none;
  padding: 0;
  margin: 10px 0;
}

.location-list li {
  margin: 8px 0;
  padding: 8px;
  background-color: #fff;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
}

button {
  background-color: #007acc;
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
  margin-right: 10px;
  margin-bottom: 5px;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
}

button:hover:not(:disabled) {
  background-color: #005f99;
}

button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

.error {
  color: #d32f2f;
  font-weight: bold;
}

@media (prefers-color-scheme: dark) {
  .debug-info {
    background-color: #2a2a2a;
    border-color: #555;
    color: #f0f0f0;
  }

  .debug-info h3,
  .debug-section h4 {
    color: #f0f0f0;
  }

  code {
    background-color: #404040;
    color: #f0f0f0;
  }

  .location-list li {
    background-color: #333;
    border-color: #555;
    color: #f0f0f0;
  }

  button {
    background-color: #0d7377;
  }

  button:hover:not(:disabled) {
    background-color: #0a5a5e;
  }
}
</style>
