//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 913/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk913<F: Float>(t10709: F, t959: F, t2904: F, t2925: F, t950: F, t2880: F, t2888: F, t931: F, t2924: F, t952: F, t2932: F, t2836: F, t914: F) -> (F, F, F, F, F, F) {
    let t10711 = F::new(0.35089341735807877242e1) * t959 * t10709;
    let t10713 = t2904 * t950 * t2925;
    let t10715 = F::new(0.35089341735807877242e1) * t959 * t10713;
    let t10717 = t2880 * t2888 * t931;
    let t10720 = t952 * t2924;
    let t10723 = t2924 * t2932;
    let t10724 = t10723 * t950;
    let t10727 = t914 * t2836;
    (t10711, t10715, t10717, t10720, t10724, t10727)
}
