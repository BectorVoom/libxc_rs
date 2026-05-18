//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1440/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1440<F: Float>(t103515: F, t103528: F, t103744: F, t103799: F, t103810: F, t15659: F, t22348: F, t24589: F, t24812: F, t27460: F, t27489: F, t27507: F, t27516: F, t27536: F, t27549: F, t29745: F, t29776: F, t29781: F, t29782: F, t3624: F, t3625: F, t5975: F, t5979: F, t6218: F, t7283: F, t7362: F, t7373: F, t8066: F, t8082: F, t85963: F, t85971: F, t85972: F, t95033: F) -> F {
    let t109283 = F::new(0.49348022005446793095e-1) * t24812 * t27489 * t103515 * t15659 - F::new(0.82246703342411321826e-2) * t103744 + F::new(0.54831135561607547884e-2) * t95033 - F::new(0.49348022005446793095e-1) * t7373 * t27536 * t29781 - F::new(0.65797362673929057459e-1) * t27507 * t29745 - F::new(3.0) * t3624 * t8082 * t3625 * t6218 - F::new(0.43864908449286038307e-1) * t103799 - F::new(0.82246703342411321826e-2) * t7283 * t7362 * t27460 * t5979 - F::new(0.16449340668482264365e-1) * t7283 * t7362 * t27460 * t5975 - F::new(0.16449340668482264365e-1) * t103810 - F::new(0.13159472534785811492e0) * t27507 * t29782 + F::new(0.82246703342411321826e-2) * t24589 * t103528 * t8066 - F::new(0.10966227112321509577e-1) * t27549 * t27516 * t29776 - F::new(0.49348022005446793095e-1) * t85963 * t85971 * t22348 * t85972;
    t109283
}
