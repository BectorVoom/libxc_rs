//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2953/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2953<F: Float>(t12652: F, t13536: F, t10236: F, t17691: F, t13779: F, t17183: F, t2986: F, t10186: F, t10235: F, t13769: F, t13798: F, t13839: F, t13851: F, t1539: F, t17748: F, t17795: F, t23494: F, t43055: F, t4531: F, t47919: F, t47927: F, t47941: F, t48217: F, t48221: F, t48269: F) -> (F, F) {
    let t61524 = t13536 * t12652;
    let t61528 = t10236 * t17691;
    let t61557 = t2986 * t13779 * t17183;
    let t61560 = F::cast_from(0.34567901234567901234e-2_f64) * t2986 * t13798 * t61524 - F::cast_from(0.14814814814814814814e-2_f64) * t2986 * t10235 * t61528 + F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t4531 * t47919 + F::cast_from(0.44444444444444444444e-2_f64) * t2986 * t13769 * t47941 - F::cast_from(0.14814814814814814814e-2_f64) * t2986 * t48217 * t13839 - F::cast_from(0.74074074074074074072e-3_f64) * t2986 * t13769 * t48269 - F::cast_from(0.17283950617283950617e-2_f64) * t2986 * t48221 * t47927 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t13851 * t17748 - F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t4531 * t23494 * t1539 + F::cast_from(0.14814814814814814814e-2_f64) * t10186 * t17795 - F::cast_from(0.37037037037037037036e-3_f64) * t61557 + F::cast_from(0.12345679012345679012e-3_f64) * t43055;
    (t61524, t61560)
}
