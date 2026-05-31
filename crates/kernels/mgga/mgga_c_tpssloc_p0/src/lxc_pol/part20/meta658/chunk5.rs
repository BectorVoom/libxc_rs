//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2445/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2445<F: Float>(t10390: F, t10860: F, t13536: F, t14235: F, t1622: F, t3070: F, t3073: F, t42397: F, t42648: F, t43114: F, t43118: F, t43298: F, t4641: F, t49964: F, t49966: F, t49972: F, t49976: F, t49984: F, t49987: F, t49989: F) -> F {
    let t49991 = -t43298 * t1622 / F::cast_from(288.0_f64) + t49964 / F::cast_from(768.0_f64) + t49966 / F::cast_from(1152.0_f64) + F::cast_from(19.0_f64) / F::cast_from(864.0_f64) * t42648 * t1622 + t4641 * t10860 / F::cast_from(3072.0_f64) - t49972 / F::cast_from(216.0_f64) - t43114 / F::cast_from(3456.0_f64) + F::cast_from(5.0_f64) / F::cast_from(1728.0_f64) * t3070 * t42397 * t13536 * t49976 + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t10390 * t14235 + t43118 / F::cast_from(4608.0_f64) - t49984 * t3073 / F::cast_from(144.0_f64) - t49987 / F::cast_from(144.0_f64) - t49989 / F::cast_from(144.0_f64);
    t49991
}
