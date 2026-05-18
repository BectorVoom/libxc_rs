//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1095/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1095<F: Float>(t2857: F, t4105: F, t11873: F, t11875: F, t11942: F, t11880: F, t11885: F, t11890: F, t11896: F, t11899: F, t11904: F, t11908: F, t11938: F, t11952: F, t9221: F, t9223: F, t9226: F, t9228: F, t9331: F) -> (F, F) {
    let t11982 = F::new(2.0) * t2857 * t4105;
    let t11988 = F::new(0.41203703703703703704e-2) * t11873;
    let t11989 = F::new(0.12361111111111111111e-1) * t11875;
    let t11990 = F::new(0.61805555555555555556e-2) * t11942;
    let t11999 = -t9331 + F::new(0.82407407407407407407e-2) * t9221 + F::new(0.20601851851851851852e-2) * t9223 - F::new(0.61805555555555555556e-2) * t9226 - F::new(0.30902777777777777778e-2) * t9228 + F::new(0.41203703703703703704e-2) * t11938 + t11988 - t11989 - t11990 + F::new(0.10300925925925925926e-1) * t11880 - F::new(0.37083333333333333333e-1) * t11885 - F::new(0.12361111111111111111e-1) * t11890 - F::new(0.61805555555555555555e-2) * t11896 + F::new(0.55625000000000000001e-1) * t11899 + F::new(0.37083333333333333334e-1) * t11904 + F::new(0.18541666666666666667e-1) * t11908 + F::new(0.92708333333333333333e-2) * t11952;
    (t11982, t11999)
}
