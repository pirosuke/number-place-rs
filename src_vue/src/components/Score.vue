<template>
    <g :transform="'translate(' + x + ',' + y + ')'">
        <a xlink:href="#">
            <rect
                x="0"
                y="0"
                width="180"
                height="40"
                stroke="#fff"
                fill="#000"
                fill-opacity="0.25"
                rx="5"
                ry="5" />
            <text
                x="10"
                y="30"
                width="180"
                fill="#9f9"
                stroke="#9f9"
                font-size="25">
                {{ scoreFormatted }}
            </text>
        </a>
    </g>
</template>

<script>
export default {
    components: {
    },
    props: [
        'x',
        'y',
    ],
    data: function () {
        return {
            score: 0,
        };
    },
    created () {
    },
    mounted() {
    },
    computed: {
        scoreFormatted() {
            return this.score.toString().padStart(10, '0')
        }
    },
    methods: {
        addScore(newScore) {
            this.score += newScore
        },

        resetScore() {
            this.score = 0
        },
    }
}
</script>

<style>

</style>
