//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1051/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1051<F: Float>(t114456: F, t114513: F, t114515: F, t114517: F, t114520: F, t114525: F, t114527: F, t114529: F, t114531: F, t115972: F, t115978: F, t115980: F, t2363: F, t23880: F, t23917: F, t24478: F, t31795: F, t577: F, t7010: F, t7235: F, t83980: F, t8508: F) -> F {
    let t115981 = F::cast_from(0.135e2_f64) * t7010 * t23917 + t114513 + t114515 + t114517 + t114520 + t114456 + F::cast_from(54.0_f64) * t23880 * t24478 + t8508 + t114525 + t114527 + t114529 + t114531 + F::cast_from(0.135e2_f64) * t31795 * t2363 + F::cast_from(0.45e1_f64) * t115972 * t577 + F::cast_from(54.0_f64) * t83980 * t7235 + t115978 + t115980;
    t115981
}
