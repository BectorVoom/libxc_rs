//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1249/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1249<F: Float>(t1012: F, t10515: F, t6753: F, t1933: F, t23479: F, t82916: F, t1025: F, t10360: F, t10463: F, t10493: F, t1929: F, t1932: F, t1934: F, t1937: F, t1941: F, t23433: F, t23529: F, t23544: F, t3057: F, t3064: F, t3123: F, t3134: F, t378: F, t612: F, t6765: F, t82941: F, t82944: F, t82951: F, t82953: F, t82956: F, t82961: F) -> (F,) {
    let t82964 = t1012 * t6753 * t10515;
    let t82971 = t1933 * t82916 * t23479;
    let t82979 = t23544 * t3057 / 768.0 + t6765 * t10463 / 2304.0 + 5.0 / 2304.0 * t23544 * t3064 + 0.60559134141210586284e-3 * t82941 - 0.48447307312968469026e-2 * t82944 + t23433 * t3123 / 512.0 + t6765 * t10493 / 384.0 - 0.30279567070605293142e-3 * t82951 + t82953 / 384.0 - t82956 * t3134 / 48.0 - t23529 * t3057 / 144.0 + t82961 / 768.0 + 19.0 / 288.0 * t82964 * t1025 + t10360 * t1941 * t378 / 1536.0 - 0.60559134141210586284e-3 * t82971 - 0.72670960969452703541e-1 / t1929 / t612 * t1932 * t1934 * t1937;
    (t82979,)
}
