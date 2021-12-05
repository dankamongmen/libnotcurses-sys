//!

use std::fmt;

/// A little versatile Counter.
#[derive(Debug)]
pub struct Counter {
    /// The starting value.
    pub starting: isize,
    /// The current value.
    pub current: isize,
    /// The amount to increase the current value on each
    /// [`update`][Self#method.update], until it reaches its `limit`.
    pub step: isize,
    /// An optional *limit* value that, when reached, will prevent further
    /// updating the counter.
    ///
    /// If set, it must have the same sign as the `step`. If not set,
    /// the natural limit will be |[`isize::MAX`]|.
    pub limit: Option<isize>,
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{1} {2:+} (from {0}{3})",
            self.starting,
            self.current,
            self.step,
            self.limit.map_or("".into(), |v| format![" to {}", v])
        )
    }
}

impl Counter {
    /// Returns a new `Counter`.
    ///
    /// Note that `limit` and `step` must be both positive or negative together.
    pub fn new(value: isize, step: isize, limit: Option<isize>) -> Self {
        if let Some(lim) = limit {
            assert![(step < 0) == (lim < 0)];
        }
        Self {
            starting: value,
            current: value,
            step,
            limit,
        }
    }

    /// If a `limit` value has been set, returns `true` when the counter has
    /// reached its `limit` value, and if not, increments its `current` value.
    ///
    /// If there's no `limit` value, always returns `false` while incrementing
    /// the `current` value.
    pub fn update(&mut self) -> bool {
        let limit = if let Some(lim) = self.limit {
            lim.abs()
        } else {
            isize::MAX
        };

        if self.current.abs() >= limit {
            true
        } else {
            self.current += self.step;
            false
        }
    }

    /// Like [`update`] but wraps the value if it reaches the [`limit`].
    ///
    /// [`update`]: Counter#method.update
    /// [`limit`]: Counter#field.update
    pub fn wrapping_update(&mut self) -> bool {
        if self.update() {
            self.reset();
            true
        } else {
            false
        }
    }

    /// Returns true if the counter has reached its `limit` value.
    ///
    /// When there's no limit, it will always return `false`.
    pub fn is_done(&self) -> bool {
        if let Some(lim) = self.limit {
            self.current >= lim
        } else {
            false
        }
    }

    /// Returns the `current` value, and sets it at the `starting` value.
    pub fn reset(&mut self) -> isize {
        let old = self.current;
        self.current = self.starting;
        old
    }
}
