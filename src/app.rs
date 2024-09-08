use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags};
use leptos_router::{components::*, path};

use leptos::either::Either;
use serde::{Deserialize, Serialize};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html> 
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=|| ()>
                <Route path=path!("") view=Home />
                <ParentRoute path=path!("") view=DrawerAtendimentosMember>
                    <Route
                        path=path!(
                            "/pacientes/:paciente_id/atendimentos/:atendimento_id/evolucoes/:evolucao_id"
                        )
                        view=VisualizarEvolucao
                    />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <A href="/pacientes/pppppppp/atendimentos/aaaaaaaa/evolucoes/eeeeeeee">
            Go to the bug page (client side navigation)
        </A>
        <br />
        <a href="/pacientes/pppppppp/atendimentos/aaaaaaaa/evolucoes/eeeeeeee">
            Go to the bug page (browser native navigation)
        </a>
    }
}

#[component]
pub fn VisualizarEvolucao() -> impl IntoView {
    let evolucao_id = query_evolucao_id();

    let evolucao = get_evolucao(evolucao_id.into());

    view! {
        <Suspense>
            {move || {
                let evolucao = evolucao.get();
                match evolucao {
                    Some(Ok(_evolucao)) => {
                        Either::Left(
                            view! {
                                <h1 class="text-2xl font-bold">Evolução</h1>
                                <Cabecalho />
                            },
                        )
                    }
                    _ => Either::Right(view! { <div>Erro ao carregar evolução</div> }),
                }
            }}
        </Suspense>
        <A href="/">Go to the home page</A>
        <br />
        <a href="/">Go to the home page (browser native navigation)</a>
    }
}

#[component]
pub fn Cabecalho() -> impl IntoView {
    let paciente_id = query_paciente_id();
    let paciente = get_paciente_cadastro_summary(paciente_id.into());

    view! {
        <Suspense>
            {move || {
                let paciente = paciente.get();
                tracing::debug!("paciente: {:#?}", paciente);
                match paciente {
                    Some(Ok(paciente)) => {
                        let paciente_clone = paciente.clone();
                        let paciente_id_clone = paciente.id.clone();
                        tracing::debug!("paciente: {:#?}", paciente);
                        Either::Left({
                            view! {
                                <p>Paciente: {paciente.nome}</p>
                                <p>Id: {paciente.id.to_string()}</p>
                                <PingResult paciente=paciente_clone />
                                <p>
                                    Client Side Data Status:
                                    {move || {
                                        if paciente.id == "pppppppp" {
                                            tracing::debug!("Data Status CORRECT");
                                            Either::Left(view! { <span>CORRECT</span> })
                                        } else {
                                            tracing::debug!("Data Status INCORRECT");
                                            Either::Right(view! { <span>INCORRECT</span> })
                                        }
                                    }}
                                </p>
                                {move || {
                                    tracing::warn!(
                                        "paciente id correct?: {:#?}", paciente_id_clone == "pppppppp"
                                    )
                                }}
                            }
                        })
                    }
                    _ => Either::Right(view! { <div>Erro ao carregar paciente</div> }),
                }
            }}
        </Suspense>
    }
}

#[component]
fn PingResult(paciente: PacienteCadastroSummary) -> impl IntoView {
    let ping_action = Action::new(|paciente: &PacienteCadastroSummary| {
        let paciente = paciente.clone();
        async move { srv_paciente_cadastro_summary_ping(paciente).await }
    });

    let paciente = paciente.clone();
    let _ = ping_action.dispatch(paciente);

    view! {
        <Suspense>

            <p>
                Server Ping:
                {move || {
                    let ping_result = ping_action.value().get();
                    tracing::debug!("ping_result: {:#?}", ping_result);
                    ping_result.map(|result| result.unwrap_or_default()).unwrap_or_default()
                }}
            </p>

        </Suspense>
    }
}

#[component]
pub fn DrawerAtendimentosMember() -> impl IntoView {
    let Sidebar = || {
        let evolucao_id = query_evolucao_id();

        let _evolucao = get_evolucao(evolucao_id.into());

        view! {
            <Suspense>
                <p>SIDEBAR</p>
            </Suspense>
        }
    };

    // Works
    /*
    view! {
        <Outlet />

        <Sidebar />
    }
   */

    // Don't work (manifests bug)
    view! {
        <Outlet />

        {Sidebar}
    }
}

type PacienteId = String;
type EvolucaoId = String;

pub fn query_paciente_id() -> Memo<Result<PacienteId, String>> {
    Memo::new( |_| {
        Ok(PacienteId::from("pppppppp"))
    })
}

pub fn query_evolucao_id() -> Memo<Result<EvolucaoId, String>> {
    Memo::new(|_| {
        Ok(EvolucaoId::from("eeeeeeee"))
    })
}

pub fn get_paciente_cadastro_summary(
    query_id: MaybeSignal<Result<PacienteId, String>>,
) -> Resource<Result<PacienteCadastroSummary, String>> {
    Resource::new(
        move || query_id.get(),
        |query_id| async move {
            match query_id {
                Ok(paciente_id) => srv_load_paciente_cadastro_summary(paciente_id)
                    .await
                    .map_err(|e| e.to_string()),
                Err(e) => Err(e),
            }
        },
    )
}

pub fn get_evolucao(
    evolucao_id: MaybeSignal<Result<EvolucaoId, String>>,
) -> Resource<Result<EvolucaoSoapFull, String>> {
    Resource::new(
        move || evolucao_id.get(),
        |query_id| async move {
            match query_id {
                Ok(evolucao_id) => srv_load_evolucao(evolucao_id)
                    .await
                    .map_err(|e| e.to_string()),
                Err(e) => Err(e),
            }
        },
    )
}

#[server(LoadPacienteCadastroSummarySrv)]
pub async fn srv_load_paciente_cadastro_summary(
    paciente_id: PacienteId,
) -> Result<PacienteCadastroSummary, ServerFnError<String>> {
    // fake API delay
    tokio::time::sleep(std::time::Duration::from_millis(250)).await;

    Ok(PacienteCadastroSummary {
        id: paciente_id,
        nome: Some("John Doe".to_string()),
    })
}

#[server(PacienteCadastroSummaryPingSrv)]
pub async fn srv_paciente_cadastro_summary_ping(
    paciente: PacienteCadastroSummary,
) -> Result<String, ServerFnError<String>> {
    let status = if paciente.id == "pppppppp" {
        "CORRECT"
    } else {
        "INCORRECT"
    };
    Ok("Pong paciente_id: ".to_string()
        + &paciente.id.to_string()
        + "; Server Side Data Status: "
        + status)
}

#[server(LoadLastEvolucaoSrv)]
async fn srv_load_evolucao(
    evolucao_id: EvolucaoId,
) -> Result<EvolucaoSoapFull, ServerFnError<String>> {
    // fake API delay
    tokio::time::sleep(std::time::Duration::from_millis(250)).await;

    Ok(EvolucaoSoapFull {
        id: evolucao_id,
        paciente_id: PacienteId::from("pppppppp"),
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PacienteCadastroSummary {
    pub id: PacienteId,
    pub nome: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvolucaoSoapFull {
    pub id: EvolucaoId,
    pub paciente_id: PacienteId,
}
