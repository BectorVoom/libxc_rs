//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1909/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1909<F: Float>(t27860: F, t27867: F, t27878: F, t27905: F, t3: F, t112: F, t8110: F, t1458: F, t24969: F, t24972: F, t26533: F, t26535: F, t26537: F, t26539: F, t26541: F, t26544: F, t26547: F, t26549: F, t26552: F, t26554: F, t4072: F, t5376: F, t577: F, t671: F, t7423: F) -> (F, F, F, F) {
    let t27907 = t27860 + t27867 + t27878 + t27905;
    let t27908 = t3 * t27907;
    let t27921 = t8110 * t112;
    let t27930 = F::new(0.45e1) * t27907 * t577 + F::new(0.135e2) * t27921 * t671 + F::new(0.135e2) * t24969 * t1458 + F::new(27.0) * t24972 * t5376 + F::new(0.135e2) * t7423 * t4072 + t26533 + t26535 + t26537 + t26539 + t26541 + t26544 + t26547 + t26549 + t26552 + t26554;
    (t27907, t27908, t27921, t27930)
}
