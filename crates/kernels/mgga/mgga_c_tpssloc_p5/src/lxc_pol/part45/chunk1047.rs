//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1047/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1047<F: Float>(t114360: F, t115766: F, t115771: F, t115773: F, t115777: F, t115781: F, t115809: F, t115915: F, t115919: F, t115920: F, t115922: F, t115924: F, t115927: F, t115929: F, t2040: F, t2075: F, t22559: F, t24433: F, t24442: F, t26103: F, t574: F, t6517: F, t6862: F, t7040: F, t7050: F, t90044: F) -> F {
    let t115934 = -F::new(2.0) * t90044 * t2040 - F::new(4.0) * t26103 * t7050 + t115766 - F::new(2.0) * t7040 * t6862 - t115771 - t22559 * t2075 - t115773 + t115777 - t115781 + (t115809 + t115915) * t574 - t115919 - t115920 + t115922 + t115924 - t115927 - t115929 - F::new(6.0) * t114360 * t24433 - F::new(2.0) * t6517 * t24442;
    t115934
}
