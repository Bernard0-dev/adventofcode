const fs = require('node:fs')
const readline = require('node:readline')


const DIRECTIONS = {
    'U': [0, 1],
    'R': [1, 0],
    'D': [0, -1],
    'L': [-1, 0],
}

const sumOfPoints = (p1, p2) => {
    return [p1[0] + p2[0], p1[1] + p2[1]]
}

const subtractionOfPoints = (p1, p2) => {
    return [p1[0] - p2[0], p1[1] - p2[1]]
}

const distanceBetweenPoints = (p1, p2) => {
    const positiveDifference = subtractionOfPoints(p1, p2).map(s => Math.abs(s))
    return Math.max(...positiveDifference)
}

const calcTailMoves = async() => {
    const fileStream = fs.createReadStream('input.txt')

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity
    })

    let tailPositions = [[0, 0]]

    let headPosition = [0, 0]

    for await (const line of rl) {
        let [direction, times] = line.split(' ')

        for (let i = 0; i < times; i++) {
            const newHeadPosition = sumOfPoints(headPosition, DIRECTIONS[direction])

            if (distanceBetweenPoints(newHeadPosition, tailPositions[tailPositions.length - 1]) > 1) {
                tailPositions.push(headPosition)
            }

            headPosition = newHeadPosition
        }
    }

    final = new Set()
    tailPositions.map(position => final.add(position.toString()))

    console.log(final.size)
}

calcTailMoves()