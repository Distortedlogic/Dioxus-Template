use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize)]
pub struct DioxusConfig {
	pub application: ApplicationConfig,

	#[serde(default)]
	pub web: WebConfig,

	#[serde(default)]
	pub desktop: DesktopConfig,

	#[serde(default)]
	pub bundle: BundleConfig,
}

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
	#[serde(default = "asset_dir_default")]
	pub asset_dir: PathBuf,

	#[serde(default)]
	pub sub_package: Option<String>,
}

pub fn asset_dir_default() -> PathBuf {
	PathBuf::from("assets")
}

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
	#[serde(default)]
	pub app: WebAppConfig,

	#[serde(default)]
	pub proxy: Vec<WebProxyConfig>,

	#[serde(default)]
	pub watcher: WebWatcherConfig,

	#[serde(default)]
	pub resource: WebResourceConfig,

	#[serde(default)]
	pub https: WebHttpsConfig,

	/// Whether to enable pre-compression of assets and wasm during a web build in release mode
	#[serde(default = "true_bool")]
	pub pre_compress: bool,

	/// The wasm-opt configuration
	#[serde(default)]
	pub wasm_opt: WasmOptConfig,
}

impl Default for WebConfig {
	fn default() -> Self {
		Self {
			pre_compress: true_bool(),
			app: Default::default(),
			https: Default::default(),
			wasm_opt: Default::default(),
			proxy: Default::default(),
			watcher: Default::default(),
			resource: Default::default(),
		}
	}
}

/// The wasm-opt configuration
#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize, Default)]
pub struct WasmOptConfig {
	/// The wasm-opt level to use for release builds [default: s]
	/// Options:
	/// - z: optimize aggressively for size
	/// - s: optimize for size
	/// - 1: optimize for speed
	/// - 2: optimize for more for speed
	/// - 3: optimize for even more for speed
	/// - 4: optimize aggressively for speed
	#[serde(default)]
	pub level: WasmOptLevel,

	/// Keep debug symbols in the wasm file
	#[serde(default = "false_bool")]
	pub debug: bool,
}

/// The wasm-opt level to use for release web builds [default: 4]
#[derive(JsonSchema, Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum WasmOptLevel {
	/// Optimize aggressively for size
	#[serde(rename = "z")]
	Z,
	/// Optimize for size
	#[serde(rename = "s")]
	S,
	/// Don't optimize
	#[serde(rename = "0")]
	Zero,
	/// Optimize for speed
	#[serde(rename = "1")]
	One,
	/// Optimize for more for speed
	#[serde(rename = "2")]
	Two,
	/// Optimize for even more for speed
	#[serde(rename = "3")]
	Three,
	/// Optimize aggressively for speed
	#[serde(rename = "4")]
	#[default]
	Four,
}

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize)]
pub struct WebAppConfig {
	#[serde(default = "default_title")]
	pub title: String,
	pub base_path: Option<String>,
}

impl WebAppConfig {
	/// Get the normalized base path for the application with `/` trimmed from both ends. If the base path is not set, this will return `.`.
	pub fn base_path(&self) -> &str {
		match &self.base_path {
			Some(path) => path.trim_matches('/'),
			None => ".",
		}
	}
}

impl Default for WebAppConfig {
	fn default() -> Self {
		Self { title: default_title(), base_path: None }
	}
}

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize)]
pub struct WebProxyConfig {
	pub backend: String,
}

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize)]
pub struct WebWatcherConfig {
	#[serde(default = "watch_path_default")]
	pub watch_path: Vec<PathBuf>,

	#[serde(default)]
	pub reload_html: bool,

	#[serde(default = "true_bool")]
	pub index_on_404: bool,
}

impl Default for WebWatcherConfig {
	fn default() -> Self {
		Self { watch_path: watch_path_default(), reload_html: false, index_on_404: true }
	}
}

fn watch_path_default() -> Vec<PathBuf> {
	vec![PathBuf::from("src"), PathBuf::from("examples")]
}

#[derive(JsonSchema, Default, Debug, Clone, Serialize, Deserialize)]
pub struct WebResourceConfig {
	pub dev: WebDevResourceConfig,
	pub style: Option<Vec<PathBuf>>,
	pub script: Option<Vec<PathBuf>>,
}

#[derive(JsonSchema, Default, Debug, Clone, Serialize, Deserialize)]
pub struct WebDevResourceConfig {
	#[serde(default)]
	pub style: Vec<PathBuf>,
	#[serde(default)]
	pub script: Vec<PathBuf>,
}

#[derive(JsonSchema, Debug, Default, Clone, Serialize, Deserialize)]
pub struct WebHttpsConfig {
	pub enabled: Option<bool>,
	pub mkcert: Option<bool>,
	pub key_path: Option<String>,
	pub cert_path: Option<String>,
}

fn true_bool() -> bool {
	true
}

fn false_bool() -> bool {
	false
}

pub fn default_title() -> String {
	"dioxus | ⛺".into()
}

/// Represents configuration items for the desktop platform.
#[derive(JsonSchema, Debug, Default, Clone, Serialize, Deserialize)]
pub struct DesktopConfig {}

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize, Default)]
pub struct BundleConfig {
	pub identifier: Option<String>,
	pub publisher: Option<String>,
	pub icon: Option<Vec<String>>,
	pub resources: Option<Vec<String>>,
	pub copyright: Option<String>,
	pub category: Option<String>,
	pub short_description: Option<String>,
	pub long_description: Option<String>,
	pub external_bin: Option<Vec<String>>,
	pub deb: Option<DebianSettings>,
	pub macos: Option<MacOsSettings>,
	pub windows: Option<WindowsSettings>,
}

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize, Default)]
pub struct DebianSettings {
	// OS-specific settings:
	/// the list of debian dependencies.
	pub depends: Option<Vec<String>>,
	/// the list of dependencies the package provides.
	pub provides: Option<Vec<String>>,
	/// the list of package conflicts.
	pub conflicts: Option<Vec<String>>,
	/// the list of package replaces.
	pub replaces: Option<Vec<String>>,
	/// List of custom files to add to the deb package.
	/// Maps the path on the debian package to the path of the file to include (relative to the current working directory).
	pub files: HashMap<PathBuf, PathBuf>,
	/// Path to a custom desktop file Handlebars template.
	///
	/// Available variables: `categories`, `comment` (optional), `exec`, `icon` and `name`.
	pub desktop_template: Option<PathBuf>,
	/// Define the section in Debian Control file. See : <https://www.debian.org/doc/debian-policy/ch-archive.html#s-subsections>
	pub section: Option<String>,
	/// Change the priority of the Debian Package. By default, it is set to `optional`.
	/// Recognized Priorities as of now are :  `required`, `important`, `standard`, `optional`, `extra`
	pub priority: Option<String>,
	/// Path of the uncompressed Changelog file, to be stored at /usr/share/doc/package-name/changelog.gz. See
	/// <https://www.debian.org/doc/debian-policy/ch-docs.html#changelog-files-and-release-notes>
	pub changelog: Option<PathBuf>,
	/// Path to script that will be executed before the package is unpacked. See
	/// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
	pub pre_install_script: Option<PathBuf>,
	/// Path to script that will be executed after the package is unpacked. See
	/// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
	pub post_install_script: Option<PathBuf>,
	/// Path to script that will be executed before the package is removed. See
	/// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
	pub pre_remove_script: Option<PathBuf>,
	/// Path to script that will be executed after the package is removed. See
	/// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
	pub post_remove_script: Option<PathBuf>,
}

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize, Default)]
pub struct WixSettings {
	pub language: Vec<(String, Option<PathBuf>)>,
	pub template: Option<PathBuf>,
	pub fragment_paths: Vec<PathBuf>,
	pub component_group_refs: Vec<String>,
	pub component_refs: Vec<String>,
	pub feature_group_refs: Vec<String>,
	pub feature_refs: Vec<String>,
	pub merge_refs: Vec<String>,
	pub skip_webview_install: bool,
	pub license: Option<PathBuf>,
	pub enable_elevated_update_task: bool,
	pub banner_path: Option<PathBuf>,
	pub dialog_image_path: Option<PathBuf>,
	pub fips_compliant: bool,
	/// MSI installer version in the format `major.minor.patch.build` (build is optional).
	///
	/// Because a valid version is required for MSI installer, it will be derived from [`PackageSettings::version`] if this field is not set.
	///
	/// The first field is the major version and has a maximum value of 255. The second field is the minor version and has a maximum value of 255.
	/// The third and fourth fields have a maximum value of 65,535.
	///
	/// See <https://learn.microsoft.com/en-us/windows/win32/msi/productversion> for more info.
	pub version: Option<String>,
	/// A GUID upgrade code for MSI installer. This code **_must stay the same across all of your updates_**,
	/// otherwise, Windows will treat your update as a different app and your users will have duplicate versions of your app.
	///
	/// By default, tauri generates this code by generating a Uuid v5 using the string `<productName>.exe.app.x64` in the DNS namespace.
	/// You can use Tauri's CLI to generate and print this code for you by running `tauri inspect wix-upgrade-code`.
	///
	/// It is recommended that you set this value in your tauri config file to avoid accidental changes in your upgrade code
	/// whenever you want to change your product name.
	pub upgrade_code: Option<uuid::Uuid>,
}

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize, Default)]
pub struct MacOsSettings {
	pub frameworks: Option<Vec<String>>,
	pub minimum_system_version: Option<String>,
	pub license: Option<String>,
	pub exception_domain: Option<String>,
	pub signing_identity: Option<String>,
	pub provider_short_name: Option<String>,
	pub entitlements: Option<String>,
	pub info_plist_path: Option<PathBuf>,
	/// List of custom files to add to the application bundle.
	/// Maps the path in the Contents directory in the app to the path of the file to include (relative to the current working directory).
	pub files: HashMap<PathBuf, PathBuf>,
	/// Preserve the hardened runtime version flag, see <https://developer.apple.com/documentation/security/hardened_runtime>
	///
	/// Settings this to `false` is useful when using an ad-hoc signature, making it less strict.
	pub hardened_runtime: bool,
}

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize)]
pub struct WindowsSettings {
	pub digest_algorithm: Option<String>,
	pub certificate_thumbprint: Option<String>,
	pub timestamp_url: Option<String>,
	pub tsp: bool,
	pub wix: Option<WixSettings>,
	pub icon_path: Option<PathBuf>,
	pub webview_install_mode: WebviewInstallMode,
	pub webview_fixed_runtime_path: Option<PathBuf>,
	pub allow_downgrades: bool,
	pub nsis: Option<NsisSettings>,
	/// Specify a custom command to sign the binaries.
	/// This command needs to have a `%1` in it which is just a placeholder for the binary path,
	/// which we will detect and replace before calling the command.
	///
	/// Example:
	/// ```text
	/// sign-cli --arg1 --arg2 %1
	/// ```
	///
	/// By Default we use `signtool.exe` which can be found only on Windows so
	/// if you are on another platform and want to cross-compile and sign you will
	/// need to use another tool like `osslsigncode`.
	pub sign_command: Option<CustomSignCommandSettings>,
}

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize)]
pub struct NsisSettings {
	pub template: Option<PathBuf>,
	pub license: Option<PathBuf>,
	pub header_image: Option<PathBuf>,
	pub sidebar_image: Option<PathBuf>,
	pub installer_icon: Option<PathBuf>,
	pub install_mode: NSISInstallerMode,
	pub languages: Option<Vec<String>>,
	pub custom_language_files: Option<HashMap<String, PathBuf>>,
	pub display_language_selector: bool,
	pub start_menu_folder: Option<String>,
	pub installer_hooks: Option<PathBuf>,
	/// Try to ensure that the WebView2 version is equal to or newer than this version,
	/// if the user's WebView2 is older than this version,
	/// the installer will try to trigger a WebView2 update.
	pub minimum_webview2_version: Option<String>,
}

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize)]
pub enum NSISInstallerMode {
	CurrentUser,
	PerMachine,
	Both,
}

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize)]
pub enum WebviewInstallMode {
	Skip,
	DownloadBootstrapper { silent: bool },
	EmbedBootstrapper { silent: bool },
	OfflineInstaller { silent: bool },
	FixedRuntime { path: PathBuf },
}

impl Default for WebviewInstallMode {
	fn default() -> Self {
		Self::OfflineInstaller { silent: false }
	}
}

#[derive(JsonSchema, Clone, Copy, Debug)]
pub enum PackageType {
	MacOsBundle,
	IosBundle,
	WindowsMsi,
	Deb,
	Rpm,
	AppImage,
	Dmg,
	Updater,
}

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize)]
pub struct CustomSignCommandSettings {
	/// The command to run to sign the binary.
	pub cmd: String,
	/// The arguments to pass to the command.
	///
	/// "%1" will be replaced with the path to the binary to be signed.
	pub args: Vec<String>,
}
