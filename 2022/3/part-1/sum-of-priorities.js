const fs = require('node:fs')
const readline = require('node:readline')


const UPPERCASE_OFFSET = 38
const LOWERCASE_OFFSET = 96

const convertCharToPriority = (character) => {
    const charCode = character.charCodeAt()

    if (charCode > 90) return charCode - LOWERCASE_OFFSET
    return charCode - UPPERCASE_OFFSET
}

const getDuplicate = (set, string) => {
    const size = set.size
    for (let c of string) {
        set.add(c)
        if (set.size === size) {
            return c
        } else {
            set.delete(c)
        }
    }
}

const sumPriorities = async() => {
    const fileStream = fs.createReadStream('../input.txt')

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity
    })

    let sum = 0

    for await (const line of rl) {
        const first = line.slice(0, line.length/2)
        const second = line.slice(line.length/2)

        const set = new Set(first.split(''))

        let duplicate = getDuplicate(set, second)

        const priority = convertCharToPriority(duplicate)

        sum += priority
    }
    console.log(sum)
}

sumPriorities()