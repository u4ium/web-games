use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use boolinator::Boolinator;
use yew::{classes, prelude::*};

use engines::chess::{
    ai::AiPlayer,
    board::{
        coordinates::{ColumnIndex, Coordinate, Move},
        grid::Square,
        piece::Colour,
        BoardState,
    },
};

mod square;
use square::ChessSquare;

pub use Colour::*;

#[derive(Debug)]
pub enum InputState {
    NoPieceSelected,
    PieceSelected {
        location: Coordinate,
        destinations: HashSet<Coordinate>,
    },
}
use InputState::*;

#[derive(Debug)]
pub enum ChessBoardState {
    AwaitingInput(InputState),
    // AwaitingOtherPlayers,
    // ShowingError(&'static str),
}
use ChessBoardState::*;

#[derive(Debug)]
pub struct ChessBoard {
    board_state: Arc<Mutex<BoardState>>,
    ai: Option<Arc<Mutex<AiPlayer>>>,
    state: ChessBoardState,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ChessBoardProps {
    pub players: Vec<Colour>,
    #[prop_or_default]
    pub touch_move: bool,
    #[prop_or_default]
    pub show_moves: bool,
    #[prop_or_default]
    pub ai: u8,
}

pub enum ChessBoardMessage {
    Click(Coordinate),
    // SetState(ChessBoardState),
}
use ChessBoardMessage::*;

impl Component for ChessBoard {
    type Message = ChessBoardMessage;
    type Properties = ChessBoardProps;

    fn create(ctx: &Context<Self>) -> Self {
        let ai_level = ctx.props().ai;
        let ai = if ai_level > 0 {
            Some(Arc::new(Mutex::new(AiPlayer::new(ai_level))))
        } else {
            None
        };

        Self {
            ai,
            board_state: Arc::new(Mutex::new(Default::default())),
            state: AwaitingInput(NoPieceSelected),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, message: Self::Message) -> bool {
        match message {
            // SetState(new_state) => {
            //     self.state = new_state;
            // }
            Click(clicked) => {
                let current_player = self.board_state.lock().unwrap().get_next_player();
                if !self.is_playable(ctx, current_player) {
                    // TODO: display error
                    return false;
                }
                match self.state {
                    AwaitingInput(NoPieceSelected) => {
                        let destinations: HashSet<_> = self
                            .board_state
                            .lock()
                            .unwrap()
                            .get_legal_moves_from(clicked, current_player)
                            .into_iter()
                            .map(|m| m.to)
                            .collect();

                        if destinations.len() > 0 {
                            self.state = AwaitingInput(PieceSelected {
                                location: clicked,
                                destinations,
                            })
                        } else {
                            // TODO display error
                            return false;
                        }
                    }
                    AwaitingInput(PieceSelected { location, .. }) => {
                        if !ctx.props().touch_move && location == clicked {
                            self.state = AwaitingInput(NoPieceSelected);
                        } else if let Ok(_) = self.board_state.lock().unwrap().try_move(Move {
                            from: location,
                            to: clicked,
                        }) {
                            // TODO
                            self.state = AwaitingInput(NoPieceSelected);
                            // if self.is_playable(ctx, self.board_state.lock().unwrap().get_next_player())
                            // {
                            //     let state = self.board_state.clone();
                            //     let ai = self.ai.clone();
                            //     ctx.link().send_future(async {
                            //         if let Some(ai) = ai {
                            //             if Self::ai_move(state, ai).await {
                            //                 SetState(AwaitingInput)
                            //             } else {
                            //                 SetState(ShowingError(""))
                            //             }
                            //         } else {
                            //             SetState(ShowingError("")) // TODO not if 2-player
                            //         }
                            //     });
                            //     ctx.link().send_message(SetState(AwaitingOtherPlayers));
                            // }
                        } else {
                            // TODO display error
                            return false;
                        }
                    } //
                      // _ => {
                      //     return false;
                      // }
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let current_player = self.board_state.lock().unwrap().get_next_player();
        html! {
            <table class="chess-board">
                <tbody>
                    {for self.board_state.as_ref().lock().unwrap().board.iter().map(|(rank, row)| {
                        let r = rank as u8; // TODO
                        html! {
                            <tr key={r}>
                                <th scope="row" class="rank-label">
                                    {rank}
                                </th>

                                {for row.iter().map(|(file, square)| {
                                    let f = file as u8;
                                    let coordinate = Coordinate{
                                        column: file,
                                        row: rank,
                                    };
                                    let classes = self.get_square_classes(coordinate, square, current_player);
                                    let on_click = ctx.link().callback(move |_e| Click(coordinate));

                                    html! {
                                        <td
                                            key={f}
                                            class={classes}
                                            onclick={on_click}
                                        >
                                            <ChessSquare square={square.clone()} />
                                        </td>
                                    }
                                })}
                            </tr>
                        }
                    })}
                </tbody>
                <tfoot>
                    <tr class="file-labels">
                        <th class="rank-label file-label"></th>
                        {for ColumnIndex::get_columns().iter().enumerate().map(|(index, file)| {
                            html!{
                                <th scope="col" key={index} class="file-label">
                                    {file}
                                </th>
                            }
                        })}
                    </tr>
                </tfoot>
                <caption>
                    {match self.state {
                        AwaitingInput(_) => "Choose your move...",
                        // AwaitingOtherPlayers => "Wait for opponent",
                        // ShowingError(e) => e,
                    }}
                </caption>
            </table>
        }
    }
}

impl ChessBoard {
    fn get_square_classes(
        &self,
        coordinate: Coordinate,
        square: &Square,
        current_player: Colour,
    ) -> Classes {
        let [selected, selectable, can_move_to] = match self.state {
            AwaitingInput(NoPieceSelected) => {
                if let Some(piece) = square {
                    [false, piece.colour == current_player, false]
                } else {
                    [false, false, false]
                }
            }
            AwaitingInput(PieceSelected { location, .. }) if location == coordinate => {
                [true, false, false]
            }
            AwaitingInput(PieceSelected {
                ref destinations, ..
            }) if destinations.contains(&coordinate) => [false, false, true],
            _ => [false, false, false],
        };

        let [is_white_piece, is_black_piece] = match square {
            None => [false, false],
            Some(p) if p.colour == White => [true, false],
            Some(p) if p.colour == Black => [false, true],
            _ => unreachable!(),
        };

        classes! {
            selectable.as_some("selectable"),
            selected.as_some("selected"),
            can_move_to.as_some("can-move-to"),
            is_white_piece.as_some("is-white-piece"),
            is_black_piece.as_some("is-black-piece"),
        }
    }

    fn is_playable(&self, ctx: &Context<Self>, current_player: Colour) -> bool {
        ctx.props()
            .players
            .iter()
            .find(|&&p| p == current_player)
            .is_some()
    }

    // async fn ai_move(state: Arc<Mutex<BoardState>>, ai: Arc<Mutex<AiPlayer>>) -> bool {
    //     let mut state = state.lock().unwrap();
    //     let ai_move = ai.lock().unwrap().get_move(&mut state).unwrap();
    //     state.try_move(ai_move).is_ok() // TODO: keep Result<(), Error>
    // }
}
