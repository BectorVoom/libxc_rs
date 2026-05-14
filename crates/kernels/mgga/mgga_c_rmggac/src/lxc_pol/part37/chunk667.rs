//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 667/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk667<F: Float>(t236: F, t495: F, t589: F, t14125: F, t69009: F, t498: F, t68421: F, t68422: F, t321: F, t21714: F, t333: F, t511: F, t352: F, t515: F, t68455: F, t9122: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t73717 = t236 * t589 * t495;
    let t73719 = t69009 * t14125 * t73717;
    let t73722 = t236 * t589 * t498;
    let t73724 = t68421 * t68422 * t73722;
    let t73727 = t236 * t589 * t321;
    let t73729 = t68421 * t21714 * t73727;
    let t73732 = t511 * t589 * t333;
    let t73734 = t68421 * t14125 * t73732;
    let t73737 = t515 * t589 * t352;
    let t73739 = t68421 * t14125 * t73737;
    let t73743 = t68455 * t14125 * t9122;
    (t73717, t73719, t73722, t73724, t73727, t73729, t73732, t73734, t73737, t73739, t73743)
}
