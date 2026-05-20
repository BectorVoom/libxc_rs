//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1370/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1370<F: Float>(t2775: F, t283: F, t135: F, t3142: F, t973: F, t3147: F, t3152: F, t248: F, t3101: F, t3132: F, t3130: F, t225: F, t3167: F) -> (F, F, F, F, F, F) {
    let t10969 = F::new(1.0) / t283 / t2775;
    let t10981 = t135 * t3142;
    let t10982 = t973 * t10981;
    let t10984 = t135 * t3147;
    let t10985 = t973 * t10984;
    let t10993 = t135 * t3152;
    let t10994 = t973 * t10993;
    let t11002 = t248 * t3101 * t3132;
    let t11003 = t3130 * t11002;
    let t11010 = t3167 * t225;
    (t10969, t10982, t10985, t10994, t11003, t11010)
}
