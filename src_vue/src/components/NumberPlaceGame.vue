<template>
    <v-container style="height: 100%;">
        <StartDialog
            :open.sync="isDialogOpened"
            :message="message" />
        <svg
            id="number-place-game"
            ref="numberPlaceGame"
            style="width:100%; height: 80%;" >
            <Score
                x="380"
                y="10"
                ref="score" />
            <Board 
                x="10"
                y="10" />
        </svg>
    </v-container>
</template>

<script>
import { mapGetters, mapState } from 'vuex'
import StartDialog from './StartDialog'
import Score from './Score'
import Board from './Board'

export default {
    components: {
        StartDialog,
        Score,
        Board,
    },
    props: [
    ],
    data: function () {
        return {
            isGameOver: false,
            isDialogOpened: false,
            maxX: 0,
            maxY: 0,
        };
    },
    created () {
    },
    mounted() {
        this.resetArea()
    },
    watch: {
        isDialogOpened: function(val) {
            if (!val) {
                this.$refs.numberPlaceGame.focus()
                this.resetGame()
            }
        }
    },
    computed: {
        message() {
            if (this.isGameOver) {
                return "Game Over!"
            }

            return ""
        },
    },
    methods: {
        resetArea() {
        },

        resetGame() {
        },
    }
}
</script>

<style>
#snake-game {
    border: 1px solid #000;
    background-color: #000;
}
</style>
