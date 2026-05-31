//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1460/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1460<F: Float>(t265: F, t504: F, t104677: F, t106606: F, t109096: F, t109137: F, t109743: F, t109778: F, t109809: F, t109844: F, t109888: F, t109927: F, t1256: F, t1763: F, t193: F, t21994: F, t22408: F, t24909: F, t27838: F, t336: F, t4700: F, t6270: F, t6274: F, t7398: F, t86524: F, t95925: F) -> F {
    let t505 = t265 < t504;
    let t109953 = piecewise3::<F>(t505, t193 * t336 * (t109096 + t109137 + t109743 + t109778 + t109809 + t109844 + t109888 + t109927) * t1256 - F::cast_from(3.0_f64) * t4700 * t104677 * t1763 + F::cast_from(6.0_f64) * t4700 * t95925 * t6274 - F::cast_from(3.0_f64) * t4700 * t27838 * t6270 - F::cast_from(6.0_f64) * t4700 * t86524 * t21994 + F::cast_from(6.0_f64) * t4700 * t24909 * t1763 * t6270 - t4700 * t7398 * t22408, t106606);
    t109953
}
