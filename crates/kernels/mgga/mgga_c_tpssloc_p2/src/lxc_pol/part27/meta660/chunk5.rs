//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2310/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2310<F: Float>(t81066: F, t1307: F, t1352: F, t1834: F, t22633: F, t6976: F, t16037: F, t1992: F, t22897: F, t26423: F, t81159: F, t215: F, t22839: F) -> (F, F, F, F, F) {
    let t90903 = F::cast_from(0.16449340668482264365e-1_f64) * t81066;
    let t90907 = t22633 * t6976 * t1834 * t1307 * t1352;
    let t90910 = t1992 * t22897 * t16037;
    let t90912 = t81159 * t26423;
    let t90913 = F::cast_from(0.76763589786250567036e-1_f64) * t90912;
    let t90914 = t22839 * t215;
    (t90903, t90907, t90910, t90913, t90914)
}
