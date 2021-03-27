<template>
    <g :transform="'translate(' + x + ',' + y + ')'">
        <rect
            x="0"
            y="0"
            width="360"
            height="360"
            stroke="#000"
            stroke-width="3"
            fill-opacity="0"
            fill="#fff" />
        <line
            v-for="point in points"
            v-bind:key="point"
            stroke="#000"
            :stroke-width="point % 3 == 0 ? 3 : 1"
            :x1="point * 40"
            :y1="0"
            :x2="point * 40"
            :y2="360" />
        <line
            v-for="point in points"
            v-bind:key="point"
            stroke="#000"
            :stroke-width="point % 3 == 0 ? 3 : 1"
            :x1="0"
            :y1="point * 40"
            :x2="360"
            :y2="point * 40" />
        <text
            v-for="cell in numberCells"
            :key="cell.id"
            :x="cell.x"
            :y="cell.y"
            width="40"
            fill="#00c"
            stroke="#00c"
            font-size="25">
            {{ cell.num }}
        </text>
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
            numbers: [
                [0,0,8,2,0,0,5,0,0],
                [1,0,5,0,6,0,2,3,4],
                [3,0,0,0,7,5,9,0,0],
                [0,8,6,4,0,0,0,0,0],
                [4,1,0,0,0,0,7,5,0],
                [5,2,0,0,0,1,8,4,6],
                [0,0,0,0,0,0,4,7,2],
                [0,5,0,0,0,2,6,8,0],
                [0,0,1,0,8,0,3,0,0],
            ],
        }
    },
    created () {
    },
    mounted() {
    },
    computed: {
        points() {
            return [...Array(9).keys()].map(i => ++i)
        },
        numberCells() {
            const cells = []
            for (const [rowIndex, row] of this.numbers.entries()) {
                for (const [colIndex, col] of row.entries()) {
                    cells.push({
                        id: rowIndex + ":" + colIndex,
                        num: col === 0 ? "" : "" + col,
                        is_hint: col !== 0,
                        x: colIndex * 40 + 10,
                        y: rowIndex * 40 + 30,
                    })
                }
            }
            return cells
        },
    },
    methods: {
    },
}
</script>

<style>

</style>
