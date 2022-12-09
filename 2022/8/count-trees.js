const fs = require('node:fs')
const readline = require('node:readline')


const countEdges = (matrix) => {
    const width = matrix[0].length
    const height = matrix.length

    return (width * 2) + (height* 2) - 4
}

const countTrees = async() => {
    const fileStream = fs.createReadStream('input.txt')

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity
    })

    let count = 0

    let matrix = []

    let trees_positions = new Set()

    for await (const line of rl) {
       matrix.push(line.split('')) 
    }

    count += countEdges(matrix)

    let minTopToBottom = matrix[0]
    let minBottomToTop = matrix[matrix.length-1]

    for (let row = 1; row < matrix.length-1; row++) {
        let minLeftToRight = matrix[row][0]
        let minRightToLeft = matrix[row][matrix[row].length - 1]
        for (let col = 1; col < matrix[row].length-1; col++) {
            const possibleTree = matrix[row][col]
            // left to right
            if (possibleTree > minLeftToRight) {
                minLeftToRight = possibleTree
                trees_positions.add(`${col},${row}`)
            }

            // top to bottom
            if (possibleTree > minTopToBottom[col]) {
                minTopToBottom[col] = possibleTree
                trees_positions.add(`${col},${row}`)
            }

        }

        for (let col = matrix[row].length-2; col > 0; col--) {
            const possibleTree = matrix[row][col]
            // right to left
            if (possibleTree > minRightToLeft) {
                minRightToLeft = possibleTree
                trees_positions.add(`${col},${row}`)
            }
        }
    }

    for (let row = matrix.length-2; row > 0; row--) {
        for (let col = 1; col < matrix[row].length-1; col++) {
            const possibleTree = matrix[row][col]
            if (possibleTree > minBottomToTop[col]) {
                minBottomToTop[col] = possibleTree
                trees_positions.add(`${col},${row}`)
            }
        }
    }

    count += trees_positions.size

    console.log(count) 
    
}

countTrees()