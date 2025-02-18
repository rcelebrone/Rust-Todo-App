# Todo List

Um aplicativo para gerenciar tarefas, feito em Rust e com frontend em HTMX.

## Funcionalidades

- Criação de tarefas
- Edição de tarefas
- Exclusão de tarefas
- Marcação de tarefas como concluídas
- Filtro de tarefas por status (tudo, ativo, concluído)

## Backend

O backend é feito em Rust, utilizando a biblioteca Rocket para criar uma API REST.

### Endpoints

- `GET /todos`: Retorna a lista de tarefas
- `POST /todos`: Cria uma nova tarefa
- `GET /todos/:id`: Retorna uma tarefa específica
- `PUT /todos/:id`: Edita uma tarefa específica
- `DELETE /todos/:id`: Exclui uma tarefa específica

### Modelagem de dados

A modelagem de dados é feita utilizando a biblioteca `serde` para serializar e deserializar os dados.

## Frontend

O frontend é feito em HTMX, utilizando a biblioteca `htmx` para criar rotas e fazer requisições AJAX.

## Executar o projeto

Para executar o projeto, basta clonar o repositório e executar o comando `cargo run` no diretório do backend.

## Contribuir

Se você quiser contribuir com o projeto, basta criar um fork do repositório e enviar um pull request com as alterações.
