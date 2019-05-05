use super::Position;
use super::Velocity;
use super::Figure;

/// # 直線の衝突判定器
/// - スプライトからアローを発射し、交わるセルとの衝突を判定する
pub struct LinearCollider<'a> {
    figure: &'a Figure,
    position: &'a Position,
    velocity: &'a Velocity,

    /// # 傾き
    /// - y 軸に平行な直線の場合、傾きが発散するため None とする
    /// - x 軸に平行な直線の場合、傾きは 0
    a: Option<f32>,

    /// # 切片
    /// - y 軸に平行な直線の場合、切片は存在しないため None とする
    /// - x 軸に平行な直線の場合、傾きは y と同値
    b: Option<f32>,
}

impl<'a> LinearCollider<'a> {

    pub fn new(
        figure: &'a Figure,
        position: &'a Position,
        velocity: &'a Velocity
    ) -> LinearCollider<'a> {
        // 移動方向が x 軸に平行な場合、y 軸に平行な場合、それ以外の場合で分ける
        if velocity.goes_horizontal_only() {
            // x 軸に平行
            LinearCollider{ figure, position, velocity, a: Some(0.0), b: Some(velocity.y) }
        } else if velocity.goes_vertical_only() {
            // y 軸に平行
            // 傾きが発散し、切片も存在しない
            LinearCollider{ figure, position, velocity, a: None, b: None }
        } else {
            // 傾きのある直線のため、a, b を求める
            let (a, b) = calc_coef(&position, &velocity);
            LinearCollider{ figure, position, velocity, a: Some(a), b: Some(b) }
        }
    }

    /// - 下方向のイテレーション用ベクトルを生成する
    /// - step の間隔でイテレーションする
    /// - y の最後を含む
    pub fn create_seq_down(&self, step:i32) -> Vec<i32> {
        vec![]
    }

    /// - 全アローが上方向に突入するセルの座標を求める
    /// Vec<(irow, icol)> の形式で返す
    pub fn create_coordinates_up(&self, step: i32) -> Vec<(usize, usize)> {
        // 最初のアローが交わる cells を、(irow, x) の形で保持する。
        // x 座標を step でずらしてアローを作成
        // 各アローが交わる cells の x 座標をセルサイズで割り戻して (irow, icol) の形にする
        // 重複するセルを排除し、衝突判定の計算
        //
        // ※ アローの最初のセルで重複する場合、残りもすべて重複する
        let mut coordinates: Vec<(usize, usize)> = vec![];

        let y_beg = self.position.y.round() as usize;
        let y_end = (self.position.y + self.velocity.y).round() as usize;

        let start_x_beg = self.position.x.round() as usize;
        let start_x_end = (self.position.x + self.figure.width).round() as usize;
        let offset = start_x_beg % (step as usize);
        let start_xs = (start_x_beg..start_x_end)
            .filter(|x| (x - offset) % (step as usize) == 0);

        match self.a {
            Some(aa) => {
                let cell_first_ys = (y_beg..y_end)
                    .filter(|y| y % 63 == 0); // TODO: 固定値(セルサイズ)
                let mut irow_x_arrow: Vec<(usize, usize)> = vec![];
                for y in cell_first_ys {
                    let x = calc_x(y, aa, self.b.unwrap());
                    irow_x_arrow.push((y / 64, x)) // TODO: 固定値(セルサイズ)
                }

                // 各アローを計算して coordinates にセル座標を詰めていく
                for start_x in start_xs {
                    let diff = start_x - (self.position.x.round() as usize);
                    for (irow, x) in &irow_x_arrow {
                        let icol = (x + diff) / 64;
                        // 最初のセルがすでに含まれる場合、残りのセルも含まれるため break する
                        if coordinates.contains(&(irow.clone(), icol)) {
                            break;
                        }
                        coordinates.push((irow.clone(), (x + diff) / 64)) // TODO: 固定値(セルサイズ)
                    }
                }
            }
            None => {
                let irows = (y_beg .. y_end)
                    .filter(|y| y % 63 == 0) // TODO: 固定値(セルサイズ)
                    .map(|y| y / 63 - 1) // TODO: 固定値(セルサイズ)
                    .collect::<Vec<usize>>();
                // 交わるセルない同じ場合、空の vec をリターンする
                if irows.is_empty() {
                    return vec![];
                }

                // 各アローを計算して coordinates にセル座標を詰めていく
                for start_x in start_xs {
                    let icol = start_x / 64;
                    for irow in &irows {
                        // 最初のセルがすでに含まれる場合、残りのセルも含まれるため break する
                        if coordinates.contains(&(irow.clone(), icol)) {
                            break;
                        }
                        coordinates.push((irow.clone(), icol));
                    }
                }
            }
        }
        coordinates
    }

}

fn calc_coef(position: &Position, velocity: &Velocity) -> (f32, f32) {
    let x0 = position.x;
    let y0 = position.y;
    let x1 = position.x + velocity.x;
    let y1 = position.y + velocity.y;
    let a = (y1 - y0) / (x1 - x0);
    let b = (x1 * y0 - x0 * y1) / (x1 - x0);
    (a, b)
}

fn calc_x(y: usize, a: f32, b: f32) -> usize {
    ((y as f32) / a -  b / a).round() as usize
}

fn calc_y(x: usize, a: f32, b: f32) -> usize {
    (a * (x as f32) + b).round() as usize
}
