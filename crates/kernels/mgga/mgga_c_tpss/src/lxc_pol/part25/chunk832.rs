//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 832/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk832<F: Float>(t509: F, t5935: F, t1270: F, t1845: F, t5757: F, t1163: F, t118: F, t1273: F, t1760: F, t1796: F, t1800: F, t1830: F, t1834: F, t1846: F, t2056: F, t3499: F, t485: F, t544: F, t5706: F, t5799: F, t5801: F, t5809: F, t5816: F, t5820: F, t5895: F, t5905: F, t5910: F, t624: F, t626: F, t646: F) -> (F, F, F, F) {
    let t5936 = t509 * t5935;
    let t5937 = t5936 * t1270;
    let t5939 = t1845 * t5757;
    let t5941 = -t1163 * t1796 - t118 * t5895 + t1273 * t1834 + F::new(3.0) * t1760 * t5910 + t1760 * t5937 - t1760 * t5939 - F::new(2.0) * t1800 * t2056 - F::new(2.0) * t1800 * t3499 - t1830 * t624 + t1846 * t5706 - t485 * t5799 + t544 * t5905 - F::new(2.0) * t5801 * t646 - F::new(2.0) * t5809 * t626 - F::new(2.0) * t5816 * t626 - F::new(2.0) * t5820 * t626;
    (t5936, t5937, t5939, t5941)
}
