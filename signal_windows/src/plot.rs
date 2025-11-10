use bulks::*;
use plotters::prelude::*;

type T = f64;

const CLAMP: T = 200.0;
const PLOT_RES: (u32, u32) = (1024, 760);
const PLOT_CAPTION_FONT: (&str, u32) = ("sans", 20);
const PLOT_MARGIN: u32 = 5;
const PLOT_LABEL_AREA_SIZE: u32 = 30;

pub fn plot_curves<const M: usize>(
    plot_title: &str, plot_path: &str,
    xy: [impl IntoIterator<Item = (T, T)>; M]
) -> Result<(), Box<dyn std::error::Error>>
{
    let xy = xy.into_bulk()
        .map(|xy| xy.into_iter().collect::<Vec<_>>())
        .collect::<[_; _], _>();

    let x_min = xy.iter().flatten().map(|x| x.0).filter(|x| x.is_finite()).reduce(T::min).unwrap();
    let x_max = xy.iter().flatten().map(|x| x.0).filter(|x| x.is_finite()).reduce(T::max).unwrap();
    
    let y_min = xy.iter().flatten().map(|&x| x.1).filter(|x| x.is_finite()).reduce(T::min).unwrap();
    let y_max = xy.iter().flatten().map(|&x| x.1).filter(|x| x.is_finite()).reduce(T::max).unwrap();
    
    let x = xy.each_ref().map(|x| x.iter().map(|&x| x.0.clamp(x_min, x_max)));
    let y = xy.each_ref().map(|y| y.iter().map(|&y| y.1.clamp(y_min, y_max)));
    
    let area = BitMapBackend::new(plot_path, PLOT_RES).into_drawing_area();
    
    area.fill(&WHITE)?;
    
    let mut chart = ChartBuilder::on(&area)
        .caption(plot_title, PLOT_CAPTION_FONT.into_font())
        .margin(PLOT_MARGIN)
        .x_label_area_size(PLOT_LABEL_AREA_SIZE)
        .y_label_area_size(PLOT_LABEL_AREA_SIZE)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;
    
    chart.configure_mesh()
        .set_all_tick_mark_size(0.1)
        .draw()?;
    
    let mut j = 0;
    for (i, (x, y)) in x.into_iter().zip(y).enumerate()
    {
        let color = {
            loop {
                let color = Palette99::pick(j);
                let (r, g, b) = color.rgb();
                if (r as u16 + g as u16 + b as u16) < ((255.0*3.0*0.6) as u16)
                {
                    break color;
                }
                j += 1;
            }
        };
        chart.draw_series(LineSeries::new(
                x.zip(y),
                &color
            ))?
            .label(format!("{}", i))
            .legend(move |(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], color.mix(0.5).filled()));

        j += 1;
    }
    
    chart.configure_series_labels()
        .border_style(BLACK)
        .draw()?;
        
    // To avoid the IO failure being ignored silently, we manually call the present function
    area.present().expect("Unable to write result to file");

    Ok(())
}