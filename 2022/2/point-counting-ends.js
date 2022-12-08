
const fs = require('node:fs')
const readline = require('node:readline')

const DECYPHER = {
    "A": "X",
    "B": "Y",
    "C": "Z",
}

const SHAPE_POINTS = {
    "X": 1,
    "Y": 2,
    "Z": 3,
}

const RESULT_DIRECTIONS = {
    "X": -1,
    "Y": 0,
    "Z": 1,
}

const RESULT_POINTS = {
    "X": 0,
    "Y": 3,
    "Z": 6,
}

const shapes = ["X", "Y", "Z"]

const mod = (n, m) => {
    return ((n % m) + m) % m;
}

const shape_points = (elf, result) => {
    const idx = shapes.indexOf(DECYPHER[elf])
    const player_idx = mod((idx + RESULT_DIRECTIONS[result]), shapes.length)

    return SHAPE_POINTS[shapes[player_idx]]
}

const pointCounting = async() => {
    const fileStream = fs.createReadStream('input.txt')

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity
    })


    let points = 0

    for await (const line of rl) {
        let round_points = 0

        let [elf, result] = line.split(' ')

        round_points += shape_points(elf, result)

        round_points += RESULT_POINTS[result]

        points += round_points
    }
    console.log(points)
}

pointCounting()