import { createReadStream } from "fs";
import { createInterface } from "readline";


async function getLists(): Promise<[number[], number[]]> {
    const stream = createReadStream('../input');

    const rl = createInterface({
        input: stream,
        crlfDelay: Infinity
    });

    let left = [];
    let right = [];

    for await (const line of rl) {
        const split = line.replace(/\s{1,}/, " ").split(" ");
        left.push(parseInt(split[0]))
        right.push(parseInt(split[1]))
    }

    left.sort();
    right.sort();

    return [left, right];
}

function partOne(left: number[], right: number[]) {
    let result = 0;

    for (let i = 0; i < left.length; i++) {
        const leftValue = left[i];
        const rightValue = right[i];

        result += Math.abs(leftValue - rightValue);
    }

    console.log(`Part one result: ${result}`);
}

function partTwo(left: number[], right: number[]) {
    let result = 0;

    for (const leftValue of left) {
        let rightCount = 0;
        let foundRightValue = false;

        innerLoop:
        for (const rightValue of right) {
            if (rightValue === leftValue) {
                rightCount += 1;
                if (!foundRightValue) {
                    foundRightValue = true;
                }
            } else {
                if (foundRightValue) {
                    break innerLoop;
                }
            }
        }

        result += rightCount * leftValue;
    }

    console.log(`Part two result: ${result}`);
}


(async () => {
    const [left, right] = await getLists();
    partOne(left, right);
    partTwo(left, right);
})();
