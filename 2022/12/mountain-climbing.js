const fs = require('node:fs')
const readline = require('node:readline')


const COORDS = [
    [0, 1],
    [1, 0],
    [0, -1],
    [-1, 0]
]

const sumOfPoints = (p1, p2) => {
    return [p1[0] + p2[0], p1[1] + p2[1]]
}

const getAdjacents = (position, matrix) => {
    let {x, y} = position

    let possibleCoords = COORDS.map(coord => sumOfPoints(coord, [x, y]))

    let adjacent = matrix
                    .filter(pos => JSON
                                    .stringify(possibleCoords)
                                    .includes(JSON.stringify([pos.x, pos.y])
                            )
                    )

    return adjacent
}

const getPossiblePositions = (adjacents, position, visited) => {
    return adjacents
            .filter(adj => adj.value <= position.value + 1)
            .filter(adj => !visited.includes(adj))
}

const concatUniqueItems = (arr1, arr2) => {
    for (let item of arr2) {
        if (!JSON.stringify(arr1).includes(JSON.stringify(item))) {
            arr1.push(item)
        }
    }
    return arr1
}


const calcMinSteps = async() => {
    const fileStream = fs.createReadStream('input.txt')

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity
    })

    let matrix = []

    let row = 0
    let objective = {}

    for await (const line of rl) {
        let chars = line.split('')
        chars.map((char, idx) => {
            if (char === 'S') {
                matrix.push({'x': idx, 'y': row, 'value': 96})
            } else if (char === 'E') {
                objective = {'x': idx, 'y': row, 'value': 123}
                matrix.push(objective)
            } else {
                matrix.push({'x': idx, 'y': row, 'value': char.charCodeAt()})
            }
        })
        row++
    }

    let steps = 0
    let visited = []

    let start = matrix[0]
    let possiblePositions = [start]

    visited.push(start)

    while (!possiblePositions.includes(objective)) {
        let newPossiblePositions = []

        for (let pos of possiblePositions) {
            let adjacents = getAdjacents(pos, matrix)
            let positions = getPossiblePositions(adjacents, pos, visited)
            newPossiblePositions = concatUniqueItems(newPossiblePositions, positions)
        }

        visited = visited.concat(possiblePositions)
        possiblePositions = newPossiblePositions
        steps++
    }

    console.log(steps)
}

calcMinSteps()