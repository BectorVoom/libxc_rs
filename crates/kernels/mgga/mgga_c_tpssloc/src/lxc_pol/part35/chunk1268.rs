//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1268/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1268<F: Float>(t22633: F, t28116: F, t90566: F, t22635: F, t26331: F, t26332: F, t6347: F, t107314: F, t107356: F, t107391: F, t107442: F, t1375: F, t1378: F, t1843: F, t20044: F, t2006: F, t20594: F, t20612: F, t26224: F, t26225: F, t26477: F, t28224: F, t5321: F, t568: F, t6440: F, t7750: F, t96913: F, t97558: F, t97664: F) -> (F,) {
    let t107460 = t22633 * t90566 * t28116;
    let t107464 = t26331 * t22635 * t26332 * t6347;
    let t107466 = -18.0 * t5321 * t28224 - 3.0 * t97558 * t1843 - t1375 * t1378 * (t107314 + t107356 + t107391 + t107442) - 0.34543615403812755166e0 * t97664 - 3.0 * t20044 * t7750 - 18.0 * t26224 * t26225 * t20612 + 6.0 * t26477 * t6440 + t20594 * t2006 * t568 - 3.0 * t96913 * t1843 + 0.9869604401089358619e-1 * t107460 + 0.14804406601634037928e0 * t107464;
    (t107466,)
}
