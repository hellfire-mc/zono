use rspc::Router;

#[derive(Default)]
pub struct RouterBuilder;

impl RouterBuilder {
    /// Build the RSPC router.
    pub fn build(self) -> Router {
        <Router>::new().query("version", |t| t(|ctx, input: ()| env!("CARGO_PKG_VERSION"))).build()
    }
}
