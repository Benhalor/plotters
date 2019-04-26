use plotters::prelude::*;
use chrono::offset::{TimeZone, Local};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![("2019-04-25",130.0600,131.3700,128.8300,129.1500),("2019-04-24",125.7900,125.8500,124.5200,125.0100),("2019-04-23",124.1000,125.5800,123.8300,125.4400),("2019-04-22",122.6200,124.0000,122.5700,123.7600),("2019-04-18",122.1900,123.5200,121.3018,123.3700),("2019-04-17",121.2400,121.8500,120.5400,121.7700),("2019-04-16",121.6400,121.6500,120.1000,120.7700),("2019-04-15",120.9400,121.5800,120.5700,121.0500),("2019-04-12",120.6400,120.9800,120.3700,120.9500),("2019-04-11",120.5400,120.8500,119.9200,120.3300),("2019-04-10",119.7600,120.3500,119.5400,120.1900),("2019-04-09",118.6300,119.5400,118.5800,119.2800),("2019-04-08",119.8100,120.0200,118.6400,119.9300),("2019-04-05",119.3900,120.2300,119.3700,119.8900),("2019-04-04",120.1000,120.2300,118.3800,119.3600),("2019-04-03",119.8600,120.4300,119.1500,119.9700),("2019-04-02",119.0600,119.4800,118.5200,119.1900),("2019-04-01",118.9500,119.1085,118.1000,119.0200),("2019-03-29",118.0700,118.3200,116.9600,117.9400),("2019-03-28",117.4400,117.5800,116.1300,116.9300),("2019-03-27",117.8750,118.2100,115.5215,116.7700),("2019-03-26",118.6200,118.7050,116.8500,117.9100),("2019-03-25",116.5600,118.0100,116.3224,117.6600),("2019-03-22",119.5000,119.5900,117.0400,117.0500),("2019-03-21",117.1350,120.8200,117.0900,120.2200),("2019-03-20",117.3900,118.7500,116.7100,117.5200),("2019-03-19",118.0900,118.4400,116.9900,117.6500),("2019-03-18",116.1700,117.6100,116.0500,117.5700),("2019-03-15",115.3400,117.2500,114.5900,115.9100),("2019-03-14",114.5400,115.2000,114.3300,114.5900)];

    let mut backend = BitMapBackend::new("examples/outputs/stock.png", (1024,768));
    backend.open()?;
    let root:DrawingArea<_,_> = backend.into();
    let font = Into::<FontDesc>::into("DejaVu Serif").resize(20.0);
    root.fill(&RGBColor(255,255,255))?;

    let to_date =   Local.datetime_from_str(&format!("{} 0:0", data[0].0), "%Y-%m-%d %H:%M").unwrap().date() + chrono::Duration::days(1);
    let from_date = Local.datetime_from_str(&format!("{} 0:0", data[29].0), "%Y-%m-%d %H:%M").unwrap().date() - chrono::Duration::days(1);


    let mut chart = ChartBuilder::on(&root)
        .set_x_label_size(40)
        .set_y_label_size(40)
        .caption("MSFT Stock Price", &font)
        .build_ranged::<RangedDate<_>, RangedCoordf32, _, _>(from_date..to_date, 100f32..140f32);

    let style:ShapeStyle = (&RGBColor(255,255,255)).into();

    chart.configure_mesh()
        .line_style_2(&style)
        .draw()?;
    
    chart.draw_series(LineSeries::new(data.iter().map(|x|{
        let date = Local.datetime_from_str(&format!("{} 0:0", x.0), "%Y-%m-%d %H:%M").unwrap().date();
        return (date, (x.1 + x.4) / 2.0);
    }), &RGBColor(128,128,128)))?;

    chart.draw_series(data.iter().map(|x|{
        let date = Local.datetime_from_str(&format!("{} 0:0", x.0), "%Y-%m-%d %H:%M").unwrap().date();
        let (open,low,high,close) = (x.1,x.2,x.3,x.4);
        let l = chart.backend_coord(&(date, low));
        let h = chart.backend_coord(&(date, high));
        let o = chart.backend_coord(&(date, open));
        let c = chart.backend_coord(&(date, close));
        return EmptyElement::<_>::at((date, high))
            + Path::new(vec![(0,0), (0, (l.1 - h.1))], &RGBColor(0,0,0))
            + if open > close {
                Rectangle::new([(-5, o.1 - h.1), (5, c.1 - h.1)], Into::<ShapeStyle>::into(&RGBColor(255,0,0)).filled())
            } else {
                Rectangle::new([(-5, c.1 - h.1), (5, o.1 - h.1)], Into::<ShapeStyle>::into(&RGBColor(0,255,0)).filled())
            }
        ;
    }))?;

    root.close()?;
    return Ok(());
}