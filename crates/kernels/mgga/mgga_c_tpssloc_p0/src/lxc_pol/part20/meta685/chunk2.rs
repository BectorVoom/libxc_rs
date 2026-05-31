//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2597/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2597<F: Float>(t11692: F, t11697: F, t15563: F, t15743: F, t3490: F, t15239: F, t486: F, t11698: F, t15569: F, t15498: F, t3523: F, t11734: F, t1174: F, t11774: F, t11863: F, t1216: F, t15637: F, t3440: F, t3515: F, t44932: F, t4582: F, t4984: F, t5005: F, t5024: F, t50857: F, t50861: F) -> (F, F) {
    let t52649 = t11692 * t11697 * t15563;
    let t52653 = t3490 * t15743;
    let t52659 = t486 * t15239;
    let t52664 = t15569 * t11698;
    let t52666 = t15498 * t3523;
    let t52668 = -F::cast_from(5.0_f64) / F::cast_from(864.0_f64) * t5024 * t11774 + t1174 * t3440 * t50857 / F::cast_from(216.0_f64) + t1174 * t3440 * t50861 / F::cast_from(6.0_f64) + t52649 / F::cast_from(2304.0_f64) - t5005 * t11863 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t52653 - t44932 * t4984 / F::cast_from(1024.0_f64) - t11734 * t15637 / F::cast_from(512.0_f64) - t3515 * t4582 * t52659 * t1216 / F::cast_from(1024.0_f64) + t52664 / F::cast_from(216.0_f64) + t52666 / F::cast_from(216.0_f64);
    (t52659, t52668)
}
