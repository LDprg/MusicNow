use std::{path::PathBuf, sync::LazyLock};

#[cfg(not(target_os = "android"))]
use directories::ProjectDirs;

#[cfg(not(target_os = "android"))]
static PROJECTDIRS: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("com", "LDprg", "MusicNow").expect("Couldn't create application dirs.")
});

pub fn get_cache_dir() -> PathBuf {
    #[cfg(not(target_os = "android"))]
    return PROJECTDIRS.cache_dir().to_path_buf();
    #[cfg(target_os = "android")]
    return PathBuf::from(internal_cache_dir().expect("Faild to get cache dir"));
}

pub fn get_data_dir() -> PathBuf {
    #[cfg(not(target_os = "android"))]
    return PROJECTDIRS.data_dir().to_path_buf();
    #[cfg(target_os = "android")]
    return PathBuf::from(internal_storage_dir().expect("Faild to get storage dir"));
}

#[cfg(target_os = "android")]
fn get_jni_app_dir(
    context: &jni::objects::JObject<'_>,
    env: &mut jni::JNIEnv<'_>,
    method: &str,
) -> anyhow::Result<String> {
    let dir = env
        .call_method(context, method, "()Ljava/io/File;", &[])?
        .l()?;

    let path_string = env
        .call_method(dir, "getPath", "()Ljava/lang/String;", &[])?
        .l()?;
    let path_string = jni::objects::JString::from(path_string);
    let path_string = env.get_string(&path_string)?;

    Ok(path_string.into())
}

#[cfg(target_os = "android")]
fn internal_storage_dir() -> anyhow::Result<String> {
    let android_context = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(android_context.vm().cast()) }?;
    let mut env = vm.attach_current_thread()?;
    let context = unsafe { jni::objects::JObject::from_raw(android_context.context().cast()) };

    get_jni_app_dir(&context, &mut env, "getFilesDir")
}

#[cfg(target_os = "android")]
fn internal_cache_dir() -> anyhow::Result<String> {
    let android_context = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(android_context.vm().cast()) }?;
    let mut env = vm.attach_current_thread()?;
    let context = unsafe { jni::objects::JObject::from_raw(android_context.context().cast()) };

    get_jni_app_dir(&context, &mut env, "getCacheDir")
}
