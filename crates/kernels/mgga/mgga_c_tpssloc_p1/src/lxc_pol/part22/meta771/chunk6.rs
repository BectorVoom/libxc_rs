//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2631/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2631<F: Float>(t15419: F, t21745: F, t3447: F, t20234: F, t44505: F, t1171: F, t22104: F, t15313: F, t15320: F, t18409: F, t18416: F, t4904: F, t4919: F, t4920: F, t64756: F, t64775: F, t64811: F, t65035: F, t65041: F, t65093: F, t65112: F, t65126: F) -> (F, F, F) {
    let t73491 = t3447 * t15419 * t21745;
    let t73496 = t44505 * t20234;
    let t73523 = t22104 * t1171;
    let t73525 = F::cast_from(0.8148148148148148148e-2_f64) * t64811 * t4920 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t64775 * t4904 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t18416 * t15313 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t15320 * t18409 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t4919 * t64756 - F::cast_from(0.16666666666666666666e-2_f64) * t65035 - F::cast_from(0.83333333333333333331e-3_f64) * t65041 + F::cast_from(0.44444444444444444443e-2_f64) * t65093 + F::cast_from(0.37037037037037037036e-3_f64) * t65112 - F::cast_from(0.24691358024691358024e-3_f64) * t65126 - F::cast_from(0.12674897119341563786e-1_f64) * t73523;
    (t73491, t73496, t73525)
}
