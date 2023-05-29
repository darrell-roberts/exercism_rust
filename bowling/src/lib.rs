use std::ops::Not;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Copy, Clone)]
enum FrameResult {
    Strike,
    Spare(u16),
    Open(u16, u16),
}

#[derive(Debug, Copy, Clone)]
enum Frame {
    Open(u16),
    Closed(FrameResult),
}

impl Frame {
    fn is_complete(&self) -> bool {
        // self.result.is_some()
        match self {
            Frame::Open(_) => false,
            Frame::Closed(_) => true,
        }
    }
}

#[derive(Debug, Default)]
pub struct BowlingGame {
    frames: Vec<Frame>,
    total_rolls: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            total_rolls: 0,
        }
    }

    fn results(&self) -> impl Iterator<Item = &FrameResult> {
        self.frames.iter().flat_map(|f| match f {
            Frame::Open(_) => None,
            Frame::Closed(r) => Some(r),
        })
    }

    fn open_frame(&mut self) -> Option<&mut Frame> {
        self.frames.last_mut().filter(|f| !f.is_complete())
    }

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

        self.total_rolls += 1;

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        dbg!(&self.frames);

        let bonus_frame = self
            .results()
            .nth(9)
            .map(|r| matches!(r, FrameResult::Strike | FrameResult::Spare(_)))
            .unwrap_or(false);

        let total_frames = self.frames.len();

        (total_frames == 10 || bonus_frame).then_some(())?;

        let next_rolls = |current_frame, rolls| {
            let mut rolls_score = 0;
            let mut rolls_counted = 0;

            for result in self.results().skip(current_frame) {
                if rolls_counted == rolls {
                    break;
                }
                println!("next {rolls} rolls {result:?}");
                match result {
                    FrameResult::Strike => {
                        rolls_score += 10;
                        rolls_counted += 1;
                    }
                    FrameResult::Spare(f) => {
                        rolls_score += if rolls == 1 { *f } else { 10 };
                        rolls_counted = rolls;
                    }
                    FrameResult::Open(f, s) => {
                        rolls_score += if rolls == 1 || rolls_counted == 1 {
                            *f
                        } else {
                            *f + *s
                        };
                        rolls_counted = rolls;
                    }
                }
            }
            println!("rolls_score: {rolls_score}");
            (rolls_counted == rolls).then_some(rolls_score)
        };

        let mut score: u16 = 0;

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
