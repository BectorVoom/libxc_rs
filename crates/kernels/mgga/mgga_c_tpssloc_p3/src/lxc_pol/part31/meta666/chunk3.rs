//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1958/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1958<F: Float>(t112: F, t29395: F, t100990: F, t100993: F, t12524: F, t1401: F, t1458: F, t16524: F, t19534: F, t20176: F, t24462: F, t24465: F, t27170: F, t27273: F, t27276: F, t28951: F, t29422: F, t29425: F, t33185: F, t3938: F, t5371: F, t5376: F, t5456: F, t5493: F, t55388: F, t671: F, t7230: F, t7235: F, t75795: F, t7956: F, t94127: F, t94170: F) -> F {
    let t100996 = t29395 * t112;
    let t101021 = F::cast_from(0.135e2_f64) * t7230 * t19534 + F::cast_from(0.135e2_f64) * t24462 * t5493 + F::cast_from(27.0_f64) * t55388 * t7235 + F::cast_from(0.135e2_f64) * t1401 * t100990 + F::cast_from(27.0_f64) * t100993 * t5456 + F::cast_from(0.135e2_f64) * t100996 * t671 + F::cast_from(54.0_f64) * t94170 * t5376 + F::cast_from(54.0_f64) * t75795 * t7956 + F::cast_from(54.0_f64) * t16524 * t27273 + F::cast_from(27.0_f64) * t5371 * t27170 + F::cast_from(0.135e2_f64) * t3938 * t28951 + F::cast_from(27.0_f64) * t94127 * t1458 + F::cast_from(27.0_f64) * t12524 * t29425 + F::cast_from(54.0_f64) * t33185 * t27276 + F::cast_from(54.0_f64) * t12524 * t29422 + F::cast_from(54.0_f64) * t16524 * t27276 + F::cast_from(54.0_f64) * t24465 * t20176;
    t101021
}
