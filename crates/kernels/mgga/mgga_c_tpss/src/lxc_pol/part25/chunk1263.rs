//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1263/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1263<F: Float>(t114: F, t1338: F, t6399: F, t21027: F, t5909: F, t1799: F, t5314: F, t1830: F, t4674: F, t18690: F, t21011: F, t18622: F, t19588: F, t21185: F, t21187: F) -> (F, F, F, F, F, F) {
    let t115 = F::new(1.0) < t114;
    let t21880 = t6399 * t1338;
    let t21883 = t5909 * t21027;
    let t21894 = t5314 * t1799;
    let t21897 = t1830 * t4674;
    let t21900 = t18690 * t21011;
    let t21907 = piecewise3::<F>(t115, F::new(0.0), t18622 + F::new(4.0) / F::new(3.0) * t19588 + t21185 / F::new(2.0) - t21187 / F::new(4.0));
    (t21880, t21883, t21894, t21897, t21900, t21907)
}
