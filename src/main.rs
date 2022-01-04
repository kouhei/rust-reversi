use std::io::{self, Write};

mod reversi {
    use std::io::{self, Write};
    #[derive(Debug)]
    pub enum Winner{
        Black,
        White,
        Draw
    }

    #[derive(Debug,Copy, Clone)]
    pub enum Color {
        Black,
        White
    }
    pub const BLACK:Color = Color::Black;
    pub const WHITE:Color = Color::White;
    pub const BLACK_POINT_STR:&str = "●";
    pub const WHITE_POINT_STR:&str = "◯";

    #[derive(Debug)]
    struct Point {
        x:u8,
        y:u8,
        color:Color
    }

    pub struct Reversi {
        player:Color, //手番のプレイヤーの色
        board:Vec<Point>
        /*
                 x
          0 1 2 3 4 5 6 7
        0 * * * * * * * *
        1 * * * * * * * *
        2 * * * * * * * *
      y 3 * * * B W * * *
        4 * * * W B * * *
        5 * * * * * * * *
        6 * * * * * * * *
        7 * * * * * * * *
        */
    }
    // 表示系
    impl Reversi {
        pub fn display(&self){
            const BOARD_HEADER:&str = "  0 1 2 3 4 5 6 7";
            let mut board_strs:[[&str;8];8] = [
                ["•","•","•","•","•","•","•","•"],
                ["•","•","•","•","•","•","•","•"],
                ["•","•","•","•","•","•","•","•"],
                ["•","•","•","•","•","•","•","•"],
                ["•","•","•","•","•","•","•","•"],
                ["•","•","•","•","•","•","•","•"],
                ["•","•","•","•","•","•","•","•"],
                ["•","•","•","•","•","•","•","•"],
            ];
            for p in &self.board {
                match p.color {
                    Color::Black => board_strs[p.y as usize][p.x as usize] = BLACK_POINT_STR,
                    Color::White => board_strs[p.y as usize][p.x as usize] = WHITE_POINT_STR,

                }
            }
            println!("{}",BOARD_HEADER);
            for (row_num,strs) in board_strs.iter().enumerate() {
                print!("{} ",row_num);
                for chr in strs {
                    print!("{} ", chr);
                }
                io::stdout().flush().unwrap();
                println!("");
            }
        }
    }
    // 判断系
    impl Reversi {
        pub fn get_player(&self) -> Color{
            self.player
        }
        // 座標を指定してそこに置かれている色を返す。置かれていなければNone
        fn get_color(&self, x:u8, y:u8) -> Option<&Color>{
            let point = &self.board.iter().find(|cell| cell.x == x && cell.y == y);
            match point {
                Some(p) => Some(&p.color),
                None => None
            }
        }

        //隣接する座標を返す
        fn get_neighborhoods(&self, x:i8,y:i8) -> Vec<(u8,u8)>{
            let candidates:Vec<(i8,i8)> = vec![
                (x-1,y-1),(x-1,  y),(x-1,y+1),
                (x  ,y-1),          (x  ,y+1),
                (x+1,y-1),(x+1,  y),(x+1,y+1),
            ];
            candidates.into_iter().filter(|p| self.is_valid_point(p.0 as i8,p.1 as i8)).map(|p| (p.0 as u8,p.1 as u8) ).collect()
        }
        fn is_neighborhood(&self,center:(u8,u8),target:(u8,u8)) -> bool {
            let ns = self.get_neighborhoods(center.0 as i8, center.1 as i8);
            for (nx,ny) in ns {
                if nx == target.0 && ny == target.1{
                    return true;
                }
            }
            false
        }
        fn get_empty_neighborhoods(&self,x:u8,y:u8) -> Vec<(u8,u8)>{
            self.get_neighborhoods(x as i8,y as i8).into_iter()
                .filter(|p| if let None = self.get_color(p.0, p.1) { true } else { false } )
                .collect()
        }
        fn get_neighborhood_enemies(&self,x:u8,y:u8, color:&Color) -> Vec<&Point>{
            self.board.iter()
            .filter(|p| self.is_neighborhood((x,y),(p.x,p.y)) )
            .filter(|p| match p.color {
                // 敵の色か
                Color::Black => { if let Color::Black = color { false } else { true }},
                Color::White => { if let Color::White = color { false } else { true }},
            } ).collect()
        }
        //プレイヤーがおける座標の配列
        pub fn puttable_points(&self, player:&Color) -> Vec<(u8,u8)>{
            // 現在置かれているすべてのPointに隣接するまだ置かれていない座標たち
            let candidates = self.board.iter().flat_map(|point| self.get_empty_neighborhoods(point.x, point.y));
            // それぞれの座標に置けるか検証
            candidates.filter(|c| self.can_put(c.0, c.1, player)).collect()
        }

        fn get_direction(&self, from:(u8,u8),to:(u8,u8)) -> (i8,i8) {
            (to.0 as i8 - from.0 as i8, to.1 as i8 - from.1 as i8)
        }

        fn can_put(&self,x:u8,y:u8, color:&Color) -> bool {
            let a = self.is_valid_point(x as i8, y as i8);
            let b = self.empty_point(x, y);
            let c = self.can_put_by_color(x,y,color);
            a && b && c
        }
        // 盤面に収まるindex
        fn is_valid_point(&self, x:i8,y:i8) -> bool{
            if  x >= 0 && y >= 0 && x < 8 && y < 8 {
                true
            } else {
                false
            }
        }
        // まだ置かれていないindex
        fn empty_point(&self,x:u8,y:u8) -> bool{
            match self.get_color(x, y) {
                None => true,
                _ => false
            }
        }
        // 盤面の状態的における位置か
        fn can_put_by_color(&self, x:u8,y:u8,color:&Color) -> bool {
            let neighborhood_enemies:Vec<&Point> = self.get_neighborhood_enemies(x,y,color);
            for neighbor_enem in neighborhood_enemies {
                let direction = self.get_direction((x,y),(neighbor_enem.x,neighbor_enem.y));
                if direction.0 == 0 && direction.1 == 0 {
                    panic!("[ERROR] invalid direction: {:?}, from:{:?}, to:{:?}",direction,(x,y),(neighbor_enem.x,neighbor_enem.y));
                }

                let mut next_point = ((neighbor_enem.x)as i8 + direction.0,(neighbor_enem.y)as i8 + direction.1);
                let mut result = false;
                while self.is_valid_point(next_point.0, next_point.1){
                    /* その方向の次のpointが
                     * 敵 => さらに次を調べる
                     * 自分 => trueを返す
                     * 何もない => その方向はだめ
                     * 以上を端に到達するまで行う */
                    match self.get_color(next_point.0 as u8,next_point.1 as u8) {
                        Some(Color::Black) => {
                            if let Color::Black = color {
                                result = true;
                                break;
                            } else {
                                next_point = (next_point.0 + direction.0, next_point.1 + direction.1);
                            }
                        },
                        Some(Color::White) => {
                            if let Color::White = color {
                                result = true;
                                break;
                            } else {
                                next_point = (next_point.0 + direction.0, next_point.1 + direction.1);
                            }
                        }
                        None => {
                            result = false;
                            break;
                        }
                    };
                }
                if result {
                    return true;
                }
            }
            false
        }
        pub fn is_game_over(&self) -> bool{
            self.puttable_points(&Color::Black).len() == 0 && self.puttable_points(&Color::White).len() == 0
        }
        pub fn get_winner(&self) -> Option<Winner>{
            if !self.is_game_over(){
                return None;
            }
            let black_cnt = self.board.iter().filter(|p| if let Color::Black = p.color {true}else{false} ).count();
            if black_cnt == 32 {
                Some(Winner::Draw)
            }else{
                if black_cnt > 32 {
                    Some(Winner::Black)
                } else {
                    Some(Winner::White)
                }
            }
        }
    }
    // 操作系
    impl Reversi {
        pub fn init() -> Reversi {
            Reversi{
                player: Color::Black,
                board:vec![
                    Point{x:3,y:3,color:WHITE}, Point{x:4,y:3,color:BLACK},
                    Point{x:3,y:4,color:BLACK}, Point{x:4,y:4,color:WHITE}
                ]
            }
        }
        pub fn change_player(&mut self){
            self.player = match self.player {
                Color::Black => Color::White,
                Color::White => Color::Black,
            };
        }
        fn change_color(&mut self,x:u8,y:u8,color:Color) -> bool{
            let nullable_index = self.board.iter().position(|p| p.x == x && p.y == y);
            match nullable_index {
                Some(i) => {
                    self.board[i].color = color;
                    true
                },
                None => {
                    println!("invalid reversable point:{:?}", (x,y));
                    false
                }
            }
        }
        //置けるならおく
        pub fn put(&mut self,x:u8,y:u8,color:Color) -> bool{
            if self.can_put(x, y, &color) {
                self.board.push(Point{x,y,color:color.clone()});
                self.update_by_put(x,y,color);
                true
            }else {
                false
            }
        }
        // 置いたことでひっくり返せるものをひっくり返す
        pub fn update_by_put(&mut self,x:u8,y:u8, color:Color){
            /* 隣接する敵を取得し、その方向の駒が
             * 自分 -> それまでのコマを自分のコマに
             * 相手 -> 次のコマを算出して繰り返す
             * 空   -> 終了 */
            let mut reversable_points:Vec<(u8,u8)> = Vec::new();
            let neighbor_enemies = self.get_neighborhood_enemies(x, y, &color);
            for neighbor_enem in neighbor_enemies {
                let dir = self.get_direction((x,y), (neighbor_enem.x,neighbor_enem.y));
                if dir.0 == 0 && dir.1 == 0 {
                    panic!("[ERROR] invalid direction: {:?}, from:{:?}, to:{:?}",dir,(x,y),(neighbor_enem.x,neighbor_enem.y));
                }
                let mut tmp_reversable = vec![(neighbor_enem.x,neighbor_enem.y)];
                let mut next_point = ((neighbor_enem.x)as i8 + dir.0,(neighbor_enem.y)as i8 + dir.1);

                loop {
                    if !self.is_valid_point(next_point.0, next_point.1) {
                        tmp_reversable.clear();
                            break;
                    }
                    match self.get_color(next_point.0 as u8,next_point.1 as u8) {
                        Some(Color::Black) => {
                            if let Color::Black = color {
                                break;
                            } else {
                                tmp_reversable.push((next_point.0 as u8,next_point.1 as u8));
                                next_point = (next_point.0 + dir.0, next_point.1 + dir.1);
                            }
                        },
                        Some(Color::White) => {
                            if let Color::White = color {
                                break;
                            } else {
                                tmp_reversable.push((next_point.0 as u8, next_point.1 as u8));
                                next_point = (next_point.0 + dir.0, next_point.1 + dir.1);
                            }
                        }
                        None => {
                            tmp_reversable.clear();
                            break;
                        }
                    };
                }
                reversable_points.extend(tmp_reversable);
            }
            for reversable_point in reversable_points {
                self.change_color(reversable_point.0, reversable_point.1, color);
            }
        }
    }
}

fn main() {
    let mut reversi = reversi::Reversi::init();
    loop {
        if reversi.is_game_over() {
            println!("GAME OVER!");
            break;
        }
        let player = reversi.get_player();
        reversi.display();
        if reversi.puttable_points(&player).is_empty() {
            println!("player cannot put anywhere! next.");
            reversi.change_player();
        };
        let player = reversi.get_player();
        println!("{} put at", if let reversi::Color::Black = player { "[● BLACK]" } else { "[◯ WHITE]" });
        match get_point_input() {
            Some((x,y)) => {
                if reversi.put(x,y,player) {
                    reversi.change_player();
                    println!();
                } else {
                    println!("invalid point\n");
                }
            },
            _ => println!("invalid point\n")
        }
    }
    match reversi.get_winner() {
        Some(w) => {println!("winner is {:?}", w)},
        None => {println!("error occurred")}
    }
}

fn get_point_input() -> Option<(u8,u8)>{
    let x = get_input("x");
    let y = get_input("y");
    match (x,y) {
        (Some(x),Some(y)) => Some((x,y)),
        _ => None
    }
}

fn get_input(print_str:&str) -> Option<u8>{
    print!("{}:",print_str);
    io::stdout().flush().expect("Failed to flush");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    match n.trim().parse::<u8>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}