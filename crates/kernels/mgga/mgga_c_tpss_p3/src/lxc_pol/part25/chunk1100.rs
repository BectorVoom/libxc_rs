//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1100/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1100<F: Float>(t14694: F, t14696: F, t14698: F, t14700: F, t14703: F, t14830: F, t14860: F, t14862: F, t14865: F, t14868: F, t14871: F, t14874: F, t14878: F, t14881: F, t14885: F, t14889: F, t14892: F, t14894: F, t15202: F, t198: F, t330: F, t4019: F, t4023: F, t4024: F, t995: F) -> F {
    let t15206 = t15202 * t198 * t330 * t995 - F::cast_from(2.0_f64) * t4019 * t4023 * t4024 - t14694 + t14696 - t14698 - t14700 + t14703 + t14830 - t14860 + t14862 + t14865 - t14868 - t14871 - t14874 + t14878 + t14881 + t14885 - t14889 - t14892 - t14894;
    t15206
}
