function res(res) {
    return { res };
}

function tail(f, ...args) {
    return { tail: true, f, args };
}

// TODO arity checks
function call(f, ...args) {
    let x = f(...args);
    while (x.tail) {
        x = x.f(...x.args);
    }
    return x.res;
}

const $nil = undefined;

// TODO move the definition of true and false to core.bao. They don't reference any JS builtins so
// they don't need to be here.
function $true(ifTrue, ifFalse) {
    return tail(ifTrue);
}

// TODO move the definition of true and false to core.bao. They don't reference any JS builtins so
// they don't need to be here.
function $false(ifTrue, ifFalse) {
    return tail(ifFalse);
}

function $eq(a, b) {
    return res(a === b ? $true : $false);
}

function $neq(a, b) {
    return res(a !== b ? $true : $false);
}

function $lt(a, b) {
    return res(a < b ? $true : $false);
}

function $gt(a, b) {
    return res(a > b ? $true : $false);
}

function $lte(a, b) {
    return res(a <= b ? $true : $false);
}

function $gte(a, b) {
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

function $print(x) {
    console.log(x);
    return res($nil);
}
