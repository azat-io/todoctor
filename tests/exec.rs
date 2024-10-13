use todoctor::exec::exec;

#[tokio::test]
async fn test_exec_command() {
    let stdout = exec(&["echo", "hello"]).await.unwrap();
    assert_eq!(stdout, "hello\n");
}

#[tokio::test]
async fn test_exec_command_failure() {
    let result = exec(&["non_existent_command"]).await;
    assert!(result.is_err());
}
