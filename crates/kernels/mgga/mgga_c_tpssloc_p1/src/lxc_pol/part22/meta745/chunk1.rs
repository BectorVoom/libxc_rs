//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2474/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2474<F: Float>(t1023: F, t10413: F, t14077: F, t21516: F, t21532: F, t3039: F, t3048: F, t3070: F, t3071: F, t42483: F, t42546: F, t4347: F, t4582: F, t48611: F, t48670: F, t48674: F, t5681: F, t5867: F, t5869: F, t61866: F, t70086: F, t70122: F, t70389: F, t70391: F) -> F {
    let t70396 = t42483 * t48611 * t70122 * t1023 / F::cast_from(1024.0_f64) + t48670 / F::cast_from(3456.0_f64) + t61866 / F::cast_from(768.0_f64) + t48674 / F::cast_from(5184.0_f64) - t42546 * t21532 / F::cast_from(1536.0_f64) + t3070 * t3071 * t5867 * t4347 / F::cast_from(1536.0_f64) + t10413 * t3071 * t5681 * t70086 / F::cast_from(768.0_f64) - t14077 * t5869 / F::cast_from(192.0_f64) - F::cast_from(5.0_f64) / F::cast_from(972.0_f64) * t3048 * t21516 + F::cast_from(5.0_f64) / F::cast_from(7776.0_f64) * t70389 - t3039 * t4582 * t70391 * t1023 / F::cast_from(3072.0_f64);
    t70396
}
