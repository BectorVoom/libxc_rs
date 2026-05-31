//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1233/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1233<F: Float>(t5: F, t119941: F, t119993: F, t112: F, t32781: F, t532: F, t1983: F, t6879: F, t26149: F, t8450: F, t33133: F, t7000: F, t33160: F, t6876: F) -> (F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t119995 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t119941 + t119993);
    let t119996 = t119995 * t112;
    let t119999 = t532 * t32781;
    let t120002 = F::cast_from(3.0_f64) * t1983 * t119999 * t6879;
    let t120003 = t8450 * t26149;
    let t120005 = t33133 * t7000;
    let t120008 = F::cast_from(3.0_f64) * t6876 * t33160;
    (t119996, t120002, t120003, t120005, t120008)
}
