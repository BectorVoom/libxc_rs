//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta385 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1458;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1459;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1460;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1461;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1462;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta385<F: Float>(t118: F, t5527: F, t794: F, t9549: F, t16662: F, t210: F, t214: F, t5544: F, t2576: F, t2563: F, t5555: F, t213: F, t221: F, t776: F, t13014: F, t13020: F, t13022: F, t13027: F, t4127: F, t787: F, t9579: F, t9583: F, t16781: F, t225: F, t10054: F, t5585: F, t13176: F, t1499: F, t1523: F, t1525: F, t16673: F, t16679: F, t16754: F, t16756: F, t16759: F, t16762: F, t255: F, t2617: F, t4162: F, t4166: F, t4286: F, t4291: F, t4296: F, t4298: F, t5645: F, t5648: F, t5653: F, t812: F, t861: F, t252: F, t5584: F, t828: F, t9975: F, t16758: F, t4182: F, t2732: F, t5617: F, t829: F, t4290: F, t4177: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16784, t16787, t16792, t16794, t16796) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1458::<F>(t118, t5527, t794, t9549, t16662, t210, t214, t5544, t2576, t2563, t5555, t213);
        let t16803 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1459::<F>(t16796, t221, t776, t13014, t13020, t13022, t13027, t16784, t16787, t16792, t16794, t4127, t787, t9579, t9583);
        let (t16804, t16805, t16814) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1460::<F>(t16781, t16803, t225, t10054, t5585, t13176, t1499, t1523, t1525, t16673, t16679, t16754, t16756, t16759, t16762, t255, t2617, t4162, t4166, t4286, t4291, t4296, t4298, t5645, t5648, t5653, t812, t861);
        let t16815 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1461::<F>(t252, t5584);
        let (t16816, t16817, t16820, t16823, t16825, t16828, t16830, t16836) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1462::<F>(t828, t9975, t16815, t16758, t4182, t2732, t5617, t829, t1499, t4290, t4166, t4177);
    (t16804, t16805, t16814, t16815, t16816, t16817, t16820, t16823, t16825, t16828, t16830, t16836)
}
