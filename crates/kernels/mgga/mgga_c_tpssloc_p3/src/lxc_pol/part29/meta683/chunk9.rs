//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2323/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2323<F: Float>(t15418: F, t2121: F, t4724: F, t24720: F, t27710: F, t24722: F, t11588: F, t4729: F, t14749: F, t14753: F, t15455: F, t15764: F, t2140: F, t3448: F, t488: F, t7345: F, t86341: F, t86343: F, t86348: F, t86350: F) -> F {
    let t95587 = t2121 * t15418 * t4724 / F::cast_from(324.0_f64);
    let t95588 = t27710 * t24720;
    let t95590 = F::cast_from(0.16149102437656156342e-2_f64) * t95588 * t24722;
    let t95593 = t2121 * t11588 * t4729 / F::cast_from(216.0_f64);
    let t95603 = -F::cast_from(5.0_f64) / F::cast_from(2592.0_f64) * t7345 * t15455 - t86341 / F::cast_from(864.0_f64) - t86343 / F::cast_from(432.0_f64) + t86348 / F::cast_from(5184.0_f64) - t86350 / F::cast_from(3456.0_f64) + t95587 - t95590 - t95593 - t2121 * t3448 * t14749 / F::cast_from(72.0_f64) - t2121 * t3448 * t14753 / F::cast_from(144.0_f64) + t15764 * t2140 * t488 / F::cast_from(1536.0_f64);
    t95603
}
