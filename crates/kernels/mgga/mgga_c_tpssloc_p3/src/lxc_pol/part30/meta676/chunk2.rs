//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2109/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2109<F: Float>(t3953: F, t3961: F, t3967: F, t1437: F, t4017: F, t72: F, t1433: F, t4021: F, t1865: F, t22523: F, t22554: F, t26063: F, t26067: F, t26084: F, t27966: F, t27972: F, t6490: F, t6506: F, t6510: F, t7432: F, t90308: F, t90312: F) -> F {
    let t96479 = t3953 * t3961;
    let t96482 = t3953 * t3967;
    let t96502 = t72 * t4017 * t1437;
    let t96506 = t72 * t1433 * t4021;
    let t96509 = F::new(2.0) / F::new(3.0) * t96479 * t1865 + F::new(2.0) / F::new(3.0) * t96482 * t1865 + F::new(2.0) / F::new(3.0) * t27966 * t6506 + F::new(2.0) / F::new(3.0) * t27966 * t6510 + F::new(5.0) / F::new(3.0) * t90308 * t7432 + F::new(5.0) / F::new(3.0) * t90312 * t7432 + F::new(5.0) / F::new(3.0) * t26084 * t26063 + F::new(5.0) / F::new(3.0) * t26084 * t26067 + F::new(5.0) / F::new(3.0) * t22554 * t27972 + F::new(5.0) / F::new(3.0) * t22523 * t27972 + F::new(5.0) / F::new(3.0) * t6490 * t96502 + F::new(5.0) / F::new(3.0) * t6490 * t96506;
    t96509
}
