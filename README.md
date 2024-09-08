# Hydration Bug Investigation

This project is dedicated to investigating and resolving a peculiar hydration bug encontenred in a project.

## Overview

After the page loads the WASM and starts hydrating the app, some `Resource` returns with bogus data.
Sometimes it affects the displayed HTML, sometimes it doesn't.

## Current Status

We are actively investigating the issue. It appears to only manifest in the first page load, subsequent client side navigation does not exhibit the issue.

## Steps to Reproduce

1. Build and serve the project with `cargo leptos serve`
2. Go in a browser to http://localhost:8000/pacientes/pppppppp/atendimentos/em3yp8hh/evolucoes/odz3esz5
3. Notice that in the browser console output there's a `PacienteCadastroSummary` with wrong data, namely:

```
paciente: Some(
    Ok(
        PacienteCadastroSummary {
            id: PacienteId(
                "odz3esz5",
            ),
            nome: None,
            data_nascimento: None,
            cpf: None,
        },
    ),
)
```

Notice that the `id` is incorrect, getting the `evolucao_id` instead, and not getting our expected output.

Expected Output:

```
paciente: Some(
    Ok(
        PacienteCadastroSummary {
            id: PacienteId("pppppppp"),
            nome: Some("John Doe"),
            data_nascimento: None,
            cpf: Some("999.999.999-99"),
        },
    ),
)
```
4. Note the browser output:
  ![firefox_LQo00m9UMb](https://github.com/user-attachments/assets/bcb734ad-96a3-43a6-902a-ba1132ac8602) <br />
  Should be: <br />
  ![9z12njg72E](https://github.com/user-attachments/assets/8d636ba3-f5b7-4d82-8a01-e399f09b2c78)

## Potential Causes

- Maybe some interaction between the Ids (wrapped `[u8; 8]`) and serde.
