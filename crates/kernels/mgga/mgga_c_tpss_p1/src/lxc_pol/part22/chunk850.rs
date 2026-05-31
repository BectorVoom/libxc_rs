//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 850/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk850<F: Float>(t1165: F, t1799: F, t2056: F, t4347: F, t5799: F, t5801: F, t5815: F, t645: F, t1844: F, t508: F) -> (F, F) {
    let t5905 = F::cast_from(2.0_f64) * t1165 * t5815 + F::cast_from(2.0_f64) * t1799 * t2056 + F::cast_from(2.0_f64) * t1799 * t4347 + F::cast_from(2.0_f64) * t5801 * t645 + t5799;
    let t5909 = t508 * t1844;
    (t5905, t5909)
}
