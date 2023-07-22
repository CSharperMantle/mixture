use crate::sim::FullWord;

/// A device plugged into a [`MixMachine`] to perform IO
/// operations.
///
/// This trait is used to build IO operations that may have side
/// effects. Implement the trait and insert the device to a [`MixMachine`]
/// instance to apply it.
///
/// # Example
/// ```rust
/// use mixture::sim::IODevice;
/// use mixture::sim::MixMachine;
/// use mixture::sim::FullWord;
///
/// pub struct SomeDevice {}
///
/// impl IODevice for SomeDevice {
///     fn read(&mut self, buffer: &mut [FullWord]) -> Result<(), ()> {
///         /* ... */
///         todo!()
///     }
///     fn write(&mut self, data: &[FullWord]) -> Result<(), usize> {
///         /* ... */
///         todo!()
///     }
///     fn control(&mut self, command: i16) -> Result<(), ()> {
///         /* ... */
///         todo!()
///     }
///     fn is_busy(&self) -> Result<bool, ()> {
///         /* ... */
///         todo!()
///     }
///     fn is_ready(&self) -> Result<bool, ()> {
///         /* ... */
///         todo!()
///     }
///     fn get_block_size(&self) -> usize {
///         /* ... */
///         todo!()
///     }
/// }
///
/// let mut mix = MixMachine::new();
/// mix.reset();
/// mix.io_devices[0] = Some(Box::new(SomeDevice {}));
/// ```
pub trait IODevice {
    /// Read a block of [`FullWord`]s from the device into the buffer.
    ///
    /// The amount of words in a block is defined by the device
    /// via [`IODevice::get_block_size`]. This method must write
    /// exactly one block of words on success, otherwise it will
    /// fail.
    ///
    /// The implementor is asked to check the size of provided `buffer`
    /// to avoid possible out-of-bound access.
    ///
    /// # Arguments
    /// * `buffer` - The buffer to read into.
    fn read(&mut self, buffer: &mut [FullWord]) -> Result<(), ()>;

    /// Write a block of [`FullWord`]s out through the device.
    ///
    /// This method will always try to write a whole block. It will fail
    /// if the given slice of data has a length that is not exactly equal
    /// to the block size. On the case of non-rolling-back write failures,
    /// the actual amount of words already written is returned.
    ///
    /// # Arguments
    /// * `data` - The words to write.
    fn write(&mut self, data: &[FullWord]) -> Result<(), usize>;

    /// Issue a control command to the device.
    ///
    /// # Arguments
    /// * `command` - The command to issue.
    fn control(&mut self, command: i16) -> Result<(), ()>;

    /// Check if the device is busy.
    ///
    /// Note that when a device detects any malfunctions, like
    /// paper jams, it will always appear busy.
    fn is_busy(&self) -> Result<bool, ()>;

    /// Check if the device is ready for next operations.
    fn is_ready(&self) -> Result<bool, ()>;

    /// Get the count of [`FullWord`]s in a device block,
    /// that is, read or written in a single operation.
    fn get_block_size(&self) -> usize;
}
