using Xunit;

namespace AOC2019.Test
{
    public class Day3Tests
    {
        [Theory]
        [InlineData(new[] { "R8,U5,L5,D3", "U7,R6,D4,L4" }, 6)]
        [InlineData(new[] { "R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83" }, 159)]
        [InlineData(new[] { "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7" }, 135)]
        public void FindClosestIntersectionTest(string[] input, int expected)
        {
            var actual = Day3.CalculateMinDistance(input);
            Assert.Equal(expected, actual);
        }

        [Theory]
        [InlineData(new[] { "R8,U5,L5,D3", "U7,R6,D4,L4" }, 30)]
        [InlineData(new[] { "R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83" }, 610)]
        [InlineData(new[] { "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7" }, 410)]
        public void FindLeastStepsTest(string[] input, int expected)
        {
            var actual = Day3.CalculateMinSteps(input);
            Assert.Equal(expected, actual);
        }
    }
}
