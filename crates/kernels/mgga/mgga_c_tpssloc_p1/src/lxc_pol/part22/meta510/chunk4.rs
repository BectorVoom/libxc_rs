//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1967/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1967<F: Float>(t1156: F, t21938: F, t11285: F, t21906: F, t1137: F, t21854: F, t1671: F, t18686: F, t4740: F, t6021: F, t14850: F, t6024: F) -> (F, F, F, F, F, F, F) {
    let t21939 = t21938 * t1156;
    let t21942 = t21906 * t11285;
    let t21947 = t21906 * t1156;
    let t21952 = t21854 * t1137;
    let t21956 = F::cast_from(3.0_f64) * t18686 * t1671;
    let t21958 = F::cast_from(3.0_f64) * t4740 * t6021;
    let t21960 = F::cast_from(0.48245938496077605201e2_f64) * t14850 * t6024;
    (t21939, t21942, t21947, t21952, t21956, t21958, t21960)
}
