//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2449/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2449<F: Float>(t3131: F, t221: F, t339: F, t42813: F, t10216: F, t2978: F, t10479: F, t42333: F, t10922: F, t2960: F, t1041: F, t10868: F, t248: F, t2776: F) -> (F, F, F, F, F, F) {
    let t43292 = t3131 * t3131;
    let t43307 = F::cast_from(5.0_f64) / F::cast_from(486.0_f64) * t339 * t221 * t42813;
    let t43317 = t2978 * t10216;
    let t43322 = t42333 * t10479;
    let t43325 = t2960 * t10922;
    let t43336 = t1041 * t248 * t10868 * t2776;
    (t43292, t43307, t43317, t43322, t43325, t43336)
}
