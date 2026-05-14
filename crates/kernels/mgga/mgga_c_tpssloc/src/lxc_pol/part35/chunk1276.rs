//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1276/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1276<F: Float>(t104735: F, t104787: F, t106804: F, t2110: F, t26016: F, t27298: F, t27937: F, t27979: F, t29475: F, t29478: F, t29481: F, t7428: F, t7975: F, t7978: F, t96473: F, t104740: F, t104749: F, t104818: F, t106758: F, t106800: F, t106813: F, t106816: F, t106842: F, t106849: F, t1409: F, t1860: F, t1864: F, t20217: F, t20234: F, t20245: F, t2109: F, t21510: F, t24498: F, t24514: F, t27356: F, t27956: F, t29474: F, t5392: F, t5398: F, t56: F, t67: F, t7246: F, t7251: F, t7445: F, t7974: F, t83803: F, t85539: F, t90137: F, t96157: F, t96443: F) -> (F, F) {
    let t108939 = -t106804 * t2110 / 6.0 - t27937 * t7975 / 2.0 - t27937 * t7978 / 2.0 - t7428 * t29475 / 2.0 - t7428 * t29478 - t7428 * t29481 / 2.0 + t27979 * t7975 + t27979 * t7978 - 5.0 * t96473 * t27298 - 10.0 * t26016 * t104787 - 10.0 * t26016 * t104735;
    let t108983 = -5.0 * t26016 * t104740 - 15.0 * t24514 * t106758 + 30.0 * t90137 * t104749 - 10.0 * t96443 * t27298 - t1860 * (-1232.0 / 27.0 * t20245 * t56 - 220.0 / 9.0 * t104818 * t1409 - 20.0 / 9.0 * t96157 * t5392 + 20.0 / 3.0 * t27356 * t5398 + 5.0 / 108.0 * t85539 * t20234 + 5.0 / 6.0 * t24498 * t21510 - 5.0 / 6.0 * t7251 * t20217 + t83803) * t67 * t1864 / 6.0 - t1860 * t29474 * t7445 / 2.0 - t1860 * t7974 * t27956 / 2.0 - t1860 * t2109 * t106800 / 6.0 + t106816 * t2110 + 5.0 / 2.0 * t7246 * t106813 + 5.0 / 2.0 * t7246 * t106842 + 5.0 / 6.0 * t7246 * t106849;
    (t108939, t108983)
}
