//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1319/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1319<F: Float>(t14181: F, t19703: F, t4724: F, t61033: F, t14223: F, t14256: F, t19696: F, t215: F, t14240: F, t63993: F, t14245: F, t14311: F, t5559: F) -> (F, F, F, F, F, F, F) {
    let t69934 = t19703 * t14181;
    let t69936 = t61033 * t4724;
    let t69938 = t19703 * t14223;
    let t69941 = t19696 * t215 * t14256;
    let t69945 = t63993 * t215 * t14240;
    let t69948 = t19696 * t215 * t14245;
    let t69950 = t5559 * t14311;
    (t69934, t69936, t69938, t69941, t69945, t69948, t69950)
}
