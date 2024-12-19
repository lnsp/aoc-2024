use fraction::Fraction;
use num::Zero;

pub fn task1(games: &Vec<(Vec<(usize, usize)>, (usize, usize))>) -> usize {
    games
        .iter()
        .flat_map(|(buttons, goal)| evaluate_game(buttons, *goal, (100, 100)))
        .sum()
}
pub fn task2(games: &Vec<(Vec<(usize, usize)>, (usize, usize))>) -> usize {
    games
        .iter()
        .map(|(buttons, goal)| (buttons, (goal.0 + 10000000000000, goal.1 + 10000000000000)))
        .flat_map(|(buttons, goal)| evaluate_game(buttons, goal, (usize::MAX, usize::MAX)))
        .sum()
}

fn evaluate_game(
    buttons: &Vec<(usize, usize)>,
    goal: (usize, usize),
    bounds: (usize, usize),
) -> Option<usize> {
    let (button_a, button_b) = (buttons[0], buttons[1]);

    let left_side = Fraction::from(button_b.0) * Fraction::from(button_a.1)
        / Fraction::from(button_a.0)
        - Fraction::from(button_b.1);

    let right_side = Fraction::from(goal.0) * Fraction::from(button_a.1)
        / Fraction::from(button_a.0)
        - Fraction::from(goal.1);

    let y = right_side / left_side;
    let x = (Fraction::from(goal.1) - Fraction::from(button_b.1) * y) / Fraction::from(button_a.1);

    // Check that both have almost zero fract
    if x >= Fraction::zero() && y >= Fraction::zero() && *x.denom()? == 1 && *y.denom()? == 1 {
        let xn = *x.numer()? as usize;
        let yn = *y.numer()? as usize;

        if xn <= bounds.0 && yn <= bounds.1 {
            return Some(3 * xn + yn);
        }
    }

    None
}
