//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1025/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1025<F: Float>(t1409: F, t2132: F, t2136: F, t460: F, t4928: F, t7320: F, t210: F, t7998: F, t1193: F, t8020: F, t1198: F, t2134: F, t24723: F, t24729: F, t24733: F, t24741: F, t4950: F, t4954: F, t4980: F, t4984: F, t5046: F, t7310: F, t7316: F, t7321: F, t8028: F, t8031: F, t8035: F) -> (F,) {
    let t27650 = t2132 * t1409;
    let t27651 = t27650 * t2136;
    let t27654 = t4928 * t460;
    let t27655 = t27654 * t7320;
    let t27674 = t7998 * t210;
    let t27677 = t8020 * t1193;
    let t27679 = -0.10093189023535097714e-3 * t27651 + 0.10093189023535097714e-3 * t24723 - 0.10093189023535097714e-3 * t2134 * t27655 + 0.10093189023535097714e-3 * t7316 * t8035 - t24741 * t4950 / 2304.0 - t24741 * t4954 / 2304.0 + t24729 * t4980 / 768.0 - t24733 * t4984 / 1536.0 - t7310 * t5046 / 288.0 + 0.80745512188280781712e-3 * t8028 * t7321 + 0.10093189023535097714e-3 * t8031 * t7321 + t27674 * t1198 / 108.0 - t27677 / 108.0;
    (t27679,)
}
