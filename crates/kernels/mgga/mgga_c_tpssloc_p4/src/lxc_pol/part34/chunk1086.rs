//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1086/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1086<F: Float>(t1458: F, t576: F, t2035: F, t3701: F, t7939: F, t22811: F, t85: F, t24: F, t12019: F, t566: F, t68: F, t3700: F) -> (F, F, F, F, F, F, F) {
    let t33185 = t576 * t1458;
    let t33234 = t2035 * t1458;
    let t33899 = t3701 * t7939;
    let t39041 = F::cast_from(1.0_f64) / t22811;
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t40590 = F::cast_from(1.0_f64) / t12019 / t566;
    let t40591 = t68 * t40590;
    let t40610 = t3700 * t3700;
    (t33185, t33234, t33899, t39041, t39063, t40591, t40610)
}
