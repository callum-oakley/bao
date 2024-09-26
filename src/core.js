// IO

const stdinReader = Deno.stdin.readable.getReader();
const stdoutWriter = Deno.stdout.writable.getWriter();
const stderrWriter = Deno.stderr.writable.getWriter();

let stdinChunk;

async function $read$E() {
    if (!stdinChunk || stdinChunk.length == 0) {
        const res = await stdinReader.read();
        if (res.done) {
            return $nil;
        }
        stdinChunk = res.value;
    }
    const c = stdinChunk[0];
    stdinChunk = stdinChunk.subarray(1);
    return await $some(c);
}

async function $write$E(c) {
    await stdoutWriter.write(new Uint8Array([c]));
}

async function $ewrite$E(c) {
    await stderrWriter.write(new Uint8Array(c));
}

// Unary integer functions

function $inc(a) {
    return a + 1n;
}

function $dec(a) {
    return a - 1n;
}

function $neg(a) {
    return -a;
}

function $zero$Q(a) {
    return a === 0n ? $true : $false;
}

function $pos$Q(a) {
    return a > 0n ? $true : $false;
}

function $neg$Q(a) {
    return a < 0n ? $true : $false;
}

// Binary integer functions

function $eq$Q(a, b) {
    return a === b ? $true : $false;
}

function $neq$Q(a, b) {
    return a !== b ? $true : $false;
}

function $lt$Q(a, b) {
    return a < b ? $true : $false;
}

function $gt$Q(a, b) {
    return a > b ? $true : $false;
}

function $lte$Q(a, b) {
    return a <= b ? $true : $false;
}

function $gte$Q(a, b) {
    return a >= b ? $true : $false;
}

function $add(a, b) {
    return a + b;
}

function $sub(a, b) {
    return a - b;
}

function $mul(a, b) {
    return a * b;
}

function $div(a, b) {
    return a / b;
}

function $rem(a, b) {
    return a % b;
}

function $exp(a, b) {
    return a ** b;
}
