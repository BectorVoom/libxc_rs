//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 448/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk448<F: Float>(t2022: F, t3: F, t1401: F, t1873: F, t577: F, t11: F, t2: F, t584: F, t16: F, t9: F, t587: F, t591: F) -> (F, F, F, F, F, F, F, F) {
    let t2023 = t3 * t2022;
    let t2028 = F::new(0.135e2) * t1401 * t1873;
    let t2029 = F::new(0.45e1) * t2022 * t577 + t2028;
    let t2218 = F::new(0.174e1) * t11;
    let t2219 = t2 * t584;
    let t2220 = F::new(0.696e1) * t2219;
    let t2221 = t9 * t16;
    let t2222 = F::new(0.1122e2) * t2221;
    let t2223 = t587 * t591;
    (t2023, t2029, t2218, t2219, t2220, t2221, t2222, t2223)
}
