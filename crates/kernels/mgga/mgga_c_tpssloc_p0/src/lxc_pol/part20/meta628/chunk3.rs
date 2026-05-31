//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2281/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2281<F: Float>(t41282: F, t4205: F, t9926: F, t1462: F, t40709: F, t13126: F, t2250: F, t4194: F, t4195: F, t9258: F, t12890: F, t751: F) -> (F, F, F, F, F, F) {
    let t47149 = F::cast_from(36.0_f64) * t41282;
    let t47151 = F::cast_from(4.0_f64) * t4205 * t9926;
    let t47153 = F::cast_from(4.0_f64) * t40709 * t1462;
    let t47156 = F::cast_from(36.0_f64) * t4194 * t13126 * t2250;
    let t47159 = F::cast_from(12.0_f64) * t4194 * t4195 * t9258;
    let t47160 = t12890 * t751;
    (t47149, t47151, t47153, t47156, t47159, t47160)
}
