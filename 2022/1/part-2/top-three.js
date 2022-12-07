const fs = require('node:fs')
const readline = require('node:readline')

const topThree = async() => {
    const fileStream = fs.createReadStream('../input.txt')

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity
    })

    let max = [0, 0, 0]
    let sum = 0

    for await (const line of rl) {
        if (line === "") {
            // is smaller than min
            if (sum <= max[0]) {
                sum = 0
                continue
            }

            // is bigger than max
            if (sum >= max[max.length-1]) {
                max.push(sum)
                max.shift()
                sum = 0
                continue
            }

            // is inbetween min and max in array
            for (let i = 0; i < max.length - 1; i++) {
                if (sum >= max[i] && sum <= max[i+1]) {
                    max.splice(i+1, 0, sum)
                    max.shift()
                    break
                }
            }
            
            sum = 0
        } else {
            if (!isNaN(line)) {
                sum += +line
            }
        }
    }

    // is bigger than max
    if (sum >= max[max.length-1]) {
        max.push(sum)
        max.shift()
    }

    // is inbetween min and max in array
    for (let i = 0; i < max.length - 1; i++) {
        if (sum >= max[i] && sum < max[i+1]) {
            max.splice(i+1, 0, sum)
            max.shift()
            break
        }
    }

    console.log(max.reduce((a, b) => a + b, 0))
}

topThree()