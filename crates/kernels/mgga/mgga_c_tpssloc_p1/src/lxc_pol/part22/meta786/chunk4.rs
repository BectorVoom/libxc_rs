//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2719/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2719<F: Float>(t1851: F, t671: F, t12524: F, t1395: F, t1401: F, t1458: F, t16521: F, t16524: F, t19534: F, t20162: F, t20173: F, t20176: F, t20181: F, t20347: F, t22445: F, t22448: F, t28893: F, t3938: F, t3941: F, t4072: F, t5371: F, t5376: F, t5456: F, t5493: F, t55353: F, t55388: F, t577: F, t66958: F, t75701: F, t75764: F, t75784: F) -> F {
    let t75795 = t1851 * t671;
    let t75827 = F::new(0.45e1) * t75764 * t577 + F::new(0.135e2) * t75784 * t671 + F::new(0.405e2) * t66958 * t1458 + F::new(81.0) * t55388 * t5376 + F::new(0.405e2) * t20162 * t4072 + F::new(81.0) * t55353 * t5456 + F::new(81.0) * t75795 * t5456 + F::new(162.0) * t16524 * t20176 + F::new(0.405e2) * t16521 * t5493 + F::new(81.0) * t16524 * t20181 + F::new(0.405e2) * t5371 * t19534 + F::new(27.0) * t1395 * t22445 + F::new(81.0) * t28893 * t4072 + F::new(81.0) * t12524 * t22448 + F::new(81.0) * t20173 * t22448 + F::new(81.0) * t3941 * t4072 * t5493 + F::new(81.0) * t3941 * t1458 * t19534 + F::new(0.135e2) * t3938 * t20347 + F::new(27.0) * t3941 * t20347 * t671 + F::new(0.135e2) * t1401 * t75701;
    t75827
}
