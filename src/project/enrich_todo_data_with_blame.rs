use crate::comments::prepare_blame_data::{
    prepare_blame_data, PreparedBlameData,
};
use crate::git::{get_blame_data, get_line_from_position};
use crate::types::{TodoData, TodoWithBlame};
use tokio::fs;

pub async fn enrich_todo_data_with_blame(
    todo_data: Vec<TodoData>,
) -> Vec<TodoWithBlame> {
    let todo_with_blame_tasks: Vec<_> = todo_data
        .into_iter()
        .map(|todo| {
            tokio::spawn(async move {
                if let Ok(source_code) = fs::read_to_string(&todo.path).await {
                    if let Some(line) =
                        get_line_from_position(todo.start, &source_code)
                    {
                        if let Some(blame_data) =
                            get_blame_data(&todo.path, line).await
                        {
                            let prepared_blame: PreparedBlameData =
                                prepare_blame_data(blame_data);
                            return Some(TodoWithBlame {
                                comment: todo.comment.trim().to_string(),
                                path: todo.path.clone(),
                                blame: prepared_blame,
                                kind: todo.kind,
                                line,
                            });
                        }
                    }
                }
                None
            })
        })
        .collect();

    let mut todos_with_blame: Vec<TodoWithBlame> = Vec::new();
    for task in todo_with_blame_tasks {
        if let Ok(Some(todo)) = task.await {
            todos_with_blame.push(todo);
        }
    }

    todos_with_blame.sort_by(|a, b| a.blame.date.cmp(&b.blame.date));

    todos_with_blame
}
