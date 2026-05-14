//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1093/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1093<F: Float>(t18670: F, t5489: F, t1791: F, t18351: F, t5492: F, t5791: F, t1844: F, t507: F, t3205: F) -> (F, F, F, F, F) {
    let t18671 = t18670 * t5489;
    let t18673 = t1791 * t18351;
    let t18676 = t5492 * t5791;
    let t18686 = t507 * t1844;
    let t18690 = t1844 * t3205;
    (t18671, t18673, t18676, t18686, t18690)
}
