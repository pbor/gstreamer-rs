// This file was generated by gir (https://github.com/gtk-rs/gir @ 82cdb44)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

extern crate gstreamer_gl_sys;
extern crate shell_words;
extern crate tempdir;
use std::env;
use std::error::Error;
use std::path::Path;
use std::mem::{align_of, size_of};
use std::process::Command;
use std::str;
use gstreamer_gl_sys::*;

static PACKAGES: &[&str] = &["gstreamer-gl-1.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Compiler, Box<Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Compiler { args })
    }

    pub fn define<'a, V: Into<Option<&'a str>>>(&mut self, var: &str, val: V) {
        let arg = match val.into() {
            None => format!("-D{}", var),
            Some(val) => format!("-D{}={}", var, val),
        };
        self.args.push(arg);
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}",
                               &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let mut cmd = Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}",
                           &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
    /// Number of tests that failed to compile.
    failed_to_compile: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn record_failed_to_compile(&mut self) {
        self.failed += 1;
        self.failed_to_compile += 1;
    }
    fn summary(&self) -> String {
        format!(
            "{} passed; {} failed (compilation errors: {})",
            self.passed,
            self.failed,
            self.failed_to_compile)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let tmpdir = tempdir::TempDir::new("abi").expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!("1",
               get_c_value(tmpdir.path(), &cc, "1").expect("C constant"),
               "failed to obtain correct constant value for 1");

    let mut results : Results = Default::default();
    for (i, &(name, rust_value)) in RUST_CONSTANTS.iter().enumerate() {
        match get_c_value(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            },
            Ok(ref c_value) => {
                if rust_value == c_value {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!("Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                              name, rust_value, c_value);
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("constants ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let tmpdir = tempdir::TempDir::new("abi").expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!(Layout {size: 1, alignment: 1},
               get_c_layout(tmpdir.path(), &cc, "char").expect("C layout"),
               "failed to obtain correct layout for char type");

    let mut results : Results = Default::default();
    for (i, &(name, rust_layout)) in RUST_LAYOUTS.iter().enumerate() {
        match get_c_layout(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            },
            Ok(c_layout) => {
                if rust_layout == c_layout {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!("Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                              name, rust_layout, &c_layout);
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("layout    ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

fn get_c_layout(dir: &Path, cc: &Compiler, name: &str) -> Result<Layout, Box<Error>> {
    let exe = dir.join("layout");
    let mut cc = cc.clone();
    cc.define("ABI_TYPE_NAME", name);
    cc.compile(Path::new("tests/layout.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}",
                           &abi_cmd, &output).into());
    }

    let stdout = str::from_utf8(&output.stdout)?;
    let mut words = stdout.trim().split_whitespace();
    let size = words.next().unwrap().parse().unwrap();
    let alignment = words.next().unwrap().parse().unwrap();
    Ok(Layout {size, alignment})
}

fn get_c_value(dir: &Path, cc: &Compiler, name: &str) -> Result<String, Box<Error>> {
    let exe = dir.join("constant");
    let mut cc = cc.clone();
    cc.define("ABI_CONSTANT_NAME", name);
    cc.compile(Path::new("tests/constant.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}",
                           &abi_cmd, &output).into());
    }

    Ok(str::from_utf8(&output.stdout)?.trim().to_owned())
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    ("GstGLAPI", Layout {size: size_of::<GstGLAPI>(), alignment: align_of::<GstGLAPI>()}),
    ("GstGLAllocationParams", Layout {size: size_of::<GstGLAllocationParams>(), alignment: align_of::<GstGLAllocationParams>()}),
    ("GstGLAsyncDebug", Layout {size: size_of::<GstGLAsyncDebug>(), alignment: align_of::<GstGLAsyncDebug>()}),
    ("GstGLBaseFilter", Layout {size: size_of::<GstGLBaseFilter>(), alignment: align_of::<GstGLBaseFilter>()}),
    ("GstGLBaseFilterClass", Layout {size: size_of::<GstGLBaseFilterClass>(), alignment: align_of::<GstGLBaseFilterClass>()}),
    ("GstGLBaseMemory", Layout {size: size_of::<GstGLBaseMemory>(), alignment: align_of::<GstGLBaseMemory>()}),
    ("GstGLBaseMemoryAllocator", Layout {size: size_of::<GstGLBaseMemoryAllocator>(), alignment: align_of::<GstGLBaseMemoryAllocator>()}),
    ("GstGLBaseMemoryAllocatorClass", Layout {size: size_of::<GstGLBaseMemoryAllocatorClass>(), alignment: align_of::<GstGLBaseMemoryAllocatorClass>()}),
    ("GstGLBaseMemoryError", Layout {size: size_of::<GstGLBaseMemoryError>(), alignment: align_of::<GstGLBaseMemoryError>()}),
    ("GstGLBaseMemoryTransfer", Layout {size: size_of::<GstGLBaseMemoryTransfer>(), alignment: align_of::<GstGLBaseMemoryTransfer>()}),
    ("GstGLBuffer", Layout {size: size_of::<GstGLBuffer>(), alignment: align_of::<GstGLBuffer>()}),
    ("GstGLBufferAllocationParams", Layout {size: size_of::<GstGLBufferAllocationParams>(), alignment: align_of::<GstGLBufferAllocationParams>()}),
    ("GstGLBufferAllocator", Layout {size: size_of::<GstGLBufferAllocator>(), alignment: align_of::<GstGLBufferAllocator>()}),
    ("GstGLBufferAllocatorClass", Layout {size: size_of::<GstGLBufferAllocatorClass>(), alignment: align_of::<GstGLBufferAllocatorClass>()}),
    ("GstGLBufferPool", Layout {size: size_of::<GstGLBufferPool>(), alignment: align_of::<GstGLBufferPool>()}),
    ("GstGLBufferPoolClass", Layout {size: size_of::<GstGLBufferPoolClass>(), alignment: align_of::<GstGLBufferPoolClass>()}),
    ("GstGLColorConvert", Layout {size: size_of::<GstGLColorConvert>(), alignment: align_of::<GstGLColorConvert>()}),
    ("GstGLColorConvertClass", Layout {size: size_of::<GstGLColorConvertClass>(), alignment: align_of::<GstGLColorConvertClass>()}),
    ("GstGLContext", Layout {size: size_of::<GstGLContext>(), alignment: align_of::<GstGLContext>()}),
    ("GstGLContextClass", Layout {size: size_of::<GstGLContextClass>(), alignment: align_of::<GstGLContextClass>()}),
    ("GstGLContextError", Layout {size: size_of::<GstGLContextError>(), alignment: align_of::<GstGLContextError>()}),
    ("GstGLDisplay", Layout {size: size_of::<GstGLDisplay>(), alignment: align_of::<GstGLDisplay>()}),
    ("GstGLDisplayClass", Layout {size: size_of::<GstGLDisplayClass>(), alignment: align_of::<GstGLDisplayClass>()}),
    #[cfg(any(feature = "egl", feature = "dox"))]
    ("GstGLDisplayEGL", Layout {size: size_of::<GstGLDisplayEGL>(), alignment: align_of::<GstGLDisplayEGL>()}),
    #[cfg(any(feature = "egl", feature = "dox"))]
    ("GstGLDisplayEGLClass", Layout {size: size_of::<GstGLDisplayEGLClass>(), alignment: align_of::<GstGLDisplayEGLClass>()}),
    ("GstGLDisplayType", Layout {size: size_of::<GstGLDisplayType>(), alignment: align_of::<GstGLDisplayType>()}),
    ("GstGLFilter", Layout {size: size_of::<GstGLFilter>(), alignment: align_of::<GstGLFilter>()}),
    ("GstGLFilterClass", Layout {size: size_of::<GstGLFilterClass>(), alignment: align_of::<GstGLFilterClass>()}),
    ("GstGLFormat", Layout {size: size_of::<GstGLFormat>(), alignment: align_of::<GstGLFormat>()}),
    ("GstGLFramebuffer", Layout {size: size_of::<GstGLFramebuffer>(), alignment: align_of::<GstGLFramebuffer>()}),
    ("GstGLFramebufferClass", Layout {size: size_of::<GstGLFramebufferClass>(), alignment: align_of::<GstGLFramebufferClass>()}),
    ("GstGLMemory", Layout {size: size_of::<GstGLMemory>(), alignment: align_of::<GstGLMemory>()}),
    ("GstGLMemoryAllocator", Layout {size: size_of::<GstGLMemoryAllocator>(), alignment: align_of::<GstGLMemoryAllocator>()}),
    ("GstGLMemoryAllocatorClass", Layout {size: size_of::<GstGLMemoryAllocatorClass>(), alignment: align_of::<GstGLMemoryAllocatorClass>()}),
    ("GstGLMemoryPBO", Layout {size: size_of::<GstGLMemoryPBO>(), alignment: align_of::<GstGLMemoryPBO>()}),
    ("GstGLMemoryPBOAllocator", Layout {size: size_of::<GstGLMemoryPBOAllocator>(), alignment: align_of::<GstGLMemoryPBOAllocator>()}),
    ("GstGLMemoryPBOAllocatorClass", Layout {size: size_of::<GstGLMemoryPBOAllocatorClass>(), alignment: align_of::<GstGLMemoryPBOAllocatorClass>()}),
    ("GstGLOverlayCompositor", Layout {size: size_of::<GstGLOverlayCompositor>(), alignment: align_of::<GstGLOverlayCompositor>()}),
    ("GstGLOverlayCompositorClass", Layout {size: size_of::<GstGLOverlayCompositorClass>(), alignment: align_of::<GstGLOverlayCompositorClass>()}),
    ("GstGLPlatform", Layout {size: size_of::<GstGLPlatform>(), alignment: align_of::<GstGLPlatform>()}),
    ("GstGLQuery", Layout {size: size_of::<GstGLQuery>(), alignment: align_of::<GstGLQuery>()}),
    ("GstGLQueryType", Layout {size: size_of::<GstGLQueryType>(), alignment: align_of::<GstGLQueryType>()}),
    ("GstGLRenderbuffer", Layout {size: size_of::<GstGLRenderbuffer>(), alignment: align_of::<GstGLRenderbuffer>()}),
    ("GstGLRenderbufferAllocationParams", Layout {size: size_of::<GstGLRenderbufferAllocationParams>(), alignment: align_of::<GstGLRenderbufferAllocationParams>()}),
    ("GstGLRenderbufferAllocator", Layout {size: size_of::<GstGLRenderbufferAllocator>(), alignment: align_of::<GstGLRenderbufferAllocator>()}),
    ("GstGLRenderbufferAllocatorClass", Layout {size: size_of::<GstGLRenderbufferAllocatorClass>(), alignment: align_of::<GstGLRenderbufferAllocatorClass>()}),
    ("GstGLSLError", Layout {size: size_of::<GstGLSLError>(), alignment: align_of::<GstGLSLError>()}),
    ("GstGLSLProfile", Layout {size: size_of::<GstGLSLProfile>(), alignment: align_of::<GstGLSLProfile>()}),
    ("GstGLSLStage", Layout {size: size_of::<GstGLSLStage>(), alignment: align_of::<GstGLSLStage>()}),
    ("GstGLSLStageClass", Layout {size: size_of::<GstGLSLStageClass>(), alignment: align_of::<GstGLSLStageClass>()}),
    ("GstGLSLVersion", Layout {size: size_of::<GstGLSLVersion>(), alignment: align_of::<GstGLSLVersion>()}),
    ("GstGLShader", Layout {size: size_of::<GstGLShader>(), alignment: align_of::<GstGLShader>()}),
    ("GstGLShaderClass", Layout {size: size_of::<GstGLShaderClass>(), alignment: align_of::<GstGLShaderClass>()}),
    ("GstGLStereoDownmix", Layout {size: size_of::<GstGLStereoDownmix>(), alignment: align_of::<GstGLStereoDownmix>()}),
    ("GstGLSyncMeta", Layout {size: size_of::<GstGLSyncMeta>(), alignment: align_of::<GstGLSyncMeta>()}),
    ("GstGLTextureTarget", Layout {size: size_of::<GstGLTextureTarget>(), alignment: align_of::<GstGLTextureTarget>()}),
    ("GstGLUpload", Layout {size: size_of::<GstGLUpload>(), alignment: align_of::<GstGLUpload>()}),
    ("GstGLUploadClass", Layout {size: size_of::<GstGLUploadClass>(), alignment: align_of::<GstGLUploadClass>()}),
    ("GstGLUploadReturn", Layout {size: size_of::<GstGLUploadReturn>(), alignment: align_of::<GstGLUploadReturn>()}),
    ("GstGLVideoAllocationParams", Layout {size: size_of::<GstGLVideoAllocationParams>(), alignment: align_of::<GstGLVideoAllocationParams>()}),
    ("GstGLViewConvert", Layout {size: size_of::<GstGLViewConvert>(), alignment: align_of::<GstGLViewConvert>()}),
    ("GstGLViewConvertClass", Layout {size: size_of::<GstGLViewConvertClass>(), alignment: align_of::<GstGLViewConvertClass>()}),
    ("GstGLWindow", Layout {size: size_of::<GstGLWindow>(), alignment: align_of::<GstGLWindow>()}),
    ("GstGLWindowClass", Layout {size: size_of::<GstGLWindowClass>(), alignment: align_of::<GstGLWindowClass>()}),
    ("GstGLWindowError", Layout {size: size_of::<GstGLWindowError>(), alignment: align_of::<GstGLWindowError>()}),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("GST_BUFFER_POOL_OPTION_GL_SYNC_META", "GstBufferPoolOptionGLSyncMeta"),
    ("GST_BUFFER_POOL_OPTION_GL_TEXTURE_TARGET_2D", "GstBufferPoolOptionGLTextureTarget2D"),
    ("GST_BUFFER_POOL_OPTION_GL_TEXTURE_TARGET_EXTERNAL_OES", "GstBufferPoolOptionGLTextureTargetExternalOES"),
    ("GST_BUFFER_POOL_OPTION_GL_TEXTURE_TARGET_RECTANGLE", "GstBufferPoolOptionGLTextureTargetRectangle"),
    ("GST_CAPS_FEATURE_MEMORY_GL_BUFFER", "memory:GLBuffer"),
    ("GST_CAPS_FEATURE_MEMORY_GL_MEMORY", "memory:GLMemory"),
    ("GST_GLSL_ERROR_COMPILE", "0"),
    ("GST_GLSL_ERROR_LINK", "1"),
    ("GST_GLSL_ERROR_PROGRAM", "2"),
    ("GST_GLSL_PROFILE_ANY", "-1"),
    ("GST_GLSL_PROFILE_COMPATIBILITY", "4"),
    ("GST_GLSL_PROFILE_CORE", "2"),
    ("GST_GLSL_PROFILE_ES", "1"),
    ("GST_GLSL_PROFILE_NONE", "0"),
    ("GST_GLSL_VERSION_100", "100"),
    ("GST_GLSL_VERSION_110", "110"),
    ("GST_GLSL_VERSION_120", "120"),
    ("GST_GLSL_VERSION_130", "130"),
    ("GST_GLSL_VERSION_140", "140"),
    ("GST_GLSL_VERSION_150", "150"),
    ("GST_GLSL_VERSION_300", "300"),
    ("GST_GLSL_VERSION_310", "310"),
    ("GST_GLSL_VERSION_320", "320"),
    ("GST_GLSL_VERSION_330", "330"),
    ("GST_GLSL_VERSION_400", "400"),
    ("GST_GLSL_VERSION_410", "410"),
    ("GST_GLSL_VERSION_420", "420"),
    ("GST_GLSL_VERSION_430", "430"),
    ("GST_GLSL_VERSION_440", "440"),
    ("GST_GLSL_VERSION_450", "450"),
    ("GST_GLSL_VERSION_NONE", "0"),
    ("GST_GL_ALLOCATION_PARAMS_ALLOC_FLAG_ALLOC", "1"),
    ("GST_GL_ALLOCATION_PARAMS_ALLOC_FLAG_BUFFER", "16"),
    ("GST_GL_ALLOCATION_PARAMS_ALLOC_FLAG_USER", "65536"),
    ("GST_GL_ALLOCATION_PARAMS_ALLOC_FLAG_VIDEO", "8"),
    ("GST_GL_ALLOCATION_PARAMS_ALLOC_FLAG_WRAP_GPU_HANDLE", "4"),
    ("GST_GL_ALLOCATION_PARAMS_ALLOC_FLAG_WRAP_SYSMEM", "2"),
    ("GST_GL_ALPHA", "6406"),
    ("GST_GL_API_ANY", "4294967295"),
    ("GST_GL_API_GLES1", "32768"),
    ("GST_GL_API_GLES1_NAME", "gles1"),
    ("GST_GL_API_GLES2", "65536"),
    ("GST_GL_API_GLES2_NAME", "gles2"),
    ("GST_GL_API_NONE", "0"),
    ("GST_GL_API_OPENGL", "1"),
    ("GST_GL_API_OPENGL3", "2"),
    ("GST_GL_API_OPENGL3_NAME", "opengl3"),
    ("GST_GL_API_OPENGL_NAME", "opengl"),
    ("GST_GL_BASE_MEMORY_ALLOCATOR_NAME", "GLBaseMemory"),
    ("GST_GL_BASE_MEMORY_ERROR_FAILED", "0"),
    ("GST_GL_BASE_MEMORY_ERROR_OLD_LIBS", "1"),
    ("GST_GL_BASE_MEMORY_ERROR_RESOURCE_UNAVAILABLE", "2"),
    ("GST_GL_BASE_MEMORY_TRANSFER_NEED_DOWNLOAD", "1048576"),
    ("GST_GL_BASE_MEMORY_TRANSFER_NEED_UPLOAD", "2097152"),
    ("GST_GL_BUFFER_ALLOCATOR_NAME", "GLBuffer"),
    ("GST_GL_COLOR_CONVERT_FORMATS", "{ RGBA, RGB, RGBx, BGR, BGRx, BGRA, xRGB, xBGR, ARGB, ABGR, Y444, I420, YV12, Y42B, Y41B, NV12, NV21, YUY2, UYVY, AYUV, GRAY8, GRAY16_LE, GRAY16_BE, RGB16, BGR16 }"),
    ("GST_GL_COLOR_CONVERT_VIDEO_CAPS", "video/x-raw("),
    ("GST_GL_CONTEXT_ERROR_CREATE_CONTEXT", "4"),
    ("GST_GL_CONTEXT_ERROR_FAILED", "0"),
    ("GST_GL_CONTEXT_ERROR_OLD_LIBS", "3"),
    ("GST_GL_CONTEXT_ERROR_RESOURCE_UNAVAILABLE", "5"),
    ("GST_GL_CONTEXT_ERROR_WRONG_API", "2"),
    ("GST_GL_CONTEXT_ERROR_WRONG_CONFIG", "1"),
    ("GST_GL_CONTEXT_TYPE_CGL", "gst.gl.context.CGL"),
    ("GST_GL_CONTEXT_TYPE_EAGL", "gst.gl.context.EAGL"),
    ("GST_GL_CONTEXT_TYPE_EGL", "gst.gl.context.EGL"),
    ("GST_GL_CONTEXT_TYPE_GLX", "gst.gl.context.GLX"),
    ("GST_GL_CONTEXT_TYPE_WGL", "gst.gl.context.WGL"),
    ("GST_GL_DEPTH24_STENCIL8", "35056"),
    ("GST_GL_DEPTH_COMPONENT16", "33189"),
    ("GST_GL_DISPLAY_CONTEXT_TYPE", "gst.gl.GLDisplay"),
    ("GST_GL_DISPLAY_EGL_NAME", "gst.gl.display.egl"),
    ("GST_GL_DISPLAY_TYPE_ANY", "4294967295"),
    ("GST_GL_DISPLAY_TYPE_COCOA", "4"),
    ("GST_GL_DISPLAY_TYPE_DISPMANX", "16"),
    ("GST_GL_DISPLAY_TYPE_EGL", "32"),
    ("GST_GL_DISPLAY_TYPE_GBM", "128"),
    ("GST_GL_DISPLAY_TYPE_NONE", "0"),
    ("GST_GL_DISPLAY_TYPE_VIV_FB", "64"),
    ("GST_GL_DISPLAY_TYPE_WAYLAND", "2"),
    ("GST_GL_DISPLAY_TYPE_WIN32", "8"),
    ("GST_GL_DISPLAY_TYPE_X11", "1"),
    ("GST_GL_LUMINANCE", "6409"),
    ("GST_GL_LUMINANCE_ALPHA", "6410"),
    ("GST_GL_MEMORY_ALLOCATOR_NAME", "GLMemory"),
    ("GST_GL_MEMORY_PBO_ALLOCATOR_NAME", "GLMemoryPBO"),
    ("GST_GL_MEMORY_VIDEO_FORMATS_STR", "{ RGBA, BGRA, RGBx, BGRx, ARGB, ABGR, xRGB, xBGR, RGB, BGR, RGB16, BGR16, AYUV, I420, YV12, NV12, NV21, YUY2, UYVY, Y41B, Y42B, Y444, GRAY8, GRAY16_LE, GRAY16_BE }"),
    ("GST_GL_PLATFORM_ANY", "4294967295"),
    ("GST_GL_PLATFORM_CGL", "8"),
    ("GST_GL_PLATFORM_EAGL", "16"),
    ("GST_GL_PLATFORM_EGL", "1"),
    ("GST_GL_PLATFORM_GLX", "2"),
    ("GST_GL_PLATFORM_NONE", "0"),
    ("GST_GL_PLATFORM_WGL", "4"),
    ("GST_GL_QUERY_NONE", "0"),
    ("GST_GL_QUERY_TIMESTAMP", "2"),
    ("GST_GL_QUERY_TIME_ELAPSED", "1"),
    ("GST_GL_R8", "33321"),
    ("GST_GL_RED", "6403"),
    ("GST_GL_RENDERBUFFER_ALLOCATOR_NAME", "GLRenderbuffer"),
    ("GST_GL_RG", "33319"),
    ("GST_GL_RG8", "33323"),
    ("GST_GL_RGB", "6407"),
    ("GST_GL_RGB565", "36194"),
    ("GST_GL_RGB8", "32849"),
    ("GST_GL_RGBA", "6408"),
    ("GST_GL_RGBA8", "32856"),
    ("GST_GL_STEREO_DOWNMIX_ANAGLYPH_AMBER_BLUE_DUBOIS", "2"),
    ("GST_GL_STEREO_DOWNMIX_ANAGLYPH_GREEN_MAGENTA_DUBOIS", "0"),
    ("GST_GL_STEREO_DOWNMIX_ANAGLYPH_RED_CYAN_DUBOIS", "1"),
    ("GST_GL_TEXTURE_TARGET_2D", "1"),
    ("GST_GL_TEXTURE_TARGET_2D_STR", "2D"),
    ("GST_GL_TEXTURE_TARGET_EXTERNAL_OES", "3"),
    ("GST_GL_TEXTURE_TARGET_EXTERNAL_OES_STR", "external-oes"),
    ("GST_GL_TEXTURE_TARGET_NONE", "0"),
    ("GST_GL_TEXTURE_TARGET_RECTANGLE", "2"),
    ("GST_GL_TEXTURE_TARGET_RECTANGLE_STR", "rectangle"),
    ("GST_GL_UPLOAD_DONE", "1"),
    ("GST_GL_UPLOAD_ERROR", "-1"),
    ("GST_GL_UPLOAD_RECONFIGURE", "-3"),
    ("GST_GL_UPLOAD_UNSUPPORTED", "-2"),
    ("GST_GL_WINDOW_ERROR_FAILED", "0"),
    ("GST_GL_WINDOW_ERROR_OLD_LIBS", "1"),
    ("GST_GL_WINDOW_ERROR_RESOURCE_UNAVAILABLE", "2"),
    ("GST_MAP_GL", "131072"),
];


