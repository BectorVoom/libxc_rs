//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2564/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2564<F: Float>(t1117: F, t11190: F, t21724: F, t3313: F, t4781: F, t5989: F, t11424: F, t21895: F, t1147: F, t21826: F, t1128: F, t21975: F) -> (F, F, F, F, F) {
    let t71850 = F::new(24.0) * t11190 * t21724 * t1117;
    let t71853 = F::new(18.0) * t3313 * t5989 * t4781;
    let t71855 = F::new(6.0) * t11424 * t21895;
    let t71860 = t21826 * t1147;
    let t71863 = t21975 * t1128;
    (t71850, t71853, t71855, t71860, t71863)
}
