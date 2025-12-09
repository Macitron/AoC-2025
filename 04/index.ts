const infile = Bun.argv[2];
if (!infile) {
    console.log("no input file provided");
    process.exit(1);
}
const lines = (await Bun.file(infile).text()).split('\n').map((line) => line.split(''));

const part1_ans = part1(lines);
console.log(`Part 1: ${part1_ans} rolls can be accessed`)

function part1 (rows: string[][]): number {
    let crowded_rolls = 0;
    rows.forEach((row, row_idx) => {
        row.forEach((_, col_idx) => {
            if (isCrowded(rows, row_idx, col_idx)) {
                crowded_rolls += 1;
            }
        })
    })
    
    return crowded_rolls;
}

function isCrowded(rows: string[][], row_idx: number, col_idx: number): boolean {
    let c = rows.at(row_idx)?.at(col_idx) ?? '.';
    if (c !== '@') {
        return false;
    }
    
    let rolls = 0;
    for (let vert_offset of [-1, 0, 1]) {
        for (let horiz_offset of [-1, 0, 1]) {
            if (vert_offset === 0 && horiz_offset === 0) {
                continue;
            }
            let offset_char = rows.at(row_idx + vert_offset)?.at(col_idx + horiz_offset) ?? '.';
            if (offset_char === '@') {
                rolls += 1;
            }
        }
    }
    
    return rolls < 4;
}
