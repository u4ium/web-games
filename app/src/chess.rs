use boolinator::Boolinator;
use yew::{classes, prelude::*};

use engines::chess;

#[derive(Debug)]
pub struct ChessBoard {
    link: ComponentLink<Self>,
    props: ChessBoardProps,
    state: ChessBoardData,
}

#[derive(Debug)]
struct ChessBoardData {
    board: [[char; 8]; 8],
}

impl Default for ChessBoardData {
    fn default() -> Self {
        Self {
            board: [
                ['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜'],
                ['♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟'],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                ['♙', '♙', '♙', '♙', '♙', '♙', '♙', '♙'],
                ['♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖'],
            ],
        }
    }
}

#[derive(Debug, Clone, Properties)]
pub struct ChessBoardProps {
    // pub login: Callback<String>,
}

pub enum ChessBoardMessage {
    Click {
        column_index: usize,
        row_index: usize,
    },
}
use ChessBoardMessage::*;

const FILES: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

impl Component for ChessBoard {
    type Message = ChessBoardMessage;
    type Properties = ChessBoardProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            state: Default::default(),
        }
    }

    fn update(&mut self, message: Self::Message) -> bool {
        match message {
            Click {
                column_index,
                row_index,
            } => {
                // TODO
            }
        }
        true
    }

    fn change(&mut self, _new_props: Self::Properties) -> bool {
        // TODO
        false
    }

    fn view(&self) -> Html {
        html! {
            <table class="chess-board">
                <tbody>
                    {for self.state.board.iter().enumerate().map(|(row_index, row)| {
                        let rank = 8 - row_index;
                        html! {
                            <tr key={row_index}>
                                <th class="rank-label">
                                    {rank}
                                </th>
                                {for row.iter().enumerate().map(|(column_index, square)| {
                                    let selectable = square != &' ';
                                    let on_click = self.link.callback(move |_e| Click{
                                        column_index,
                                        row_index,
                                    });
                                    html! {
                                        <td
                                            key={column_index}
                                            class=classes!{selectable.as_some("selectable")}
                                            onclick={on_click}
                                        >
                                            {square}
                                        </td>
                                    }
                                })}
                            </tr>
                        }
                    })}
                    <tr class="file-labels">
                        <th class="rank-label file-label"></th>
                        {for FILES.iter().enumerate().map(|(index, file)| {
                            html!{
                                <th key={index} class="file-label">
                                    {file}
                                </th>
                            }
                        })}
                    </tr>
                </tbody>
            </table>
        }
    }
}
