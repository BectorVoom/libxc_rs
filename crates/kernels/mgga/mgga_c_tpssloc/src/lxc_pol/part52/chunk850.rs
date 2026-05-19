//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 850/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk850<F: Float>(t5: F, t67: F, t7973: F, t1864: F, t2109: F, t7445: F, t1860: F, t2110: F, t7246: F, t7428: F, t7432: F, t7435: F, t112: F) -> (F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t7974 = t7973 * t67;
    let t7975 = t7974 * t1864;
    let t7978 = t2109 * t7445;
    let t7982 = piecewise3::<F>(t8, F::new(0.0), -t7428 * t2110 / F::new(6.0) + F::new(5.0) / F::new(6.0) * t7246 * t7432 + t7435 * t2110 / F::new(3.0) - t1860 * t7975 / F::new(6.0) - t1860 * t7978 / F::new(6.0));
    let t7983 = t7982 * t112;
    (t7974, t7975, t7978, t7982, t7983)
}
