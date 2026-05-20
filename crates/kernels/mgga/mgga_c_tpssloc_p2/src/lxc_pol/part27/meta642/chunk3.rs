//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2188/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2188<F: Float>(t25054: F, t81651: F, t82074: F, t13042: F, t13065: F, t13463: F, t1912: F, t25188: F, t25200: F, t25348: F, t2713: F, t2718: F, t2720: F, t2743: F, t4300: F, t47585: F, t6632: F, t6662: F, t6663: F, t855: F, t87861: F, t87866: F) -> F {
    let t87873 = t81651 * t82074 * t25054;
    let t87874 = F::cast_from(0.16449340668482264365e-1_f64) * t87873;
    let t87880 = F::new(4.0) * t855 * t2718 * t6662 * t4300 + F::new(2.0) * t25348 * t2720 - F::cast_from(0.3289868133696452873e-1_f64) * t87861 - t47585 * t1912 - F::cast_from(0.9869604401089358619e-1_f64) * t87866 - F::new(2.0) * t13042 * t6663 + F::new(4.0) * t2713 * t25200 - t87874 + F::new(4.0) * t13065 * t6632 - F::new(2.0) * t13463 * t6663 - t25188 * t2743;
    t87880
}
