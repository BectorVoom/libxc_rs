//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 751/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk751<F: Float>(t5: F, t27991: F, t112: F, t1868: F, t5456: F, t1873: F, t19451: F, t1441: F, t1458: F) -> (F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t27992 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t27991);
    let t27993 = t27992 * t112;
    let t27996 = t1868 * t5456;
    let t28001 = F::cast_from(2.0_f64) * t19451 * t1873;
    let t28002 = t1441 * t1458;
    (t27992, t27993, t27996, t28001, t28002)
}
