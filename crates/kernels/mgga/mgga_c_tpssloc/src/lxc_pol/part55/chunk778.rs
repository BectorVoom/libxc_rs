//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 778/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk778<F: Float>(t265: F, t504: F, t1238: F, t2121: F, t2155: F, t498: F, t7283: F, t7351: F, t8868: F, t8872: F, t8883: F, t8888: F, t8898: F, t2157: F, t1256: F, t193: F, t336: F, t3640: F, t8424: F) -> (F, F, F) {
    let t505 = t265 < t504;
    let t8900 = 0.16449340668482264365e-1 * t2121 * t8868 - 0.16449340668482264365e-1 * t7283 * t8872 + t8883 * t498 - 2.0 * t7351 * t2155 + 2.0 * t1238 * t8888 - t1238 * t8898;
    let t8904 = t2157 * t2157;
    let t8909 = piecewise3(t505, t1256 * t193 * t336 * t8900 - t193 * t336 * t3640 * t8904, t8424);
    (t8900, t8904, t8909)
}
