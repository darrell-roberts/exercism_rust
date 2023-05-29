use std::ops::Not;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

/// The result for a Frame.
#[derive(Debug, Copy, Clone)]
enum FrameResult {
    /// Single roll 10 pins.
    Strike,
    /// Two throws resulting in 10 pins retaining the total pins for the first throw.
    Spare(u16),
    /// The total pins for the first and second throw that totals under 10 pins.
    Open(u16, u16),
}

impl FrameResult {
    /// Yield all the rolls for a completed frame.
    fn rolls(&self) -> impl Iterator<Item = u16> {
        let it: Box<dyn Iterator<Item = u16>> = match self {
            FrameResult::Strike => Box::new(std::iter::once(10)),
            FrameResult::Spare(f) => Box::new([*f, 10 - *f].into_iter()),
            FrameResult::Open(f, s) => Box::new([*f, *s].into_iter()),
        };
        it
    }
}

#[derive(Debug, Copy, Clone)]
enum Frame {
    /// Frame is in progress and the first roll has been made.
    Open(u16),
    /// Frame is finished.
    Closed(FrameResult),
}

impl Frame {
    /// Is the frame in a closed state.
    fn is_complete(&self) -> bool {
        match self {
            Frame::Open(_) => false,
            Frame::Closed(_) => true,
        }
    }
}

#[derive(Debug, Default)]
pub struct BowlingGame {
    frames: Vec<Frame>,
}

impl BowlingGame {
    /// Start a new Bowling game.
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    /// Yield all the frame results.
    fn results(&self) -> impl Iterator<Item = &FrameResult> {
        self.frames.iter().flat_map(|f| match f {
            Frame::Open(_) => None,
            Frame::Closed(r) => Some(r),
        })
    }

    /// Get the open frame for modification.
    fn open_frame(&mut self) -> Option<&mut Frame> {
        self.frames.last_mut().filter(|f| !f.is_complete())
    }

    /// Is the game complete.
    fn is_complete(&self) -> bool {
        let tenth_frame = self.frames.get(9);

        let has_n_fill_rolls = |rolls| {
            let mut rolls_counted = 0;

            for result in self.results().skip(10).take(rolls) {
                match result {
                    FrameResult::Strike => rolls_counted += 1,
                    FrameResult::Spare(_) => rolls_counted += 2,
                    FrameResult::Open(_, _) => rolls_counted += 2,
                }
            }

            rolls_counted >= rolls
        };

        match tenth_frame {
            Some(Frame::Closed(FrameResult::Open(..))) => true,
            Some(Frame::Closed(FrameResult::Strike)) => has_n_fill_rolls(2),
            Some(Frame::Closed(FrameResult::Spare(_))) => has_n_fill_rolls(1),
            Some(Frame::Open(..)) => false,
            None => false,
        }
    }

    /// Handle a player roll.
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        (pins <= 10).then_some(()).ok_or(Error::NotEnoughPinsLeft)?;

        self.is_complete()
            .not()
            .then_some(())
            .ok_or(Error::GameComplete)?;

        if let Some(f @ Frame::Open(_)) = self.open_frame() {
            match &f {
                Frame::Open(first_throw) => {
                    let result = match first_throw + pins {
                        n if n > 10 => return Err(Error::NotEnoughPinsLeft),
                        10 => FrameResult::Spare(*first_throw),
                        _ => FrameResult::Open(*first_throw, pins),
                    };
                    *f = Frame::Closed(result);
                }
                Frame::Closed(_) => (),
            }
        } else {
            // is this a bonus frame?
            let tenth_frame_result =
                (self.frames.len() == 10)
                    .then(|| self.frames[9])
                    .and_then(|f| match f {
                        Frame::Open(_) => None,
                        Frame::Closed(r) => Some(r),
                    });

            let frame = match (tenth_frame_result, 10 == pins) {
                (None, true) => Frame::Closed(FrameResult::Strike),
                (None, false) => Frame::Open(pins),
                (Some(FrameResult::Strike), true) => Frame::Closed(FrameResult::Strike),
                (Some(FrameResult::Strike), false) => Frame::Open(pins),
                (Some(FrameResult::Spare(_)), true) => Frame::Closed(FrameResult::Strike),
                (Some(FrameResult::Spare(_)), false) => Frame::Closed(FrameResult::Open(pins, 0)),
                (Some(FrameResult::Open(_, _)), _) => return Err(Error::GameComplete),
            };

            self.frames.push(frame);
        }

        Ok(())
    }

    /// Compute the total score at the end of the game.
    pub fn score(&self) -> Option<u16> {
        self.is_complete().then_some(())?;

        // Get the score for the next n rolls.
        let next_rolls = |current_frame, rolls| {
            let (rolls_score, rolls_counted) = self
                .results()
                .skip(current_frame)
                .flat_map(|result| result.rolls())
                .take(rolls)
                .zip(1..)
                .fold((0, 0), |(rolls, counted), (r, _)| (rolls + r, counted + 1));

            (rolls_counted == rolls).then_some(rolls_score)
        };

        let mut score = 0;

        // Traverse the 10 frames and sum up the total score according
        // to bowling scoring rules.
        for (result, frame_num) in self.results().take(10).zip(1..) {
            match result {
                FrameResult::Strike => {
                    score += 10 + next_rolls(frame_num, 2)?;
                }
                FrameResult::Spare(_) => score += 10 + next_rolls(frame_num, 1)?,
                FrameResult::Open(f, s) => score += f + s,
            }
        }

        Some(score)
    }
}
