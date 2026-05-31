//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1327/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1327<F: Float>(t2639: F, t4236: F, t1512: F, t9674: F, t2638: F, t4166: F, t831: F, t2629: F, t4250: F, t9638: F, t1495: F, t210: F, t2379: F) -> (F, F, F, F, F, F) {
    let t13275 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t2639 * t4236;
    let t13277 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t9674 * t1512;
    let t13278 = t4166 * t2638;
    let t13280 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t13278 * t831;
    let t13283 = t4166 * t2629;
    let t13287 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t9638 * t4250;
    let t13289 = t210 * t1495 * t2379;
    (t13275, t13277, t13280, t13283, t13287, t13289)
}
