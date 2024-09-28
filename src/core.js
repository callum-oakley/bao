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

// Runtime

function tailCall(f, ...args) {
    return { tailCall: true, f, args };
}

function call(f, ...args) {
    let x = f(...args);
    while (x.tailCall) {
        x = x.f(...x.args);
    }
    return x;
}

async function readStdin() {
    // TODO
    return $nil;
}

async function writeStdout(output) {
    const bytes = [];
    call(function go(output) {
        return tailCall(
            output,
            function () {
                return $nil;
            },
            function (head, tail) {
                bytes.push(Number(head));
                return tailCall(go, tail);
            },
        );
    }, output);
    await Deno.stdout.writable.getWriter().write(new Uint8Array(bytes));
}

await writeStdout(call($main, await readStdin()));
