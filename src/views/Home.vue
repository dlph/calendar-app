<template>
  <div class="home-container">
    <header class="home-header">
      <h1>Calendar Events</h1>
      <p class="subtitle">Manage your events and schedule</p>
    </header>

    <!-- Date Picker -->
    <section class="date-picker-section" aria-labelledby="date-picker-title">
      <h2 id="date-picker-title">Date Range Selection</h2>
      <div class="picker-wrapper">
        <DatePicker v-model.range="range" mode="dateTime" />
      </div>
    </section>

    <!-- Navigation -->
    <section class="navigation-section">
      <div class="nav-cards">
        <router-link to="/events" class="nav-card">
          <div class="nav-icon">📅</div>
          <h3>View Events</h3>
          <p>Browse and manage your events</p>
          <span class="nav-arrow">→</span>
        </router-link>
      </div>
    </section>

    <!-- Quick Stats -->
    <section class="stats-section" v-if="events.length > 0">
      <h2>Quick Overview</h2>
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-number">{{ events.length }}</div>
          <div class="stat-label">Total Events</div>
        </div>
        <div class="stat-card">
          <div class="stat-number">{{ upcomingEventsCount }}</div>
          <div class="stat-label">Upcoming</div>
        </div>
      </div>
    </section>
  </div>
</template>

<script>
import { defineComponent, ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { DatePicker } from 'v-calendar'

export default defineComponent({
  name: 'Home',
  components: {
    DatePicker,
  },
  setup() {
    const range = ref({
      start: new Date(2020, 0, 6),
      end: new Date(2020, 0, 10),
    })
    const events = ref([])

    const upcomingEventsCount = computed(() => {
      const now = new Date()
      return events.value.filter(event => new Date(event.date) > now).length
    })

    const loadEvents = async () => {
      try {
        const fetchedEvents = await invoke('get_events')
        events.value = fetchedEvents
      } catch (err) {
        console.error('Failed to load events:', err)
        events.value = []
      }
    }

    onMounted(() => {
      loadEvents()
    })

    return {
      range,
      events,
      upcomingEventsCount,
    }
  },
})
</script>

<style scoped>
.home-container {
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

.home-header {
  margin-bottom: 32px;
  text-align: center;
}

.home-header h1 {
  color: var(--text-color, #1d1d1f);
  margin: 0 0 8px 0;
  font-weight: 600;
  font-size: 32px;
  letter-spacing: -0.02em;
}

.subtitle {
  color: var(--secondary-text, #86868b);
  margin: 0;
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

.date-picker-section h2 {
  color: var(--text-color, #1d1d1f);
  margin: 0 0 16px 0;
  font-weight: 500;
  font-size: 20px;
  letter-spacing: -0.01em;
}

.picker-wrapper {
  margin-top: 16px;
}

.navigation-section {
  margin-bottom: 32px;
}

.nav-cards {
  display: grid;
  gap: 16px;
}

.nav-card {
  display: flex;
  align-items: center;
  padding: 24px;
  background-color: var(--card-background, #ffffff);
  border: 1px solid var(--border-color, #d2d2d7);
  border-radius: 12px;
  text-decoration: none;
  color: inherit;
  transition: all 0.2s ease;
  position: relative;
}

.nav-card:hover {
  border-color: var(--hover-border, #007aff);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transform: translateY(-1px);
}

.nav-icon {
  font-size: 32px;
  margin-right: 16px;
}

.nav-card h3 {
  margin: 0 0 4px 0;
  font-weight: 500;
  font-size: 18px;
  color: var(--text-color, #1d1d1f);
}

.nav-card p {
  margin: 0;
  font-size: 14px;
  color: var(--secondary-text, #86868b);
  flex: 1;
}

.nav-arrow {
  font-size: 18px;
  color: var(--secondary-text, #86868b);
  margin-left: 16px;
}

.stats-section {
  margin-bottom: 32px;
}

.stats-section h2 {
  color: var(--text-color, #1d1d1f);
  margin: 0 0 16px 0;
  font-weight: 500;
  font-size: 20px;
  letter-spacing: -0.01em;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 16px;
}

.stat-card {
  padding: 20px;
  background-color: var(--card-background, #fbfbfb);
  border: 1px solid var(--border-color, #d2d2d7);
  border-radius: 12px;
  text-align: center;
}

.stat-number {
  font-size: 28px;
  font-weight: 600;
  color: var(--primary-color, #007aff);
  margin-bottom: 4px;
}

.stat-label {
  font-size: 14px;
  color: var(--secondary-text, #86868b);
  font-weight: 500;
}

/* Dark mode support */
@media (prefers-color-scheme: dark) {
  .home-container {
    --background-color: #000000;
    --text-color: #f2f2f7;
    --secondary-text: #8e8e93;
    --card-background: #1c1c1e;
    --border-color: #38383a;
    --hover-border: #0a84ff;
    --primary-color: #0a84ff;
  }
}

/* macOS-specific styling */
@supports (-webkit-backdrop-filter: blur(20px)) {
  .date-picker-section,
  .nav-card,
  .stat-card {
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
  }

  .date-picker-section,
  .stat-card {
    background-color: rgba(251, 251, 251, 0.8);
  }

  .nav-card {
    background-color: rgba(255, 255, 255, 0.8);
  }

  @media (prefers-color-scheme: dark) {
    .date-picker-section,
    .nav-card,
    .stat-card {
      background-color: rgba(28, 28, 30, 0.8);
    }
  }
}

/* Reduced motion support */
@media (prefers-reduced-motion: reduce) {
  .nav-card {
    transition: none;
  }

  .nav-card:hover {
    transform: none;
  }
}
</style>
