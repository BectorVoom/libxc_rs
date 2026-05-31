//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2136/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2136<F: Float>(t1390: F, t19631: F, t1983: F, t6878: F, t25989: F, t91655: F, t1845: F, t5356: F, t26161: F, t26162: F, t26114: F, t7468: F) -> (F, F, F, F) {
    let t96824 = t1390 * t19631;
    let t96827 = F::cast_from(3.0_f64) * t1983 * t6878 * t96824;
    let t96829 = F::cast_from(6.0_f64) * t91655 * t25989;
    let t96830 = t1845 * t5356;
    let t96833 = F::cast_from(4.0_f64) * t26161 * t26162 * t96830;
    let t96837 = F::cast_from(4.0_f64) * t26114 * t7468;
    (t96827, t96829, t96833, t96837)
}
