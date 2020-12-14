﻿using AOC2020;
using AOC2020.Helpers;
using NUnit.Framework;
using System;

namespace test
{
    [TestFixture]
    public class Day03Tests
    {
        private const string input = @"..##.........##.........##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#";

        [Test]
        public void Puzzle1()
            => Assert.AreEqual(7, Day03.CountTrees(input.Split(Environment.NewLine), (0, 0), (3, 1)));

        [Test]
        public void Puzzle2()
            => Assert.AreEqual(336, Day03.CountTrees(input.Split(Environment.NewLine), (0, 0), new Point[] { (1, 1), (3, 1), (5, 1), (7, 1), (1, 2) }));
    }
}
