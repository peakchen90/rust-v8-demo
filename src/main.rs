fn main() {
    // 初始化
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
    let handle_scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(handle_scope);
    let scope = &mut v8::ContextScope::new(handle_scope, context);

    // let code = v8::String::new(scope, "'hello'+' world'").unwrap();
    let code = v8::String::new(
        scope,
        r#"
        function max(...a) {
            return Math.max(...a);
        }

        max(1,5,2,6,3)
    "#,
    )
    .unwrap();
    let script = v8::Script::compile(scope, code, None).unwrap();

    // script.

    // let code2 = v8::String::new(scope, "max(5,2,7,3)").unwrap();

    // let script = v8::Script::compile(scope, code2, None).unwrap();
    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap();
    println!("{}", result.to_rust_string_lossy(scope));
}
