# Beginners Scratchpad

## This  Beginner's Scratchpad is evidence of the work done on this repositrory.

It's contents are open to update, change, and deletion without proior notice from the [AUTHORS](#) of this work.


Step 1
## Add Rust Framework, 


Step 2
## Add Rust Framework, and sudo dnf upgrade

Step 3
## Add Rust Framework,  add Step 2 and Step 1  

Step 4
## Add Rust Framework, and Step 3 and Step 2 and Step 1 Bazel in order to build

Step 5
## dd Rust Framework, and Step 3 and Step 2 and Step 1  Bazel in order to build and Add Bazel in order to build 


First, we check, where we are.

   ```
PS D:\BaselBuilds\projects\Rustprojects\Beginners> pwd

Path
----
D:\BaselBuilds\projects\Rustprojects\Beginners


   ```

Then we start basel. It should show you the default download starting.

...```
PS D:\BaselBuilds\projects\Rustprojects\Beginners> bazel .
2024/08/14 00:39:44 Downloading https://releases.bazel.build/7.3.0/release/bazel-7.3.0-windows-x86_64.exe...
Downloading: 7 MB out of 63 MB (12%)

...```

So after the initial download, i run into an error that says, 
...```
Path
----
D:\BaselBuilds\projects\Rustprojects\Beginners

PS D:\BaselBuilds\projects\Rustprojects\Beginners> bazel .   
2024/08/14 00:39:44 Downloading https://releases.bazel.build/7.3.0/release/bazel-7.3.0-windows-x86_64.exe...
Downloading: 63 MB out of 63 MB (100%)
Extracting Bazel installation...
install base directory 'c:\users\u/_bazel_U/install/dcd140a81a5a424cc432246b2acaa6e5.tmp.24900' could not be renamed into place after 1 second(s), trying again
install base directory 'c:\users\u/_bazel_U/install/dcd140a81a5a424cc432246b2acaa6e5.tmp.24900' could not be renamed into place after 2 second(s), trying again
install base directory 'c:\users\u/_bazel_U/install/dcd140a81a5a424cc432246b2acaa6e5.tmp.24900' could not be renamed into place after 3 second(s), trying again
install base directory 'c:\users\u/_bazel_U/install/dcd140a81a5a424cc432246b2acaa6e5.tmp.24900' could not be renamed into place after 4 second(s), trying again
install base directory 'c:\users\u/_bazel_U/install/dcd140a81a5a424cc432246b2acaa6e5.tmp.24900' could not be renamed into place after 5 second(s), trying again
install base directory 'c:\users\u/_bazel_U/install/dcd140a81a5a424cc432246b2acaa6e5.tmp.24900' could not be renamed into place after 6 second(s), trying again
Starting local Bazel server and connecting to it...
... still trying to connect to local Bazel server (38160) after 10 seconds ...
Command '.' not found. Try 'bazel help'.

...´´´

=====


PS D:\BaselBuilds\projects\Rustprojects> cd Beginners       
PS D:\cargo test  projects\Rustprojects\Beginners>
warning: Beginners v0.1.0 (D:\BaselBuilds\projects\Rustprojects\Beginners) ignoring invalid dependency `trunk` which is missing a lib target
   Compiling glib-sys v0.18.1
   Compiling pin-utils v0.1.0
   Compiling cfg-if v1.0.0
   Compiling once_cell v1.19.0
The following warnings were emitted during compilation:

warning: glib-sys@0.18.1: Could not run `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags glib-2.0 glib-2.0 >= 2.56`

error: failed to run custom build command for `glib-sys v0.18.1`

Caused by:
  process didn't exit successfully: `D:\BaselBuilds\projects\Rustprojects\Beginners\target\debug\build\glib-sys-7a84311d178c6fa7\build-script-build` (exit code: 1)
  --- stdout
  cargo:rerun-if-env-changed=GLIB_2.0_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR    
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR    
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:warning=Could not run `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags glib-2.0 glib-2.0 >= 2.56`   
  The pkg-config command could not be found.

  Most likely, you need to install a pkg-config package for your OS.

  If you've already installed it, ensure the pkg-config command is one of the
  directories in the PATH environment variable.

  If you did not expect this build to link to a pre-installed system library,
  then check documentation of the glib-sys crate for an option to
  build the library from source, or disable features or dependencies
  that require pkg-config.
warning: build failed, waiting for other jobs to finish...
PS D:\BaselBuilds\projects\Rustprojects\Beginners> cargo check 
warning: Beginners v0.1.0 (D:\BaselBuilds\projects\Rustprojects\Beginners) ignoring invalid dependency `trunk` which is missing a lib target
   Compiling glib-sys v0.18.1
   Compiling gobject-sys v0.18.0
   Compiling gio-sys v0.18.1
   Compiling atk-sys v0.18.0
The following warnings were emitted during compilation:

warning: glib-sys@0.18.1: Could not run `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags glib-2.0 glib-2.0 >= 2.56`

error: failed to run custom build command for `glib-sys v0.18.1`

Caused by:
  process didn't exit successfully: `D:\BaselBuilds\projects\Rustprojects\Beginners\target\debug\build\glib-sys-7a84311d178c6fa7\build-script-build` (exit code: 1)
  --- stdout
  cargo:rerun-if-env-changed=GLIB_2.0_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR    
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR    
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:warning=Could not run `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags glib-2.0 glib-2.0 >= 2.56`   
  The pkg-config command could not be found.

  Most likely, you need to install a pkg-config package for your OS.

  If you've already installed it, ensure the pkg-config command is one of the
  directories in the PATH environment variable.

  If you did not expect this build to link to a pre-installed system library,
  then check documentation of the glib-sys crate for an option to
  build the library from source, or disable features or dependencies
  that require pkg-config.
warning: build failed, waiting for other jobs to finish...
The following warnings were emitted during compilation:     

warning: gobject-sys@0.18.0: Could not run `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags gobject-2.0 gobject-2.0 >= 2.56`

error: failed to run custom build command for `gobject-sys v0.18.0`

Caused by:
  process didn't exit successfully: `D:\BaselBuilds\projects\Rustprojects\Beginners\target\debug\build\gobject-sys-0b2f04502675011b\build-script-build` (exit code: 1)
  --- stdout
  cargo:rerun-if-env-changed=GOBJECT_2.0_NO_PKG_CONFIG      
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR    
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR    
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:warning=Could not run `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags gobject-2.0 gobject-2.0 >= 2.56`
  The pkg-config command could not be found.

  Most likely, you need to install a pkg-config package for your OS.

  If you did not expect this build to link to a pre-installed system lib

  If you did not expect this 
  then check documentation of the gobject-sys crate for an option to
  build the library from source, or disable features or dependencies
  that require pkg-config.
PS D:\BaselBuilds\projects\Rustprojects\Beginners>
    cargo bench
    Updating crates.io index
     Locking 44 packages to latest compatible versions
      Adding atomic-waker v1.1.2
    Updating axum v0.6.20 -> v0.7.5
    Updating axum-core v0.3.4 -> v0.4.3
    Updating axum-server v0.5.1 -> v0.6.0 (latest: v0.7.1)
      Adding base64 v0.22.1
      Adding convert_case v0.6.0
    Updating crates_io_api v0.9.0 -> v0.11.0
    Updating h2 v0.3.26 -> v0.4.5
    Updating http v0.2.12 -> v1.1.0
    Updating http-body v0.4.6 -> v1.0.1
      Adding http-body-util v0.1.2
    Updating http-range-header v0.3.1 -> v0.4.1
    Updating hyper v0.14.28 -> v1.4.1
    Updating hyper-rustls v0.24.2 -> v0.27.2
      Adding hyper-util v0.1.7
      Adding lazycell v1.3.0
    Updating lightningcss v1.0.0-alpha.54 -> v1.0.0-alpha.58
      Adding lightningcss-derive v1.0.0-alpha.43
      Adding lol_html v1.2.1
    Updating parcel_selectors v0.26.4 -> v0.26.6
      Adding quinn v0.11.3
      Adding quinn-proto v0.11.6
      Adding quinn-udp v0.5.4
    Updating reqwest v0.11.27 -> v0.12.5
      Adding rustc-hash v2.0.0
      Adding rustls v0.22.4 (latest: v0.23.12)
      Adding rustls v0.23.12
    Updating rustls-native-certs v0.6.3 -> v0.7.1
    Updating rustls-pemfile v1.0.4 -> v2.1.3
      Adding rustls-pki-types v1.8.0
      Adding rustls-webpki v0.102.6
    Updating serde v1.0.197 -> v1.0.208
    Updating serde_derive v1.0.197 -> v1.0.208
      Adding sync_wrapper v1.0.1
      Adding tokio-rustls v0.25.0 (latest: v0.26.0)
      Adding tokio-rustls v0.26.0
    Updating tokio-tungstenite v0.20.1 -> v0.21.0 (latest: v0.23.1)
    Updating tower-http v0.4.4 -> v0.5.2
    Updating trunk v0.19.1 -> v0.20.3
    Updating tungstenite v0.20.1 -> v0.21.0 (latest: v0.23.0)
      Adding unicode-segmentation v1.11.0
    Updating webpki-roots v0.25.4 -> v0.26.3
    Updating winreg v0.50.0 -> v0.52.0
      Adding zeroize v1.8.1
  Downloaded serde v1.0.208
  Downloaded serde_derive v1.0.208
  Downloaded 2 crates (134.3 KB) in 4.59s
warning: Beginners v0.1.0 (D:\BaselBuilds\projects\Rustprojects\Beginners) ignoring invalid dependency `trunk` which is missing a lib target
   Compiling proc-macro2 v1.0.79
   Compiling unicode-ident v1.0.12
   Compiling serde v1.0.208
   Compiling hashbrown v0.14.3
   Compiling equivalent v1.0.1
   Compiling autocfg v1.2.0
   Compiling indexmap v2.2.6                                                                
   Compiling target-lexicon v0.12.14                                                        
   Compiling winnow v0.6.5
   Compiling quote v1.0.35
   Compiling syn v2.0.57                                                                    
   Compiling smallvec v1.13.2
   Compiling cfg-expr v0.15.8
   Compiling version-compare v0.2.0
   Compiling pkg-config v0.3.30                                                             
   Compiling heck v0.5.0
   Compiling windows_x86_64_gnu v0.48.5
   Compiling pin-project-lite v0.2.14                                                       
   Compiling windows_x86_64_gnu v0.52.4
    Building [==>                       ] 20/138: syn, windows_x86_64_gnu(build.rs), p...   



   ```

Then we start basel. It should show you the default download starting.

...```
PS D:\BaselBuilds\projects\Rustprojects\Beginners> bazel .
2024/08/14 00:39:44 Downloading https://releases.bazel.build/7.3.0/release/bazel-7.3.0-windows-x86_64.exe...
Downloading: 7 MB out of 63 MB (12%)

...```

So after the initial download, i run into an error that says, 
...```
Path
----
D:\BaselBuilds\projects\Rustprojects\Beginners

PS D:\BaselBuilds\projects\Rustprojects\Beginners> bazel .   
2024/08/14 00:39:44 Downloading https://releases.bazel.build/7.3.0/release/bazel-7.3.0-windows-x86_64.exe...
Downloading: 63 MB out of 63 MB (100%)
Extracting Bazel installation...
install base directory 'c:\users\u/_bazel_U/install/dcd140a81a5a424cc432246b2acaa6e5.tmp.24900' could not be renamed into place after 1 second(s), trying again
install base directory 'c:\users\u/_bazel_U/install/dcd140a81a5a424cc432246b2acaa6e5.tmp.24900' could not be renamed into place after 2 second(s), trying again
install base directory 'c:\users\u/_bazel_U/install/dcd140a81a5a424cc432246b2acaa6e5.tmp.24900' could not be renamed into place after 3 second(s), trying again
install base directory 'c:\users\u/_bazel_U/install/dcd140a81a5a424cc432246b2acaa6e5.tmp.24900' could not be renamed into place after 4 second(s), trying again
install base directory 'c:\users\u/_bazel_U/install/dcd140a81a5a424cc432246b2acaa6e5.tmp.24900' could not be renamed into place after 5 second(s), trying again
install base directory 'c:\users\u/_bazel_U/install/dcd140a81a5a424cc432246b2acaa6e5.tmp.24900' could not be renamed into place after 6 second(s), trying again
Starting local Bazel server and connecting to it...
... still trying to connect to local Bazel server (38160) after 10 seconds ...
Command '.' not found. Try 'bazel help'.

...´´´

=====


PS D:\BaselBuilds\projects\Rustprojects> cd Beginners       
PS D:\cargo test  projects\Rustprojects\Beginners>
warning: Beginners v0.1.0 (D:\BaselBuilds\projects\Rustprojects\Beginners) ignoring invalid dependency `trunk` which is missing a lib target
   Compiling glib-sys v0.18.1
   Compiling pin-utils v0.1.0
   Compiling cfg-if v1.0.0
   Compiling once_cell v1.19.0
The following warnings were emitted during compilation:

warning: glib-sys@0.18.1: Could not run `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags glib-2.0 glib-2.0 >= 2.56`

error: failed to run custom build command for `glib-sys v0.18.1`

Caused by:
  process didn't exit successfully: `D:\BaselBuilds\projects\Rustprojects\Beginners\target\debug\build\glib-sys-7a84311d178c6fa7\build-script-build` (exit code: 1)
  --- stdout
  cargo:rerun-if-env-changed=GLIB_2.0_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR    
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR    
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:warning=Could not run `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags glib-2.0 glib-2.0 >= 2.56`   
  The pkg-config command could not be found.

  Most likely, you need to install a pkg-config package for your OS.

  If you've already installed it, ensure the pkg-config command is one of the
  directories in the PATH environment variable.

  If you did not expect this build to link to a pre-installed system library,
  then check documentation of the glib-sys crate for an option to
  build the library from source, or disable features or dependencies
  that require pkg-config.
warning: build failed, waiting for other jobs to finish...
PS D:\BaselBuilds\projects\Rustprojects\Beginners> cargo check 
warning: Beginners v0.1.0 (D:\BaselBuilds\projects\Rustprojects\Beginners) ignoring invalid dependency `trunk` which is missing a lib target
   Compiling glib-sys v0.18.1
   Compiling gobject-sys v0.18.0
   Compiling gio-sys v0.18.1
   Compiling atk-sys v0.18.0
The following warnings were emitted during compilation:

warning: glib-sys@0.18.1: Could not run `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags glib-2.0 glib-2.0 >= 2.56`

error: failed to run custom build command for `glib-sys v0.18.1`

Caused by:
  process didn't exit successfully: `D:\BaselBuilds\projects\Rustprojects\Beginners\target\debug\build\glib-sys-7a84311d178c6fa7\build-script-build` (exit code: 1)
  --- stdout
  cargo:rerun-if-env-changed=GLIB_2.0_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR    
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR    
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:warning=Could not run `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags glib-2.0 glib-2.0 >= 2.56`   
  The pkg-config command could not be found.

  Most likely, you need to install a pkg-config package for your OS.

  If you've already installed it, ensure the pkg-config command is one of the
  directories in the PATH environment variable.

  If you did not expect this build to link to a pre-installed system library,
  then check documentation of the glib-sys crate for an option to
  build the library from source, or disable features or dependencies
  that require pkg-config.
warning: build failed, waiting for other jobs to finish...
The following warnings were emitted during compilation:     

warning: gobject-sys@0.18.0: Could not run `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags gobject-2.0 gobject-2.0 >= 2.56`

error: failed to run custom build command for `gobject-sys v0.18.0`

Caused by:
  process didn't exit successfully: `D:\BaselBuilds\projects\Rustprojects\Beginners\target\debug\build\gobject-sys-0b2f04502675011b\build-script-build` (exit code: 1)
  --- stdout
  cargo:rerun-if-env-changed=GOBJECT_2.0_NO_PKG_CONFIG      
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR    
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-pc-windows-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_pc_windows_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR    
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:warning=Could not run `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags gobject-2.0 gobject-2.0 >= 2.56`
  The pkg-config command could not be found.

  Most likely, you need to install a pkg-config package for your OS.

  If you did not expect this build to link to a pre-installed system lib

  If you did not expect this 
  then check documentation of the gobject-sys crate for an option to
  build the library from source, or disable features or dependencies
  that require pkg-config.
PS D:\BaselBuilds\projects\Rustprojects\Beginners>
    cargo bench
    Updating crates.io index
     Locking 44 packages to latest compatible versions
      Adding atomic-waker v1.1.2
    Updating axum v0.6.20 -> v0.7.5
    Updating axum-core v0.3.4 -> v0.4.3
    Updating axum-server v0.5.1 -> v0.6.0 (latest: v0.7.1)
      Adding base64 v0.22.1
      Adding convert_case v0.6.0
    Updating crates_io_api v0.9.0 -> v0.11.0
    Updating h2 v0.3.26 -> v0.4.5
    Updating http v0.2.12 -> v1.1.0
    Updating http-body v0.4.6 -> v1.0.1
      Adding http-body-util v0.1.2
    Updating http-range-header v0.3.1 -> v0.4.1
    Updating hyper v0.14.28 -> v1.4.1
    Updating hyper-rustls v0.24.2 -> v0.27.2
      Adding hyper-util v0.1.7
      Adding lazycell v1.3.0
    Updating lightningcss v1.0.0-alpha.54 -> v1.0.0-alpha.58
      Adding lightningcss-derive v1.0.0-alpha.43
      Adding lol_html v1.2.1
    Updating parcel_selectors v0.26.4 -> v0.26.6
      Adding quinn v0.11.3
      Adding quinn-proto v0.11.6
      Adding quinn-udp v0.5.4
    Updating reqwest v0.11.27 -> v0.12.5
      Adding rustc-hash v2.0.0
      Adding rustls v0.22.4 (latest: v0.23.12)
      Adding rustls v0.23.12
    Updating rustls-native-certs v0.6.3 -> v0.7.1
    Updating rustls-pemfile v1.0.4 -> v2.1.3
      Adding rustls-pki-types v1.8.0
      Adding rustls-webpki v0.102.6
    Updating serde v1.0.197 -> v1.0.208
    Updating serde_derive v1.0.197 -> v1.0.208
      Adding sync_wrapper v1.0.1
      Adding tokio-rustls v0.25.0 (latest: v0.26.0)
      Adding tokio-rustls v0.26.0
    Updating tokio-tungstenite v0.20.1 -> v0.21.0 (latest: v0.23.1)
    Updating tower-http v0.4.4 -> v0.5.2
    Updating trunk v0.19.1 -> v0.20.3
    Updating tungstenite v0.20.1 -> v0.21.0 (latest: v0.23.0)
      Adding unicode-segmentation v1.11.0
    Updating webpki-roots v0.25.4 -> v0.26.3
    Updating winreg v0.50.0 -> v0.52.0
      Adding zeroize v1.8.1
  Downloaded serde v1.0.208
  Downloaded serde_derive v1.0.208
  Downloaded 2 crates (134.3 KB) in 4.59s
warning: Beginners v0.1.0 (D:\BaselBuilds\projects\Rustprojects\Beginners) ignoring invalid dependency `trunk` which is missing a lib target
   Compiling proc-macro2 v1.0.79
   Compiling unicode-ident v1.0.12
   Compiling serde v1.0.208
   Compiling hashbrown v0.14.3
   Compiling equivalent v1.0.1
   Compiling autocfg v1.2.0
   Compiling indexmap v2.2.6                                                                
   Compiling target-lexicon v0.12.14                                                        
   Compiling winnow v0.6.5
   Compiling quote v1.0.35
   Compiling syn v2.0.57                                                                    
   Compiling smallvec v1.13.2
   Compiling cfg-expr v0.15.8
   Compiling version-compare v0.2.0
   Compiling pkg-config v0.3.30                                                             
   Compiling heck v0.5.0
   Compiling windows_x86_64_gnu v0.48.5
   Compiling pin-project-lite v0.2.14                                                       
   Compiling windows_x86_64_gnu v0.52.4
    Building [==>                       ] 20/138: syn, windows_x86_64_gnu(build.rs), p...   

