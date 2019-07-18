use std::fmt;

use crate::event::Event;

/// A collection of readiness events.
///
/// `Events` is passed as an argument to [`Poll::poll`] and will be used to
/// receive any new readiness events received since the last poll. Usually, a
/// single `Events` instance is created at the same time as a [`Poll`] and
/// reused on each call to [`Poll::poll`].
///
/// See [`Poll`] for more documentation on polling.
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// # fn try_main() -> Result<(), Box<Error>> {
/// use mio::{Events, Poll};
/// use std::time::Duration;
///
/// let mut events = Events::with_capacity(1024);
/// let poll = Poll::new()?;
///
/// assert_eq!(0, events.len());
///
/// // Register `Evented` handles with `poll`
///
/// poll.poll(&mut events, Some(Duration::from_millis(100)))?;
///
/// for event in &events {
///     println!("event={:?}", event);
/// }
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     try_main().unwrap();
/// # }
/// ```
///
/// [`Poll::poll`]: struct.Poll.html#method.poll
/// [`Poll`]: struct.Poll.html
pub struct Events {
    pub(crate) inner: Vec<Event>,
}

/// [`Events`] iterator.
///
/// This struct is created by the [`iter`] method on [`Events`].
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// # fn try_main() -> Result<(), Box<Error>> {
/// use mio::{Events, Poll};
/// use std::time::Duration;
///
/// let mut events = Events::with_capacity(1024);
/// let poll = Poll::new()?;
///
/// // Register handles with `poll`
///
/// poll.poll(&mut events, Some(Duration::from_millis(100)))?;
///
/// for event in events.iter() {
///     println!("event={:?}", event);
/// }
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     try_main().unwrap();
/// # }
/// ```
///
/// [`Events`]: struct.Events.html
/// [`iter`]: struct.Events.html#method.iter
#[derive(Debug, Clone)]
pub struct Iter<'a> {
    inner: &'a Events,
    pos: usize,
}

/// Owned [`Events`] iterator.
///
/// This struct is created by the `into_iter` method on [`Events`].
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// # fn try_main() -> Result<(), Box<Error>> {
/// use mio::{Events, Poll};
/// use std::time::Duration;
///
/// let mut events = Events::with_capacity(1024);
/// let poll = Poll::new()?;
///
/// // Register handles with `poll`
///
/// poll.poll(&mut events, Some(Duration::from_millis(100)))?;
///
/// for event in events {
///     println!("event={:?}", event);
/// }
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     try_main().unwrap();
/// # }
/// ```
/// [`Events`]: struct.Events.html
#[derive(Debug)]
pub struct IntoIter {
    inner: Events,
    pos: usize,
}

impl Events {
    /// Return a new `Events` capable of holding up to `capacity` events.
    ///
    /// # Examples
    ///
    /// ```
    /// use mio::Events;
    ///
    /// let events = Events::with_capacity(1024);
    ///
    /// assert_eq!(1024, events.capacity());
    /// ```
    pub fn with_capacity(capacity: usize) -> Events {
        Events {
            inner: Vec::with_capacity(capacity),
        }
    }

    #[deprecated(
        since = "0.6.10",
        note = "Index access removed in favor of iterator only API."
    )]
    #[doc(hidden)]
    pub fn get(&self, idx: usize) -> Option<Event> {
        self.inner.get(idx).cloned()
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.10",
        note = "Index access removed in favor of iterator only API."
    )]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns the number of `Event` values that `self` can hold.
    ///
    /// ```
    /// use mio::Events;
    ///
    /// let events = Events::with_capacity(1024);
    ///
    /// assert_eq!(1024, events.capacity());
    /// ```
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Returns `true` if `self` contains no `Event` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use mio::Events;
    ///
    /// let events = Events::with_capacity(1024);
    ///
    /// assert!(events.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns an iterator over the `Event` values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use mio::{Events, Poll};
    /// use std::time::Duration;
    ///
    /// let mut events = Events::with_capacity(1024);
    /// let poll = Poll::new()?;
    ///
    /// // Register handles with `poll`
    ///
    /// poll.poll(&mut events, Some(Duration::from_millis(100)))?;
    ///
    /// for event in events.iter() {
    ///     println!("event={:?}", event);
    /// }
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn iter(&self) -> Iter {
        Iter {
            inner: self,
            pos: 0,
        }
    }

    /// Clearing all `Event` values from container explicitly.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use mio::{Events, Poll};
    /// use std::time::Duration;
    ///
    /// let mut events = Events::with_capacity(1024);
    /// let poll = Poll::new()?;
    ///
    /// // Register handles with `poll`
    /// for _ in 0..2 {
    ///     events.clear();
    ///     poll.poll(&mut events, Some(Duration::from_millis(100)))?;
    ///
    ///     for event in events.iter() {
    ///         println!("event={:?}", event);
    ///     }
    /// }
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl<'a> IntoIterator for &'a Events {
    type Item = Event;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        let ret = self.inner.inner.get(self.pos);
        self.pos += 1;
        ret.cloned()
    }
}

impl IntoIterator for Events {
    type Item = Event;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self,
            pos: 0,
        }
    }
}

impl Iterator for IntoIter {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        let ret = self.inner.inner.get(self.pos);
        self.pos += 1;
        ret.cloned()
    }
}

impl fmt::Debug for Events {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Events")
            .field("capacity", &self.capacity())
            .finish()
    }
}