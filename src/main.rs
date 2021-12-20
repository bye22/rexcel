
#![windows_subsystem = "windows"]//关闭控制台窗口
use std::{error::Error, vec};
use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use fltk_table::{SmartTable, TableOpts};
use serde::{Deserialize, Serialize};


use fltk::{
    app,
    button::{self, *},
    enums::{Color, Event, FrameType},
    frame,
    group,
    image,
    input,
    prelude::{GroupExt, ImageExt, InputExt, TableExt, WidgetBase, WidgetExt, WindowExt},
    window,
    dialog,
};

use indicium::simple::Indexable;
use indicium::simple::SearchIndex;

const BLUE: Color = Color::from_hex(0x42A5F5);
const SEL_BLUE: Color = Color::from_hex(0x2196F3);
const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RawExcelRow {
    name: String ,
    idcard: String,
    address: String,
    fromdate: String,
    todate: String,
    credit: String,
}

impl Indexable for RawExcelRow {
    fn strings(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.idcard.clone(),
            self.address.clone(),
            self.fromdate.clone(),
            self.todate.clone(),
            self.credit.clone(),
        ]
    }
}

fn read_excel_file() -> Result<Vec<RawExcelRow>, Box<dyn Error>> {
    let path = "excel/sh_query.xlsx";
    let mut excel: Xlsx<_> = open_workbook(path)?;
    let range = excel
        .worksheet_range("result1")
        .ok_or(calamine::Error::Msg("Cannot find Sheet1"))??;

    let mut iter_result = RangeDeserializerBuilder::new()
        .has_headers(false)
        .from_range::<_, RawExcelRow>(&range)?;
    iter_result.next(); // skip first row
    iter_result.next();// skip second row
    let mut vec = Vec::new();
    for row in iter_result {
        vec.push(row?);
    }
    Ok(vec)
}

pub fn show_dialog(in_name_val: String, in_idcard_val: String) {
    let mut win = window::Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("");
    win.set_color(Color::from_rgb(240, 240, 240));
    // let mut grid = group::VGrid::new(0, 0, 400, 20, "").center_of_parent();
    // grid.set_params(1, 1, 2);
    // grid.end();
    let iter_result = read_excel_file().unwrap();
    let mut search_index: SearchIndex<usize> = SearchIndex::default();
    iter_result
        .iter()
        .enumerate()
        .for_each(|(index, element)| search_index.insert(&index, element));

    let ref result_name: Vec<&usize> = search_index.search(&in_name_val.as_str());
    let ref result_idcard: Vec<&usize> = search_index.search(&in_idcard_val.as_str());

    if result_idcard.is_empty() && result_name.is_empty() {
        let mut grid0 = group::VGrid::new(0, 200, 400, 400, "").center_of_parent();
        let mut a = frame::Frame::default().with_size(0, 40); // a filler
        a.set_label("没有此用户信息");
        println!("{}", "没有此用户信息");
        grid0.end();
    } else {
        let mut grid2 = group::VGrid::new(0, 0, 600, 240, "").center_of_parent();
        let mut table = SmartTable::default()
        .with_size(600, 400)
        .center_of_parent()
        .with_opts(TableOpts {
            rows: 5,
            cols: 2,
            editable: false,
            // cell_selection_color: Color::Red.inactive(),
            header_frame: FrameType::FlatBox,
            header_color: Color::BackGround.lighter(),
            cell_border_color: Color::White,
            ..Default::default()
        });
        table.scrollbar().deactivate();
        table.hscrollbar().deactivate();
        table.set_row_height_all(40);
        table.set_col_width(0, 100);
        table.set_col_width(1, 600);
        

        table.set_col_header(false);
        table.set_row_header(false);
        // table.set_align(Align::Right);
        table.set_cell_value(0,0,"姓名");
        table.set_cell_value(1,0,"身份证号");
        table.set_cell_value(2,0,"地址");
        table.set_cell_value(3,0,"贷款金额");
        table.set_cell_value(4,0,"起止时间");

        table.set_cell_value(0,1,&iter_result[*result_name[0]].name);
        table.set_cell_value(1,1,&iter_result[*result_name[0]].idcard);
        table.set_cell_value(2,1,&iter_result[*result_name[0]].address);
        table.set_cell_value(3,1,&iter_result[*result_name[0]].credit);
        table.set_cell_value(4,1,format!(
                "起{}-止{}",
                &&iter_result[*result_name[0]].fromdate,
                &&iter_result[*result_name[0]].todate
            ).as_str());
        
        // let mut grid = group::VGrid::new(0, 0, 350, 400, "").center_of_parent();
        // grid.set_params(10, 2, 0);
        // let name = frame::Frame::default().with_size(0, 40).with_label("姓名");
        // let name_val = frame::Frame::default().with_size(0, 40).with_label(&iter_result[*result_name[0]].name);
        // let idcard = frame::Frame::default().with_size(0, 40).with_label("身份证号");
        // let idcard_val = frame::Frame::default().with_size(0, 40).with_label(&iter_result[*result_name[0]].idcard);
        // let address = frame::Frame::default().with_size(0, 40).with_label("地址");
        // let address_val = frame::Frame::default().with_size(0, 40).with_label(&iter_result[*result_name[0]].address);
        // let credit = frame::Frame::default().with_size(0, 40).with_label("贷款金额");
        // let credit_val = frame::Frame::default().with_size(0, 40).with_label(&iter_result[*result_name[0]].credit);
        // let fromTodate = frame::Frame::default().with_size(0, 40).with_label("起止时间");
        // let fromTodate_val = frame::Frame::default().with_size(0, 40).with_label(format!(
        //     "起{}-止{}",
        //     &&iter_result[*result_name[0]].fromdate,
        //     &&iter_result[*result_name[0]].todate
        // ).as_str());
        // grid.end();

        grid2.end();

        
    }
    frame::Frame::default().with_size(0, 30); // a filler
    let mut grid1 = group::VGrid::new(0, 400, 400, 40, "");
    let mut but = button::Button::default()
        .with_size(0, 40)
        .with_label("返回查询");
    grid1.end();
    init_button_theming(&mut but);

    win.make_modal(true);
    win.end();
    win.show();
    but.set_callback(move |_b| win.hide());
}

fn init_app_theming() {
    app::background(255, 255, 255);
    app::set_visible_focus(false);
}
fn init_button_theming(but: &mut Button) {
    // Theming

    but.set_color(BLUE);
    but.set_selection_color(SEL_BLUE);
    but.set_label_color(Color::White);
    but.set_frame(FrameType::FlatBox);

    // End theming
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = app::App::default();
    init_app_theming();
    let mut wind = window::Window::default()
        .with_size(WIDTH, HEIGHT)
        .center_screen();
    wind.make_resizable(true);

    let mut pack = group::Pack::default_fill()
        .with_size(WIDTH / 2, 600)
        .center_of(&wind);
    // pack.set_spacing(10);
    let mut frame = frame::Frame::new(0, 30, 240, 240, "屯留三禾客户信息查询");
    let mut image = image::PngImage::load("logo.png").unwrap();
    image.scale(100, 100, true, true);
    frame.set_image(Some(image));

    // let mut pack1 = group::Pack::default_fill().with_size(200,100);
    let name = frame::Frame::default()
        .with_size(0, 40)
        // .left_of(&frame, 10)
        .with_label(&format!("姓名 {}", ""));
    let inp_name = input::Input::default().with_size(0, 40).right_of(&name, 0);
    // pack1.set_type(group::PackType::Horizontal);
    // pack1.end();

    // let mut pack2 = group::Pack::default_fill().with_size(100,100);
    let _idcard = frame::Frame::default()
        .with_size(0, 40)
        // .below_of(&name, 10)
        .with_label(&format!("身份证号 {}", ""));
    let inp_idcard = input::Input::default().with_size(0, 40);
    frame::Frame::default().with_size(0, 45);
    // pack2.set_type(group::PackType::Horizontal);
    // pack2.end();

    let mut but = button::Button::default()
        .with_size(0, 40)
        .with_label("点击查询");
    init_button_theming(&mut but);
    pack.set_type(group::PackType::Vertical);
    pack.end();
    wind.end();
    wind.show();


    but.handle(move |_b, event| match event {
        Event::Push => {
            let in_name_val = inp_name.value();
            let in_idcard_val = inp_idcard.value();
            println!("name:{},idcard:{}", in_name_val, in_idcard_val);
            if in_name_val=="" || in_idcard_val=="" {
                dialog::message_default("请输入姓名身份证号")
            }else{
                show_dialog(in_name_val, in_idcard_val);
            }
            true
        }
        _ => false,
    });

    Ok(app.run()?)
}
