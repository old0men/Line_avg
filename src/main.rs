use rand::prelude::*;

fn main() {
    let mut position_list: Vec<i32> = vec![];
    let mut rng = thread_rng();
    let mut counter: i8 = 0;
    while counter < 8{
        position_list.push(rng.gen_range(-100..100));
        counter += 1;
    }
    let line_one:[(i32, i32); 2] = [(position_list[0], position_list[1]), (position_list[2], position_list[3])];
    let line_two:[(i32, i32); 2] = [(position_list[4], position_list[5]), (position_list[6], position_list[7])];

    let _test_line1:[(i32, i32); 2] = [(-50, 50), (50, 50)];
    let _test_line2:[(i32, i32); 2] = [(-50, -50), (50, -50)];

    println!("position list: {:?}\n line one: {:?}\n line two: {:?}", position_list, line_one, line_two);

    fn line_calc(line1: [(i32, i32);2], line2: [(i32, i32);2]) -> ((i32, i32), (i32, i32)) {
        let [(x_position1, y_position1), (x_position2, y_position2)] = line1;
        let [(x_position3, y_position3), (x_position4, y_position4)] = line2;

        let x_avg_start_line: i32 = (x_position1 + x_position3)/2;
        let x_avg_end_line: i32 = (x_position2 + x_position4)/2;
        let y_avg_start_line: i32 = (y_position1 + y_position3)/2;
        let y_avg_end_line: i32 = (y_position2 + y_position4)/2;

        ((x_avg_start_line, y_avg_start_line), (x_avg_end_line, y_avg_end_line))

    }

    println!("the average of the lines: {:?}", line_calc(line_one, line_two));
}
