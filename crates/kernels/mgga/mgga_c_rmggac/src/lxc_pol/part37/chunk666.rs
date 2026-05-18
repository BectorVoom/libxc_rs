//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 666/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk666<F: Float>(t1587: F, t26: F, t1652: F, t880: F, t892: F, t1679: F, t2144: F, t14100: F, t14208: F, t3116: F, t4443: F, t14045: F, t14123: F) -> (F, F, F, F, F, F, F, F, F) {
    let t55986 = t26 * t1587;
    let t56399 = t26 * t1652;
    let t56828 = t892 * t880;
    let t56963 = t1679 * t2144;
    let t61965 = t1679 * t880;
    let t68336 = F::new(0.39726959900411316772e-4) * t14100;
    let t68354 = F::new(0.15965655602485078085e0) * t14208;
    let t68355 = t4443 * t3116;
    let t68357 = t14045 * t68355 * t14123;
    (t55986, t56399, t56828, t56963, t61965, t68336, t68354, t68355, t68357)
}
