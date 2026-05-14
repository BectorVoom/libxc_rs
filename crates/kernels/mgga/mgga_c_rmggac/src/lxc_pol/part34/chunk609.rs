//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 609/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk609<F: Float>(t14167: F, t68891: F, t14115: F, t68447: F, t14102: F, t7501: F, t14018: F, t3119: F, t34855: F, t2518: F, t511: F, t3352: F, t2523: F, t880: F, t1971: F, t13975: F, t7244: F) -> (F, F, F, F, F, F, F) {
    let t68892 = t68891 * t14167;
    let t68906 = t68447 * t14115;
    let t68910 = t7501 * t14102;
    let t68922 = t14018 * t34855 * t3119;
    let t68928 = t511 * t2518;
    let t68929 = t3352 * t68928;
    let t68936 = t880 * t2523;
    let t68937 = t1971 * t68936;
    let t68946 = t7244 * t13975;
    (t68892, t68906, t68910, t68922, t68929, t68937, t68946)
}
