//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2450/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2450<F: Float>(t13536: F, t17635: F, t10236: F, t21510: F, t13554: F, t10235: F, t13769: F, t13798: F, t13851: F, t13861: F, t17748: F, t17794: F, t17800: F, t21447: F, t2960: F, t2986: F, t340: F, t343: F, t42893: F, t4510: F, t4531: F, t48180: F, t61094: F, t61375: F, t61528: F, t61589: F, t68477: F, t68525: F, t69579: F, t69598: F, t69615: F, t7577: F, t884: F, t973: F, t974: F) -> (F, F, F) {
    let t69643 = t13536 * t17635;
    let t69647 = t10236 * t21510;
    let t69657 = t13554 * t17635;
    let t69665 = F::cast_from(0.22222222222222222222e-2_f64) * t2960 * t21447 - F::cast_from(0.27777777777777777777e-3_f64) * t69579 + F::cast_from(0.10288065843621399177e-3_f64) * t42893 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t974 * t340 * (t69598 + t69615) * t343 + F::cast_from(0.25925925925925925925e-2_f64) * t2986 * t13798 * t68525 + F::cast_from(0.28806584362139917695e-2_f64) * t2986 * t48180 * t68477 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t17800 * t13861 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t13851 * t17794 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t4531 * t61589 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t17800 * t17748 + F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t4531 * t61094 + F::cast_from(0.25925925925925925926e-2_f64) * t2986 * t13798 * t69643 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t10235 * t69647 + F::cast_from(0.66666666666666666665e-2_f64) * t2986 * t13769 * t61375 - F::cast_from(0.22222222222222222222e-2_f64) * t2986 * t13769 * t61528 - F::cast_from(0.66666666666666666665e-2_f64) * t2986 * t4510 * t69657 - F::cast_from(0.8333333333333333333e-3_f64) * t2986 * t17800 * t7577 * t884;
    (t69643, t69657, t69665)
}
