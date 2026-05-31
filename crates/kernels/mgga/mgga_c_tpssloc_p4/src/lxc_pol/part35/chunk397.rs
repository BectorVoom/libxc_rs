//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 397/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk397<F: Float>(t1315: F, t1327: F, t1341: F, t1360: F, t1363: F, t1811: F, t1815: F, t1827: F, t1831: F, t559: F) -> F {
    let t1834 = -t1327 - t1315 * t1811 / F::cast_from(48.0_f64) + t1815 * t559 / F::cast_from(3072.0_f64) - t1341 * t1827 / F::cast_from(3072.0_f64) - t1360 - t1363 * t1831 / F::cast_from(768.0_f64);
    t1834
}
