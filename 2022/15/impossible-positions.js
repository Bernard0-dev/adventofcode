const fs = require('node:fs')
const readline = require('node:readline')


const readInput = (line) => {
    let [sensor, beacon] = line.split(':')

    let sensorCoords = sensor.split('x=')[1].split('y=').map(coords => parseInt(coords))
    let beaconCoords = beacon.split('x=')[1].split('y=').map(coords => parseInt(coords))
    
    return [sensorCoords, beaconCoords]
}

const getPointsDistance = (p1, p2) => {
    const width = p1[0] - p2[0]
    const height = p1[1] - p2[1]

    return Math.abs(width) + Math.abs(height)
}

const combine = (a1, a2) => {
    if (a1[0] > a2[1] || a1[1] < a2[0]) return []

    return [Math.min(a1[0], a2[0]), Math.max(a1[1], a2[1])]
}

const mergePositionsArray = (positionsArray, position) => {
    // compare to impossible positions before pushing
    let overlap = []
    for (let i = 0; i < positionsArray.length; i++) {
        overlap = combine(positionsArray[i], position)

        if (overlap.length > 0) {
            positionsArray[i] = overlap
            break
        }
    }

    // no overlap with anything so push to array
    if (overlap.length === 0) {
        positionsArray.push(position)
    }
}

const calcImpossiblePositions = async() => {
    const fileStream = fs.createReadStream('input.txt')

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity
    })

    const line_number = 2000000 // 2M

    let impossiblePositionsArrays = []

    for await (const line of rl) {
        let [sensor, beacon] = readInput(line)

        // check distance from sensor to beacon
        let distance = getPointsDistance(sensor, beacon)

        // get how much distance is left after moving vertically
        // to get to line
        let distance_left = distance - (Math.abs(line_number - sensor[1]))

        // if < 0 then it's too far away and won't
        // help remove possible beacon placements
        if (distance_left < 0) continue

        // get range of X where there can't be any beacons
        let max_x = sensor[0] + distance_left
        let min_x = sensor[0] - distance_left

        let impossiblePositions = [min_x, max_x]

        mergePositionsArray(impossiblePositionsArrays, impossiblePositions)

    }

    // sort
    impossiblePositionsArrays.sort((a, b) => {
        if (a[0] > b[0]) return 1
        if (a[0] < b[0]) return -1
        return 0
    })

    // do a final combine
    const initial = impossiblePositionsArrays.shift()
    impossiblePositionsArrays = impossiblePositionsArrays.reduce((previous, current) => {
        mergePositionsArray(previous, current)

        return previous

    }, [initial])

    console.log(impossiblePositionsArrays)

    const numberOfImpossiblePositions = impossiblePositionsArrays.reduce((i, arr2) => {
        return i + (arr2[1] - arr2[0])
    }, 0)

    console.log(numberOfImpossiblePositions)

}

calcImpossiblePositions()