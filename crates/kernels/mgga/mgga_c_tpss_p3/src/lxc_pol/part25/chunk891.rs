//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 891/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk891<F: Float>(t8471: F, t946: F, t2464: F, t265: F, t2458: F, t606: F, t2719: F, t72: F, t2737: F, t2798: F, t2782: F, t2762: F, t774: F) -> (F, F, F, F, F, F, F) {
    let t8472 = t946 * t8471;
    let t8491 = F::cast_from(1.0_f64) / t265 / t2464;
    let t8493 = F::cast_from(1.0_f64) / t2458 / t606;
    let t8507 = t2719 * t72;
    let t8508 = t2737 * t8507;
    let t8509 = t2798 * t8508;
    let t8514 = t2782 * t8508;
    let t8523 = t774 * t2762;
    (t8472, t8491, t8493, t8507, t8509, t8514, t8523)
}
