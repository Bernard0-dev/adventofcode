const fs = require('node:fs')
const readline = require('node:readline')

const countCalories = async() => {
    const fileStream = fs.createReadStream('input.txt')

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity
    })

    let max = 0
    let sum = 0

    for await (const line of rl) {
        if (line === "") {
            if (sum > max) {
                max = sum
            }
            sum = 0
        } else {
            if (!isNaN(line)) {
                sum += +line
            }
        }
    }
    if (sum > max) {
        max = sum
    }

    console.log(max) 
}

countCalories()