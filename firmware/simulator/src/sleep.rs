pub(crate) async fn sleep(ms: u64) {
    web_sys::js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 1_000)
            .unwrap();
    })
    .await
    .unwrap();
}
