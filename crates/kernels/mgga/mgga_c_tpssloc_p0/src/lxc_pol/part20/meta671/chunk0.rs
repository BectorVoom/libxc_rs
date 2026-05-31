//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2521/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2521<F: Float>(t11270: F, t4740: F, t11274: F, t1657: F, t11278: F, t1671: F, t43954: F, t11180: F, t4782: F, t14914: F, t3259: F, t1254: F, t15834: F, t3640: F, t4700: F, t50816: F, t50818: F, t50821: F, t51111: F, t51113: F) -> (F, F, F, F, F, F) {
    let t51119 = F::cast_from(1.0_f64) * t4740 * t11270;
    let t51120 = t1657 * t11274;
    let t51122 = F::cast_from(0.51726012919273400301e3_f64) * t51120 * t11278;
    let t51124 = F::cast_from(1.0_f64) * t43954 * t1671;
    let t51126 = F::cast_from(3.0_f64) * t11180 * t4782;
    let t51128 = F::cast_from(3.0_f64) * t3259 * t14914;
    let t51129 = -F::cast_from(3.0_f64) * t1254 * t15834 * t3640 * t4700 - t50816 - t50818 - t50821 - t51111 - t51113 + t51119 + t51122 + t51124 + t51126 + t51128;
    (t51119, t51122, t51124, t51126, t51128, t51129)
}
