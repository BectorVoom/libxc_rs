//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2133/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2133<F: Float>(t26168: F, t7685: F, t19924: F, t24995: F, t8945: F, t19456: F, t7468: F, t26003: F, t4028: F, t2314: F, t28864: F, t4034: F) -> (F, F, F, F, F, F) {
    let t96760 = F::cast_from(6.0_f64) * t7685 * t26168;
    let t96763 = F::cast_from(12.0_f64) * t24995 * t8945 * t19924;
    let t96765 = F::cast_from(4.0_f64) * t19456 * t7468;
    let t96767 = F::cast_from(4.0_f64) * t4028 * t26003;
    let t96784 = F::cast_from(2.0_f64) * t2314 * t28864;
    let t96786 = F::cast_from(2.0_f64) * t4034 * t28864;
    (t96760, t96763, t96765, t96767, t96784, t96786)
}
