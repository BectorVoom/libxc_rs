//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1356/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1356<F: Float>(t1307: F, t7752: F, t22574: F, t8643: F, t33085: F, t6535: F, t22461: F, t7461: F, t26103: F, t25980: F, t6517: F, t26179: F, t8327: F) -> (F, F, F, F, F, F) {
    let t120705 = t7752 * t1307;
    let t120708 = F::cast_from(6.0_f64) * t22574 * t8643 * t120705;
    let t120709 = t33085 * t6535;
    let t120711 = t22461 * t7461;
    let t120714 = t26103 * t7461;
    let t120716 = t6517 * t25980;
    let t120719 = F::cast_from(2.0_f64) * t26179 * t8327;
    (t120708, t120709, t120711, t120714, t120716, t120719)
}
