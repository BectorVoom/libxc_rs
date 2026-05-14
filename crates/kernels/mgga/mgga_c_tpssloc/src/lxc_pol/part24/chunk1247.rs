//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1247/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1247<F: Float>(t3082: F, t6750: F, t607: F, t984: F, t23562: F, t343: F, t3008: F, t40: F, t23482: F, t3: F, t23563: F, t1025: F, t10428: F, t10433: F, t10444: F, t1933: F, t1934: F, t1940: F, t23437: F, t23521: F, t23537: F, t23541: F, t3077: F, t3123: F, t354: F, t378: F, t6735: F, t6747: F, t6758: F, t82880: F, t82883: F, t82885: F, t82893: F, t82897: F, t82911: F) -> (F, F, F) {
    let t82914 = t6750 * t3082;
    let t82916 = t607 * t984;
    let t82918 = t23562 * t82916 * t343;
    let t82921 = t40 * t3008;
    let t82923 = t23562 * t82921 * t343;
    let t82926 = t23482 * t3;
    let t82927 = t82926 * t23563;
    let t82932 = -t82880 * t1025 / 48.0 + t82883 / 768.0 + t82885 / 432.0 - t3077 * t6758 * t378 / 96.0 + 0.60559134141210586284e-3 * t82893 - 0.30279567070605293142e-3 * t82897 - 0.30279567070605293142e-3 * t1933 * t1934 * t3008 * t6735 + t23537 * t10428 / 256.0 - t23541 * t10433 / 512.0 - 209.0 / 1296.0 * t354 * t1940 * t10444 * t378 + 0.30279567070605293142e-3 * t82911 * t23521 - t82914 / 2304.0 - 0.60559134141210586284e-3 * t82918 * t6747 - 0.30279567070605293142e-3 * t82923 * t6747 + 0.48447307312968469026e-2 * t82927 * t6747 - t23437 * t3123 / 96.0;
    (t82916, t82921, t82932)
}
