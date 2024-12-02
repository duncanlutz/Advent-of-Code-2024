"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __asyncValues = (this && this.__asyncValues) || function (o) {
    if (!Symbol.asyncIterator) throw new TypeError("Symbol.asyncIterator is not defined.");
    var m = o[Symbol.asyncIterator], i;
    return m ? m.call(o) : (o = typeof __values === "function" ? __values(o) : o[Symbol.iterator](), i = {}, verb("next"), verb("throw"), verb("return"), i[Symbol.asyncIterator] = function () { return this; }, i);
    function verb(n) { i[n] = o[n] && function (v) { return new Promise(function (resolve, reject) { v = o[n](v), settle(resolve, reject, v.done, v.value); }); }; }
    function settle(resolve, reject, d, v) { Promise.resolve(v).then(function(v) { resolve({ value: v, done: d }); }, reject); }
};
Object.defineProperty(exports, "__esModule", { value: true });
const fs_1 = require("fs");
const readline_1 = require("readline");
function getLists() {
    var _a, e_1, _b, _c;
    return __awaiter(this, void 0, void 0, function* () {
        const stream = (0, fs_1.createReadStream)('../input');
        const rl = (0, readline_1.createInterface)({
            input: stream,
            crlfDelay: Infinity
        });
        let left = [];
        let right = [];
        try {
            for (var _d = true, rl_1 = __asyncValues(rl), rl_1_1; rl_1_1 = yield rl_1.next(), _a = rl_1_1.done, !_a;) {
                _c = rl_1_1.value;
                _d = false;
                try {
                    const line = _c;
                    const split = line.replace(/\s{1,}/, " ").split(" ");
                    left.push(parseInt(split[0]));
                    right.push(parseInt(split[1]));
                }
                finally {
                    _d = true;
                }
            }
        }
        catch (e_1_1) { e_1 = { error: e_1_1 }; }
        finally {
            try {
                if (!_d && !_a && (_b = rl_1.return)) yield _b.call(rl_1);
            }
            finally { if (e_1) throw e_1.error; }
        }
        left.sort();
        right.sort();
        return [left, right];
    });
}
function partOne(left, right) {
    let result = 0;
    for (let i = 0; i < left.length; i++) {
        const leftValue = left[i];
        const rightValue = right[i];
        result += Math.abs(leftValue - rightValue);
    }
    console.log(`Part one result: ${result}`);
}
function partTwo(left, right) {
    let result = 0;
    for (const leftValue of left) {
        let rightCount = 0;
        let foundRightValue = false;
        innerLoop: for (const rightValue of right) {
            if (rightValue === leftValue) {
                rightCount += 1;
                if (!foundRightValue) {
                    foundRightValue = true;
                }
            }
            else {
                if (foundRightValue) {
                    break innerLoop;
                }
            }
        }
        result += rightCount * leftValue;
    }
    console.log(`Part two result: ${result}`);
}
(() => __awaiter(void 0, void 0, void 0, function* () {
    const [left, right] = yield getLists();
    partOne(left, right);
    partTwo(left, right);
}))();
