//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 852/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk852<F: Float>(t2924: F, t952: F, t2932: F, t950: F, t2836: F, t914: F, t2792: F, t2844: F, t912: F, t2842: F, t2880: F, t933: F, t10662: F, t913: F, t2860: F, t919: F) -> (F, F, F, F, F, F, F) {
    let t10720 = t952 * t2924;
    let t10723 = t2924 * t2932;
    let t10724 = t10723 * t950;
    let t10727 = t914 * t2836;
    let t10729 = 6.0 * t2792 * t10727;
    let t10731 = t2836 * t2844 * t912;
    let t10733 = 0.48245938496077605201e2 * t2842 * t10731;
    let t10734 = t933 * t2880;
    let t10737 = t10662 * t913;
    let t10739 = 6.0 * t2842 * t10737;
    let t10740 = t919 * t2860;
    (t10720, t10724, t10729, t10733, t10734, t10739, t10740)
}
