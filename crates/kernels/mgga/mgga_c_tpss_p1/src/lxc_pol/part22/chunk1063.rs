//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1063/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1063<F: Float>(t2723: F, t8561: F, t3932: F, t3931: F, t2725: F, t2459: F, t969: F, t11476: F, t11594: F, t11598: F, t11602: F, t11609: F, t2722: F, t2740: F, t3945: F, t8559: F, t8568: F, t8989: F, t9031: F, t9033: F, t9038: F, t967: F) -> F {
    let t11612 = t8561 * t2723;
    let t11613 = t3932 * t11612;
    let t11614 = t3931 * t11613;
    let t11617 = t3932 * t2725;
    let t11618 = t3931 * t11617;
    let t11621 = t969 * t2459;
    let t11622 = t11621 * t11476;
    let t11623 = t3931 * t11622;
    let t11628 = -t2740 * t11594 / F::cast_from(1152.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t2740 * t11598 + t2740 * t11602 / F::cast_from(2304.0_f64) - t8989 * t3945 / F::cast_from(432.0_f64) + t2722 * t11609 / F::cast_from(1536.0_f64) + t8559 * t11614 / F::cast_from(512.0_f64) - t8568 * t11618 / F::cast_from(512.0_f64) + t967 * t11623 / F::cast_from(768.0_f64) + F::cast_from(19.0_f64) / F::cast_from(2592.0_f64) * t9031 + t9033 / F::cast_from(1296.0_f64) + t9038;
    t11628
}
