//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1006/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1006<F: Float>(t13724: F, t13752: F, t13791: F, t13864: F, t219: F, t5428: F, t10180: F, t1265: F, t5432: F, t1656: F, t3365: F, t4516: F) -> (F, F, F, F, F) {
    let t13866 = t13724 + t13752 + t13791 + t13864;
    let t13867 = param_beta * t13866;
    let t13869 = t5428 * t219;
    let t13880 = t10180 * t5432 * t1265;
    let t13884 = t3365 * t1656 * t4516;
    (t13866, t13867, t13869, t13880, t13884)
}
