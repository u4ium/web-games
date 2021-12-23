use engines::chess::board::{
    grid::Square,
    piece::{Colour::Black, Piece},
};
use yew::{function_component, html, Properties};

#[derive(PartialEq, Properties)]
pub struct ChessSquareProps {
    pub square: Square,
}

#[function_component(ChessSquare)]
pub fn chess_square(props: &ChessSquareProps) -> Html {
    html! {
        props.square.and_then(|Piece {piece_type, ..}|
            Some(Piece::new(piece_type, Black).to_string())
        ).unwrap_or_default()
    }
}
