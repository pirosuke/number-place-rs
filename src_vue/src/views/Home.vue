<template>
  <NumberPlaceGame />
</template>

<script>
  import NumberPlaceGame from '../components/NumberPlaceGame'
  import { mapGetters, mapState } from 'vuex'

  export default {
    components: {
      NumberPlaceGame,
    },
    data () {
      return {
      }
    },

    watch: {
    },

    async mounted() {
    },
    computed: {
        ...mapState({
        }),
    },
    methods: {
    }
  }
</script>

<style scoped>

</style>