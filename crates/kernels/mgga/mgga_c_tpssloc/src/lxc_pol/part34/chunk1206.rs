//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1206/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1206<F: Float>(t102917: F, t102922: F, t107044: F, t1375: F, t1843: F, t20609: F, t2092: F, t27009: F, t27068: F, t29361: F, t3887: F, t5321: F, t6440: F, t6460: F, t6461: F, t7194: F, t74849: F, t7936: F, t90659: F, t90663: F) -> F {
    let t107716 = F::new(6.0) * t1375 * t3887 * t7936 * t6460 - F::new(3.0) * t102922 * t1843 + F::new(6.0) * t27009 * t6440 - t74849 * t2092 - F::new(3.0) * t27068 * t6461 - F::cast_from(0.38381794893125283518e0_f64) * t90659 - F::new(6.0) * t7194 * t20609 - F::cast_from(0.49348022005446793095e-1_f64) * t90663 - F::new(3.0) * t5321 * t29361 - F::cast_from(0.16449340668482264365e-1_f64) * t107044 - F::new(6.0) * t102917 * t1843;
    t107716
}
