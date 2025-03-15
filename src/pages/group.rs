use backend::api::GetError;
use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};

#[derive(Params, PartialEq)]
struct GroupParams {
    group: Option<String>,
}

#[component]
pub fn GroupPage() -> impl IntoView {
    let param = use_params::<GroupParams>();
    let group = move || {
        param
            .read()
            .as_ref()
            .ok()
            .and_then(|x| x.group.clone())
            .unwrap_or_else(|| "testid".to_string())
    };
    let group_resource = Resource::new(group, async |group| get_group(group).await);
    let group_view = Suspend::new(async move {
        match group_resource.await {
            Ok(Some(g)) => Ok(view! {
                <ul>{move || {
                    g.members.iter()
                        .map(|mem| view!{<li>{mem.name.to_owned()}</li>})
                        .collect::<Vec<_>>()
                }}
                </ul>
            }),
            Ok(None) => Err(GetError::GroupNotFound),
            Err(_) => Err(GetError::ServerError),
        }
    });

    view! {
        <Suspense fallback=move || view!{<p>{"Loading group..."}</p>}>
            <ErrorBoundary fallback=|errors| {
                view! {
                    <div class="error">
                        <h1>"Failure"</h1>
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, err)| view! {<li>{err.to_string()}</li>})
                                    .collect::<Vec<_>>()
                            }}
                        </ul>
                    </div>
                }
            }>{group_view}</ErrorBoundary>
        </Suspense>
    }
}

#[server]
pub async fn get_group(id: String) -> Result<Option<backend::Group>, ServerFnError> {
    Ok(backend::api::get_group(&id))
}
