<template>
    <v-layout row justify-center>
        <v-dialog v-model="open" persistent max-width="300px">
            <v-card>
                <v-card-title>
                    <span class="headline">{{message}}</span>
                </v-card-title>
                <v-card-actions>
                    <v-spacer></v-spacer>
                    <v-btn color="blue darken-1" @click="handleStart">Start</v-btn>
                    <v-spacer></v-spacer>
                </v-card-actions>
            </v-card>
        </v-dialog>
    </v-layout>
</template>

<script>
export default {
    props: [
        'open',
        'message',
    ],
    data: function () {
        return {
        };
    },
    created () {
    },
    mounted() {
    },
    computed: {
    },
    methods: {
        handleStart() {
            this.$emit('update:open', false)
        },
    }
}
</script>

<style>

</style>
