//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2567/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2567<F: Float>(t1254: F, t3633: F, t1157: F, t1164: F, t14829: F, t3375: F, t14966: F, t3378: F, t15823: F, t225: F, t15800: F, t15808: F) -> (F, F, F, F, F, F) {
    let t51906 = t1254 * t3633;
    let t51913 = F::cast_from(0.35089341735807877242e1_f64) * t1164 * t3375 * t14829 * t1157;
    let t51916 = F::cast_from(0.10526802520742363173e2_f64) * t1164 * t14966 * t3378;
    let t51925 = t15823 * t225;
    let t51928 = t15800 * t225;
    let t51937 = t15808 * t225;
    (t51906, t51913, t51916, t51925, t51928, t51937)
}
