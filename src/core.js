import { bold, red } from "jsr:@std/fmt@^1.0.2/colors";

function res(res) {
    return { res };
}

function tail(f, ...args) {
    return { tail: true, f, args };
}

function call(f, ...args) {
    try {
        let x = f(...args);
        while (x.tail) {
            x = x.f(...x.args);
        }
        return x.res;
    } catch {
        // TODO expose some way to do user defined errors? Maybe ewrite! and exit! functions?
        console.error(`${bold(red("error"))}`);
        Deno.exit(1);
    }
}

// TODO read!

// TODO replace with write!
function $print$E(x) {
    console.log(x);
    return res($nil);
}

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
