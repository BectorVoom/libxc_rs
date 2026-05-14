//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 865/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk865<F: Float>(t10981: F, t973: F, t135: F, t3147: F, t9258: F, t998: F, t974: F, t3152: F, t2770: F, t976: F, t9288: F, t248: F, t3101: F, t3132: F, t3130: F, t1025: F, t1041: F, t1046: F, t10932: F, t10937: F, t10944: F, t10949: F, t10952: F, t10957: F, t10962: F, t10965: F, t10972: F, t2960: F, t3043: F, t3048: F, t3057: F, t3064: F, t3073: F, t3117: F, t3134: F, t3143: F, t3148: F, t3153: F) -> (F,) {
    let t10982 = t973 * t10981;
    let t10984 = t135 * t3147;
    let t10985 = t973 * t10984;
    let t10987 = t998 * t9258;
    let t10988 = t974 * t10987;
    let t10993 = t135 * t3152;
    let t10994 = t973 * t10993;
    let t10996 = t976 * t2770;
    let t10997 = t10996 * t9288;
    let t10998 = t974 * t10997;
    let t11002 = t248 * t3101 * t3132;
    let t11003 = t3130 * t11002;
    let t11005 = -t973 * t10932 / 36.0 - t10937 * t3073 / 144.0 + 5.0 / 4608.0 * t3117 * t3064 + 7.0 / 648.0 * t973 * t10944 + t10949 * t3134 / 512.0 - t10952 * t3043 / 1024.0 + 19.0 / 864.0 * t10957 * t1046 + t10962 * t1025 / 1024.0 + t10965 * t1046 / 1536.0 + 5.0 / 5184.0 * t1041 * t10972 - t3048 * t3057 / 288.0 - t2960 * t3143 / 36.0 - t2960 * t3148 / 27.0 + t10982 / 288.0 + t10985 / 216.0 + t973 * t10988 / 288.0 + t2960 * t3153 / 18.0 - t10994 / 144.0 + t973 * t10998 / 48.0 + t11003 / 768.0;
    (t11005,)
}
