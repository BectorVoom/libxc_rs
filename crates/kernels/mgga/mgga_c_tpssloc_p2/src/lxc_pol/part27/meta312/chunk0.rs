//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1380/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1380<F: Float>(t10948: F, t3038: F, t3087: F, t372: F, t364: F, t354: F, t1009: F, t3020: F, t1011: F, t1019: F, t1040: F, t3077: F) -> (F, F, F, F, F, F, F) {
    let t10952 = t10948 * t3038;
    let t10955 = t3087 * t372;
    let t10956 = t364 * t10955;
    let t10957 = t354 * t10956;
    let t10960 = t3020 * t1009;
    let t10961 = t10960 * t1011;
    let t10962 = t10961 * t1019;
    let t10965 = t3077 * t1040;
    (t10952, t10955, t10957, t10960, t10961, t10962, t10965)
}
