const fs = require('node:fs')
const readline = require('node:readline')

const overlaps = (section1, section2) => {
    const [min1, max1] = section1.split('-').map(i => parseInt(i))
    const [min2, max2] = section2.split('-').map(i => parseInt(i))

    if (max1 < min2 || min1 > max2) {
        return 0
    }
    return 1
}

const countOverlaps = async() => {
    const fileStream = fs.createReadStream('input.txt')

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity
    })

    let count = 0

    for await (const line of rl) {
        const [first, second] = line.split(',')

        count += overlaps(first, second)
    }

    console.log(count) 
    
}

countOverlaps()