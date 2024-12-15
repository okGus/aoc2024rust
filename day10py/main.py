from typing import List, Set, Tuple


def is_bounded(matrix: List[int], r: int, c: int):
    if r >= 0 and c >= 0 and r < len(matrix) and c < len(matrix[0]):
        return True
    return False


def dfs(matrix: List[int], seen: Set[Tuple[int, int]], r: int, c: int, res: List[int]):
    if not is_bounded(matrix, r, c) or (r, c) in seen:
        return

    val = matrix[r][c]
    seen.add((r, c))

    if val == 9:
        res[0] += 1
        return

    if r - 1 >= 0 and val + 1 == matrix[r - 1][c]:
        dfs(matrix, seen, r - 1, c, res)
    if c + 1 < len(matrix[0]) and val + 1 == matrix[r][c + 1]:
        dfs(matrix, seen, r, c + 1, res)
    if r + 1 < len(matrix) and val + 1 == matrix[r + 1][c]:
        dfs(matrix, seen, r + 1, c, res)
    if c - 1 >= 0 and val + 1 == matrix[r][c - 1]:
        dfs(matrix, seen, r, c - 1, res)


def solution(matrix: List[int]):
    total_score = 0
    trailhead_scores = []

    for r in range(len(matrix)):
        for c in range(len(matrix[0])):
            if matrix[r][c] == 0:
                seen = set()
                trailhead_score = [0]

                dfs(matrix, seen, r, c, trailhead_score)

                trailhead_scores.append(trailhead_score[0])
                total_score += trailhead_score[0]
    print("Trailhead scores", trailhead_scores)
    print("Total Trailhead score", total_score)


def main():
    with open("day10input.txt", 'r') as f:
        line = f.readlines()
        matrix = [list(map(int, lines.strip())) for lines in line]

    for sub in matrix:
        print(sub)
    solution(matrix)


if __name__ == "__main__":
    main()
