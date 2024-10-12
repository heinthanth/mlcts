use leptos::*;
use leptos_meta::{provide_meta_context as use_ctx, *};
use leptos_router::*;

mod demo_page;

use demo_page::DemoPage;

#[component]
pub fn App() -> impl IntoView
{
  use_ctx();

  let html_props = HtmlProps::builder()
    .lang("en")
    .dir("ltr")
    .attributes(vec![("data-theme", "light".into_attribute())])
    .build();

  (Html(html_props), create_router())
}

/// Create Leptos Router
///
/// # Returns
///
/// Router
fn create_router() -> impl IntoView
{
  let routes = vec![create_router_routes_switch()];
  let children = routes.into_iter().map(|c| c.into_view());

  let router_props = RouterProps::builder()
    .children(Box::new(|| Fragment::lazy(|| children.collect())))
    .build();

  return Router(router_props);
}

/// Create Individual Route mapping
///
/// # Returns
///
/// Vector of Route
fn create_routes() -> impl IntoIterator<Item = impl IntoView>
{
  vec![
    RouteProps::builder().path("/").view(DemoPage),
    RouteProps::builder().path("/*").view(DemoPage),
  ]
  .into_iter()
  .map(|props| Route(props.build()))
}

/// Create Router Routes Switch
///
/// # Returns
///
/// Router's Routes Switch
fn create_router_routes_switch() -> impl IntoView
{
  let routes = create_routes().into_iter().map(|route| route.into_view());
  let routes_props = RoutesProps::builder()
    .children(Box::new(|| Fragment::lazy(|| routes.collect())))
    .build();

  return Routes(routes_props);
}
