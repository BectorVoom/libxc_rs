//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 894/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk894<F: Float>(t254: F, t853: F, t776: F, t865: F, t799: F, t10143: F, t1081: F, t28: F, t40772: F, t2018: F, t3734: F, t1307: F, t1385: F) -> (F, F, F, F, F, F, F) {
    let t87013 = t853 * t254;
    let t87036 = t776 * t865;
    let t87755 = t799 * t254;
    let t89849 = t10143 * t1081;
    let t89953 = t40772 * t28;
    let t90065 = t3734 * t2018;
    let t90506 = t1307 * t1385;
    (t87013, t87036, t87755, t89849, t89953, t90065, t90506)
}
