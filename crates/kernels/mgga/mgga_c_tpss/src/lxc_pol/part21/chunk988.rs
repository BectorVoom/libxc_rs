//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 988/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk988<F: Float>(t10509: F, t10512: F, t10518: F, t10520: F, t10521: F, t10522: F, t10523: F, t10524: F, t10526: F, t10528: F, t7954: F, t7960: F, t7972: F, t7975: F, t8112: F, t8117: F) -> (F,) {
    let t10682 = t10509 + t10512 - t7954 - t7960 + t7972 + t7975 + t10518 + t10520 + t10521 + t10522 + t8112 - t8117 + t10523 - t10524 + t10526 + t10528;
    (t10682,)
}
