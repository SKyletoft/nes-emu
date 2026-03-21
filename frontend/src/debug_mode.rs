#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundView {
	Both,
	Bg1Only,
	Bg2Only,
}

impl BackgroundView {
	pub fn next(self) -> Self {
		match self {
			BackgroundView::Both => BackgroundView::Bg1Only,
			BackgroundView::Bg1Only => BackgroundView::Bg2Only,
			BackgroundView::Bg2Only => BackgroundView::Both,
		}
	}

	pub fn prev(self) -> Self {
		match self {
			BackgroundView::Both => BackgroundView::Bg2Only,
			BackgroundView::Bg1Only => BackgroundView::Both,
			BackgroundView::Bg2Only => BackgroundView::Bg1Only,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugMode {
	Disabled,
	Backgrounds(BackgroundView),
	Sprites(u8 /* is 0..64 */),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugBackgroundMode {
	Black,
	White,
	Checkerboard,
	Palette0,
}

impl DebugBackgroundMode {
	pub fn next(self) -> Self {
		match self {
			Self::Black => Self::White,
			Self::White => Self::Checkerboard,
			Self::Checkerboard => Self::Palette0,
			Self::Palette0 => Self::Black,
		}
	}
}
