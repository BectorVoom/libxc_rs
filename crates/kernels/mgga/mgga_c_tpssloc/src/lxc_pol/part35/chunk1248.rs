//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1248/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1248<F: Float>(t109: F, t106944: F, t106946: F, t106948: F, t81438: F, t86586: F, t96713: F, t96721: F, t1268: F, t1458: F, t5449: F, t1873: F, t19451: F, t7467: F, t1983: F, t2019: F, t74014: F) -> (F, F, F, F, F, F) {
    let t110 = 1.0 < t109;
    let t106951 = piecewise3(t110, 0.0, -t81438 - 11.0 / 3.0 * t86586 - 2.0 * t96713 + t96721 - 3.0 / 4.0 * t106944 + 3.0 / 4.0 * t106946 - t106948 / 8.0);
    let t106953 = 2.0 * t1268 * t106951;
    let t106956 = t5449 * t1458;
    let t106958 = 6.0 * t106956 * t1873;
    let t106960 = 6.0 * t19451 * t7467;
    let t106964 = t1983 * t2019 * t74014;
    (t106951, t106953, t106956, t106958, t106960, t106964)
}
