pub async fn draw() {

display.draw(
            ProFont14Point::render_str("Liebe Agni-Pagni")
                .with_stroke(Some(Color::Red))
                .with_fill(Some(Color::White))
                .translate(Coord::new(1, -4))
                .into_iter(),
        );
        display.draw(
            ProFont14Point::render_str("zum Geburtstag")
                .with_stroke(Some(Color::Red))
                .with_fill(Some(Color::White))
                .translate(Coord::new(1, 13))
                .into_iter(),
        );
        display.draw(
        ProFont14Point::render_str("wir wünschen dir")
                    .with_stroke(Some(Color::Red))
                    .with_fill(Some(Color::White))
                    .translate(Coord::new(1, 27))
                    .into_iter(),
            );
        display.draw(
        ProFont12Point::render_str("alles Gute und viel Glück")
                    .with_stroke(Some(Color::Red))
                    .with_fill(Some(Color::White))
                    .translate(Coord::new(1, 42))
                    .into_iter(),
            );
        display.draw(
                ProFont9Point::render_str("Irrigatron bittet dir seine Dinst")
                    .with_stroke(Some(Color::Black))
                    .with_fill(Some(Color::White))
                    .translate(Coord::new(1, 57))
                    .into_iter(),
            );
        display.draw(
            ProFont9Point::render_str("Geburehfreie und treu")
                .with_stroke(Some(Color::Black))
                .with_fill(Some(Color::White))
                .translate(Coord::new(1, 66))
                .into_iter(),
        );
        display.draw(
            ProFont9Point::render_str("arbeitet auf deine auf Plantagen")
                .with_stroke(Some(Color::Black))
                .with_fill(Some(Color::White))
                .translate(Coord::new(1, 76))
                .into_iter(),
        );
        display.draw(
            ProFont9Point::render_str("Unkompliziert und gute Zuhorer,")
                .with_stroke(Some(Color::Black))
                .with_fill(Some(Color::White))
                .translate(Coord::new(1, 85))
                .into_iter(),
        );
        display.draw(
            ProFont9Point::render_str("Kostonlose Updates und Nachrustung")
                .with_stroke(Some(Color::Red))
                .with_fill(Some(Color::White))
                .translate(Coord::new(1, 94))
                .into_iter(),
        );
}
