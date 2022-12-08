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

const WINS = {
    "A": "Y",
    "B": "Z",
    "C": "X"
}

const calc_result_points = (elf, player) => {
    if (DECYPHER[elf] === player) return 3
    if (WINS[elf] === player) return 6
    return 0
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

        let [elf, player] = line.split(' ')

        round_points += SHAPE_POINTS[player]

        round_points += calc_result_points(elf, player)

        points += round_points
    }
    console.log(points)
}

pointCounting()