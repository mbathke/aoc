function find_factor(num: number, numbers: string[]): number {
  return numbers.filter(n => Number.parseInt(n, 10) === num).length;
}

async function main() {
  const filepath = Deno.args[1];

  if (!filepath) {
    console.log(`Usage: ${Deno.args[0]} <path/to/input/file>`);
  }

  const text = await Deno.readTextFile(filepath);
  const lines = text.split("\n");
  lines.pop();

  const leftNumbers = [];
  const rightNumbers = [];

  for (const line of lines) {
    const split = line.split("   ");
    leftNumbers.push(split[0]);
    rightNumbers.push(split[1]);
  }

  leftNumbers.sort();
  rightNumbers.sort();

  let totalDistance = 0;
  let totalSimilarityScore = 0;

  for (let i = 0; i < leftNumbers.length; i++) {
    const leftNum = Number.parseInt(leftNumbers[i]);
    const rightNum = Number.parseInt(rightNumbers[i]);

    console.log(
      `Index: ${i}: left number: ${leftNum}; right number ${
        rightNum
      }`,
    );

    const dist = Math.abs(leftNum - rightNum);
    totalDistance += dist;

    const simScore = leftNum * find_factor(leftNum, rightNumbers);
    totalSimilarityScore += simScore;
  }

  console.log(`The total distance of all fields is ${totalDistance}`);
  console.log(`The similarity score is ${totalSimilarityScore}`);
}

main();
