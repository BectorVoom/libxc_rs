//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2131/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2131<F: Float>(t50077: F, t3070: F, t43198: F, t4578: F, t4574: F, t10510: F, t4641: F, t1020: F, t1616: F, t248: F, t43216: F, t10882: F, t48569: F) -> (F, F, F, F, F, F) {
    let t50078 = t50077 / F::cast_from(162.0_f64);
    let t50147 = t3070 * t43198 * t4578;
    let t50148 = t50147 / F::cast_from(6912.0_f64);
    let t50169 = t3070 * t43198 * t4574;
    let t50170 = t50169 / F::cast_from(6912.0_f64);
    let t50174 = t4641 * t10510;
    let t50175 = t50174 / F::cast_from(4608.0_f64);
    let t50181 = t1020 * t248 * t43216 * t1616;
    let t50193 = t48569 * t10882;
    (t50078, t50148, t50170, t50175, t50181, t50193)
}
