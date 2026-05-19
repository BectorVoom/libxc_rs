//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 835/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk835<F: Float>(t462: F, t8891: F, t493: F, t8882: F, t2121: F, t470: F) -> (F, F, F) {
    let t8892 = t462 * t8891;
    let t8895 = t493 * t8882;
    let t8897 = F::cast_from(0.16449340668482264365e-1_f64) * t2121 * t8892 + t470 * t8895;
    (t8892, t8895, t8897)
}
