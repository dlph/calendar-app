<template>
  <main class="app-container">
    <header class="app-header">
      <h1>Calendar Events</h1>
    </header>

    <!-- Date Picker -->
    <section class="date-picker-section" aria-labelledby="date-picker-title">
      <h2 id="date-picker-title">Date Range Selection</h2>
      <div class="picker-wrapper">
        <DatePicker v-model.range="range" mode="dateTime" />
      </div>
    </section>

    <!-- Events List -->
    <section class="events-section" aria-labelledby="events-title">
      <div class="events-header">
        <h2 id="events-title">Your Events</h2>
        <button
          @click="loadEvents"
          class="refresh-btn"
          type="button"
          aria-label="Refresh events list"
        >
          <span class="refresh-icon">⟲</span>
          Refresh
        </button>
      </div>

      <div v-if="error" class="error-message" role="alert" aria-live="polite">
        <span class="error-icon">⚠</span>
        {{ error }}
      </div>

      <div v-if="events.length === 0 && !error" class="no-events" role="status">
        <div class="no-events-icon">📅</div>
        <p>No events saved yet</p>
        <small>Create your first event to get started</small>
      </div>

      <ul v-if="events.length > 0" class="events-list" role="list">
        <li
          v-for="event in events"
          :key="event.id"
          class="event-item"
          role="listitem"
        >
          <div class="event-content">
            <h3 class="event-title">{{ event.title }}</h3>
            <time class="event-date" :datetime="event.date">
              {{ new Date(event.date).toLocaleString() }}
            </time>
          </div>
          <button
            @click="deleteEvent(event.id)"
            class="delete-btn"
            type="button"
            :aria-label="`Delete event: ${event.title}`"
          >
            <span class="delete-icon">×</span>
            Delete
          </button>
        </li>
      </ul>
    </section>
  </main>
</template>

<script type="ts">
import { defineComponent, ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Calendar, DatePicker } from 'v-calendar'
import 'v-calendar/style.css'

import DebugInfo from './DebugInfo.vue'

export default defineComponent({
  name: 'CalendarApp',
  setup() {
    const currentDate = ref(new Date())
    const events = ref([])
    const error = ref('')
    const newEventTitle = ref('')

    const getEventsForDate = date => {
      const dateStr = date.toISOString().split('T')[0]
      return events.value.filter(event => event.date.startsWith(dateStr))
    }

    const loadEvents = async () => {
      try {
        const fetchedEvents = await invoke('get_events')
        events.value = fetchedEvents
      } catch (err) {
        error.value = `Failed to load events: ${err}`
        events.value = []
      }
    }

    const addEvent = async () => {
      if (!selectedDate.value || !newEventTitle.value.trim()) return
      try {
        const event = {
          id: Math.random().toString(36).substring(2),
          title: newEventTitle.value.trim(),
          date: selectedDate.value.toISOString(),
        }
        await invoke('add_event', { event })
        events.value.push(event)

        newEventTitle.value = ''
        showModal.value = false
      } catch (err) {
        error.value = `Failed to add event: ${err}`
      }
    }

    const deleteEvent = async id => {
      try {
        await invoke('delete_event', { id })
        events.value = events.value.filter(e => e.id !== id)

        error.value = ''
      } catch (err) {
        error.value = `Failed to delete event: ${err}`
      }
    }

    const showEventModal = date => {
      selectedDate.value = date
      showModal.value = true
      newEventTitle.value = ''
    }

    loadEvents()

    const range = ref({
      start: new Date(2020, 0, 6),
      end: new Date(2020, 0, 10),
    });

    return {
      range,
      events,
      error,
      getEventsForDate,
      newEventTitle,
      showEventModal,
      error,
      addEvent,
      deleteEvent,
      loadEvents,
    }
  },
  components: {
    DebugInfo,
    Calendar,
    DatePicker,
  },
})
</script>

<style scoped>
.app-container {
  max-width: 900px;
  margin: 0 auto;
  padding: 20px;
  font-family:
    -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu',
    'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
  background-color: var(--background-color, #ffffff);
  color: var(--text-color, #1d1d1f);
  line-height: 1.5;
}

.app-header {
  margin-bottom: 32px;
  text-align: center;
}

h1 {
  color: var(--text-color, #1d1d1f);
  margin: 0;
  font-weight: 600;
  font-size: 28px;
  letter-spacing: -0.02em;
}

h2 {
  color: var(--text-color, #1d1d1f);
  margin: 0 0 16px 0;
  font-weight: 500;
  font-size: 20px;
  letter-spacing: -0.01em;
}

h3 {
  margin: 0;
  font-weight: 500;
  font-size: 16px;
}

.date-picker-section {
  margin-bottom: 32px;
  padding: 20px;
  background-color: var(--card-background, #fbfbfb);
  border: 1px solid var(--border-color, #d2d2d7);
  border-radius: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.picker-wrapper {
  margin-top: 16px;
}

.events-section {
  border-top: 1px solid var(--border-color, #d2d2d7);
  padding-top: 32px;
}

.events-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.error-message {
  background-color: var(--error-background, #fff5f5);
  color: var(--error-color, #dc2626);
  padding: 12px 16px;
  border-radius: 8px;
  margin-bottom: 16px;
  border: 1px solid var(--error-border, #fecaca);
  font-size: 14px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.error-icon {
  font-size: 16px;
}

.no-events {
  color: var(--secondary-text, #86868b);
  text-align: center;
  padding: 48px 20px;
  background-color: var(--card-background, #fbfbfb);
  border: 1px solid var(--border-color, #d2d2d7);
  border-radius: 12px;
}

.no-events-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.no-events p {
  margin: 0 0 8px 0;
  font-size: 18px;
  font-weight: 500;
}

.no-events small {
  font-size: 14px;
  opacity: 0.8;
}

.events-list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.event-item {
  border: 1px solid var(--border-color, #d2d2d7);
  border-radius: 12px;
  padding: 20px;
  background-color: var(--card-background, #ffffff);
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition:
    border-color 0.2s ease,
    box-shadow 0.2s ease,
    transform 0.1s ease;
}

.event-item:hover {
  border-color: var(--hover-border, #007aff);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transform: translateY(-1px);
}

.event-content {
  flex: 1;
}

.event-title {
  font-weight: 500;
  color: var(--text-color, #1d1d1f);
  font-size: 16px;
  margin-bottom: 4px;
}

.event-date {
  color: var(--secondary-text, #86868b);
  font-size: 14px;
  font-variant-numeric: tabular-nums;
}

.delete-btn {
  background-color: var(--danger-color, #ff3b30);
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 20px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 4px;
  min-width: 80px;
  justify-content: center;
}

.delete-btn:hover {
  background-color: var(--danger-hover, #d70015);
  transform: scale(1.02);
}

.delete-btn:active {
  background-color: var(--danger-active, #a20010);
  transform: scale(0.98);
}

.delete-icon {
  font-size: 16px;
  line-height: 1;
}

.refresh-btn {
  background-color: var(--primary-color, #007aff);
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 20px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 6px;
}

.refresh-btn:hover {
  background-color: var(--primary-hover, #0056cc);
  transform: scale(1.02);
}

.refresh-btn:active {
  background-color: var(--primary-active, #004499);
  transform: scale(0.98);
}

.refresh-icon {
  font-size: 16px;
  line-height: 1;
}

/* Dark mode support */
@media (prefers-color-scheme: dark) {
  .app-container {
    --background-color: #000000;
    --text-color: #f2f2f7;
    --secondary-text: #8e8e93;
    --card-background: #1c1c1e;
    --border-color: #38383a;
    --hover-border: #0a84ff;
    --error-background: #2c1810;
    --error-color: #ff453a;
    --error-border: #ff453a;
    --danger-color: #ff453a;
    --danger-hover: #ff6961;
    --danger-active: #ff8a80;
    --primary-color: #0a84ff;
    --primary-hover: #409cff;
    --primary-active: #66b3ff;
  }
}

/* macOS-specific styling */
@supports (-webkit-backdrop-filter: blur(20px)) {
  .date-picker-section,
  .event-item {
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
  }

  .date-picker-section {
    background-color: rgba(251, 251, 251, 0.8);
  }

  .event-item {
    background-color: rgba(255, 255, 255, 0.8);
  }

  @media (prefers-color-scheme: dark) {
    .date-picker-section {
      background-color: rgba(28, 28, 30, 0.8);
    }

    .event-item {
      background-color: rgba(28, 28, 30, 0.8);
    }
  }
}

/* Windows-specific adjustments */
@supports (-ms-ime-align: auto) {
  .app-container {
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  }

  .date-picker-section,
  .event-item {
    border-radius: 8px;
  }

  .delete-btn,
  .refresh-btn {
    border-radius: 4px;
  }

  .no-events {
    border-radius: 8px;
  }
}

/* Linux-specific adjustments */
@supports (font-family: 'Ubuntu') {
  .app-container {
    font-family: 'Ubuntu', 'Roboto', 'Liberation Sans', sans-serif;
  }
}

/* High contrast mode support */
@media (prefers-contrast: high) {
  .event-item {
    border-width: 2px;
  }

  .delete-btn,
  .refresh-btn {
    border: 2px solid currentColor;
  }
}

/* Reduced motion support */
@media (prefers-reduced-motion: reduce) {
  .event-item,
  .delete-btn,
  .refresh-btn {
    transition: none;
  }

  .event-item:hover {
    transform: none;
  }

  .delete-btn:hover,
  .refresh-btn:hover {
    transform: none;
  }
}
</style>
