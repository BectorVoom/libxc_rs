//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1096/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1096<F: Float>(t102192: F, t102194: F, t102215: F, t102217: F, t102219: F, t102221: F, t102248: F, t106816: F, t2032: F, t26954: F, t27979: F, t7782: F, t91996: F, t96443: F, t102163: F, t102168: F, t102187: F, t102303: F, t106758: F, t106800: F, t106804: F, t1860: F, t2031: F, t23963: F, t26016: F, t27937: F, t28935: F, t7428: F, t84280: F, t90137: F, t92003: F, t96473: F) -> (F, F) {
    let t108743 = -2.0 * t106816 * t2032 - 2.0 * t27979 * t7782 + 80.0 / 3.0 * t102192 + 40.0 / 3.0 * t102194 + 16.0 / 3.0 * t102215 + 32.0 / 3.0 * t102217 + 80.0 / 3.0 * t102219 + 32.0 / 3.0 * t102221 - 80.0 * t102248 + 88.0 / 9.0 * t91996 + 20.0 * t96443 * t26954;
    let t108763 = 10.0 * t96473 * t26954 + 20.0 * t26016 * t102163 + 10.0 * t26016 * t102168 + 30.0 * t23963 * t106758 - 60.0 * t90137 * t102187 + t106804 * t2032 / 3.0 + t27937 * t7782 + t7428 * t28935 + t1860 * t2031 * t106800 / 3.0 + 88.0 / 9.0 * t92003 - t84280 - 8.0 / 3.0 * t102303;
    (t108743, t108763)
}
