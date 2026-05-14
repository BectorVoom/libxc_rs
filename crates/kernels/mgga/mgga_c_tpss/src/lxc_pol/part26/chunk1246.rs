//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1246/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1246<F: Float>(t5: F, t22174: F, t117: F, t1338: F, t20957: F, t21222: F, t21224: F, t21226: F, t21229: F, t21231: F, t21233: F, t22110: F, t4674: F, t5986: F, t1322: F, t1339: F, t1600: F, t1663: F, t1897: F, t1899: F, t21184: F, t21193: F, t21198: F, t21202: F, t21205: F, t21213: F, t21238: F, t21240: F, t21247: F, t21254: F, t4631: F, t4638: F, t4641: F, t485: F, t544: F, t5463: F, t6480: F, t6540: F, t6544: F) -> (F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t22175 = piecewise3(t8, 0.0, t22174);
    let t22176 = t22175 * t117;
    let t22182 = 4.0 * t1338 * t20957 + 2.0 * t4674 * t5986 + t21222 + t21224 + t21226 + t21229 + t21231 + t21233 + 2.0 * t22110 + t22176;
    let t22197 = -2.0 * t1322 * t6540 - 4.0 * t1339 * t20957 - 2.0 * t1600 * t6480 + 2.0 * t1663 * t6544 - t1897 * t4631 - 2.0 * t1897 * t4638 + t1899 * t5463 - t22176 * t485 + t22182 * t544 - 4.0 * t4641 * t5986 - t21184 - t21193 - t21198 - t21202 - t21205 - t21213 - t21238 - t21240 + t21247 + t21254;
    (t22175, t22176, t22182, t22197)
}
