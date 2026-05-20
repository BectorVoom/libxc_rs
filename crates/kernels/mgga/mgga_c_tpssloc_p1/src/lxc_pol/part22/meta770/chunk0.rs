//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2621/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2621<F: Float>(t14730: F, t17635: F, t1193: F, t22104: F, t22038: F, t3448: F, t20234: F, t44607: F, t15376: F, t18446: F, t11569: F, t15313: F, t18410: F, t18413: F, t18420: F, t18424: F, t18428: F, t18443: F, t18466: F, t18470: F, t18475: F, t3447: F, t3451: F, t4900: F, t4905: F, t4908: F, t4909: F, t64624: F, t64627: F, t64632: F, t64811: F, t71189: F, t71197: F, t71201: F) -> (F, F, F) {
    let t73138 = t14730 * t17635;
    let t73142 = t22104 * t1193;
    let t73169 = t3448 * t22038;
    let t73181 = t44607 * t20234;
    let t73188 = t15376 * t18446;
    let t73192 = F::cast_from(0.74074074074074074072e-3_f64) * t64624 - F::cast_from(0.37037037037037037036e-3_f64) * t64627 - F::cast_from(0.86419753086419753084e-3_f64) * t64632 - F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t4908 * t71189 - F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t4908 * t71201 - F::cast_from(0.59259259259259259258e-2_f64) * t15376 * t18466 + F::cast_from(0.29629629629629629629e-2_f64) * t15376 * t18470 - F::cast_from(0.17777777777777777778e-1_f64) * t15376 * t18475 + F::cast_from(0.81481481481481481478e-2_f64) * t64811 * t4905 - F::cast_from(0.16296296296296296296e-1_f64) * t64811 * t4909 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t18420 * t15313 + F::cast_from(0.27777777777777777777e-3_f64) * t3447 * t73169 * t3451 - F::cast_from(0.22222222222222222221e-2_f64) * t15376 * t18410 + F::cast_from(0.44444444444444444442e-2_f64) * t15376 * t18413 + F::cast_from(0.88888888888888888884e-2_f64) * t15376 * t18424 - F::cast_from(0.44444444444444444442e-2_f64) * t15376 * t18428 - F::cast_from(0.22222222222222222221e-2_f64) * t3447 * t11569 * t73181 + F::cast_from(0.13333333333333333332e-1_f64) * t3447 * t4900 * t71197 - F::cast_from(0.14814814814814814814e-2_f64) * t73188 + F::cast_from(0.69135802469135802468e-2_f64) * t15376 * t18443;
    (t73138, t73142, t73192)
}
