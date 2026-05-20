//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1263/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1263<F: Float>(t24932: F, t7468: F, t27888: F, t26003: F, t7266: F, t1874: F, t96238: F, t27863: F, t6535: F, t26142: F, t25985: F, t8690: F) -> (F, F, F, F, F, F, F) {
    let t123138 = t24932 * t7468;
    let t123140 = t27888 * t7468;
    let t123142 = t7266 * t26003;
    let t123155 = t96238 * t1874;
    let t123164 = t27863 * t6535;
    let t123168 = t7266 * t26142;
    let t123173 = t8690 * t25985;
    (t123138, t123140, t123142, t123155, t123164, t123168, t123173)
}
