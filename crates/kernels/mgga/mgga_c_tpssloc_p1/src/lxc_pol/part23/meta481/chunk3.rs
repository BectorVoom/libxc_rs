//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1442/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1442<F: Float>(t1740: F, t48: F, t338: F, t11546: F, t1174: F, t15390: F, t18321: F, t3447: F, t44566: F, t463: F, t4919: F, t52124: F, t6127: F, t64878: F, t64881: F, t64885: F, t64979: F, t73433: F, t73444: F, t73451: F, t75836: F, sigma2: F) -> (F, F, F) {
    let t78504 = F::cast_from(1.0_f64) / t48 / t1740;
    let t78505 = sigma2 * t78504;
    let t78506 = t78505 * t338;
    let t78516 = -F::cast_from(0.32592592592592592592e-1_f64) * t73433 - F::cast_from(0.32921810699588477364e-2_f64) * t52124 + F::cast_from(0.66666666666666666664e-2_f64) * t3447 * t4919 * t73444 - F::cast_from(0.44444444444444444444e-2_f64) * t3447 * t15390 * t73451 - F::cast_from(0.1086419753086419753e-1_f64) * t64878 + F::cast_from(0.11111111111111111111e-2_f64) * t64881 + F::cast_from(0.11111111111111111111e-2_f64) * t64885 + F::cast_from(0.21547325102880658436e0_f64) * t78506 * t463 - F::cast_from(0.1037037037037037037e-1_f64) * t1174 * t11546 * t44566 * t75836 - F::cast_from(0.32592592592592592591e-1_f64) * t18321 * t6127 + F::cast_from(0.37037037037037037036e-3_f64) * t64979;
    (t78505, t78506, t78516)
}
