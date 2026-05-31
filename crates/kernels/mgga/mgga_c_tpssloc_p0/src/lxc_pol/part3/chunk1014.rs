//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1014/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1014<F: Float>(t13123: F, t2375: F, t184: F, t3966: F, t607: F, t4194: F, t12606: F, t185: F, t707: F, t4094: F, t706: F, t708: F) -> (F, F, F, F) {
    let t13124 = t13123 * t2375;
    let t13125 = F::cast_from(0.10843581300301739842e-1_f64) * t13124;
    let t13126 = t184 * t3966;
    let t13127 = t13126 * t607;
    let t13129 = F::cast_from(24.0_f64) * t4194 * t13127;
    let t13130 = t185 * t12606;
    let t13132 = F::cast_from(4.0_f64) * t707 * t13130;
    let t13133 = t706 * t4094;
    let t13135 = F::cast_from(8.0_f64) * t13133 * t708;
    (t13125, t13129, t13132, t13135)
}
