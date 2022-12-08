const fs = require('node:fs')
const readline = require('node:readline')

const GROUP_SIZE = 3

const UPPERCASE_OFFSET = 38
const LOWERCASE_OFFSET = 96

const convertCharToPriority = (character) => {
    const charCode = character.charCodeAt()

    if (charCode > 90) return charCode - LOWERCASE_OFFSET
    return charCode - UPPERCASE_OFFSET
}

const getDuplicates = (set, string) => {
    const size = set.size
    let duplicates = new Set()
    for (let c of string) {
        set.add(c)
        if (set.size === size) {
            duplicates.add(c)
        } else {
            set.delete(c)
        }
    }
    return duplicates
}

const sumPriorities = async() => {
    const fileStream = fs.createReadStream('input.txt')

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity
    })

    let sum = 0

    let line_num = 1
    let line_array = []

    for await (let line of rl) {
        if (line_num === GROUP_SIZE) {
            let duplicates = new Set(line.split(''))
            for (let string of line_array) {
                duplicates = getDuplicates(duplicates, string)
            }

            let char = duplicates.values().next().value
            sum += convertCharToPriority(char)

            line_num = 1
            line_array = []
            continue
        }

        line_array.push(line)

        line_num += 1
    }
    console.log(sum)
}

sumPriorities()