//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1397/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1397<F: Float>(t106951: F, t1268: F, t1458: F, t5449: F, t1873: F, t19451: F, t7467: F, t1983: F, t2019: F, t74014: F, t1390: F, t2018: F, t20356: F) -> (F, F, F, F, F, F) {
    let t106953 = F::new(2.0) * t1268 * t106951;
    let t106956 = t5449 * t1458;
    let t106958 = F::new(6.0) * t106956 * t1873;
    let t106960 = F::new(6.0) * t19451 * t7467;
    let t106964 = t1983 * t2019 * t74014;
    let t106968 = F::new(6.0) * t1983 * t20356 * t2018 * t1390;
    (t106953, t106956, t106958, t106960, t106964, t106968)
}
