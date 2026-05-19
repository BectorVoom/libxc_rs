//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1209/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1209<F: Float>(t22648: F, t6897: F, t794: F, t12021: F, t12030: F, t1375: F, t1386: F, t3888: F, t6963: F, t6992: F, t80704: F, t80709: F, t80711: F, t80714: F, t80722: F, t80725: F, t80728: F, t80735: F) -> F {
    let t80738 = t6897 * t794 * t22648;
    let t80740 = -F::new(3.0) * t80704 * t1386 - F::cast_from(0.24674011002723396548e-1_f64) * t80709 - F::cast_from(0.78134368175290755733e-1_f64) * t80711 - F::cast_from(0.49348022005446793095e-1_f64) * t80714 + F::new(6.0) * t12030 * t6963 - F::new(18.0) * t1375 * t12021 * t6992 * t3888 + F::cast_from(0.19190897446562641759e0_f64) * t80722 + F::cast_from(0.12337005501361698274e-1_f64) * t80725 - F::cast_from(0.34543615403812755166e0_f64) * t80728 - F::cast_from(0.19739208802178717238e0_f64) * t80735 - F::cast_from(0.12337005501361698274e-1_f64) * t80738;
    t80740
}
