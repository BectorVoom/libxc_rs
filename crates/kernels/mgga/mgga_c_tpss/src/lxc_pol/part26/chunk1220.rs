//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1220/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1220<F: Float>(t20789: F, t20946: F, t20956: F, t20983: F, t3: F, t1281: F, t1670: F, t1904: F, t20105: F, t20107: F, t20109: F, t20111: F, t20115: F, t20118: F, t20121: F, t20123: F, t20127: F, t20130: F, t4556: F, t4559: F, t548: F, t6067: F, t6552: F) -> (F, F, F, F) {
    let t20985 = t20789 + t20946 + t20956 + t20983;
    let t20986 = t3 * t20985;
    let t20997 = param_d * t20985;
    let t21007 = 3.0 * t1281 * t6552 + 3.0 * t1670 * t6067 + 6.0 * t1904 * t4556 + 3.0 * t1904 * t4559 + t20997 * t548 + t20105 + t20107 + t20109 + t20111 + t20115 + t20118 + t20121 + t20123 + t20127 + t20130;
    (t20985, t20986, t20997, t21007)
}
