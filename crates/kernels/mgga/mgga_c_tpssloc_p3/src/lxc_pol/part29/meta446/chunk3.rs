//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1758/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1758<F: Float>(t22740: F, t3792: F, t22897: F, t1992: F, t1336: F, t2013: F, t22743: F, t22746: F, t22749: F, t22753: F, t22871: F, t22874: F, t22877: F, t22879: F, t22884: F, t22888: F, t22896: F, t3773: F, t544: F) -> (F, F, F) {
    let t22898 = t22740 * t3792;
    let t22899 = t22897 * t22898;
    let t22900 = t1992 * t22899;
    let t22903 = -F::cast_from(0.82246703342411321825e-2_f64) * t22743 + t22746 + F::cast_from(0.49348022005446793095e-1_f64) * t22749 + t22753 + t544 * t22871 - F::new(2.0) * t1336 * t22874 - t1336 * t22877 - t1336 * t22879 - F::cast_from(0.3289868133696452873e-1_f64) * t22884 - F::cast_from(0.16449340668482264365e-1_f64) * t22888 + t22896 + F::cast_from(0.16449340668482264365e-1_f64) * t22900 + t3773 * t2013;
    (t22898, t22899, t22903)
}
