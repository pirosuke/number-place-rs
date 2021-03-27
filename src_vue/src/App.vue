<template>
  <v-app>
    <v-app-bar
      app
      color="indigo"
      dark
    >
      <v-toolbar-title class="headline">
        <span>Number Place</span>
      </v-toolbar-title>
      <v-spacer></v-spacer>
      <v-btn
        link
        href="https://github.com/pirosuke/number-place"
        target="_blank">
        GitHub
      </v-btn>
    </v-app-bar>

    <v-main>
      <v-container fluid class="fill-height">
        <router-view/>
      </v-container>
    </v-main>
  </v-app>
</template>

<script>
import { mapGetters, mapState } from 'vuex'

export default {
  name: 'App',
  components: {
  },
  data () {
    return {
    }
  },
  computed: {
    ...mapState({
    })
  },

  mounted() {
  },

  created () {
  },

  destroyed () {
  },

  methods: {
  },
}
</script>
