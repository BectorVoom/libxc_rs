//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 840/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk840<F: Float>(t118: F, t1307: F, t794: F, t3739: F, t116: F, t534: F, t212: F, t2586: F, t1324: F, t225: F, t2600: F, t541: F) -> (F, F, F, F, F, F) {
    let t3741 = t118 * t794 * t1307;
    let t3742 = t3739 * t3741;
    let t3748 = t534 * t116;
    let t3749 = t3748 * t212;
    let t3751 = F::cast_from(0.83333333333333333332e-3_f64) * t2586 * t3749;
    let t3758 = t1324 * t225;
    let t3762 = F::new(35.0) / F::new(432.0) * t2600 * t541;
    (t3741, t3742, t3749, t3751, t3758, t3762)
}
