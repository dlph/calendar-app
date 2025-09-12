<template>
  <div class="events-container">
    <header class="events-header">
      <div class="header-content">
        <div class="title-section">
          <router-link to="/" class="back-button" aria-label="Back to home">
            ← Back
          </router-link>
          <h1>Your Events</h1>
        </div>
        <button
          @click="loadEvents"
          class="refresh-btn"
          type="button"
          aria-label="Refresh events list"
          :disabled="loading"
        >
          <span class="refresh-icon" :class="{ spinning: loading }">⟲</span>
          {{ loading ? 'Loading...' : 'Refresh' }}
        </button>
      </div>
    </header>

    <!-- Search and Filter -->
    <section class="filter-section">
      <div class="search-box">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Search events..."
          class="search-input"
          aria-label="Search events"
        />
        <span class="search-icon">🔍</span>
      </div>
      <select v-model="sortBy" class="sort-select" aria-label="Sort events by">
        <option value="date-asc">Date (Oldest First)</option>
        <option value="date-desc">Date (Newest First)</option>
        <option value="title-asc">Title (A-Z)</option>
        <option value="title-desc">Title (Z-A)</option>
      </select>
    </section>

    <!-- Error Message -->
    <div v-if="error" class="error-message" role="alert" aria-live="polite">
      <span class="error-icon">⚠</span>
      {{ error }}
      <button @click="error = ''" class="dismiss-error" aria-label="Dismiss error">×</button>
    </div>

    <!-- Events List -->
    <section class="events-list-section">
      <!-- Empty State -->
      <div v-if="filteredEvents.length === 0 && !loading && !error" class="empty-state">
        <div class="empty-icon">📅</div>
        <h2>{{ searchQuery ? 'No matching events' : 'No events found' }}</h2>
        <p v-if="searchQuery">
          Try adjusting your search terms or
          <button @click="searchQuery = ''" class="clear-search">clear the search</button>
        </p>
        <p v-else>Create your first event to get started</p>
      </div>

      <!-- Events Grid -->
      <div v-if="filteredEvents.length > 0" class="events-grid">
        <div
          v-for="event in paginatedEvents"
          :key="event.id"
          class="event-card"
          :class="{ 'past-event': isPastEvent(event) }"
        >
          <div class="event-header">
            <h3 class="event-title">{{ event.title }}</h3>
            <div class="event-badges">
              <span v-if="isPastEvent(event)" class="badge past">Past</span>
              <span v-else-if="isToday(event)" class="badge today">Today</span>
              <span v-else-if="isUpcoming(event)" class="badge upcoming">Upcoming</span>
            </div>
          </div>

          <div class="event-details">
            <time class="event-date" :datetime="event.date">
              <span class="date-icon">📅</span>
              {{ formatEventDate(event.date) }}
            </time>
            <div class="event-time">
              <span class="time-icon">🕒</span>
              {{ formatEventTime(event.date) }}
            </div>
          </div>

          <div class="event-actions">
            <button
              @click="deleteEvent(event.id)"
              class="delete-btn"
              type="button"
              :aria-label="`Delete event: ${event.title}`"
              :disabled="deleting === event.id"
            >
              <span v-if="deleting === event.id" class="delete-icon spinning">⟲</span>
              <span v-else class="delete-icon">🗑</span>
              {{ deleting === event.id ? 'Deleting...' : 'Delete' }}
            </button>
          </div>
        </div>
      </div>

      <!-- Pagination -->
      <div v-if="totalPages > 1" class="pagination">
        <button
          @click="currentPage--"
          :disabled="currentPage === 1"
          class="pagination-btn"
          aria-label="Previous page"
        >
          ← Previous
        </button>
        <span class="pagination-info">
          Page {{ currentPage }} of {{ totalPages }}
        </span>
        <button
          @click="currentPage++"
          :disabled="currentPage === totalPages"
          class="pagination-btn"
          aria-label="Next page"
        >
          Next →
        </button>
      </div>
    </section>

    <!-- Add Event FAB -->
    <button
      @click="showAddModal = true"
      class="add-event-fab"
      type="button"
      aria-label="Add new event"
    >
      <span class="fab-icon">+</span>
    </button>

    <!-- Add Event Modal -->
    <div v-if="showAddModal" class="modal-overlay" @click="closeModal">
      <div class="modal-content" @click.stop>
        <header class="modal-header">
          <h2>Add New Event</h2>
          <button @click="closeModal" class="close-btn" aria-label="Close modal">×</button>
        </header>
        <form @submit.prevent="addEvent" class="add-event-form">
          <div class="form-group">
            <label for="event-title">Event Title</label>
            <input
              id="event-title"
              v-model="newEventTitle"
              type="text"
              placeholder="Enter event title..."
              class="form-input"
              required
              maxlength="100"
            />
          </div>
          <div class="form-group">
            <label for="event-date">Date & Time</label>
            <input
              id="event-date"
              v-model="newEventDate"
              type="datetime-local"
              class="form-input"
              required
            />
          </div>
          <div class="form-actions">
            <button type="button" @click="closeModal" class="cancel-btn">
              Cancel
            </button>
            <button
              type="submit"
              class="submit-btn"
              :disabled="!newEventTitle.trim() || !newEventDate || adding"
            >
              <span v-if="adding" class="spinner">⟲</span>
              {{ adding ? 'Adding...' : 'Add Event' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script>
import { defineComponent, ref, computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export default defineComponent({
  name: 'Events',
  setup() {
    const events = ref([])
    const error = ref('')
    const loading = ref(false)
    const deleting = ref(null)
    const adding = ref(false)
    const searchQuery = ref('')
    const sortBy = ref('date-asc')
    const currentPage = ref(1)
    const itemsPerPage = 12

    // Modal state
    const showAddModal = ref(false)
    const newEventTitle = ref('')
    const newEventDate = ref('')

    // Computed properties
    const filteredEvents = computed(() => {
      let filtered = events.value

      // Apply search filter
      if (searchQuery.value) {
        const query = searchQuery.value.toLowerCase()
        filtered = filtered.filter(event =>
          event.title.toLowerCase().includes(query)
        )
      }

      // Apply sorting
      filtered.sort((a, b) => {
        switch (sortBy.value) {
          case 'date-desc':
            return new Date(b.date) - new Date(a.date)
          case 'title-asc':
            return a.title.localeCompare(b.title)
          case 'title-desc':
            return b.title.localeCompare(a.title)
          default: // date-asc
            return new Date(a.date) - new Date(b.date)
        }
      })

      return filtered
    })

    const totalPages = computed(() =>
      Math.ceil(filteredEvents.value.length / itemsPerPage)
    )

    const paginatedEvents = computed(() => {
      const start = (currentPage.value - 1) * itemsPerPage
      const end = start + itemsPerPage
      return filteredEvents.value.slice(start, end)
    })

    // Watch for search/sort changes to reset pagination
    watch([searchQuery, sortBy], () => {
      currentPage.value = 1
    })

    // Methods
    const loadEvents = async () => {
      loading.value = true
      error.value = ''
      try {
        const fetchedEvents = await invoke('get_events')
        events.value = fetchedEvents
      } catch (err) {
        error.value = `Failed to load events: ${err}`
        events.value = []
      } finally {
        loading.value = false
      }
    }

    const addEvent = async () => {
      if (!newEventTitle.value.trim() || !newEventDate.value) return

      adding.value = true
      try {
        const event = {
          id: Math.random().toString(36).substring(2),
          title: newEventTitle.value.trim(),
          date: new Date(newEventDate.value).toISOString(),
        }

        await invoke('add_event', { event })
        events.value.push(event)
        closeModal()
        error.value = ''
      } catch (err) {
        error.value = `Failed to add event: ${err}`
      } finally {
        adding.value = false
      }
    }

    const deleteEvent = async (id) => {
      deleting.value = id
      try {
        await invoke('delete_event', { id })
        events.value = events.value.filter(e => e.id !== id)
        error.value = ''
      } catch (err) {
        error.value = `Failed to delete event: ${err}`
      } finally {
        deleting.value = null
      }
    }

    const closeModal = () => {
      showAddModal.value = false
      newEventTitle.value = ''
      newEventDate.value = ''
    }

    // Date utilities
    const isPastEvent = (event) => {
      return new Date(event.date) < new Date()
    }

    const isToday = (event) => {
      const eventDate = new Date(event.date)
      const today = new Date()
      return eventDate.toDateString() === today.toDateString()
    }

    const isUpcoming = (event) => {
      const eventDate = new Date(event.date)
      const today = new Date()
      const oneWeekFromNow = new Date(today.getTime() + 7 * 24 * 60 * 60 * 1000)
      return eventDate > today && eventDate <= oneWeekFromNow
    }

    const formatEventDate = (dateString) => {
      return new Date(dateString).toLocaleDateString(undefined, {
        weekday: 'short',
        year: 'numeric',
        month: 'short',
        day: 'numeric'
      })
    }

    const formatEventTime = (dateString) => {
      return new Date(dateString).toLocaleTimeString(undefined, {
        hour: '2-digit',
        minute: '2-digit'
      })
    }

    // Initialize
    onMounted(() => {
      loadEvents()
    })

    return {
      events,
      error,
      loading,
      deleting,
      adding,
      searchQuery,
      sortBy,
      currentPage,
      totalPages,
      filteredEvents,
      paginatedEvents,
      showAddModal,
      newEventTitle,
      newEventDate,
      loadEvents,
      addEvent,
      deleteEvent,
      closeModal,
      isPastEvent,
      isToday,
      isUpcoming,
      formatEventDate,
      formatEventTime,
    }
  },
})
</script>

<style scoped>
.events-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
  font-family:
    -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu',
    'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
  background-color: var(--background-color, #ffffff);
  color: var(--text-color, #1d1d1f);
  line-height: 1.5;
  min-height: 100vh;
  padding-bottom: 80px; /* Space for FAB */
}

/* Header */
.events-header {
  margin-bottom: 24px;
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 16px;
}

.title-section {
  display: flex;
  align-items: center;
  gap: 16px;
}

.back-button {
  color: var(--primary-color, #007aff);
  text-decoration: none;
  font-weight: 500;
  padding: 8px 12px;
  border-radius: 8px;
  transition: background-color 0.2s ease;
}

.back-button:hover {
  background-color: var(--hover-background, rgba(0, 122, 255, 0.1));
}

.events-header h1 {
  margin: 0;
  font-size: 28px;
  font-weight: 600;
  color: var(--text-color, #1d1d1f);
}

.refresh-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  background-color: var(--primary-color, #007aff);
  color: white;
  border: none;
  padding: 10px 16px;
  border-radius: 20px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s ease;
}

.refresh-btn:hover:not(:disabled) {
  background-color: var(--primary-hover, #0056cc);
}

.refresh-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* Filter Section */
.filter-section {
  display: flex;
  gap: 16px;
  margin-bottom: 24px;
  flex-wrap: wrap;
}

.search-box {
  position: relative;
  flex: 1;
  min-width: 200px;
}

.search-input {
  width: 100%;
  padding: 12px 16px 12px 40px;
  border: 1px solid var(--border-color, #d2d2d7);
  border-radius: 8px;
  font-size: 14px;
  background-color: var(--input-background, #ffffff);
  color: var(--text-color, #1d1d1f);
}

.search-icon {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--secondary-text, #86868b);
}

.sort-select {
  padding: 12px 16px;
  border: 1px solid var(--border-color, #d2d2d7);
  border-radius: 8px;
  font-size: 14px;
  background-color: var(--input-background, #ffffff);
  color: var(--text-color, #1d1d1f);
  cursor: pointer;
}

/* Error Message */
.error-message {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: var(--error-background, #fff5f5);
  color: var(--error-color, #dc2626);
  padding: 12px 16px;
  border-radius: 8px;
  margin-bottom: 16px;
  border: 1px solid var(--error-border, #fecaca);
  font-size: 14px;
}

.dismiss-error {
  background: none;
  border: none;
  font-size: 18px;
  cursor: pointer;
  color: inherit;
  padding: 0 4px;
}

/* Empty State */
.empty-state {
  text-align: center;
  padding: 48px 20px;
  color: var(--secondary-text, #86868b);
}

.empty-icon {
  font-size: 64px;
  margin-bottom: 16px;
}

.empty-state h2 {
  margin: 0 0 8px 0;
  color: var(--text-color, #1d1d1f);
}

.clear-search {
  background: none;
  border: none;
  color: var(--primary-color, #007aff);
  cursor: pointer;
  text-decoration: underline;
}

/* Events Grid */
.events-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
  margin-bottom: 32px;
}

.event-card {
  background-color: var(--card-background, #ffffff);
  border: 1px solid var(--border-color, #d2d2d7);
  border-radius: 12px;
  padding: 20px;
  transition: all 0.2s ease;
}

.event-card:hover {
  border-color: var(--hover-border, #007aff);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transform: translateY(-2px);
}

.event-card.past-event {
  opacity: 0.7;
}

.event-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 12px;
  gap: 12px;
}

.event-title {
  margin: 0;
  font-size: 16px;
  font-weight: 500;
  color: var(--text-color, #1d1d1f);
  flex: 1;
}

.event-badges {
  display: flex;
  gap: 4px;
}

.badge {
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.badge.past {
  background-color: var(--gray-light, #f1f1f1);
  color: var(--gray-dark, #666);
}

.badge.today {
  background-color: var(--orange-light, #fff3cd);
  color: var(--orange-dark, #856404);
}

.badge.upcoming {
  background-color: var(--green-light, #d1edff);
  color: var(--green-dark, #0066cc);
}

.event-details {
  margin-bottom: 16px;
}

.event-date,
.event-time {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: var(--secondary-text, #86868b);
  margin-bottom: 4px;
}

.event-actions {
  display: flex;
  justify-content: flex-end;
}

.delete-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  background-color: var(--danger-color, #ff3b30);
  color: white;
  border: none;
  padding: 8px 12px;
  border-radius: 16px;
  cursor: pointer;
  font-size: 12px;
  font-weight: 500;
  transition: all 0.2s ease;
}

.delete-btn:hover:not(:disabled) {
  background-color: var(--danger-hover, #d70015);
}

.delete-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* Pagination */
.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 16px;
  margin-top: 32px;
}

.pagination-btn {
  padding: 8px 16px;
  border: 1px solid var(--border-color, #d2d2d7);
  border-radius: 8px;
  background-color: var(--card-background, #ffffff);
  color: var(--text-color, #1d1d1f);
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s ease;
}

.pagination-btn:hover:not(:disabled) {
  background-color: var(--hover-background, #f5f5f7);
}

.pagination-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.pagination-info {
  font-size: 14px;
  color: var(--secondary-text, #86868b);
}

/* FAB */
.add-event-fab {
  position: fixed;
  bottom: 24px;
  right: 24px;
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background-color: var(--primary-color, #007aff);
  color: white;
  border: none;
  cursor: pointer;
  font-size: 24px;
  font-weight: 300;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
  transition: all 0.2s ease;
  z-index: 100;
}

.add-event-fab:hover {
  transform: scale(1.1);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3);
}

/* Modal */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--card-background, #ffffff);
  border-radius: 12px;
  width: 90%;
  max-width: 400px;
  max-height: 90vh;
  overflow: auto;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 20px 0 20px;
}

.modal-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: var(--secondary-text, #86868b);
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
}

.close-btn:hover {
  background-color: var(--hover-background, #f5f5f7);
}

.add-event-form {
  padding: 20px;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 6px;
  font-weight: 500;
  color: var(--text-color, #1d1d1f);
}

.form-input {
  width: 100%;
  padding: 12px;
  border: 1px solid var(--border-color, #d2d2d7);
  border-radius: 8px;
  font-size: 14px;
  background-color: var(--input-background, #ffffff);
  color: var(--text-color, #1d1d1f);
  box-sizing: border-box;
}

.form-actions {
  display: flex;
  gap: 12px;
  margin-top: 24px;
}

.cancel-btn {
  flex: 1;
  padding: 12px;
  border: 1px solid var(--border-color, #d2d2d7);
  border-radius: 8px;
  background-color: transparent;
  color: var(--text-color, #1d1d1f);
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
}

.submit-btn {
  flex: 1;
  padding: 12px;
  border: none;
  border-radius: 8px;
  background-color: var(--primary-color, #007aff);
  color: white;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
}

.submit-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* Animations */
.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  .events-container {
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
    --danger-color: #ff453a;
    --danger-hover: #ff6961;
    --error-background: #2c1810;
    --error-color: #ff453a;
    --error-border: #ff453a;
    --gray-light: #2c2c2e;
    --gray-dark: #aeaeb2;
    --orange-light: #2d1b00;
    --orange-dark: #ffb800;
    --green-light: #002a40;
    --green-dark: #64d2ff;
  }
}

/* Responsive */
@media (max-width: 768px) {
  .events-container {
    padding: 16px;
  }

  .header-content {
    flex-direction: column;
    align-items: stretch;
  }

  .title-section {
    justify-content: space-between;
  }

  .events-grid {
    grid-template-columns: 1fr;
    gap: 16px;
  }

  .filter-section {
    flex-direction: column;
  }

  .add-event-fab {
    bottom: 16px;
    right: 16px;
  }
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
  .event-card,
  .delete-btn,
  .refresh-btn,
  .add-event-fab {
    transition: none;
  }

  .event-card:hover,
  .add-event-fab:hover {
    transform: none;
  }

  .spinning {
    animation: none;
  }
}
</style>
