function res(res) {
    return { res };
}

function tail(f, ...args) {
    return { tail: true, f, args };
}

function call(f, ...args) {
    let x = f(...args);
    while (x.tail) {
        x = x.f(...x.args);
    }
    return x.res;
}

const $nil = undefined;

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

function $print$E(x) {
    console.log(x);
    return res($nil);
}
