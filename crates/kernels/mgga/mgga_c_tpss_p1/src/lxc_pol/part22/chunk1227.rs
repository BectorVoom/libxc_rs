//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1227/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1227<F: Float>(t1795: F, t2061: F, t10456: F, t1165: F, t13146: F, t1799: F, t18627: F, t18680: F, t18898: F, t2056: F, t2105: F, t4347: F, t5801: F, t5815: F, t645: F, t7798: F) -> (F, F) {
    let t18903 = t1795 * t2061;
    let t18919 = F::cast_from(4.0_f64) * t10456 * t1799 + F::cast_from(2.0_f64) * t1165 * t18627 + F::cast_from(2.0_f64) * t13146 * t1799 + F::cast_from(2.0_f64) * t1799 * t7798 + F::cast_from(4.0_f64) * t18898 * t645 + F::cast_from(4.0_f64) * t2056 * t5815 + F::cast_from(2.0_f64) * t2105 * t5801 + F::cast_from(4.0_f64) * t4347 * t5815 + t18680 + F::cast_from(2.0_f64) * t18903;
    (t18903, t18919)
}
