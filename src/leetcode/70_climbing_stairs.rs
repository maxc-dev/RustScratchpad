pub fn main() {
    let mut climber = ClimbingStairs::new();
    let answer = climber.climb_stairs(10);
    println!("Answer: {:?}", answer);
}

struct ClimbingStairs {
    cache: Vec<i32>,
}

impl ClimbingStairs {
    fn new() -> Self {
        ClimbingStairs {
            cache: vec![0; 50],
        }
    }

    pub fn climb_stairs(&mut self, n: i32) -> i32 {
        if n <= 3 {
            return n;
        }

        if self.cache[n as usize] == 0 {
            self.cache[n as usize] = self.climb_stairs(n - 1) + self.climb_stairs(n - 2);
        }

        self.cache[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_cases() {
        let mut climber = ClimbingStairs::new();
        assert_eq!(climber.climb_stairs(1), 1);
        assert_eq!(climber.climb_stairs(2), 2);
        assert_eq!(climber.climb_stairs(3), 3);
    }

    #[test]
    fn test_small_values() {
        let mut climber = ClimbingStairs::new();
        assert_eq!(climber.climb_stairs(4), 5);
        assert_eq!(climber.climb_stairs(5), 8);
    }

    #[test]
    fn test_larger_values() {
        let mut climber = ClimbingStairs::new();
        assert_eq!(climber.climb_stairs(10), 89);
        assert_eq!(climber.climb_stairs(20), 10946);
    }

    #[test]
    fn test_exceed_initial_cache_size() {
        let mut climber = ClimbingStairs::new();
        assert_eq!(climber.climb_stairs(45), 1836311903);
    }
}