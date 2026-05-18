//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1436/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1436<F: Float>(t1760: F, t5392: F, t103130: F, t103143: F, t103149: F, t103188: F, t103314: F, t1238: F, t1653: F, t1716: F, t22004: F, t22394: F, t24589: F, t24601: F, t27406: F, t27820: F, t29536: F, t29794: F, t29804: F, t29808: F, t29816: F, t3598: F, t5055: F, t6146: F, t7283: F, t7351: F, t8002: F, t8010: F, t85652: F, t94458: F, t94514: F) -> (F, F) {
    let t109060 = t5392 * t1760;
    let t109096 = F::new(0.82246703342411321826e-2) * t24589 * t24601 * t103314 * t1653 + F::new(0.16449340668482264365e-1) * t24589 * t24601 * t85652 * t109060 + F::new(0.16449340668482264365e-1) * t24589 * t27820 * t29816 + F::new(0.54831135561607547883e-2) * t103130 + F::new(0.16449340668482264365e-1) * t24589 * t94458 * t29808 + F::new(0.16449340668482264365e-1) * t24589 * t103143 * t8002 + F::new(0.54831135561607547883e-2) * t103149 - F::new(0.13159472534785811492e0) * t27406 * t29804 + F::new(0.49348022005446793095e-1) * t7283 * t1716 * t103188 - F::new(0.16449340668482264365e-1) * t24589 * t94514 * t29808 - F::new(0.24674011002723396548e-1) * t7283 * t6146 * t8010 + F::new(6.0) * t7351 * t22004 + F::new(6.0) * t1238 * t3598 * t29794 * t1760 - t7351 * t22394 + F::new(6.0) * t5055 * t29536;
    (t109060, t109096)
}
