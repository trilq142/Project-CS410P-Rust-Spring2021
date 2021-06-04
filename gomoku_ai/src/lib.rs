pub const MAX: usize = 20;

impl Player {
    //Return true -> this player is the winner
    pub fn add_new_point(
        &mut self,
        point: Point,
        matrix: &mut [[u8; MAX]; MAX],
        test_point: &mut f32,
    ) -> bool {
        let mut yes_test = false;
        if *test_point > 0.0 {
            yes_test = true;
        }

        let origin_test_point = *test_point;

        if self.check_line(&point, matrix, test_point) {
            if !yes_test {
                matrix[point.x][point.y] = self.side;
                self.point_dic.push(point);
                *test_point = origin_test_point;
            } else if *test_point == 0.0 {
                *test_point = origin_test_point;
            }
            return true;
        }
        if yes_test {
            if *test_point == 0.0 {
                *test_point = origin_test_point;
            }
        } else {
            *test_point = origin_test_point;
            matrix[point.x][point.y] = self.side;
            self.point_dic.push(point);
        }
        false
    }
}

pub fn find_best_move(
    mut ai: Player,
    mut user: Player,
    mut matrix: [[u8; MAX]; MAX],
) -> Option<Point> {
    let mut hash: HashMap<Point, i32> = HashMap::new();
    get_all_board_move(&mut hash, ai.point_dic.clone(), &matrix);
    get_all_board_move(&mut hash, user.point_dic.clone(), &matrix);
    let new_board = hash.keys().cloned().collect::<Vec<Point>>();
    let mut final_board: Vec<(Point, f32)> = Vec::new();
    for point in new_board {
        let mut test_for_ai = 0.0001;
        let mut test_for_user = 0.0001;

        if ai.add_new_point(point.clone(), &mut matrix, &mut test_for_ai) {
            test_for_ai = 1000.0;
        }

        if user.add_new_point(point.clone(), &mut matrix, &mut test_for_user) {
            test_for_user = 1000.0;
        }
        test_for_ai += 1.0;
        // if test_for_user > test_for_ai {
        //     final_board.push((point, test_for_user));
        // } else {
        //     final_board.push((point, test_for_ai));
        // }
        final_board.push((point, test_for_ai + test_for_user));
    }
    final_board.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    final_board.reverse();
    println!("###Considering points:");
    for (point, score) in &final_board {
        print!("({},{})={} ", point.x, point.y, score);
    }
    println!();

    if !final_board.is_empty() {
        let (result, _) = &final_board[0];
        return Some(result.clone());
    }
    None
}
