import {Navigation} from './navigation';
import {Output, TestCase} from './types';

const assert = require('node:assert');

const testCases: TestCase[] = [
  {
    title: 'Should pass basic case',
    input: {
      edges: [
        {from: 'A', to: 'B', distance: 30},
        {from: 'B', to: 'C', distance: 10},
      ],
      packages: [{name: 'K1', weight: 5, from: 'A', to: 'C'}],
      trains: [{name: 'Q1', capacity: 6, start: 'B'}],
    },
    expectedOutput: [
      {W: 0, T: 'Q1', N1: 'B', P1: [], N2: 'A', P2: []},
      {W: 30, T: 'Q1', N1: 'A', P1: ['K1'], N2: 'B', P2: []},
      {W: 60, T: 'Q1', N1: 'B', P1: [], N2: 'C', P2: ['K1']},
    ],
  },
  {
    title: 'Should handle invalid package starting location',
    input: {
      edges: [
        {from: 'A', to: 'B', distance: 30},
        {from: 'B', to: 'C', distance: 10},
      ],
      packages: [{name: 'K1', weight: 5, from: 'X', to: 'C'}],
      trains: [{name: 'Q1', capacity: 6, start: 'B'}],
    },
    expectedOutput: [],
  },
  {
    title: 'Should handle invalid package destination',
    input: {
      edges: [
        {from: 'A', to: 'B', distance: 30},
        {from: 'B', to: 'C', distance: 10},
      ],
      packages: [{name: 'K1', weight: 5, from: 'A', to: 'X'}],
      trains: [{name: 'Q1', capacity: 6, start: 'B'}],
    },
    expectedOutput: [],
  },
  {
    title: 'Should handle insufficient train capacity',
    input: {
      edges: [
        {from: 'A', to: 'B', distance: 30},
        {from: 'B', to: 'C', distance: 10},
      ],
      packages: [{name: 'K1', weight: 5, from: 'A', to: 'C'}],
      trains: [{name: 'Q1', capacity: 1, start: 'B'}],
    },
    expectedOutput: [],
  },
  {
    title: 'Should deliver multiple packages if the capacity is sufficient',
    input: {
      edges: [
        {from: 'A', to: 'B', distance: 30},
        {from: 'B', to: 'C', distance: 10},
      ],
      packages: [
        {name: 'K1', weight: 5, from: 'A', to: 'C'},
        {name: 'K2', weight: 5, from: 'A', to: 'C'},
        {name: 'K3', weight: 5, from: 'A', to: 'C'},
      ],
      trains: [
        {name: 'Q1', capacity: 5, start: 'B'},
        {name: 'Q2', capacity: 15, start: 'B'},
        {name: 'Q3', capacity: 5, start: 'B'},
      ],
    },
    expectedOutput: [
      {W: 0, T: 'Q2', N1: 'B', P1: [], N2: 'A', P2: []},
      {W: 30, T: 'Q2', N1: 'A', P1: ['K1', 'K2', 'K3'], N2: 'B', P2: []},
      {W: 60, T: 'Q2', N1: 'B', P1: [], N2: 'C', P2: ['K1', 'K2', 'K3']},
    ],
  },
  {
    title: 'Should utilize shortest route, even with more checkpoints',
    input: {
      edges: [
        // its faster to go from to B via C, and faster to go to D via both B and C
        {from: 'A', to: 'B', distance: 40},
        {from: 'A', to: 'C', distance: 10},
        {from: 'B', to: 'C', distance: 20},
        {from: 'B', to: 'D', distance: 10},
        {from: 'C', to: 'D', distance: 50},
      ],
      packages: [{name: 'K1', weight: 5, from: 'A', to: 'D'}],
      trains: [{name: 'Q1', capacity: 5, start: 'B'}],
    },
    expectedOutput: [
      {W: 0, T: 'Q1', N1: 'B', P1: [], N2: 'C', P2: []},
      {W: 20, T: 'Q1', N1: 'C', P1: [], N2: 'A', P2: []},
      {W: 30, T: 'Q1', N1: 'A', P1: ['K1'], N2: 'C', P2: []},
      {W: 40, T: 'Q1', N1: 'C', P1: [], N2: 'B', P2: []},
      {W: 60, T: 'Q1', N1: 'B', P1: [], N2: 'D', P2: ['K1']},
    ],
  },
  {
    title:
      'Should move multiple trains in parallel for fastest delivery (6-way crossroads shape)',
    input: {
      edges: [
        {from: 'A', to: 'X', distance: 10},
        {from: 'B', to: 'X', distance: 10},
        {from: 'C', to: 'X', distance: 10},
        {from: 'D', to: 'X', distance: 10},
        {from: 'E', to: 'X', distance: 10},
        {from: 'F', to: 'X', distance: 10},
      ],
      packages: [
        {name: 'K1', weight: 5, from: 'X', to: 'D'},
        {name: 'K2', weight: 5, from: 'X', to: 'E'},
        {name: 'K3', weight: 5, from: 'X', to: 'F'},
      ],
      trains: [
        {name: 'Q1', capacity: 15, start: 'A'},
        {name: 'Q2', capacity: 15, start: 'B'},
        {name: 'Q3', capacity: 15, start: 'C'},
      ],
    },
    expectedOutput: [
      {W: 0, T: 'Q1', N1: 'A', P1: [], N2: 'X', P2: []},
      {W: 10, T: 'Q1', N1: 'X', P1: ['K1'], N2: 'D', P2: ['K1']},
      {W: 0, T: 'Q2', N1: 'B', P1: [], N2: 'X', P2: []},
      {W: 10, T: 'Q2', N1: 'X', P1: ['K2'], N2: 'E', P2: ['K2']},
      {W: 0, T: 'Q3', N1: 'C', P1: [], N2: 'X', P2: []},
      {W: 10, T: 'Q3', N1: 'X', P1: ['K3'], N2: 'F', P2: ['K3']},
    ],
  },
  {
    title:
      'Should move only nearby train for fastest delivery if the other train is too far away (4-way crossroads shape)',
    input: {
      edges: [
        {from: 'A', to: 'X', distance: 50},
        {from: 'B', to: 'X', distance: 10},
        {from: 'C', to: 'X', distance: 10},
        {from: 'D', to: 'X', distance: 10},
      ],
      packages: [
        {name: 'K1', weight: 5, from: 'X', to: 'C'},
        {name: 'K2', weight: 5, from: 'X', to: 'D'},
      ],
      trains: [
        {name: 'Q1', capacity: 5, start: 'A'},
        {name: 'Q2', capacity: 5, start: 'B'},
      ],
    },
    expectedOutput: [
      {W: 0, T: 'Q2', N1: 'B', P1: [], N2: 'X', P2: []},
      {W: 10, T: 'Q2', N1: 'X', P1: ['K1'], N2: 'C', P2: ['K1']},
      {W: 20, T: 'Q2', N1: 'C', P1: [], N2: 'X', P2: []},
      {W: 30, T: 'Q2', N1: 'X', P1: ['K2'], N2: 'D', P2: ['K2']},
    ],
  },
  {
    title:
      'Should pick up multiple packages in multiple trains in parallel using shortest route',
    input: {
      edges: [
        {from: 'A', to: 'B', distance: 30},
        {from: 'B', to: 'G', distance: 30},
        {from: 'G', to: 'H', distance: 20},
        {from: 'H', to: 'B', distance: 20},
        {from: 'B', to: 'C', distance: 100},
        {from: 'C', to: 'D', distance: 30},
        {from: 'D', to: 'E', distance: 30},
        {from: 'C', to: 'F', distance: 50},
        {from: 'F', to: 'E', distance: 20},
      ],
      packages: [
        {name: 'K1', weight: 5, from: 'A', to: 'G'},
        {name: 'K2', weight: 5, from: 'A', to: 'H'},
        {name: 'K3', weight: 5, from: 'B', to: 'H'},
        {name: 'K4', weight: 5, from: 'H', to: 'E'},
        {name: 'K5', weight: 5, from: 'E', to: 'A'},
        {name: 'K6', weight: 5, from: 'F', to: 'C'},
        {name: 'K7', weight: 5, from: 'F', to: 'G'},
      ],
      trains: [
        {name: 'Q1', capacity: 20, start: 'B'},
        {name: 'Q2', capacity: 20, start: 'C'},
      ],
    },
    expectedOutput: [
      {W: 0, T: 'Q1', N1: 'B', P1: [], N2: 'A', P2: []},
      {W: 30, T: 'Q1', N1: 'A', P1: ['K1', 'K2'], N2: 'B', P2: []},
      {W: 60, T: 'Q1', N1: 'B', P1: ['K3'], N2: 'H', P2: ['K2', 'K3']},
      {W: 80, T: 'Q1', N1: 'H', P1: ['K4'], N2: 'G', P2: ['K1']},
      {W: 100, T: 'Q1', N1: 'G', P1: [], N2: 'B', P2: []},
      {W: 130, T: 'Q1', N1: 'B', P1: [], N2: 'C', P2: []},
      {W: 230, T: 'Q1', N1: 'C', P1: [], N2: 'D', P2: []},
      {W: 290, T: 'Q1', N1: 'D', P1: [], N2: 'E', P2: ['K4']},
      {W: 0, T: 'Q2', N1: 'C', P1: [], N2: 'D', P2: []},
      {W: 30, T: 'Q2', N1: 'D', P1: [], N2: 'E', P2: []},
      {W: 60, T: 'Q2', N1: 'E', P1: ['K5'], N2: 'F', P2: []},
      {W: 80, T: 'Q2', N1: 'F', P1: ['K6', 'K7'], N2: 'C', P2: ['K6']},
      {W: 130, T: 'Q2', N1: 'C', P1: [], N2: 'B', P2: []},
      {W: 230, T: 'Q2', N1: 'B', P1: [], N2: 'A', P2: ['K5']},
      {W: 260, T: 'Q2', N1: 'A', P1: [], N2: 'B', P2: []},
      {W: 290, T: 'Q2', N1: 'B', P1: [], N2: 'G', P2: ['K7']},
    ],
  },
];

function test() {
  let i = 0;
  // for (const testCase of [testCases[8]]) {
  for (const testCase of testCases) {
    let solution: Output | null = null;

    try {
      console.log(`Running test case ${i}: ${testCase.title}`);
      console.time('Time taken');
      const nav = new Navigation(testCase.input);
      solution = nav.solve();
      assert.deepEqual(solution, testCase.expectedOutput, testCase.title);
      console.timeEnd('Time taken');
      console.log('Success!');
    } catch (e) {
      console.error('Failed!');
      console.log('Expected:', testCase.expectedOutput);
      console.log('Received:', solution);
    }

    i++;
  }
}

test();
