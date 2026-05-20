//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2144/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2144<F: Float>(t87233: F, t25068: F, t2703: F, t81764: F, t23127: F, t4257: F, t1512: F, t81807: F, t25146: F, t2686: F, t81824: F, t81821: F) -> (F, F, F, F, F, F, F, F) {
    let t87234 = F::cast_from(0.13457585364713463618e-3_f64) * t87233;
    let t87235 = t25068 * t2703;
    let t87237 = F::new(119.0) / F::new(864.0) * t81764;
    let t87241 = t23127 * t4257;
    let t87243 = t81807 * t1512;
    let t87245 = t25146 * t2686;
    let t87247 = t81824 * t1512;
    let t87248 = F::new(7.0) / F::new(1152.0) * t87247;
    let t87249 = t81821 * t1512;
    (t87234, t87235, t87237, t87241, t87243, t87245, t87248, t87249)
}
