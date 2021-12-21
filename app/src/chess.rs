use std::collections::HashSet;

use boolinator::Boolinator;
use yew::{classes, prelude::*};

use engines::chess::{
    ai::AiPlayer,
    board::{
        coordinates::{Coordinate, Move},
        piece::Colour,
        BoardState,
    },
    Player,
};

pub use Colour::*;

#[derive(Debug)]
pub struct ChessBoard {
    link: ComponentLink<Self>,
    props: ChessBoardProps,
    state: BoardState,
    selected: Option<Coordinate>,
    destinations: HashSet<Coordinate>,
    ai: Option<AiPlayer>,
}

#[derive(Debug, Clone, Properties)]
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
}
use ChessBoardMessage::*;

const FILES: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']; // TODO: get from engine

impl Component for ChessBoard {
    type Message = ChessBoardMessage;
    type Properties = ChessBoardProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let ai = if props.ai > 0 {
            Some(AiPlayer::new(props.ai))
        } else {
            None
        };
        Self {
            link,
            props,
            ai,
            state: Default::default(),
            selected: Default::default(),
            destinations: Default::default(),
        }
    }

    fn update(&mut self, message: Self::Message) -> bool {
        match message {
            Click(clicked) => {
                let current_player = self.state.get_next_player();
                if !self.is_playable(current_player) {
                    // TODO: display error
                    return false;
                }
                if let Some(selected) = self.selected {
                    if !self.props.touch_move && selected == clicked {
                        self.selected = None;
                        self.destinations.clear();
                    } else if let Ok(_) = self.state.try_move(Move {
                        from: selected,
                        to: clicked,
                    }) {
                        self.selected = None;
                        self.destinations.clear();
                        // TODO: await next move?
                        // self.await_move();
                        if let Some(ai) = &mut self.ai {
                            let ai_move = ai.get_move(&mut self.state);
                            self.state.try_move(ai_move.unwrap()).unwrap();
                        }
                    } else {
                        // TODO display error
                        return false;
                    }
                } else {
                    self.destinations = self
                        .state
                        .get_legal_moves_from(clicked, current_player)
                        .into_iter()
                        .map(|m| m.to)
                        .collect();

                    if self.destinations.len() > 0 {
                        self.selected = Some(clicked);
                    } else {
                        // TODO display error
                        return false;
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, _new_props: Self::Properties) -> bool {
        // TODO
        false
    }

    fn view(&self) -> Html {
        let current_player = self.state.get_next_player();
        let is_playable = self.is_playable(current_player);

        html! {
            <table class="chess-board">
                <tbody>
                    {for self.state.board.iter().map(|(rank, row)| {
                        let r = rank as u8; // TODO
                        html! {
                            <tr key={r}>
                                <th class="rank-label">
                                    {r}
                                </th>
                                {for row.iter().map(|(file, square)| {
                                    let f = file as u8;
                                    let coordinate = Coordinate{
                                        column: file,
                                        row: rank,
                                    };

                                    let selected = match self.selected {
                                        Some(c) if c  == coordinate => true,
                                        _ => false
                                    };
                                    let selectable = is_playable && self.selected.is_none() && match square {
                                        Some(piece) if piece.colour == current_player => true,
                                        _ => false,
                                    };
                                    let can_move_to = self.props.show_moves && self.destinations.contains(&coordinate);
                                    let classes = classes!{
                                        selectable.as_some("selectable"),
                                        selected.as_some("selected"),
                                        can_move_to.as_some("can-move-to")
                                    };

                                    let on_click = self.link.callback(move |_e| Click(coordinate));

                                    html! {
                                        <td
                                            key={f}
                                            class={classes}
                                            onclick={on_click}
                                        >
                                            {square.and_then(|piece| Some(piece.to_string())).unwrap_or(String::new())}
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

impl ChessBoard {
    fn is_playable(&self, current_player: Colour) -> bool {
        self.props
            .players
            .iter()
            .find(|&&p| p == current_player)
            .is_some()
    }

    async fn await_move(&mut self) {
        if let Some(ai) = &mut self.ai {
            let ai_move = ai.get_move(&mut self.state);
            self.state.try_move(ai_move.unwrap()).unwrap();
        }
    }
}
