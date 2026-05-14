//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 750/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk750<F: Float>(t15302: F, t56828: F, t12200: F, t1587: F, t2044: F, t3076: F, t15306: F, t36596: F, t1614: F, t7273: F, t1615: F, t1986: F, t3141: F, t797: F, t14374: F, t15322: F) -> (F, F, F, F, F, F) {
    let t75648 = t56828 * t15302;
    let t75652 = t12200 * t2044 * t3076 * t1587;
    let t75654 = t36596 * t15306;
    let t75658 = t7273 * t2044 * t3076 * t1614;
    let t75662 = t3141 * t1986 * t797 * t1615;
    let t75664 = t14374 * t15322;
    (t75648, t75652, t75654, t75658, t75662, t75664)
}
