//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1488/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1488<F: Float>(t28: F, t265: F, t504: F, t41606: F, t43920: F, t43990: F, t44373: F, t45387: F, t10150: F, t1081: F, t11122: F, t11957: F, t1260: F, t2250: F, t2756: F, t3231: F, t3644: F, t39110: F, t39448: F, t506: F, t52: F, t607: F, t873: F, t9258: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t45390 = piecewise3::<F>(t505, t43920 + t43990 + t44373 + t45387, t41606);
    let t45402 = piecewise3::<F>(t401, t41606 * t28 / F::new(2.0) + F::new(2.0) * t10150 * t1081 + F::new(3.0) * t2756 * t3231 + F::new(2.0) * t873 * t11122 + t265 * t39448 / F::new(2.0), t45390 * t52 / F::new(2.0) - F::new(2.0) * t11957 * t607 - F::new(3.0) * t3644 * t2250 - F::new(2.0) * t1260 * t9258 - t506 * t39110 / F::new(2.0));
    t45402
}
