use crate::sim::*;

/// An IO device used in [`MixMachine`].
#[derive(Copy, Clone)]
pub struct IODevice {
    /// Callback function for inputting data to [`MixMachine`].
    ///
    /// # Callback Arguments
    /// * `0` - The mutable memory of the [`MixMachine`].
    /// * `1` - The amount of [`Word<6, false>`]s to input.
    ///
    /// # Callback Return Value
    /// * [`Ok(())`] - All [`Word<6, false>`]s have been written.
    /// * [`Err(())`] - The writing fails.
    pub in_handler: fn(&mut mem::Mem, u16) -> Result<(), ()>,

    /// Callback function for outputting data from [`MixMachine`].
    ///
    /// # Callback Arguments
    /// * `0` - The memory of the [`MixMachine`].
    /// * `1` - The amount of [`Word<6, false>`]s to output.
    ///
    /// # Callback Return Value
    /// * [`Ok(())`] - All [`Word<6, false>`]s have been written.
    /// * [`Err(())`] - The writing fails.
    pub out_handler: fn(&mem::Mem, u16) -> Result<(), ()>,

    /// Callback function for issuing control commands to an [`IODevice`].
    ///
    /// # Callback Arguments
    /// * `0` - The command to the [`IODevice`].
    ///
    /// # Callback Return Value
    /// * [`Ok(())`] - The operation succeeds.
    /// * [`Err(())`] - The operation fails.
    pub control_handler: fn(i16) -> Result<(), ()>,

    /// Callback function for checking if an [`IODevice`] is ready
    /// for a next batch of data.
    ///
    /// # Callback Return Value
    /// * [`Ok(bool)`] - The state of the device.
    /// * [`Err(())`] - The operation fails.
    pub is_ready_handler: fn() -> Result<bool, ()>,

    /// Callback function for checking if an [`IODevice`] is busy
    /// with its own operation.
    ///
    /// # Callback Return Value
    /// * [`Ok(bool)`] - The state of the device.
    /// * [`Err(())`] - The operation fails.
    pub is_busy_handler: fn() -> Result<bool, ()>,
}
