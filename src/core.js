// Unary integer functions

function $inc(a) {
    return res(a + 1n);
}

function $dec(a) {
    return res(a - 1n);
}

function $neg(a) {
    return res(-a);
}

function $zero$Q(a) {
    return res(a === 0n ? $true : $false);
}

function $pos$Q(a) {
    return res(a > 0n ? $true : $false);
}

function $neg$Q(a) {
    return res(a < 0n ? $true : $false);
}

// Binary integer functions

function $eq$Q(a, b) {
    return res(a === b ? $true : $false);
}

function $neq$Q(a, b) {
    return res(a !== b ? $true : $false);
}

function $lt$Q(a, b) {
    return res(a < b ? $true : $false);
}

function $gt$Q(a, b) {
    return res(a > b ? $true : $false);
}

function $lte$Q(a, b) {
    return res(a <= b ? $true : $false);
}

function $gte$Q(a, b) {
    return res(a >= b ? $true : $false);
}

function $add(a, b) {
    return res(a + b);
}

function $sub(a, b) {
    return res(a - b);
}

function $mul(a, b) {
    return res(a * b);
}

function $div(a, b) {
    return res(a / b);
}

function $rem(a, b) {
    return res(a % b);
}

function $exp(a, b) {
    return res(a ** b);
}

// Runtime

function res(res) {
    return { res };
}

function tail(f, ...args) {
    return { f, args };
}

function call(f, ...args) {
    let x = f(...args);
    while (x.f) {
        x = x.f(...x.args);
    }
    return x.res;
}

async function readStdin() {
    // TODO
    return res($nil);
}

async function writeStdout(output) {
    const bytes = [];
    call(function go(output) {
        return tail(
            output,
            function () {
                return res($nil);
            },
            function (first, rest) {
                bytes.push(Number(first));
                return tail(go, rest);
            },
        );
    }, output);
    await Deno.stdout.writable.getWriter().write(new Uint8Array(bytes));
}

await writeStdout(call($main, await readStdin()));
