//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1163/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1163<F: Float>(t24574: F, t32547: F, t2121: F, t3427: F, t8867: F, t7299: F, t7348: F, t117813: F, t2122: F, t32499: F, t32520: F, t32530: F) -> (F, F, F, F, F, F, F) {
    let t117834 = t24574 * t32547;
    let t117838 = F::cast_from(0.36554090374405031922e-2_f64) * t2121 * t3427 * t8867;
    let t117840 = t7299 * t7348;
    let t117855 = t2122 * t117813;
    let t117897 = t24574 * t32499;
    let t117910 = t24574 * t32520;
    let t117924 = t24574 * t32530;
    (t117834, t117838, t117840, t117855, t117897, t117910, t117924)
}
