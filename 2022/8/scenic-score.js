const fs = require('node:fs')
const readline = require('node:readline')


const getScore = (height, index, array) => {
    let scoreNeg = 0
    let scorePos = 0

    for (let i = index - 1; i >= 0; i--) {
        scoreNeg += 1
        if (array[i] >= height) break
    }

    for (let i = index + 1; i < array.length; i++) {
        scorePos += 1
        if (array[i] >= height) break
    }

    return scoreNeg * scorePos
}

const bestScenicScore = async() => {
    const fileStream = fs.createReadStream('input.txt')

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity
    })

    let matrix = []

    for await (const line of rl) {
       matrix.push(line.split('')) 
    }

    let score = 0

    for (let row = 1; row < matrix.length - 1; row++) {
        for (let col = 1; col < matrix[row].length - 1; col++) {
            const treeHeight = matrix[row][col]

            const m_column = matrix.map(value => value[col])
            const m_row = matrix[row]

            const verticalScore = getScore(treeHeight, row, m_column)
            const horizontalScore = getScore(treeHeight, col, m_row)

            const treeScore = verticalScore * horizontalScore

            if (treeScore > score) score = treeScore
        }
    }

    console.log(score)

}

bestScenicScore()