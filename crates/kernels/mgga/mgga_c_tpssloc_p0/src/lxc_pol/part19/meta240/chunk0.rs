//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 970/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk970<F: Float>(t11148: F, t11219: F, t136: F, t11154: F, t3297: F, t11161: F, t11170: F, t11195: F, t11197: F, t11200: F, t11204: F, t11206: F, t11209: F, t11211: F, t11213: F, t11215: F, t11217: F) -> (F, F, F, F, F) {
    let t11220 = t11219 * t11148;
    let t11221 = t136 * t11220;
    let t11223 = t3297 * t11154;
    let t11224 = t136 * t11223;
    let t11228 = -t11195 - F::new(0.28483875e1) * t11197 + F::new(0.46074375e0) * t11200 - t11204 + F::cast_from(0.49293999999999999999e0_f64) * t11206 + F::cast_from(0.82156666666666666667e-1_f64) * t11209 + F::cast_from(0.27385555555555555556e0_f64) * t11211 + F::cast_from(0.5477111111111111111e-1_f64) * t11213 - F::cast_from(0.32862666666666666666e0_f64) * t11215 - F::cast_from(0.16431333333333333333e0_f64) * t11217 + F::cast_from(0.36514074074074074075e-1_f64) * t11221 - F::cast_from(0.16431333333333333333e0_f64) * t11224 - F::cast_from(0.59793333333333333333e0_f64) * t11161 + F::new(0.17938e1) * t11170;
    (t11220, t11221, t11223, t11224, t11228)
}
