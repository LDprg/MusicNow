use std::{path::PathBuf, sync::LazyLock};

#[cfg(not(target_os = "android"))]
use directories::ProjectDirs;

#[cfg(not(target_os = "android"))]
const PROJECTDIRS: LazyLock<ProjectDirs> = LazyLock::new(|| {
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
fn internal_storage_dir() -> anyhow::Result<PathBuf> {
    use jni::objects::{JObject, JString};
    use jni::JNIEnv;

    let (tx, rx) = std::sync::mpsc::channel();

    fn run(env: &mut JNIEnv<'_>, activity: &JObject<'_>) -> anyhow::Result<PathBuf> {
        let files_dir = env
            .call_method(activity, "getFilesDir", "()Ljava/io/File;", &[])?
            .l()?;
        let files_dir: JString<'_> = env
            .call_method(files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])?
            .l()?
            .into();
        let files_dir: String = env.get_string(&files_dir)?.into();
        Ok(PathBuf::from(files_dir))
    }

    dioxus::mobile::wry::prelude::dispatch(move |env, activity, _webview| {
        tx.send(run(env, activity)).unwrap()
    });

    rx.recv().unwrap()
}

#[cfg(target_os = "android")]
fn internal_cache_dir() -> anyhow::Result<PathBuf> {
    use jni::objects::{JObject, JString};
    use jni::JNIEnv;

    let (tx, rx) = std::sync::mpsc::channel();

    fn run(env: &mut JNIEnv<'_>, activity: &JObject<'_>) -> anyhow::Result<PathBuf> {
        let files_dir = env
            .call_method(activity, "getCacheDir", "()Ljava/io/File;", &[])?
            .l()?;
        let files_dir: JString<'_> = env
            .call_method(files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])?
            .l()?
            .into();
        let files_dir: String = env.get_string(&files_dir)?.into();
        Ok(PathBuf::from(files_dir))
    }

    dioxus::mobile::wry::prelude::dispatch(move |env, activity, _webview| {
        tx.send(run(env, activity)).unwrap()
    });

    rx.recv().unwrap()
}

