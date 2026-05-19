//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1406/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1406<F: Float>(t106982: F, t106986: F, t106991: F, t107024: F, t107048: F, t107220: F, t107230: F, t107238: F, t107270: F, t107466: F, t107486: F, t12021: F, t1375: F, t1390: F, t1983: F, t20609: F, t26477: F, t28111: F, t28224: F, t3887: F, t5215: F, t533: F, t6439: F, t6460: F, t6461: F, t6958: F, t7749: F, t90503: F, t90521: F, t96848: F, t96868: F, t96878: F, t96893: F, t97571: F, t97573: F, t97599: F) -> F {
    let t107492 = t1983 * t533 * (-F::new(3.0) * t26477 * t6461 - F::new(6.0) * t6958 * t20609 - F::new(18.0) * t5215 * t28224 + F::new(6.0) * t5215 * t28111 - F::cast_from(0.38381794893125283518e0_f64) * t90521 + F::cast_from(0.19190897446562641759e0_f64) * t90503 + t107220 + t107270 - F::cast_from(0.74022033008170189643e-1_f64) * t96848 - F::cast_from(0.24674011002723396548e-1_f64) * t106986 + F::cast_from(0.57572692339687925277e-1_f64) * t96868 + F::cast_from(0.12337005501361698274e-1_f64) * t96878 + t107048 + t107466 - F::cast_from(0.49348022005446793095e-1_f64) * t107230 - F::cast_from(0.14804406601634037928e0_f64) * t106991 - F::cast_from(0.12337005501361698274e-1_f64) * t97599 + F::cast_from(0.24674011002723396548e-1_f64) * t96893 + F::cast_from(0.49348022005446793095e-1_f64) * t106982 + t107486 - F::cast_from(0.49348022005446793095e-1_f64) * t107238 + t107024 - F::new(18.0) * t1375 * t12021 * t7749 * t6439 + F::new(6.0) * t1375 * t3887 * t7749 * t6460 - F::cast_from(0.24674011002723396548e-1_f64) * t97571 + F::cast_from(0.11514538467937585055e0_f64) * t97573) * t1390;
    t107492
}
