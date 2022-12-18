const fs = require('node:fs')
const readline = require('node:readline')


// TODO divide processing into worker threads

const [min, max] = [0, 4000000]

const getTuningFrequency = (point) => {
    return (4000000 * point[0]) + point[1]
}

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
    if (a1[0] > a2[1] + 1 || a1[1] < a2[0] - 1) return []

    return [Math.min(a1[0], a2[0]), Math.max(a1[1], a2[1])]
}

const mergePositionsArray = (positionsArray, position) => {
    if (!positionsArray) {
        return [position]
    }

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
    return positionsArray
}

const arrayContainsGrid = (arr) => {
    return arr[0] <= min && arr[1] >= max
}

const getOpenBeaconPosition = (arr) => {
    if (arr.length > 1) {
        console.log(arr[0][1])
        return arr[0][1] + 1
    }

    if (arr.length === 1) {
        if (arr[0] > min) {
            return min
        }
        if (arr[1] < max) {
            return max
        }
    }
    return -1
}


const calcBeaconFrequency = async() => {
    console.time('time')
    const fileStream = fs.createReadStream('input.txt')

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity
    })

    let impossiblePositionsArrays = []

    for await (const line of rl) {
        let [sensor, beacon] = readInput(line)

        // check distance from sensor to beacon
        let distance = getPointsDistance(sensor, beacon)

        for (let line_number = min; line_number <= max; line_number++) {
            // get how much distance is left after moving vertically
            // to get to line
            // TODO can be improved by just using the cycle
            // -- until 0, then ++
            let distance_left = distance - (Math.abs(line_number - sensor[1]))

            // if < 0 then it's too far away and won't
            // help remove possible beacon placements
            if (distance_left < 0) continue

            // get range of X where there can't be any beacons
            let max_x = sensor[0] + distance_left
            let min_x = sensor[0] - distance_left

            let impossiblePositions = [min_x, max_x]

            impossiblePositionsArrays[line_number] = mergePositionsArray(impossiblePositionsArrays[line_number], impossiblePositions)
        }
    }

    let possibleBeaconPositions = []

    // iterate through all the lines
    for (let i = 0; i < impossiblePositionsArrays.length; i++) {
        if (impossiblePositionsArrays[i].length === 1) {
            continue
        }

        // sort
        impossiblePositionsArrays[i].sort((a, b) => {
            if (a[0] > b[0]) return 1
            if (a[0] < b[0]) return -1
            return 0
        })

        // do a final combine
        const initial = impossiblePositionsArrays[i].shift()
        impossiblePositionsArrays[i] = impossiblePositionsArrays[i].reduce((previous, current) => {
            return mergePositionsArray(previous, current)
        }, [initial])

        let openBeaconPosition = getOpenBeaconPosition(impossiblePositionsArrays[i])
        // check for possible beacon positions
        if (openBeaconPosition >= 0) {
            possibleBeaconPositions.push([openBeaconPosition, i])
        }
    }

    console.log(possibleBeaconPositions)

    const tuningFrequency = getTuningFrequency(possibleBeaconPositions[0])

    console.log(tuningFrequency)

    console.timeEnd('time')
}

calcBeaconFrequency()