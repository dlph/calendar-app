<template>
  <div class="container">
    <h1>Calendar</h1>
    <div class="flex justify-between items-center mb-4">
      <button class="btn" @click="prevMonth">Previous</button>
      <h2>{{ currentMonth }} {{ currentYear }}</h2>
      <button class="btn" @click="nextMonth">Next</button>
    </div>
    <div class="grid">
      <div v-for="day in daysOfWeek" :key="day" class="day-header">
        {{ day }}
      </div>
      <div
        v-for="(day, index) in calendarDays"
        :key="index"
        class="day"
        :class="{
          'day-not-current': !day.isCurrentMonth,
          'day-today': isToday(day.date),
          'day-current': day.isCurrentMonth,
        }"
        @click="day.isCurrentMonth && showEventModal(day.date)"
      >
        <span>{{ day.date.getDate() }}</span>
        <ul class="event-list">
          <li
            v-for="event in getEventsForDate(day.date)"
            :key="event.id"
            class="event"
          >
            {{ event.title }}
            <button class="event-delete" @click.stop="deleteEvent(event.id)">
              x
            </button>
          </li>
        </ul>
      </div>
    </div>
    <div v-if="showModal" class="modal-overlay">
      <div class="modal">
        <h3>Add Event for {{ selectedDate.toDateString() }}</h3>
        <input v-model="newEventTitle" type="text" placeholder="Event title" />
        <div class="modal-footer">
          <button class="btn btn-gray" @click="showModal = false">
            Cancel
          </button>
          <button class="btn" @click="addEvent">Save</button>
        </div>
      </div>
    </div>
    <div v-if="error" class="error">{{ error }}</div>
    <DebugInfo />
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import DebugInfo from './DebugInfo.vue'

interface CalendarEvent {
  id: string
  title: string
  date: string
}

export default defineComponent({
  name: 'CalendarApp',
  setup() {
    const currentDate = ref(new Date())
    const events = ref<CalendarEvent[]>([])
    const showModal = ref(false)
    const selectedDate = ref<Date>(new Date())
    const newEventTitle = ref('')
    const error = ref('')

    const daysOfWeek = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']

    const currentMonth = computed(() =>
      currentDate.value.toLocaleString('default', { month: 'long' })
    )
    const currentYear = computed(() => currentDate.value.getFullYear())

    const calendarDays = computed(() => {
      const year = currentDate.value.getFullYear()
      const month = currentDate.value.getMonth()
      const firstDay = new Date(year, month, 1)
      const lastDay = new Date(year, month + 1, 0)
      const days: { date: Date; isCurrentMonth: boolean }[] = []

      const firstWeekday = firstDay.getDay()
      for (let i = firstWeekday - 1; i >= 0; i--) {
        const date = new Date(year, month, -i)
        days.push({ date, isCurrentMonth: false })
      }

      for (let i = 1; i <= lastDay.getDate(); i++) {
        days.push({ date: new Date(year, month, i), isCurrentMonth: true })
      }

      const totalDays = days.length
      for (let i = 1; i <= 42 - totalDays; i++) {
        days.push({ date: new Date(year, month + 1, i), isCurrentMonth: false })
      }

      return days
    })

    const isToday = (date: Date) => {
      const today = new Date()
      return (
        date.getDate() === today.getDate() &&
        date.getMonth() === today.getMonth() &&
        date.getFullYear() === today.getFullYear()
      )
    }

    const getEventsForDate = (date: Date) => {
      const dateStr = date.toISOString().split('T')[0]
      return events.value.filter(event => event.date.startsWith(dateStr))
    }

    const loadEvents = async (): Promise<void> => {
      try {
        const fetchedEvents: CalendarEvent[] = await invoke('get_events')
        events.value = fetchedEvents
      } catch (err) {
        error.value = `Failed to load events: ${err}`
        events.value = []
      }
    }

    const addEvent = async (): Promise<void> => {
      if (!selectedDate.value || !newEventTitle.value.trim()) return
      try {
        const event: CalendarEvent = {
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

    const deleteEvent = async (id: string): Promise<void> => {
      try {
        await invoke('delete_event', { id })
        events.value = events.value.filter(e => e.id !== id)

        error.value = ''
      } catch (err) {
        error.value = `Failed to delete event: ${err}`
      }
    }

    const prevMonth = () => {
      currentDate.value = new Date(
        currentDate.value.getFullYear(),
        currentDate.value.getMonth() - 1,
        1
      )
    }
    const nextMonth = () => {
      currentDate.value = new Date(
        currentDate.value.getFullYear(),
        currentDate.value.getMonth() + 1,
        1
      )
    }

    const showEventModal = (date: Date) => {
      selectedDate.value = date
      showModal.value = true
      newEventTitle.value = ''
    }

    loadEvents()

    return {
      daysOfWeek,
      currentMonth,
      currentYear,
      calendarDays,
      isToday,
      getEventsForDate,
      showModal,
      selectedDate,
      newEventTitle,
      error,
      prevMonth,
      nextMonth,
      showEventModal,
      addEvent,
      deleteEvent,
    }
  },
  components: {
    DebugInfo,
  },
})
</script>
