//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2289/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2289<F: Float>(t17611: F, t6755: F, t1933: F, t1934: F, t5836: F, t17659: F, t6765: F, t1597: F, t17178: F, t17183: F, t18036: F, t1920: F, t23437: F, t23529: F, t2987: F, t4509: F, t5857: F, t5869: F, t5909: F, t6735: F, t7573: F, t83016: F, t83220: F, t88503: F, t88517: F) -> F {
    let t99687 = t6755 * t17611;
    let t99692 = t1933 * t1934 * t5836;
    let t99707 = t6765 * t17659;
    let t99709 = -t83220 * t5909 / F::new(216.0) - t23437 * t5869 / F::new(288.0) + t99687 / F::new(2304.0) + t83016 * t18036 / F::new(1152.0) - t88503 + t88517 - F::cast_from(0.10093189023535097714e-3_f64) * t99692 * t6735 - t1920 * t2987 * t17183 / F::new(144.0) + t1920 * t4509 * t17178 / F::new(216.0) - F::cast_from(0.20186378047070195428e-3_f64) * t1933 * t7573 * t1597 * t6735 - t23529 * t5857 / F::new(432.0) + t99707 / F::new(3456.0);
    t99709
}
