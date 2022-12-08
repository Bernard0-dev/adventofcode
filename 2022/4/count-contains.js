const fs = require('node:fs')
const readline = require('node:readline')

const contains = (section1, section2) => {
    const [min1, max1] = section1.split('-').map(i => parseInt(i))
    const [min2, max2] = section2.split('-').map(i => parseInt(i))

    if (min2 >= min1 && max2 <= max1) return 1
    return 0
}

const isBiggerArea = (section1, section2) => {
    const [min1, max1] = section1.split('-')
    const [min2, max2] = section2.split('-')

    const size1 = max1 - min1
    const size2 = max2 - min2

    if (size1 > size2) return 1
    if (size2 > size1) return -1
    return 0
}

const countSections = async() => {
    const fileStream = fs.createReadStream('input.txt')

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity
    })

    let count = 0

    for await (const line of rl) {
        const [first, second] = line.split(',')

        if (first === second) {
            count += 1 
            continue
        }
        
        const firstIsBigger = isBiggerArea(first, second)

        if (firstIsBigger === 1) {
            count += contains(first, second)
            continue
        }
        if (firstIsBigger === -1) {
            count += contains(second, first)
        }
    }

    console.log(count) 
    
}

countSections()