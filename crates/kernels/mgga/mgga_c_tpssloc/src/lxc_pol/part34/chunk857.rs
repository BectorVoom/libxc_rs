//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 857/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk857<F: Float>(t11285: F, t21906: F, t1156: F, t1137: F, t21854: F, t1671: F, t18686: F, t4740: F, t6021: F, t14850: F, t6024: F, t21723: F, t3315: F, t11190: F, t11444: F, t14702: F, t18203: F, t18219: F, t18229: F, t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F) -> (F, F, F, F, F, F, F, F) {
    let t21942 = t21906 * t11285;
    let t21947 = t21906 * t1156;
    let t21952 = t21854 * t1137;
    let t21956 = 3.0 * t18686 * t1671;
    let t21958 = 3.0 * t4740 * t6021;
    let t21960 = 0.48245938496077605201e2 * t14850 * t6024;
    let t21961 = t21723 * t3315;
    let t21963 = 0.96491876992155210402e2 * t11190 * t21961;
    let t21975 = -t11444 + 0.2283111111111111111e-1 * t14702 + 0.11415555555555555555e-1 * t18203 - 0.34246666666666666665e-1 * t18219 - 0.17123333333333333333e-1 * t18229 + 0.19025925925925925925e-1 * t21760 - 0.68493333333333333331e-1 * t21764 - 0.34246666666666666665e-1 * t21767 + 0.10274e0 * t21771 + 0.10274e0 * t21774 + 0.17123333333333333333e-1 * t21778;
    (t21942, t21947, t21952, t21956, t21958, t21960, t21963, t21975)
}
