//! Statically dispatched, borrow-preserving model contracts.

/// A model evaluated at fixed-width parameter samples.
pub trait StudyModel<T, const PARAMETERS: usize>: Sync {
    /// Model failure.
    type Error;
    /// Borrowed or owned response family.
    type Response<'a>
    where
        Self: 'a,
        T: 'a;

    /// Evaluate one sample.
    ///
    /// # Errors
    ///
    /// Returns the model's typed failure.
    fn evaluate<'a>(
        &'a self,
        parameters: &'a [T; PARAMETERS],
    ) -> Result<Self::Response<'a>, Self::Error>;
}

/// Convert a possibly borrowed response into an owned result slot.
pub trait ResponseReducer<M, T, const PARAMETERS: usize>
where
    M: StudyModel<T, PARAMETERS>,
{
    /// Owned output.
    type Output;

    /// Reduce before the response borrow ends.
    fn reduce<'a>(&self, response: M::Response<'a>) -> Self::Output
    where
        M: 'a,
        T: 'a;
}

/// A model fitted on a resample that retains its trained state.
///
/// The implementor holds hyperparameters; [`Self::fit`] returns the trained
/// state, which the caller keeps and passes to every [`Self::predict`].
/// [`crate::Bagging`] fits one state per bootstrap resample, and
/// [`crate::BaggedModel::predict`] calls only [`Self::predict`], never
/// [`Self::fit`]. The trait adds no bound on `Fitted`, so a state may be the
/// observations themselves.
///
/// # Examples
///
/// ```
/// use tyche_core::TrainableModel;
///
/// /// Fitted state: the sum of the resampled observations.
/// struct Total;
///
/// impl TrainableModel<f64> for Total {
///     type Observation = f64;
///     type Input = ();
///     type Fitted = f64;
///     type Error = core::convert::Infallible;
///
///     fn fit(&self, data: &[f64], resample: &[usize]) -> Result<f64, Self::Error> {
///         Ok(resample.iter().map(|&index| data[index]).sum())
///     }
///
///     fn predict(&self, fitted: &f64, _: &()) -> Result<f64, Self::Error> {
///         Ok(*fitted)
///     }
/// }
///
/// let fitted = Total.fit(&[1.0, 3.0], &[0, 1, 1]).unwrap();
/// assert_eq!(Total.predict(&fitted, &()).unwrap(), 7.0);
/// ```
pub trait TrainableModel<T> {
    /// One training observation.
    type Observation;
    /// Prediction input.
    type Input: ?Sized;
    /// Trained state retained between predictions.
    type Fitted;
    /// Fitting or prediction failure.
    type Error;

    /// Fit on the observations selected by `resample`, whose indices are below
    /// `observations.len()` and repeat under sampling with replacement.
    ///
    /// # Errors
    ///
    /// Returns the model's typed failure.
    fn fit(
        &self,
        observations: &[Self::Observation],
        resample: &[usize],
    ) -> Result<Self::Fitted, Self::Error>;

    /// Predict from retained trained state.
    ///
    /// # Errors
    ///
    /// Returns the model's typed failure.
    fn predict(&self, fitted: &Self::Fitted, input: &Self::Input) -> Result<T, Self::Error>;
}
