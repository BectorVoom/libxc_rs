//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1000/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1000<F: Float>(t12742: F, t13613: F, t13615: F, t13616: F, t13617: F, t13621: F, t13622: F, t7945: F, t7954: F, t7960: F, t7972: F, t7975: F, t9886: F, t9900: F, t9903: F, t9906: F) -> F {
    let t13804 = t13613 + t7945 - t13615 + t9886 - t13616 + t13617 + t9900 + t9903 - t9906 - t13621 - t7954 + t12742 + t13622 - t7960 + t7972 + t7975;
    t13804
}
