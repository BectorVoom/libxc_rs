//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2460/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2460<F: Float>(t10305: F, t10390: F, t10857: F, t10858: F, t10891: F, t14041: F, t14103: F, t1539: F, t1616: F, t3070: F, t3071: F, t3121: F, t3130: F, t3131: F, t42397: F, t43325: F, t43332: F, t43336: F, t43341: F, t43350: F, t43352: F, t43354: F, t4347: F, t4582: F, t4593: F) -> F {
    let t50423 = t43325 / F::cast_from(81.0_f64) + F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t3070 * t42397 * t1616 * t10305 + t10390 * t14041 / F::cast_from(1536.0_f64) + t3070 * t3071 * t4347 * t3121 / F::cast_from(1536.0_f64) + t3070 * t3071 * t1539 * t10858 / F::cast_from(4608.0_f64) + t43332 / F::cast_from(216.0_f64) + t43336 / F::cast_from(3456.0_f64) - F::cast_from(5.0_f64) / F::cast_from(20736.0_f64) * t43341 + t3130 * t4582 * t4593 * t3131 * t10857 / F::cast_from(1536.0_f64) + t10891 * t14103 / F::cast_from(192.0_f64) + t43350 / F::cast_from(1536.0_f64) - t43352 / F::cast_from(4608.0_f64) - F::cast_from(19.0_f64) / F::cast_from(2592.0_f64) * t43354;
    t50423
}
