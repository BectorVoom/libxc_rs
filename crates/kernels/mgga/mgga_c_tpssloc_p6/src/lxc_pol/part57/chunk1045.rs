//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1045/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1045<F: Float>(t127560: F, t127562: F, t128908: F, t128909: F, t128922: F, t128924: F, t128926: F, t128943: F, t128970: F, t1869: F, t2075: F, t27993: F, t28002: F, t28030: F, t29197: F, t29378: F, t33133: F, t33204: F, t33350: F, t4028: F, t574: F, t7451: F, t7458: F, t7890: F, t7941: F, t8450: F, t8529: F) -> F {
    let t128973 = t8450 * t29378 - t127560 - t127562 - t27993 * t2075 + F::new(2.0) * t33133 * t7941 - F::new(2.0) * t7451 * t7890 - t1869 * t29197 + t128908 + t128909 - F::new(4.0) * t28002 * t8529 - F::new(4.0) * t4028 * t33350 - F::new(4.0) * t4028 * t33204 - F::new(2.0) * t28030 * t8529 - F::new(4.0) * t7458 * t33350 - t128922 - t128924 + t128926 + (t128943 + t128970) * t574;
    t128973
}
