#! /usr/bin/env node

var input = "L5, R1, R4, L5, L4, R3, R1, L1, R4, R5, L1, L3, R4, L2, L4, R2, L4, L1, R3, R1, R1, L1, R1, L5, R5, R2, L5, R2, R1, L2, L4, L4, R191, R2, R5, R1, L1, L2, R5, L2, L3, R4, L1, L1, R1, R50, L1, R1, R76, R5, R4, R2, L5, L3, L5, R2, R1, L1, R2, L3, R4, R2, L1, L1, R4, L1, L1, R185, R1, L5, L4, L5, L3, R2, R3, R1, L5, R1, L3, L2, L2, R5, L1, L1, L3, R1, R4, L2, L1, L1, L3, L4, R5, L2, R3, R5, R1, L4, R5, L3, R3, R3, R1, R1, R5, R2, L2, R5, L5, L4, R4, R3, R5, R1, L3, R1, L2, L2, R3, R4, L1, R4, L1, R4, R3, L1, L4, L1, L5, L2, R2, L1, R1, L5, L3, R4, L1, R5, L5, L5, L1, L3, R1, R5, L2, L4, L5, L1, L1, L2, R5, R5, L4, R3, L2, L1, L3, L4, L5, L5, L2, R4, R3, L5, R4, R2, R1, L5"

// var input = "R2, L3";
// var input = "R2, R2, R2";
// var input = "R5, L5, R5, R3"

var headings = [[0,1], [1,0], [0,-1], [-1,0]];
var heading = 0;
var pos = [0, 0];

function handleTurn(turn) {
  let dir = turn[0];
  let length = +turn.substr(1);
  // console.log(dir, length);
  if (dir === 'R') {
    heading = (heading + 1) % headings.length;
  } else {
    heading = (heading + headings.length - 1) % headings.length;
  }
  length = headings[heading].map(x => x * length);
  pos = pos.map((x, i) => x + length[i]);
  // console.log(heading, length, pos)
}

let data = input.split(', ');
// console.log('data:', data);
// console.log();
for (let index in data) {
  let turn = data[index];
  handleTurn(turn);
}
// console.log();
let distance = pos.reduce((i, j) => i + j);
console.log(distance, Math.abs(distance));
